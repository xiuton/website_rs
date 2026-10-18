//! AI 文章摘要 — 基于 TextRank 算法的提取式摘要
//! 展示所有文章的 AI 自动摘要，支持点击跳转原文

use crate::components::PageHead;
use crate::routes::Route;
use crate::utils::summaries;
use dioxus::prelude::*;

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
        s if s.contains("其他") || s.contains("未分类") => "#94a3b8",
        _ => "#9ca3af",
    }
}

#[component]
pub fn AiSummaryView() -> Element {
    // use_resource 自带 loading/error 状态，不再手写 signal + effect。
    // 底层 load_summaries 仍然走 typed cache，重复进入页面只解析一次。
    let summaries_resource = use_resource(|| async { summaries::load_summaries().await });
    let nav = use_navigator();

    let go_post = move |slug: String| {
        nav.push(Route::BlogPostView { slug });
    };

    rsx! {
        PageHead {
            title: "AI 摘要".to_string(),
            description: "基于 TextRank 的提取式文章摘要。".to_string(),
            canonical: "/ai-summary".to_string(),
        }
        div { class: "ai-summary-container",
            div { class: "ai-summary-header",
                h1 { "AI 文章摘要" }
                p { class: "ai-summary-subtitle",
                    "基于 TextRank 算法的提取式摘要 — 自动从每篇文章中提取核心句子"
                }
            }

            {match &*summaries_resource.read() {
                None => rsx! {
                    div { class: "ai-summary-loading",
                        div { class: "ai-summary-spinner" }
                        p { "TextRank 正在分析文章..." }
                    }
                },
                Some(None) => rsx! {
                    div { class: "ai-summary-error",
                        p { "无法加载 AI 摘要数据，请稍后重试。" }
                    }
                },
                Some(Some(list)) => rsx! {
                    div { class: "ai-summary-list",
                        {list.iter().map(|s| {
                            let color = category_color(&s.category);
                            rsx! {
                                div {
                                    key: "{s.slug}",
                                    class: "ai-summary-card",
                                    onclick: {
                                        let slug = s.slug.clone();
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
                        })}
                    }
                },
            }}
        }
    }
}
