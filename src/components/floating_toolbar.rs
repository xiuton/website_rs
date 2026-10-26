//! 全站悬浮工具栏。
//!
//! 文章页原本有一套固定的 `.blog-nav`，现在提升为全局组件：
//! - 通用动作：后退、首页、回到顶部
//! - 页面动作：文章宽屏/系列目录、首页搜索、搜索/书签聚焦输入框、系列页章节目录
//!
//! 宽屏状态用 `GlobalSignal` 共享，文章页只负责读取 class，不再自己渲染工具栏。

use crate::components::icons::{
    BackArrowIcon, BookmarkIcon, HomeIcon, MoonIcon, ScrollBottomIcon, ScrollTopIcon, SearchIcon, SunIcon, TocIcon,
    WideModeIcon,
};
use crate::routes::Route;
use crate::utils::{dark_mode, storage};
use crate::BLOG_POSTS;
use dioxus::prelude::*;

/// 文章宽屏阅读状态；工具栏与 BlogPostView 共享。
pub static BLOG_WIDE_MODE: GlobalSignal<bool> = Signal::global(|| false);

/// 工具栏可见性配置用的页面分类。
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ToolbarPage {
    Home,
    About,
    Dev,
    Tags,
    Search,
    Rss,
    KnowledgeGraph,
    AiSummary,
    BlogPost,
    Series,
    CircleGenerator,
    Test,
    NotFound,
}

impl ToolbarPage {
    fn from_route(route: &Route) -> Self {
        match route {
            Route::Home => Self::Home,
            Route::About => Self::About,
            Route::Dev => Self::Dev,
            Route::Tags => Self::Tags,
            Route::Search => Self::Search,
            Route::Rss => Self::Rss,
            Route::KnowledgeGraphView => Self::KnowledgeGraph,
            Route::AiSummaryView => Self::AiSummary,
            Route::BlogPostView { .. } => Self::BlogPost,
            Route::SeriesView { .. } => Self::Series,
            Route::CircleGenerator => Self::CircleGenerator,
            #[cfg(feature = "dev-pages")]
            Route::Playground => Self::Dev,
            #[cfg(feature = "dev-pages")]
            Route::Test => Self::Test,
            Route::NotFound { .. } => Self::NotFound,
        }
    }
}

/// 在这里配置不显示悬浮工具栏的页面。
///
/// - 图谱页有完整的画布/缩放控件，不需要再叠加全局工具栏
/// - 测试页是组件测试场，工具栏没有实际用途
const HIDE_TOOLBAR_ON: &[ToolbarPage] = &[ToolbarPage::KnowledgeGraph, ToolbarPage::Test, ToolbarPage::CircleGenerator];

fn should_show_toolbar(route: &Route) -> bool {
    !HIDE_TOOLBAR_ON.contains(&ToolbarPage::from_route(route))
}

#[derive(Clone, Debug, PartialEq, serde::Deserialize)]
struct TocEntry {
    id: String,
    text: String,
    level: u8,
}

/// 从正文 DOM 读取 h2/h3，生成目录。
fn read_toc() -> Vec<TocEntry> {
    let script = r#"
        JSON.stringify(
            [...document.querySelectorAll('.blog-content h2[id], .blog-content h3[id]')].map(h => ({
                id: h.id,
                text: (h.dataset.headingContent || h.textContent || '').trim(),
                level: Number(h.tagName.slice(1))
            }))
        )
    "#;
    let raw = js_sys::eval(script)
        .ok()
        .and_then(|value| value.as_string())
        .unwrap_or_default();
    serde_json::from_str(&raw).unwrap_or_default()
}

fn scroll_to_id(id: &str) {
    let script = format!("document.getElementById({id:?})?.scrollIntoView({{behavior:'smooth',block:'start'}});");
    let _ = js_sys::eval(&script);
}

fn scroll_to_top() {
    if let Some(window) = web_sys::window() {
        window.scroll_to_with_x_and_y(0.0, 0.0);
    }
}

