//! 马尔可夫链文本生成器
//! 基于 trigram 模型，在 build.rs 预计算成紧凑格式，前端直接加载使用。
//!
//! 模型格式（`static/markov.json`）：
//! ```json
//! {"tokens":["a","b"],"articles":{"slug":{"s":[[0,1]],"c":[[[0,1],[[2,3]]]]}}}
//! ```
//! `tokens` 是全局字典，转移只存 `u32` 下标，避免重复保存字符串。

use serde::Deserialize;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;

/// 单篇文章的马尔可夫链数据
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleChain {
    /// 起始 token 对（token 字典下标）
    #[serde(default)]
    pub s: Vec<[u32; 2]>,
    /// 转移表：`[[prev1, prev2], [[next, count], ...]]`
    #[serde(default)]
    pub c: Vec<([u32; 2], Vec<[u32; 2]>)>,
}

/// 整个 markov 模型：token 字典 + 各文章转移表
#[derive(Debug, Clone, Deserialize)]
pub struct MarkovModel {
    #[serde(default)]
    pub tokens: Vec<String>,
    #[serde(default)]
    pub articles: HashMap<String, ArticleChain>,
}

/// 所有文章的马尔可夫链数据（slug → ArticleChain），保留旧名字方便调用方使用
pub type MarkovData = MarkovModel;

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
    tokens: Vec<String>,
    token_ids: HashMap<String, u32>,
    articles: HashMap<String, HashMap<[u32; 2], Vec<[u32; 2]>>>,
    starters: HashMap<String, Vec<[u32; 2]>>,
}

impl MarkovGenerator {
    pub fn new(model: MarkovData) -> Self {
        let token_ids: HashMap<String, u32> = model
            .tokens
            .iter()
            .enumerate()
            .map(|(i, token)| (token.clone(), i as u32))
            .collect();

        let mut articles = HashMap::with_capacity(model.articles.len());
        let mut starters = HashMap::with_capacity(model.articles.len());
        for (slug, chain) in model.articles {
            let mut map = HashMap::with_capacity(chain.c.len());
            for (key, nexts) in chain.c {
                map.insert(key, nexts);
            }
            starters.insert(slug.clone(), chain.s);
            articles.insert(slug, map);
        }

        Self { tokens: model.tokens, token_ids, articles, starters }
    }

    /// 使用指定文章的马尔可夫链生成续写文本
    /// - `slug`: 文章标识
    /// - `seed`: 可选的种子文本（将从中提取起始 token 对）
    /// - `max_tokens`: 最多生成的 token 数
    pub fn generate(&self, slug: &str, seed: Option<&str>, max_tokens: usize) -> String {
        let Some(chain) = self.articles.get(slug) else {
            return String::new();
        };
        let Some(starters) = self.starters.get(slug) else {
            return String::new();
        };
        if chain.is_empty() || starters.is_empty() {
            return String::new();
        }

        let start_pair = seed
            .and_then(|text| self.find_seed_pair(text, chain))
            .or_else(|| self.pick_starter(starters));
        let Some((mut prev1, mut prev2)) = start_pair else {
            return String::new();
        };

        // 与旧实现一致：从起始 token 对开始拼接
        let mut output_tokens: Vec<u32> = vec![prev1, prev2];

        let mut skipped = 0;
        for _ in 0..(max_tokens * 2) {
            if output_tokens.len() >= max_tokens {
                break;
            }
            let Some(nexts) = chain.get(&[prev1, prev2]) else {
                break;
            };
            let Some(next) = self.weighted_pick(nexts) else {
                break;
            };

            if is_disallowed(self.token_str(next)) {
                skipped += 1;
                if skipped > 20 {
                    break;
                }
                continue;
            }

            output_tokens.push(next);
            prev1 = prev2;
            prev2 = next;
        }

        output_tokens.iter().map(|id| self.token_str(*id)).collect::<String>()
    }

    fn token_str(&self, id: u32) -> &str {
        self.tokens.get(id as usize).map(String::as_str).unwrap_or("")
    }

    /// 从种子文本中找匹配的起始 token 对
    fn find_seed_pair(&self, seed: &str, chain: &HashMap<[u32; 2], Vec<[u32; 2]>>) -> Option<(u32, u32)> {
        let tokens = tokenize_seed(seed);
        if tokens.len() < 2 {
            return None;
        }
        // 从末尾开始找匹配
        for i in (1..tokens.len()).rev() {
            let (Some(&a), Some(&b)) = (self.token_ids.get(&tokens[i - 1]), self.token_ids.get(&tokens[i])) else {
                continue;
            };
            if chain.contains_key(&[a, b]) {
                return Some((a, b));
            }
        }
        None
    }

    fn pick_starter(&self, starters: &[[u32; 2]]) -> Option<(u32, u32)> {
        if starters.is_empty() {
            return None;
        }
        // 尝试若干次，尽量跳过以 markdown 符号开头的起始对
        for _ in 0..20 {
            let idx = (js_sys::Math::random() * starters.len() as f64) as usize;
            let [a, b] = starters[idx.min(starters.len() - 1)];
            if !is_disallowed(self.token_str(a)) && !is_disallowed(self.token_str(b)) {
                return Some((a, b));
            }
        }
        let [a, b] = starters[0];
        Some((a, b))
    }

    /// 加权随机选择 next token
    fn weighted_pick(&self, nexts: &[[u32; 2]]) -> Option<u32> {
        let total: u32 = nexts.iter().map(|pair| pair[1]).sum();
        if total == 0 {
            return None;
        }
        let r = js_sys::Math::random() * total as f64;
        let mut acc = 0.0_f64;
        for pair in nexts {
            acc += pair[1] as f64;
            if r < acc {
                return Some(pair[0]);
            }
        }
        nexts.first().map(|pair| pair[0])
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
