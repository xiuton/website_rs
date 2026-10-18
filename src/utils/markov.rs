//! 马尔可夫链文本生成器
//! 基于 trigram 模型，在 build.rs 预计算，前端直接加载使用

use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// 单篇文章的马尔可夫链数据
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleChain {
    /// 起始 token 对（"tok1||tok2"）
    pub s: Vec<String>,
    /// 转移概率链：key="prev1||prev2" → {next_token: count}
    pub c: HashMap<String, HashMap<String, u32>>,
}

/// 所有文章的马尔可夫链数据（slug → ArticleChain）
pub type MarkovData = HashMap<String, ArticleChain>;

/// 加载 markov.json
///
/// 容错说明：`trunk serve` / `trunk build` 在重建期间会短暂清空 `dist/`，
/// 此时请求会拿到 SPA 兜底页（HTML）甚至空响应，直接解析就会出现
/// `expected value at line 1 column 1` 这种难懂的报错。
/// 因此这里做一次「非 JSON 响应就重试」的处理，并给出能定位问题的错误信息。
pub async fn load_markov() -> Result<MarkovData, JsValue> {
    const URL: &str = "/static/markov.json";
    const MAX_ATTEMPTS: u32 = 3;

    let mut last_error = String::from("未知错误");

    for attempt in 1..=MAX_ATTEMPTS {
        match fetch_markov(URL).await {
            Ok(data) => return Ok(data),
            Err(LoadError::NotJson { status, content_type, preview }) => {
                last_error = format!(
                    "{} 返回的不是 JSON（status={} content-type={} 开头={:?}）。\
                     通常是静态资源缺失或构建尚未完成——请确认 dist/static/markov.json 已生成",
                    URL, status, content_type, preview
                );
            }
            Err(LoadError::Other(msg)) => {
                last_error = msg;
            }
        }

        if attempt < MAX_ATTEMPTS {
            sleep_ms(400 * attempt).await;
        }
    }

    Err(JsValue::from_str(&last_error))
}

