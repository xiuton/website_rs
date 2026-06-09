use dioxus::prelude::*;
use crate::utils::title;
use crate::utils::search::{SearchEngine, SearchResult, highlight_matches};
use crate::utils::knowledge_graph::{self, KnowledgeGraph};
use dioxus_router::prelude::use_navigator;
use crate::routes::Route;

/// 创建一个延迟指定毫秒数的 Promise
async fn delay(ms: i32) {
    let promise = js_sys::Promise::new(&mut |resolve, _| {
        let _ = web_sys::window()
            .unwrap()
            .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, ms);
    });
    let _ = wasm_bindgen_futures::JsFuture::from(promise).await;
}

/// 更新浏览器地址栏的 ?q= 参数（不触发导航）
fn update_url(query: &str) {
    let Some(window) = web_sys::window() else { return };
    let Ok(history) = window.history() else { return };
    let new_url = if query.is_empty() {
        "/search".to_string()
    } else {
        format!("/search?q={}", js_sys::encode_uri_component(query))
    };
    let _ = history.replace_state_with_url(
        &wasm_bindgen::JsValue::NULL,
        "",
        Some(&new_url),
    );
}

/// 从 URL 中读取初始查询参数
fn get_initial_query() -> String {
    let Some(window) = web_sys::window() else { return String::new() };
    let Ok(search) = window.location().search() else { return String::new() };
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else { return String::new() };
    params.get("q").unwrap_or_default()
}

