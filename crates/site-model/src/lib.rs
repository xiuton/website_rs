//! 站点数据契约。
//!
//! 构建脚本、预渲染工具和前端共用这里的类型，避免出现
//! `PostData` / `PostJson` / `BlogPost` 三套字段定义各自漂移。
//!
//! 所有通过 JSON 落盘的结构都带 `schema_version`，方便以后演进格式时
//! 在读取端做兼容或显式报错。

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// 当前 JSON 数据格式版本。
pub const SCHEMA_VERSION: u32 = 1;

/// 一篇文章的元数据（不含正文）。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostMeta {
    pub slug: String,
    pub title: String,
    pub date: String,
    pub author: String,
    pub category: String,
    pub tags: Vec<String>,
    pub summary: String,
    pub series: String,
    pub order: i32,
    pub sort_key: String,
    pub catalog: String,
    pub filename: String,
    pub series_summary: String,
}

/// `static/posts-meta.json`：预渲染与工具使用的文章列表。
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PostsMeta {
    pub schema_version: u32,
    pub posts: Vec<PostMeta>,
}

impl PostsMeta {
    pub fn new(posts: Vec<PostMeta>) -> Self {
        Self { schema_version: SCHEMA_VERSION, posts }
    }
}

/// 单篇文章的 markov 转移表。
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ArticleChain {
    /// 起始 token 对（token 字典下标）。
    #[serde(default)]
    pub s: Vec<[u32; 2]>,
    /// 转移表：`[[prev1, prev2], [[next, count], ...]]`。
    #[serde(default)]
    pub c: Vec<([u32; 2], Vec<[u32; 2]>)>,
}

/// `static/markov.json`。
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct MarkovModel {
    #[serde(default)]
    pub schema_version: u32,
    #[serde(default)]
    pub tokens: Vec<String>,
    #[serde(default)]
    pub articles: HashMap<String, ArticleChain>,
}

impl MarkovModel {
    pub fn new(tokens: Vec<String>, articles: HashMap<String, ArticleChain>) -> Self {
        Self { schema_version: SCHEMA_VERSION, tokens, articles }
    }
}

/// 生产环境存在的导航链接（顺序即展示顺序）。
///
/// 前端导航栏与预渲染工具共用，避免两边维护两份链接。
pub const PUBLIC_NAV_LINKS: &[(&str, &str)] = &[
    ("/", "首页"),
    ("/about", "关于"),
    ("/tags", "书签"),
    ("/search", "搜索"),
    ("/knowledge-graph", "图谱"),
    ("/ai-summary", "AI摘要"),
    ("/dev", "开发"),
];

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_post() -> PostMeta {
        PostMeta {
            slug: "20240101".into(),
            title: "标题".into(),
            date: "2024-01-01".into(),
            author: "干徒".into(),
            category: "Rust".into(),
            tags: vec!["rust".into(), "web".into()],
            summary: "摘要".into(),
            series: String::new(),
            order: 0,
            sort_key: "0001".into(),
            catalog: String::new(),
            filename: "20240101".into(),
            series_summary: String::new(),
        }
    }

    #[test]
    fn posts_meta_roundtrip_keeps_schema() {
        let meta = PostsMeta::new(vec![sample_post()]);
        let json = serde_json::to_string(&meta).expect("serialize posts meta");
        let parsed: PostsMeta = serde_json::from_str(&json).expect("deserialize posts meta");
        assert_eq!(parsed.schema_version, SCHEMA_VERSION);
        assert_eq!(parsed.posts.len(), 1);
        assert_eq!(parsed.posts[0].title, "标题");
    }

    #[test]
    fn markov_roundtrip_keeps_transitions() {
        let article = ArticleChain { s: vec![[0, 1]], c: vec![([0, 1], vec![[2, 3]])] };
        let model =
            MarkovModel::new(vec!["a".into(), "b".into(), "c".into()], HashMap::from([("slug".into(), article)]));
        let json = serde_json::to_string(&model).expect("serialize markov");
        let parsed: MarkovModel = serde_json::from_str(&json).expect("deserialize markov");
        assert_eq!(parsed.schema_version, SCHEMA_VERSION);
        assert_eq!(parsed.articles["slug"].c[0].1[0], [2, 3]);
    }
}
