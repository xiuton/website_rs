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
pub async fn load_markov() -> Result<MarkovData, JsValue> {
    let resp = gloo_net::http::Request::get("/markov.json")
        .send()
        .await
        .map_err(|e| JsValue::from_str(&format!("请求失败: {}", e)))?;

    let text = resp
        .text()
        .await
        .map_err(|e| JsValue::from_str(&format!("读取响应失败: {}", e)))?;

    serde_json::from_str(&text).map_err(|e| JsValue::from_str(&format!("JSON 解析失败: {}", e)))
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
    pub fn generate(
        &self,
        slug: &str,
        seed: Option<&str>,
        max_tokens: usize,
    ) -> String {
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
            // 从 starters 中随机选一个
            let idx = (js_sys::Math::random() * chain.s.len() as f64) as usize;
            let pair = &chain.s[idx.min(chain.s.len() - 1)];
            Self::split_pair(pair)
        }) {
            Some(p) => p,
            None => return String::new(),
        };

        let mut output_tokens: Vec<String> = if start_pair.is_some() {
            Vec::new()
        } else {
            vec![prev1.clone(), prev2.clone()]
        };

        for _ in 0..max_tokens {
            let key = format!("{}||{}", prev1, prev2);
            let nexts = match chain.c.get(&key) {
                Some(n) => n,
                None => break, // 无后续，停止生成
            };

            let next = self.weighted_pick(nexts);
            match next {
                Some(token) => {
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