fn scroll_to_bottom() {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };

    let root_height = document.document_element().map(|element| element.scroll_height()).unwrap_or(0);
    let body_height = document.body().map(|body| body.scroll_height()).unwrap_or(0);
    window.scroll_to_with_x_and_y(0.0, root_height.max(body_height) as f64);
}

fn go_back() {
    if let Some(window) = web_sys::window() {
        if let Ok(history) = window.history() {
            let _ = history.back();
        }
    }
}

fn focus_selector(selector: &str) {
    let script = format!("document.querySelector({selector:?})?.focus({{preventScroll:false}});");
    let _ = js_sys::eval(&script);
}

fn scroll_to_selector(selector: &str) {
    let script = format!("document.querySelector({selector:?})?.scrollIntoView({{behavior:'smooth',block:'start'}});");
    let _ = js_sys::eval(&script);
}

#[component]
pub fn FloatingToolbar() -> Element {
    let route = use_route::<Route>();
    let is_home = matches!(&route, Route::Home);
    let is_search = matches!(&route, Route::Search);
    let is_tags = matches!(&route, Route::Tags);
    let is_series = matches!(&route, Route::SeriesView { .. });
    let mut is_dark = dark_mode::use_dark_mode();

    let post = match &route {
        Route::BlogPostView { slug } => BLOG_POSTS.iter().find(|post| post.slug == slug.trim_end_matches('/')),
        _ => None,
    };

    // 系列目录 slug：优先 catalog，否则用章节 slug（与 SeriesNav 的规则一致）
    let series_slug = post.filter(|post| !post.series.is_empty()).map(|post| {
        let entry = if post.catalog.is_empty() {
            post
        } else {
            BLOG_POSTS
                .iter()
                .find(|candidate| candidate.catalog == post.catalog && !candidate.series.is_empty())
                .unwrap_or(post)
        };
        if entry.catalog.is_empty() {
            entry.slug.to_string()
        } else {
            entry.catalog.to_string()
        }
    });

    // 只在客户端初始化一次，避免 SSR 访问 localStorage
    #[cfg(target_arch = "wasm32")]
    use_effect(move || {
        *BLOG_WIDE_MODE.write() = storage::get_blog_wide_mode();
    });

    let wide = BLOG_WIDE_MODE();
    let mut toc_open = use_signal(|| false);
    let mut toc_items = use_signal(Vec::<TocEntry>::new);
    let mut toc_active = use_signal(|| None::<String>);
    let toc_entries = toc_items.read().clone();

    // 所有 hooks 都必须在提前返回之前执行
    if !should_show_toolbar(&route) {
        return rsx! {};
    }

    rsx! {
        if post.is_some() && toc_open() {
            div {
                class: "toc-backdrop",
                aria_hidden: "true",
                onclick: move |_| toc_open.set(false),
            }
        }
        nav {
            class: if post.is_some() && wide { "floating-toolbar wide" } else { "floating-toolbar" },
            aria_label: "页面工具栏",

            if !is_home {
                button {
                    class: "tool-btn",
                    r#type: "button",
                    aria_label: "后退",
                    onclick: move |_| go_back(),
                    BackArrowIcon {}
                }
                Link {
                    to: Route::Home,
                    class: "tool-btn",
                    aria_label: "首页",
                    HomeIcon {}
                }
            }

            // 文章页：目录 + 宽屏阅读 + 系列目录
            if let Some(post) = post {
                button {
                    class: if toc_open() { "tool-btn active" } else { "tool-btn" },
                    r#type: "button",
                    aria_label: "文章目录",
                    onclick: move |_| {
                        let items = read_toc();
                        toc_items.set(items);
                        toc_open.set(!toc_open());
                    },
                    TocIcon {}
                }
                if !post.series.is_empty() {
                    if let Some(slug) = series_slug.clone() {
                        Link {
                            to: Route::SeriesView { slug },
                            class: "tool-btn",
                            aria_label: "系列目录",
                            BookmarkIcon {}
                        }
                    }
                }
                button {
                    class: if wide { "tool-btn active" } else { "tool-btn" },
                    r#type: "button",
                    aria_label: if wide { "关闭宽屏" } else { "宽屏模式" },
                    onclick: move |_| {
                        let next = !BLOG_WIDE_MODE();
                        *BLOG_WIDE_MODE.write() = next;
                        storage::set_blog_wide_mode(next);
                    },
                    WideModeIcon {}
                }
            }

            // 首页：搜索入口
            if is_home {
                Link {
                    to: Route::Search,
                    class: "tool-btn",
                    aria_label: "搜索文章",
                    SearchIcon {}
                }
            }

            // 搜索页：聚焦搜索框
            if is_search {
                button {
                    class: "tool-btn",
                    r#type: "button",
                    aria_label: "聚焦搜索框",
                    onclick: move |_| focus_selector(".search-box-input"),
                    SearchIcon {}
                }
            }

            // 书签页：聚焦搜索框
            if is_tags {
                button {
                    class: "tool-btn",
                    r#type: "button",
                    aria_label: "聚焦书签搜索",
                    onclick: move |_| focus_selector(".bookmarks-container .search-input"),
                    SearchIcon {}
                }
            }

            // 系列页：跳到章节目录
            if is_series {
                button {
                    class: "tool-btn",
                    r#type: "button",
                    aria_label: "章节目录",
                    onclick: move |_| scroll_to_selector(".series-chapter-list"),
                    BookmarkIcon {}
                }
            }

            if !is_home {
                span { class: "tool-divider" }
            }
            button {
                class: "tool-btn theme-toggle",
                r#type: "button",
                aria_label: if is_dark() {
                    "切换到浅色模式"
                } else {
                    "切换到深色模式"
                },
                onclick: move |event: Event<MouseData>| dark_mode::toggle_theme(&mut is_dark, &event),
                if is_dark() {
                    SunIcon {}
                } else {
                    MoonIcon {}
                }
            }
            button {
                class: "tool-btn",
                r#type: "button",
                aria_label: "回到顶部",
                onclick: move |_| scroll_to_top(),
                ScrollTopIcon {}
            }

            button {
                class: "tool-btn",
                r#type: "button",
                aria_label: "回到底部",
                onclick: move |_| scroll_to_bottom(),
                ScrollBottomIcon {}
            }

            if post.is_some() && toc_open() {
                aside {
                    class: "toc-panel",
                    aria_label: "文章标题目录",
                    div { class: "toc-panel-header",
                        span { "文章目录" }
                        button {
                            class: "toc-close",
                            r#type: "button",
                            aria_label: "关闭目录",
                            onclick: move |_| toc_open.set(false),
                            "✕"
                        }
                    }
                    if toc_entries.is_empty() {
                        p { class: "toc-empty", "本文暂无标题目录" }
                    } else {
                        nav { class: "toc-list", aria_label: "文章标题导航",
                            {toc_entries.iter().map(|item| {
                                let item_id = item.id.clone();
                                let active = toc_active() == Some(item_id.clone());
                                rsx! {
                                    button {
                                        key: "{item.id}",
                                        class: if active {
                                            "toc-item toc-level-{item.level} active"
                                        } else {
                                            "toc-item toc-level-{item.level}"
                                        },
                                        r#type: "button",
                                        aria_label: item.text.clone(),
                                        onclick: move |_| {
                                            scroll_to_id(&item_id);
                                            toc_active.set(Some(item_id.clone()));
                                        },
                                        "{item.text}"
                                    }
                                }
                            })}
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn toolbar_is_hidden_on_configured_pages() {
        assert!(!should_show_toolbar(&Route::KnowledgeGraphView));
        assert!(!should_show_toolbar(&Route::Test));
        assert!(should_show_toolbar(&Route::Home));
        assert!(should_show_toolbar(&Route::BlogPostView { slug: "demo".into() }));
    }
}
