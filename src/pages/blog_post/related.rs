use dioxus::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

use crate::routes::Route;
use crate::BLOG_POSTS;

/// 嵌入数据结构
#[derive(Debug, Clone, Deserialize)]
struct RelatedEntry {
    slug: String,
    score: f64,
}

/// 相关文章推荐组件（基于 TF-IDF 余弦相似度）
#[component]
pub(super) fn RelatedArticles(slug: String, visible: Signal<bool>) -> Element {
    let mut loaded = use_signal(|| false);
    let mut related = use_signal(Vec::<(String, f64)>::new);

    use_effect(move || {
        if !visible() || *loaded.read() {
            return;
        }
        let s = slug.clone();
        spawn(async move {
            if let Some(text) = crate::utils::cache::fetch_text("/static/embeddings.json").await {
                if let Ok(data) = serde_json::from_str::<HashMap<String, Vec<RelatedEntry>>>(&text) {
                    if let Some(entries) = data.get(&s) {
                        let mut items: Vec<(String, f64)> =
                            entries.iter().take(5).map(|e| (e.slug.clone(), e.score)).collect();
                        items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                        related.set(items);
                    }
                }
            }
            loaded.set(true);
        });
    });

    let items = related.read().clone();

    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "related-articles-semantic",
            h3 { class: "related-articles-semantic-title",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    width: "18", height: "18",
                    fill: "none", stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                    path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                }
                "相关文章"
            }
            div { class: "related-articles-semantic-list",
                {items.iter().map(|(related_slug, score)| {
                    let title = BLOG_POSTS.iter()
                        .find(|p| p.slug == *related_slug)
                        .map(|p| p.title)
                        .unwrap_or(related_slug);
                    let pct = (score * 100.0) as u8;
                    let to = Route::BlogPostView { slug: related_slug.clone() };
                    rsx! {
                        Link {
                            to,
                            class: "related-articles-semantic-item",
                            span { class: "related-articles-semantic-item-title", "{title}" }
                            span { class: "related-articles-semantic-item-score", "{pct}%" }
                        }
                    }
                })}
            }
        }
    }
}
