//! 客户端搜索引擎：加载预生成的 search-index.json
//! 使用 TF-IDF 余弦相似度进行排序，支持语义相关性（通过预计算的文章间相似度）

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
    /// 关键词 -> TF-IDF 分数（预计算，来自 build.rs）
    pub keywords: Vec<(String, f64)>,
    /// 相关文章 slug 列表（预计算余弦相似度 top-5）
    pub related: Vec<String>,
}

/// 搜索结果
#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: String,
    /// 相关性分数（0.0 ~ 1.0+），越高越相关
    pub score: f64,
    /// 相关文章 slug 列表
    pub related: Vec<String>,
}

/// 搜索引擎状态
pub struct SearchEngine {
    entries: Vec<SearchEntry>,
    /// slug → entry 的快速查找表
    slug_map: HashMap<String, usize>,
    loaded: bool,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            slug_map: HashMap::new(),
            loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    /// 从 JSON 字符串加载索引
    pub fn load(&mut self, json: &str) -> Result<(), serde_json::Error> {
        self.entries = serde_json::from_str(json)?;
        self.slug_map = self
            .entries
            .iter()
            .enumerate()
            .map(|(i, e)| (e.slug.clone(), i))
            .collect();
        self.loaded = true;
        Ok(())
    }

    /// 根据 slug 获取文章
    pub fn get_by_slug(&self, slug: &str) -> Option<&SearchEntry> {
        self.slug_map.get(slug).and_then(|&i| self.entries.get(i))
    }

    /// 获取某篇文章的相关文章（返回 SearchResult，按相似度排序）
    pub fn get_related(&self, slug: &str, limit: usize) -> Vec<SearchResult> {
        let entry = match self.get_by_slug(slug) {
            Some(e) => e,
            None => return Vec::new(),
        };

        entry
            .related
            .iter()
            .filter_map(|related_slug| {
                self.get_by_slug(related_slug).map(|e| SearchResult {
                    slug: e.slug.clone(),
                    title: e.title.clone(),
                    summary: e.summary.clone(),
                    tags: e.tags.clone(),
                    category: e.category.clone(),
                    score: 0.0,
                    related: Vec::new(),
                })
            })
            .take(limit)
            .collect()
    }

    /// 分词：与 build.rs 中的 tokenize() 保持一致
    pub fn tokenize(text: &str) -> Vec<String> {
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

    /// 搜索：TF-IDF 余弦相似度排序
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.trim().is_empty() {
            return Vec::new();
        }

        let query_tokens = Self::tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        // 计算查询词的 TF 向量（仅非零项）
        let mut query_tf: HashMap<String, f64> = HashMap::new();
        for t in &query_tokens {
            *query_tf.entry(t.clone()).or_insert(0.0) += 1.0;
        }
        let query_len = query_tokens.len() as f64;
        for v in query_tf.values_mut() {
            *v /= query_len;
        }

        // 对每篇文章计算余弦相似度
        let mut results: Vec<SearchResult> = self
            .entries
            .iter()
            .filter_map(|entry| {
                // 构建 doc TF-IDF 查找表
                let doc_map: HashMap<&str, f64> = entry
                    .keywords
                    .iter()
                    .map(|(k, v)| (k.as_str(), *v))
                    .collect();

                // 计算点积：sum(query_tf(t) * doc_tfidf(t))
                let dot_product: f64 = query_tf
                    .iter()
                    .filter_map(|(qt, qv)| doc_map.get(qt.as_str()).map(|dv| qv * dv))
                    .sum();

                if dot_product <= 0.0 {
                    return None;
                }

                // 计算文档向量模长
                let doc_magnitude: f64 = entry
                    .keywords
                    .iter()
                    .map(|(_, v)| v * v)
                    .sum::<f64>()
                    .sqrt();

                if doc_magnitude < 1e-10 {
                    return None;
                }

                // 余弦相似度 = 点积 / (|query| * |doc|)
                // query 模长 = 1/sqrt(len) (因为 TF 归一化后查询向量模长为 1/sqrt(n))
                // 这里直接除以 doc_magnitude 并乘上 sqrt(len) 作为补偿
                // 简化：直接用 dot_product / doc_magnitude，因为查询模长对排序无影响
                let score = dot_product / doc_magnitude;

                // 标题精确匹配加分（提升精准命中）
                let title_lower = entry.title.to_lowercase();
                let exact_title_bonus = query_tokens
                    .iter()
                    .filter(|qt| title_lower.contains(qt.as_str()))
                    .count() as f64
                    * 0.2;

                let final_score = score + exact_title_bonus;

                if final_score > 0.0 {
                    Some(SearchResult {
                        slug: entry.slug.clone(),
                        title: entry.title.clone(),
                        summary: entry.summary.clone(),
                        tags: entry.tags.clone(),
                        category: entry.category.clone(),
                        score: final_score,
                        related: entry.related.clone(),
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

/// 在文本中高亮匹配的查询词（用 `<mark>` 标签包裹）
/// 返回 HTML 字符串，适合配合 `dangerous_inner_html` 使用
pub fn highlight_matches(text: &str, query: &str) -> String {
    let tokens = SearchEngine::tokenize(query);
    if tokens.is_empty() {
        return text.to_string();
    }

    let mut result = text.to_string();
    for token in &tokens {
        let lower_token = token.to_lowercase();
        // 找出所有匹配位置并替换（从后向前避免偏移问题）
        let lower_result = result.to_lowercase();
        let mut positions: Vec<usize> = Vec::new();
        let mut start = 0;
        while let Some(pos) = lower_result[start..].find(&lower_token) {
            let abs_pos = start + pos;
            positions.push(abs_pos);
            start = abs_pos + token.len();
        }
        // 从后往前替换
        for &pos in positions.iter().rev() {
            let before = &result[..pos];
            let matched = &result[pos..pos + token.len()];
            let after = &result[pos + token.len()..];
            result = format!("{}<mark>{}</mark>{}", before, matched, after);
        }
    }
    result
}