/// 简易异步延时（避免为一次重试引入 gloo-timers 依赖）
async fn sleep_ms(ms: u32) {
    let promise = js_sys::Promise::new(&mut |resolve, _reject| {
        if let Some(window) = web_sys::window() {
            let _ = window.set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms as i32);
        }
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// 加载失败的两类原因：响应不是 JSON / 其他（请求或读取失败）
enum LoadError {
    NotJson { status: u16, content_type: String, preview: String },
    Other(String),
}

/// 发起一次请求并解析
async fn fetch_markov(url: &str) -> Result<MarkovData, LoadError> {
    let resp = gloo_net::http::Request::get(url)
        .send()
        .await
        .map_err(|e| LoadError::Other(format!("请求 {} 失败: {}", url, e)))?;

    let status = resp.status();
    let content_type = resp.headers().get("content-type").unwrap_or_default();

    let text = resp
        .text()
        .await
        .map_err(|e| LoadError::Other(format!("读取 {} 响应失败: {}", url, e)))?;

    // 先看首字符：HTML 兜底页 / 空响应都不必再走 serde，直接判定为非 JSON
    if !text.trim_start().starts_with(['{', '[']) {
        return Err(LoadError::NotJson { status, content_type, preview: text.chars().take(60).collect() });
    }

    serde_json::from_str(&text).map_err(|e| {
        LoadError::Other(format!(
            "解析 {} 失败: {}（内容长度 {} 字节，content-type={}）",
            url,
            e,
            text.len(),
            content_type
        ))
    })
}

/// 需要过滤的 markdown 语法符号
const DISALLOWED_CHARS: [char; 6] = ['#', '`', '~', '|', '>', '*'];

/// 判断 token 是否为无用符号
fn is_disallowed(token: &str) -> bool {
    if token.len() == 1 {
        let c = token.chars().next().unwrap();
        DISALLOWED_CHARS.contains(&c)
    } else {
        false
    }
}

/// 马尔可夫链生成器
#[derive(Debug, Clone)]
pub struct MarkovGenerator {
    data: MarkovData,
}

impl MarkovGenerator {
    pub fn new(data: MarkovData) -> Self {
        Self { data }
    }

    /// 使用指定文章的马尔可夫链生成续写文本
    /// - `slug`: 文章标识
    /// - `seed`: 可选的种子文本（将从中提取起始 token 对）
    /// - `max_tokens`: 最多生成的 token 数
    pub fn generate(&self, slug: &str, seed: Option<&str>, max_tokens: usize) -> String {
        let chain = match self.data.get(slug) {
            Some(c) => c,
            // 文章没有模型时从所有文章随机选一个种子
            None => return String::new(),
        };

        if chain.c.is_empty() || chain.s.is_empty() {
            return String::new();
        }

        // 确定起始 token 对
        let start_pair = if let Some(seed_text) = seed {
            self.find_seed_pair(seed_text, chain)
        } else {
            None
        };

        let (mut prev1, mut prev2) = match start_pair.clone().or_else(|| {
            // 从 starters 中随机选一个（跳过含语法符号的起始对）
            let mut attempts = 0;
            loop {
                let idx = (js_sys::Math::random() * chain.s.len() as f64) as usize;
                let pair = &chain.s[idx.min(chain.s.len() - 1)];
                if let Some(p) = Self::split_pair(pair) {
                    if !is_disallowed(&p.0) && !is_disallowed(&p.1) {
                        break Some(p);
                    }
                }
                attempts += 1;
                if attempts > 20 {
                    break Self::split_pair(pair);
                }
            }
        }) {
            Some(p) => p,
            None => return String::new(),
        };

        let mut output_tokens: Vec<String> = if start_pair.is_some() {
            Vec::new()
        } else if !is_disallowed(&prev1) && !is_disallowed(&prev2) {
            vec![prev1.clone(), prev2.clone()]
        } else {
            Vec::new()
        };

        let mut skipped = 0;
        for _ in 0..(max_tokens * 2) {
            if output_tokens.len() >= max_tokens {
                break;
            }
            let key = format!("{}||{}", prev1, prev2);
            let nexts = match chain.c.get(&key) {
                Some(n) => n,
                None => break,
            };

            let next = self.weighted_pick(nexts);
            match next {
                Some(token) => {
                    if is_disallowed(&token) {
                        skipped += 1;
                        if skipped > 20 {
                            break;
                        }
                        continue;
                    }
                    output_tokens.push(token.clone());
                    prev1 = prev2;
                    prev2 = token;
                }
                None => break,
            }
        }

        output_tokens.concat()
    }

    /// 从种子文本中找匹配的起始 token 对
    fn find_seed_pair(&self, seed: &str, chain: &ArticleChain) -> Option<(String, String)> {
        let tokens = tokenize_seed(seed);
        if tokens.len() < 2 {
            return None;
        }
        // 从末尾开始找匹配
        for i in (1..tokens.len()).rev() {
            let key = format!("{}||{}", tokens[i - 1], tokens[i]);
            if chain.c.contains_key(&key) || chain.s.contains(&key) {
                return Some((tokens[i - 1].clone(), tokens[i].clone()));
            }
        }
        None
    }

    fn split_pair(pair: &str) -> Option<(String, String)> {
        let mut parts = pair.splitn(2, "||");
        let a = parts.next()?.to_string();
        let b = parts.next()?.to_string();
        Some((a, b))
    }

    /// 加权随机选择 next token
    fn weighted_pick(&self, nexts: &HashMap<String, u32>) -> Option<String> {
        let total: u32 = nexts.values().sum();
        if total == 0 {
            return None;
        }
        let r = js_sys::Math::random() * total as f64;
        let mut acc = 0.0_f64;
        for (token, count) in nexts {
            acc += *count as f64;
            if r < acc {
                return Some(token.clone());
            }
        }
        nexts.keys().next().cloned()
    }
}

/// 对种子文本做简易分词（与 build.rs markov_tokenize 一致）
fn tokenize_seed(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
            continue;
        }
        // 跳过 markdown 语法符号（与 build.rs 一致）
        if DISALLOWED_CHARS.contains(&c) {
            i += 1;
            continue;
        }
        if c.is_ascii_alphanumeric() {
            let mut word = String::new();
            while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                word.push(chars[i].to_ascii_lowercase());
                i += 1;
            }
            tokens.push(word);
            continue;
        }
        tokens.push(c.to_string());
        i += 1;
    }

    tokens
}
