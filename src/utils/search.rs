//! 客户端搜索引擎：加载预生成的 search-index.json + semantic.json
//! TF-IDF 余弦相似度 + 词共现语义扩展 + 自动补全

use serde::Deserialize;
use std::collections::HashMap;

// ============================================================================
// 数据结构
// ============================================================================

/// 搜索索引中的单篇文章
#[derive(Debug, Clone, Deserialize)]
pub struct SearchEntry {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub tags: Vec<String>,
    pub category: String,
    pub keywords: Vec<(String, f64)>,
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
    pub score: f64,
    pub related: Vec<String>,
}

/// 自动补全建议
#[derive(Debug, Clone)]
pub struct Suggestion {
    pub text: String,
    /// "title" | "tag"
    pub kind: &'static str,
}

/// 语义索引（词共现 + 自动补全 + tag 关联）
#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
struct SemanticData {
    cooccur: HashMap<String, HashMap<String, f64>>,
    suggestions: Vec<String>,
    tag_relations: HashMap<String, Vec<String>>,
}

impl Default for SemanticData {
    fn default() -> Self {
        Self {
            cooccur: HashMap::new(),
            suggestions: Vec::new(),
            tag_relations: HashMap::new(),
        }
    }
}

// ============================================================================
// 搜索引擎
// ============================================================================

pub struct SearchEngine {
    entries: Vec<SearchEntry>,
    slug_map: HashMap<String, usize>,
    loaded: bool,
    /// 词共现数据
    cooccur: HashMap<String, HashMap<String, f64>>,
    /// 自动补全建议
    suggestions: Vec<String>,
    /// tag → 关联 tag
    tag_relations: HashMap<String, Vec<String>>,
    /// 语义数据是否已加载
    semantic_loaded: bool,
}

impl SearchEngine {
    pub fn new() -> Self {
        Self {
            entries: Vec::new(),
            slug_map: HashMap::new(),
            loaded: false,
            cooccur: HashMap::new(),
            suggestions: Vec::new(),
            tag_relations: HashMap::new(),
            semantic_loaded: false,
        }
    }

    pub fn is_loaded(&self) -> bool {
        self.loaded
    }

    pub fn is_semantic_loaded(&self) -> bool {
        self.semantic_loaded
    }

    /// 从 JSON 字符串加载搜索索引
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

    /// 加载语义索引（词共现 + 建议）
    pub fn load_semantic(&mut self, json: &str) -> Result<(), serde_json::Error> {
        let data: SemanticData = serde_json::from_str(json)?;
        self.cooccur = data.cooccur;
        self.suggestions = data.suggestions;
        self.tag_relations = data.tag_relations;
        self.semantic_loaded = true;
        Ok(())
    }

    /// 根据 slug 获取文章
    pub fn get_by_slug(&self, slug: &str) -> Option<&SearchEntry> {
        self.slug_map.get(slug).and_then(|&i| self.entries.get(i))
    }

    /// 获取某篇文章的相关文章
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

    /// 分词
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

    /// 查询扩展：基于词共现矩阵，为查询词找到语义相关词
    /// 返回 (扩展词, 权重)，权重 < 1.0 表示降权
    fn expand_query(&self, tokens: &[String]) -> Vec<(String, f64)> {
        let mut expanded: Vec<(String, f64)> = tokens.iter().map(|t| (t.clone(), 1.0)).collect();
        let mut seen: std::collections::HashSet<String> = tokens.iter().cloned().collect();

        for token in tokens {
            if let Some(neighbors) = self.cooccur.get(token) {
                let mut related: Vec<(&String, &f64)> = neighbors.iter().collect();
                related.sort_by(|a, b| b.1.partial_cmp(a.1).unwrap_or(std::cmp::Ordering::Equal));
                // 取 top-5 相关词，权重 0.35
                for (word, strength) in related.into_iter().take(5) {
                    if seen.insert(word.clone()) {
                        // 权重 = min(共现强度 * 2.0, 0.4)，上限 0.4
                        let weight = (strength * 2.0).min(0.4);
                        expanded.push((word.clone(), weight));
                    }
                }
            }
        }
        expanded
    }

