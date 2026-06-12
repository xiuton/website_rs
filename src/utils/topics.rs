//! LDA 主题模型数据加载
//! 数据由 build.rs 在构建时预计算，存储于 /static/lda-topics.json

use serde::Deserialize;
use std::collections::HashMap;

/// 完整的 LDA 主题数据
#[derive(Debug, Clone, Deserialize)]
pub struct LdaData {
    /// 主题 ID → [(词, 概率)]
    pub topics: HashMap<String, Vec<(String, f64)>>,
    /// 主题名称列表
    pub topic_names: Vec<String>,
    /// slug → { topic_id: probability }
    pub articles: HashMap<String, HashMap<String, f64>>,
}

/// 单篇文章的主题分布
#[derive(Debug, Clone)]
pub struct ArticleTopic {
    pub topic_id: String,
    pub topic_name: String,
    pub probability: f64,
}

impl LdaData {
    /// 获取某篇文章的主题分布（按概率降序，取前 N 个）
    pub fn article_topics(&self, slug: &str, top_n: usize) -> Vec<ArticleTopic> {
        let Some(topics) = self.articles.get(slug) else {
            return Vec::new();
        };

        let mut result: Vec<ArticleTopic> = topics
            .iter()
            .map(|(tid, prob)| {
                let name = tid
                    .parse::<usize>()
                    .ok()
                    .and_then(|i| self.topic_names.get(i))
                    .cloned()
                    .unwrap_or_else(|| format!("主题 {}", tid));
                ArticleTopic {
                    topic_id: tid.clone(),
                    topic_name: name,
                    probability: *prob,
                }
            })
            .collect();

        result.sort_by(|a, b| b.probability.partial_cmp(&a.probability).unwrap_or(std::cmp::Ordering::Equal));
        result.truncate(top_n);
        result
    }

    /// 获取话题名称
    pub fn topic_name(&self, topic_id: &str) -> String {
        topic_id
            .parse::<usize>()
            .ok()
            .and_then(|i| self.topic_names.get(i))
            .cloned()
            .unwrap_or_else(|| format!("主题 {}", topic_id))
    }
}

/// 加载 LDA 主题数据
pub async fn load_lda() -> Option<LdaData> {
    let resp = gloo_net::http::Request::get("/static/lda-topics.json")
        .send()
        .await
        .ok()?;
    let text = resp.text().await.ok()?;
    serde_json::from_str(&text).ok()
}