//! AI 文章摘要 — 基于 TextRank 算法的提取式摘要
//! 展示所有文章的 AI 自动摘要，支持点击跳转原文

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use crate::routes::Route;
use crate::utils::title;
use crate::utils::summaries::{self, ArticleSummary};

/// 社区颜色映射
fn category_color(cat: &str) -> &'static str {
    match cat {
        s if s.contains("Rust") => "#f74c00",
        s if s.contains("Dioxus") => "#7c3aed",
        s if s.contains("Windows") => "#0078d4",
        s if s.contains("Golang") || s.contains("Gin") => "#00add8",
        s if s.contains("前端") => "#f7df1e",
        s if s.contains("TypeScript") => "#3178c6",
        s if s.contains("Vue") => "#42b883",
        s if s.contains("Web Component") => "#e67910",
        s if s.contains("AI") || s.contains("Artificial") => "#10b981",
        s if s.contains("技术") => "#6366f1",
        s if s.contains("React") => "#61dafb",
        _ => "#9ca3af",
    }
}

#[component]
pub fn AiSummaryView() -> Element {
    title::set_page_title("AI 摘要 - 干徒");

    let mut summaries = use_signal(Vec::<ArticleSummary>::new);
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let nav = use_navigator();

    use_effect(move || {
        spawn(async move {
            match summaries::load_summaries().await {
                Some(list) => {
                    summaries.set(list);
                    loading.set(false);
                }
                None => {
                    error_msg.set(Some("无法加载 AI 摘要数据，请稍后重试。".into()));
                    loading.set(false);
                }
            }
        });
    });

    let go_post = move |slug: String| {
        nav.push(Route::BlogPostView { slug });
    };

    rsx! {
        div { class: "ai-summary-container",
            div { class: "ai-summary-header",
                h1 { "AI 文章摘要" }
                p { class: "ai-summary-subtitle",
                    "基于 TextRank 算法的提取式摘要 — 自动从每篇文章中提取核心句子"
                }
            }

            if loading() {
                div { class: "ai-summary-loading",
                    div { class: "ai-summary-spinner" }
                    p { "TextRank 正在分析文章..." }
                }
            } else if let Some(ref msg) = *error_msg.read() {
                div { class: "ai-summary-error",
                    p { "{msg}" }
                }
            } else {
                div { class: "ai-summary-list",
                    {
                        summaries.read().iter().map(|s| {
                            let color = category_color(&s.category);
                            rsx! {
                                div {
                                    key: "{s.slug}",
                                    class: "ai-summary-card",
                                    onclick: {
                                        let slug = s.slug.clone();
                                        let go_post = go_post;
                                        move |_| go_post(slug.clone())
                                    },
                                    div { class: "ai-summary-card-header",
                                        span {
                                            class: "ai-summary-category",
                                            style: "background-color: {color}20; color: {color}; border-color: {color}40",
                                            "{s.category}"
                                        }
                                        span { class: "ai-summary-date", "{s.date}" }
                                    }
                                    h3 { class: "ai-summary-title", "{s.title}" }
                                    p { class: "ai-summary-text", "{s.summary}" }
                                    div { class: "ai-summary-tags",
                                        {
                                            s.tags.iter().map(|tag| {
                                                rsx! {
                                                    span {
                                                        key: "{s.slug}-{tag}",
                                                        class: "ai-summary-tag",
                                                        "{tag}"
                                                    }
                                                }
                                            })
                                        }
                                    }
                                    div { class: "ai-summary-read-more", "阅读全文 →" }
                                }
                            }
                        })
                    }
                }
            }
        }
    }
}