    /// 搜索：TF-IDF 余弦相似度 + 词共现语义扩展
    pub fn search(&self, query: &str) -> Vec<SearchResult> {
        if query.trim().is_empty() {
            return Vec::new();
        }

        let query_tokens = Self::tokenize(query);
        if query_tokens.is_empty() {
            return Vec::new();
        }

        // 查询扩展（语义）
        let expanded_tokens = if self.semantic_loaded {
            self.expand_query(&query_tokens)
        } else {
            query_tokens.iter().map(|t| (t.clone(), 1.0)).collect()
        };

        // 构建扩展后的查询向量
        let mut query_vec: HashMap<String, f64> = HashMap::new();
        for (token, weight) in &expanded_tokens {
            *query_vec.entry(token.clone()).or_insert(0.0) += weight;
        }

        // 对每篇文章计算余弦相似度
        let mut results: Vec<SearchResult> = self
            .entries
            .iter()
            .filter_map(|entry| {
                let doc_map: HashMap<&str, f64> = entry
                    .keywords
                    .iter()
                    .map(|(k, v)| (k.as_str(), *v))
                    .collect();

                // 点积
                let dot_product: f64 = query_vec
                    .iter()
                    .filter_map(|(qt, qv)| doc_map.get(qt.as_str()).map(|dv| qv * dv))
                    .sum();

                if dot_product <= 0.0 {
                    return None;
                }

                // 查询向量模长（归一化因子）
                let query_magnitude: f64 = query_vec
                    .values()
                    .map(|v| v * v)
                    .sum::<f64>()
                    .sqrt()
                    .max(1e-10);

                let doc_magnitude: f64 = entry
                    .keywords
                    .iter()
                    .map(|(_, v)| v * v)
                    .sum::<f64>()
                    .sqrt()
                    .max(1e-10);

                // 余弦相似度 = 点积 / (|查询向量| × |文档向量|)
                let mut score = dot_product / (query_magnitude * doc_magnitude);

                // 标题精确匹配加分
                let title_lower = entry.title.to_lowercase();
                let exact_title_bonus = query_tokens
                    .iter()
                    .filter(|qt| title_lower.contains(qt.as_str()))
                    .count() as f64
                    * 0.2;
                score += exact_title_bonus;

                if score > 0.0 {
                    Some(SearchResult {
                        slug: entry.slug.clone(),
                        title: entry.title.clone(),
                        summary: entry.summary.clone(),
                        tags: entry.tags.clone(),
                        category: entry.category.clone(),
                        score,
                        related: entry.related.clone(),
                    })
                } else {
                    None
                }
            })
            .collect();

        results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        results
    }

    // ════════════════════════════════════════════════════
    // 自动补全
    // ════════════════════════════════════════════════════

    /// 自动补全：基于输入前缀模糊匹配标题和标签
    /// 返回最多 8 条建议
    pub fn autocomplete(&self, prefix: &str) -> Vec<Suggestion> {
        if prefix.trim().is_empty() {
            return Vec::new();
        }

        let prefix_lower = prefix.to_lowercase();
        let tokens = Self::tokenize(prefix);
        if tokens.is_empty() && prefix_lower.len() < 1 {
            return Vec::new();
        }

        let mut results: Vec<Suggestion> = Vec::new();
        let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();

        for suggestion in &self.suggestions {
            let sug_lower = suggestion.to_lowercase();

            // 精确前缀匹配优先
            let _prefix_score = if sug_lower.starts_with(&prefix_lower) {
                3
            } else if sug_lower.contains(&prefix_lower) {
                1
            } else {
                // 检查 token 级别匹配
                let sug_tokens = Self::tokenize(suggestion);
                let match_count = tokens.iter().filter(|t| sug_tokens.contains(t)).count();
                if match_count > 0 {
                    1
                } else {
                    continue;
                }
            };

            if seen.insert(suggestion.clone()) {
                // 判断是标题还是标签
                let kind = if self.entries.iter().any(|e| e.title == *suggestion) {
                    "title"
                } else {
                    "tag"
                };

                results.push(Suggestion {
                    text: suggestion.clone(),
                    kind,
                });
            }

            // 达到上限
            if results.len() >= 8 {
                break;
            }
        }

        // 排序：标题优先，前缀匹配优先
        results.sort_by(|a, b| {
            let a_is_title = if a.kind == "title" { 1 } else { 0 };
            let b_is_title = if b.kind == "title" { 1 } else { 0 };
            b_is_title.cmp(&a_is_title)
                .then_with(|| {
                    let a_prefix = a.text.to_lowercase().starts_with(&prefix_lower);
                    let b_prefix = b.text.to_lowercase().starts_with(&prefix_lower);
                    b_prefix.cmp(&a_prefix)
                })
        });

        results
    }

    /// 获取 tag 的关联 tag（来自知识图谱共现）
    pub fn get_tag_relations(&self, tag: &str) -> Vec<String> {
        self.tag_relations.get(tag).cloned().unwrap_or_default()
    }

    /// 分批获取搜索结果（用于分页）
    pub fn search_with_expansion(
        &self,
        query: &str,
        offset: usize,
        limit: usize,
    ) -> (Vec<SearchResult>, usize) {
        let all = self.search(query);
        let total = all.len();
        let page: Vec<SearchResult> = all.into_iter().skip(offset).take(limit).collect();
        (page, total)
    }
}

impl Default for SearchEngine {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// 高亮工具
// ============================================================================

/// 在文本中高亮匹配的查询词
pub fn highlight_matches(text: &str, query: &str) -> String {
    let tokens = SearchEngine::tokenize(query);
    if tokens.is_empty() {
        return text.to_string();
    }

    let mut result = text.to_string();
    for token in &tokens {
        let lower_token = token.to_lowercase();
        let lower_result = result.to_lowercase();
        let mut positions: Vec<usize> = Vec::new();
        let mut start = 0;
        while let Some(pos) = lower_result[start..].find(&lower_token) {
            let abs_pos = start + pos;
            positions.push(abs_pos);
            start = abs_pos + token.len();
        }
        for &pos in positions.iter().rev() {
            let before = &result[..pos];
            let matched = &result[pos..pos + token.len()];
            let after = &result[pos + token.len()..];
            result = format!("{}<mark>{}</mark>{}", before, matched, after);
        }
    }
    result
}
