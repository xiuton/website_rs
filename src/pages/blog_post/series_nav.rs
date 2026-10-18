//! 系列章节导航（多章节文档底部的折叠目录 + 上下篇）。
//!
//! 独立成组件是 Dioxus 0.7 的要求：hook 必须在组件顶层无条件调用，
//! 不能再像 0.6 那样写在 `Option::map` 的闭包里。

use dioxus::prelude::*;

use crate::models::BlogPost;
use crate::routes::Route;

#[component]
pub(super) fn SeriesNav(series: String, chapters: Vec<&'static BlogPost>, idx: usize) -> Element {
    let total = chapters.len();
    let prev = if idx > 0 { Some(chapters[idx - 1]) } else { None };
    let next = chapters.get(idx + 1);
    let series_slug = {
        let entry = chapters[0];
        if entry.catalog.is_empty() {
            entry.slug
        } else {
            entry.catalog
        }
    };

    // 折叠状态由信号驱动：始终显式写出 open 属性（默认折叠），
    // 原生点击通过 ontoggle 同步回信号
    let mut toc_open = use_signal(|| false);

    rsx! {
        // 章节列表默认折叠。
        // 结构：动作行（目录页）→ 可折叠的章节列表 → 上下篇导航。
        // 动作行在 <details> 之前，始终可见，也不会误触发折叠。
        div { class: "series-nav",
            div { class: "series-nav-actions",
                // 唯一的可点控件：跳转到系列目录页（胶囊按钮）
                Link {
                    to: Route::SeriesView { slug: series_slug.to_string() },
                    class: "series-nav-btn",
                    span { "目录页" }
                }
            }
            details {
                class: "series-nav-details",
                open: toc_open(),
                ontoggle: move |_| {
                    let next = !toc_open();
                    toc_open.set(next);
                },
                summary { class: "series-nav-summary",
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        width: "16", height: "16",
                        fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        class: "section-icon",
                        path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                        path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                    }
                    div { class: "series-nav-heading",
                        span { class: "series-nav-title", "{series}" }
                        span { class: "series-nav-count", "共 {total} 章" }
                    }
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        width: "14", height: "14",
                        fill: "none", stroke: "currentColor",
                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                        class: "series-nav-chevron",
                        path { d: "m6 9 6 6 6-6" }
                    }
                }
                div { class: "series-nav-list",
                    {chapters.iter().enumerate().map(|(i, ch)| {
                        let to = Route::BlogPostView { slug: ch.slug.to_string() };
                        let is_current = i == idx;
                        rsx! {
                            Link {
                                to,
                                class: if is_current { "series-nav-item active" } else { "series-nav-item" },
                                span { class: "series-nav-item-index", "{i + 1}" }
                                span { class: "series-nav-item-title", "{ch.title}" }
                                if is_current {
                                    span { class: "series-nav-item-now", "本篇" }
                                }
                            }
                        }
                    })}
                }
            }
            div { class: "series-nav-pager",
                if let Some(prev) = prev {
                    Link {
                        to: Route::BlogPostView { slug: prev.slug.to_string() },
                        class: "series-nav-page prev",
                        span { class: "series-nav-page-label", "上一篇" }
                        span { class: "series-nav-page-title", "{prev.title}" }
                    }
                } else {
                    span { class: "series-nav-page disabled", "已经是第一章" }
                }
                if let Some(next) = next {
                    Link {
                        to: Route::BlogPostView { slug: next.slug.to_string() },
                        class: "series-nav-page next",
                        span { class: "series-nav-page-label", "下一篇" }
                        span { class: "series-nav-page-title", "{next.title}" }
                    }
                } else {
                    span { class: "series-nav-page disabled", "已经是最后一章" }
                }
            }
        }
    }
}
