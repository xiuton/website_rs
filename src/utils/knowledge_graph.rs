//! 知识图谱数据结构与加载
//! 从 /static/knowledge-graph.json 加载 build.rs 预计算的图数据

use serde::Deserialize;
use std::collections::HashMap;

/// 整张知识图谱
#[derive(Debug, Clone, Deserialize)]
pub struct KnowledgeGraph {
    pub articles: HashMap<String, ArticleNode>,
    #[allow(dead_code)]
    pub tags: HashMap<String, serde_json::Value>,
    pub node_count: usize,
}

/// 单篇文章的知识图谱节点
#[derive(Debug, Clone, Deserialize)]
pub struct ArticleNode {
    pub pagerank: String,
    pub community: String,
    pub related: RelatedNodes,
    #[serde(rename = "explore")]
    pub explore_data: ExploreData,
}

/// RWR 计算出的关联节点，按类型分组
#[derive(Debug, Clone, Deserialize)]
pub struct RelatedNodes {
    pub articles: Vec<RelatedArticle>,
    pub tags: Vec<RelatedTag>,
    pub categories: Vec<RelatedCategory>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedArticle {
    pub slug: String,
    pub score: String,
    pub reason: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedTag {
    pub name: String,
    pub score: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RelatedCategory {
    pub name: String,
    pub score: String,
}

/// BFS 探索数据：1-hop / 2-hop，按类型分组
#[derive(Debug, Clone, Deserialize)]
pub struct ExploreData {
    pub hop1: ExploreGroup,
    pub hop2: ExploreGroup,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ExploreGroup {
    pub articles: Vec<String>,
    pub tags: Vec<String>,
    pub categories: Vec<String>,
}

/// 百分比格式："0.1234" -> "12%"
pub fn pct_str(s: &str) -> String {
    if let Ok(v) = s.parse::<f64>() {
        format!("{:.0}%", (v * 100.0).round())
    } else {
        s.to_string()
    }
}

/// 加载知识图谱
pub async fn load_graph() -> Option<KnowledgeGraph> {
    let resp = gloo_net::http::Request::get("/static/knowledge-graph.json")
        .send()
        .await
        .ok()?;
    let text = resp.text().await.ok()?;
    serde_json::from_str(&text).ok()
}