//! AI 文章摘要 — TextRank 提取式摘要
//! 数据由 build.rs 在构建时预计算，存储于 /static/summaries.json

use serde::Deserialize;

/// 单篇文章的 AI 摘要
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleSummary {
    pub slug: String,
    pub title: String,
    pub summary: String,
    pub date: String,
    pub tags: Vec<String>,
    pub category: String,
}

/// 加载 AI 摘要列表（会话级缓存）
pub async fn load_summaries() -> Option<Vec<ArticleSummary>> {
    let text = crate::utils::cache::fetch_text("/static/summaries.json").await?;
    serde_json::from_str(&text).ok()
}