#[component]
pub fn Search() -> Element {
    title::set_page_title("搜索 - 干徒");

    let index_loaded = use_signal(|| false);
    let engine_ref = use_signal(|| SearchEngine::new());
    let mut kg_data = use_signal(|| Option::<KnowledgeGraph>::None);

    // 一次性加载搜索索引
    use_effect(move || {
        spawn(async move {
            let mut engine_ref = engine_ref;
            let mut index_loaded = index_loaded;
            let resp = gloo_net::http::Request::get("/static/search-index.json")
                .send()
                .await;
            if let Ok(resp) = resp {
                if let Ok(text) = resp.text().await {
                    let mut e = SearchEngine::new();
                    if e.load(&text).is_ok() {
                        engine_ref.set(e);
                        index_loaded.set(true);
                    }
                }
            }
        });
    });

    // 加载知识图谱
    use_effect(move || {
        spawn(async move {
            if let Some(graph) = knowledge_graph::load_graph().await {
                kg_data.set(Some(graph));
            }
        });
    });

    let mut query = use_signal(|| get_initial_query());
    let mut results = use_signal(|| Vec::<SearchResult>::new());
    const PAGE_SIZE: usize = 15;
    let mut visible_count = use_signal(|| PAGE_SIZE);

    // 防抖搜索
    use_effect(move || {
        let q = query();
        if q.is_empty() {
            results.set(Vec::new());
            visible_count.set(PAGE_SIZE);
            update_url("");
            return;
        }
        update_url(&q);
        visible_count.set(PAGE_SIZE);
        spawn(async move {
            delay(300).await;
            if query() != q { return; }
            if !index_loaded() {
                let mut waited = 0;
                while !index_loaded() && waited < 50 {
                    delay(100).await;
                    waited += 1;
                    if query() != q { return; }
                }
            }
            if !index_loaded() { return; }
            let engine = engine_ref.read();
            results.set(engine.search(&q));
        });
    });

    let nav = use_navigator();
    let result_list = results.read().clone();
    let result_count = result_list.len();
    let is_loaded = index_loaded();
    let query_str = query.read().clone();

    // 预计算探索式导航数据（在 RSX 外，避免 Dioxus 宏解析冲突）
    let top_slug = result_list.first().map(|r: &SearchResult| r.slug.clone());
    let kg_ref = kg_data.read();
    let explore = match (&top_slug, kg_ref.as_ref()) {
        (Some(slug), Some(kg)) => kg.articles.get(slug).map(|n| n.explore_data.clone()),
        _ => None,
    };
    drop(kg_ref);

    // 预计算 hop2 文章 slug → title 映射
    let engine = engine_ref.read();
    let hop2_titles: std::collections::HashMap<String, String> = explore
        .as_ref()
        .map(|e| {
            e.hop2.articles.iter().filter_map(|s| {
                engine.get_by_slug(s).map(|entry| (s.clone(), entry.title.clone()))
            }).collect()
        })
        .unwrap_or_default();
    drop(engine);

    rsx! {
        div { class: "search-container",
            // ── 搜索栏（核心焦点）─
            div { class: "search-hero",
                h1 { class: "search-hero-title", "探索文章" }
                div { class: "search-box",
                    // 搜索图标
                    svg { class: "search-box-icon",
                        width: "18", height: "18", view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor", stroke_width: "2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "M21 21l-4.35-4.35" }
                    }
                    input {
                        class: "search-box-input",
                        r#type: "text",
                        placeholder: "输入关键词搜索文章...",
                        aria_label: "搜索文章",
                        autofocus: true,
                        oninput: move |evt| { query.set(evt.data.value()); },
                        value: "{query}"
                    }
                    if !query_str.is_empty() {
                        button {
                            class: "search-box-clear",
                            aria_label: "清除搜索",
                            onclick: move |_| { query.set(String::new()); },
                            svg {
                                width: "16", height: "16", view_box: "0 0 24 24",
                                fill: "none", stroke: "currentColor", stroke_width: "2",
                                stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M18 6L6 18" }
                                path { d: "M6 6l12 12" }
                            }
                        }
                    }
                }
            }

            // ── 加载中 ──
            if !is_loaded && query_str.is_empty() {
                div { class: "search-status",
                    div { class: "search-status-spinner" }
                    span { "正在加载搜索索引..." }
                }
            }

            // ── 有搜索结果 ──
            if is_loaded && !query_str.is_empty() && result_count > 0 {
                div { class: "search-meta",
                    "共找到 "
                    span { class: "search-meta-count", "{result_count}" }
                    " 篇相关文章"
                }
                div { class: "search-results",
                    {result_list.iter().take(visible_count()).map(|r| {
                        let slug = r.slug.clone();
                        let title = r.title.clone();
                        let summary = r.summary.clone();
                        let tags = r.tags.clone();
                        let category = r.category.clone();
                        let nav = nav.clone();
                        let key = slug.clone();
                        let q = query_str.clone();
                        let score = r.score;

                        let title_html = highlight_matches(&title, &q);
                        let summary_html = if !summary.is_empty() {
                            highlight_matches(&summary, &q)
                        } else {
                            String::new()
                        };
                        // 相关性百分比（cap 到 100%）
                        let score_pct = ((score * 100.0).min(100.0)) as u32;

                        rsx! {
                            div {
                                class: "search-result-card",
                                key: "{key}",
                                onclick: move |_| { nav.push(Route::BlogPostView { slug: slug.clone() }); },
                                // 相关性指示条
                                div {
                                    class: "search-result-bar",
                                    style: "width: {score_pct}%"
                                },
                                div { class: "search-result-body",
                                    div {
                                        class: "search-result-title",
                                        dangerous_inner_html: "{title_html}"
                                    }
                                    if !summary_html.is_empty() {
                                        div {
                                            class: "search-result-summary",
                                            dangerous_inner_html: "{summary_html}"
                                        }
                                    }
                                    div { class: "search-result-foot",
                                        div { class: "search-result-tags",
                                            for tag in &tags {
                                                span { class: "search-result-tag", "#{tag}" }
                                            }
                                        }
                                        div { class: "search-result-right",
                                            div { class: "search-result-relevance",
                                                div { class: "search-result-relevance-dot" }
                                                "{score_pct}%"
                                            }
                                            if !category.is_empty() {
                                                span { class: "search-result-category", "{category}" }
                                            }
                                            svg {
                                                width: "14", height: "14", view_box: "0 0 24 24",
                                                fill: "none", stroke: "currentColor", stroke_width: "2",
                                                stroke_linecap: "round", stroke_linejoin: "round",
                                                class: "search-result-arrow",
                                                path { d: "M5 12h14" }
                                                path { d: "M12 5l7 7-7 7" }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    })}
                }

                // 加载更多按钮
                if visible_count() < result_count {
                    div { class: "search-load-more",
                        button {
                            class: "search-load-more-btn",
                            onclick: move |_| { visible_count.set(visible_count() + PAGE_SIZE); },
                            "加载更多（已显示 {visible_count()} / 共 {result_count} 篇）"
                        }
                    }
                }

                // 探索式导航：基于预计算的 explore 数据
                if let Some(ref data) = explore {
                    if !data.hop1.tags.is_empty() || !data.hop2.articles.is_empty() {
                        div { class: "search-explore",
                                div { class: "search-explore-header",
                                    svg {
                                        width: "16", height: "16", view_box: "0 0 24 24",
                                        fill: "none", stroke: "currentColor", stroke_width: "2",
                                        stroke_linecap: "round", stroke_linejoin: "round",
                                        class: "search-explore-icon",
                                        circle { cx: "12", cy: "12", r: "3" }
                                        path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                                    }
                                    span { "探索式导航" }
                                    span { class: "search-explore-sub", "基于知识图谱的智能关联" }
                                }
                                // 标签维度
                                if !data.hop1.tags.is_empty() {
                                    div { class: "search-explore-section",
                                        div { class: "search-explore-label",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 24 24",
                                                width: "14", height: "14",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                class: "search-explore-label-icon",
                                                path { d: "M12 2H2v10l9.28 9.29a2 2 0 0 0 2.83 0l6.89-6.89a2 2 0 0 0 0-2.83L12 2z" }
                                                path { d: "M7 7h.01" }
                                            }
                                            "关联标签"
                                        }
                                        div { class: "search-explore-tags",
                                            {data.hop1.tags.iter().map(|t| {
                                                rsx! { span { class: "search-explore-tag", "{t}" } }
                                            })}
                                        }
                                    }
                                }
                                // 二度关联文章
                                if !data.hop2.articles.is_empty() {
                                    div { class: "search-explore-section",
                                        div { class: "search-explore-label",
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 24 24",
                                                width: "14", height: "14",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                class: "search-explore-label-icon",
                                                line { x1: "6", y1: "3", x2: "6", y2: "15" }
                                                circle { cx: "18", cy: "6", r: "3" }
                                                circle { cx: "6", cy: "18", r: "3" }
                                                path { d: "M18 9a9 9 0 0 1-9 9" }
                                            }
                                            "二度关联文章"
                                        }
                                        div { class: "search-explore-links",
                                            {data.hop2.articles.iter().map(|s| {
                                                let nav = nav.clone();
                                                let slug = s.clone();
                                                let title = hop2_titles.get(s).cloned().unwrap_or_else(|| s.clone());
                                                rsx! {
                                                    div {
                                                        class: "search-explore-link",
                                                        onclick: move |_| { nav.push(Route::BlogPostView { slug: slug.clone() }); },
                                                        svg {
                                                            width: "12", height: "12", view_box: "0 0 24 24",
                                                            fill: "none", stroke: "currentColor", stroke_width: "2",
                                                            stroke_linecap: "round", stroke_linejoin: "round",
                                                            path { d: "M5 12h14" }
                                                            path { d: "M12 5l7 7-7 7" }
                                                        }
                                                        "{title}"
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                                // 直达分类
                                if !data.hop1.categories.is_empty() {
                                    div { class: "search-explore-section",
                                        div { class: "search-explore-label", "相关分类" }
                                        div { class: "search-explore-tags",
                                            {data.hop1.categories.iter().map(|c| {
                                                rsx! { span { class: "search-explore-tag search-explore-tag--cat", "{c}" } }
                                            })}
                                        }
                                    }
                                }
                            }
                        }
                    }
            }

            // ── 搜索无结果 ──
            if is_loaded && !query_str.is_empty() && result_count == 0 {
                div { class: "search-empty",
                    svg {
                        width: "56", height: "56", view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor", stroke_width: "1.2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        class: "search-empty-icon",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "M21 21l-4.35-4.35" }
                        path { d: "M8 11h6" }
                    }
                    p { class: "search-empty-text", "未找到与「{query_str}」相关的文章" }
                    p { class: "search-empty-hint", "试试其他关键词，例如：Rust、WASM、JavaScript" }
                }
            }

            // ── 初始状态 ──
            if is_loaded && query_str.is_empty() {
                div { class: "search-empty search-empty--initial",
                    svg {
                        width: "56", height: "56", view_box: "0 0 24 24",
                        fill: "none", stroke: "currentColor", stroke_width: "1.2",
                        stroke_linecap: "round", stroke_linejoin: "round",
                        class: "search-empty-icon",
                        circle { cx: "11", cy: "11", r: "8" }
                        path { d: "M21 21l-4.35-4.35" }
                    }
                    p { class: "search-empty-text", "输入关键词开始搜索" }
                    p { class: "search-empty-hint", "支持中英文关键词，例如：Rust、WASM、性能优化" }
                }
            }
        }
    }
}