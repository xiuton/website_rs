//! 客户端搜索引擎：加载预生成的 search-index.json，对查询词进行 TF 评分排序

use serde::Deserialize;
use std::collections::HashMap;

/// 搜索索引中的单篇文章
#[derive(Debug, Clone, Deserialize)]
pub struct SearchEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: String,
    /// 关键词 -> TF 分数 (0.0 ~ 1.0+)
    pub keywords: Vec<(String, f64)>,
}

/// 搜索结果
#[derive(Debug, Clone)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: String,
    /// 相关性分数，越高越相关
    pub score: f64,
}

/// 搜索引擎状态
pub struct SearchEngine {
    entries: Vec<SearchEntry>,
    loaded: bool,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// 从 JSON 字符串加载索引
    pub fn load(&mut self, json: &str) -> Result<(), serde_json::Error> {
        self.entries = serde_json::from_str(json)?;
        self.loaded = true;
        Ok(())
    }

    /// 分词：与 build.rs 中的 tokenize() 保持一致
    fn tokenize(text: &str) -> Vec<String> {
        let mut tokens: Vec<String> = Vec::new();
        let chars: Vec<char> = text.chars().collect();
        let mut i = 0;

        while i < chars.len() {
            let c = chars[i];

            if c.is_ascii_punctuation() || c.is_whitespace() {
                i += 1;
                continue;
            }

            if c.is_ascii_alphanumeric() {
                let mut word = String::new();
                while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                    word.push(chars[i].to_ascii_lowercase());
                    i += 1;
                }
                if word.len() >= 2 {
                    tokens.push(word);
                }
            } else {
                if i + 1 < chars.len()
                    && !chars[i + 1].is_ascii_punctuation()
                    && !chars[i + 1].is_whitespace()
                {
                    let bigram: String = [c, chars[i + 1]].iter().collect();
                    tokens.push(bigram);
                }
                tokens.push(c.to_string());
                i += 1;
            }
        }

        tokens
    }

    /// 搜索，返回按相关性排序的结果
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.trim().is_empty() {
            return Vec::new();
        }

        let query_tokens = Self::tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        let mut results: Vec<SearchResult> = self
            .entries
            .iter()
            .filter_map(|entry| {
                let mut score = 0.0;

                // 标题词权重最高
                let title_lower = entry.title.to_lowercase();
                for qt in &query_tokens {
                    if title_lower.contains(qt.as_str()) {
                        score += 2.0;
                    }
                }

                // 标签词权重高
                for tag in &entry.tags {
                    let tag_lower = tag.to_lowercase();
                    for qt in &query_tokens {
                        if tag_lower.contains(qt.as_str()) {
                            score += 1.5;
                        }
                    }
                }

                // 关键词 TF 匹配
                let kw_map: HashMap<&str, f64> = entry
                    .keywords
                    .iter()
                    .map(|(k, v)| (k.as_str(), *v))
                    .collect();

                for qt in &query_tokens {
                    if let Some(&tf) = kw_map.get(qt.as_str()) {
                        score += tf * 3.0;
                    }
                }

                // 摘要模糊匹配（低权重）
                let summary_lower = entry.summary.to_lowercase();
                for qt in &query_tokens {
                    if summary_lower.contains(qt.as_str()) {
                        score += 0.5;
                    }
                }

                if score > 0.0 {
                    Some(SearchResult {
                        slug: entry.slug.clone(),
                        title: entry.title.clone(),
                        summary: entry.summary.clone(),
                        tags: entry.tags.clone(),
                        category: entry.category.clone(),
                        score,
                    })
                } else {
                    None
                }
            })
            .collect();

        // 按分数降序
        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}