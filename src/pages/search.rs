use crate::components::PageHead;
use crate::routes::Route;
use crate::utils::knowledge_graph::{self, KnowledgeGraph};
use crate::utils::search::{highlight_matches, SearchEngine, SearchResult, Suggestion};
use dioxus::prelude::*;

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
    let _ = history.replace_state_with_url(&wasm_bindgen::JsValue::NULL, "", Some(&new_url));
}

/// 从 URL 中读取初始查询参数
fn get_initial_query() -> String {
    let Some(window) = web_sys::window() else {
        return String::new();
    };
    let Ok(search) = window.location().search() else {
        return String::new();
    };
    let Ok(params) = web_sys::UrlSearchParams::new_with_str(&search) else {
        return String::new();
    };
    params.get("q").unwrap_or_default()
}

/// 搜索页一次性需要的全部数据。
struct SearchData {
    engine: SearchEngine,
    graph: Option<std::rc::Rc<KnowledgeGraph>>,
}

/// 索引 + 语义 + RAKE + 知识图谱；底层走 cache 的文本缓存。
async fn load_search_data() -> Option<SearchData> {
    let index = crate::utils::cache::fetch_text("/static/search-index.json").await?;
    let mut engine = SearchEngine::new();
    engine.load(&index).ok()?;

    if let Some(semantic) = crate::utils::cache::fetch_text("/static/semantic.json").await {
        engine.load_semantic(&semantic).ok();
    }
    if let Some(rake) = crate::utils::cache::fetch_text("/static/rake-keywords.json").await {
        engine.load_rake(&rake).ok();
    }

    let graph = knowledge_graph::load_graph().await;
    Some(SearchData { engine, graph })
}

#[component]
pub fn Search() -> Element {
    // 搜索数据按需加载：只有用户真正输入（或 URL 自带 q）后才开始加载。
    let mut activated = use_signal(|| !get_initial_query().is_empty());
    let search_data = use_resource(move || {
        let enabled = activated();
        async move {
            if !enabled {
                return None;
            }
            load_search_data().await
        }
    });

    let mut query = use_signal(get_initial_query);
    let mut results = use_signal(Vec::<SearchResult>::new);
    let mut suggestions = use_signal(Vec::<Suggestion>::new);
    let mut show_suggestions = use_signal(|| false);
    // 标记用户刚点击了建议（非响应式使用：effect 只读不写，避免二次触发）
    let mut suggestion_clicked = use_signal(|| false);
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
            if query() != q {
                return;
            }
            if search_data.read().is_none() {
                let mut waited = 0;
                while search_data.read().is_none() && waited < 50 {
                    delay(100).await;
                    waited += 1;
                    if query() != q {
                        return;
                    }
                }
            }
            let data = search_data.read();
            if let Some(Some(data)) = &*data {
                results.set(data.engine.search(&q));
            }
        });
    });

    // 自动补全
    use_effect(move || {
        let q = query();
        if q.is_empty() || q.is_empty() {
            suggestions.set(Vec::new());
            show_suggestions.set(false);
            return;
        }
        // 用户刚点击了建议 → 不弹窗（标志由 oninput 自动清零）
        if *suggestion_clicked.read() {
            show_suggestions.set(false);
            return;
        }
        spawn(async move {
            delay(150).await;
            if query() != q {
                return;
            }
            let data = search_data.read();
            let Some(Some(data)) = &*data else {
                return;
            };
            let ac = data.engine.autocomplete(&q);
            if !ac.is_empty() {
                suggestions.set(ac);
                show_suggestions.set(true);
            } else {
                show_suggestions.set(false);
            }
        });
    });

    let nav = use_navigator();
    let result_list = results.read().clone();
    let result_count = result_list.len();
    let query_str = query.read().clone();

    // 预计算探索式导航数据（在 RSX 外，避免 Dioxus 宏解析冲突）
    let data_guard = search_data.read();
    let data = match &*data_guard {
        Some(Some(data)) => Some(data),
        _ => None,
    };
    let is_loaded = data.is_some();
    let loading_index = activated() && !is_loaded;
    let top_slug = result_list.first().map(|r: &SearchResult| r.slug.clone());
    let explore = match (&top_slug, data.and_then(|d| d.graph.as_deref())) {
        (Some(slug), Some(kg)) => kg.articles.get(slug).map(|n| n.explore_data.clone()),
        _ => None,
    };

    // 预计算 hop2 文章 slug → title 映射
    let hop2_titles: std::collections::HashMap<String, String> = match (&explore, data) {
        (Some(e), Some(data)) => e
            .hop2
            .articles
            .iter()
            .filter_map(|s| data.engine.get_by_slug(s).map(|entry| (s.clone(), entry.title.clone())))
            .collect(),
        _ => std::collections::HashMap::new(),
    };
    drop(data_guard);

    rsx! {
        PageHead {
            title: "搜索".to_string(),
            description: "搜索文章、标签与系列文档。".to_string(),
            canonical: "/search".to_string(),
        }
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
                        oninput: move |evt| {
                            activated.set(true);
                            suggestion_clicked.set(false);
                            query.set(evt.data.value());
                        },
                        onfocus: move |_| {
                            suggestion_clicked.set(false);
                            // 重新触发自动补全 effect
                            let q = query();
                            if !q.is_empty() {
                                query.set(q);
                            }
                        },
                        onblur: move |_| {
                            spawn(async move {
                                delay(200).await;
                                show_suggestions.set(false);
                            });
                        },
                        value: "{query}"
                    }
                    if !query_str.is_empty() {
                        button {
                            class: "search-box-clear",
                            aria_label: "清除搜索",
                            onclick: move |_| { suggestion_clicked.set(false); query.set(String::new()); show_suggestions.set(false); },
                            svg {
                                width: "16", height: "16", view_box: "0 0 24 24",
                                fill: "none", stroke: "currentColor", stroke_width: "2",
                                stroke_linecap: "round", stroke_linejoin: "round",
                                path { d: "M18 6L6 18" }
                                path { d: "M6 6l12 12" }
                            }
                        }
                    }
                    // 自动补全下拉
                    if show_suggestions() {
                        div {
                            class: "search-autocomplete",
                            {
                                let items = suggestions.read().clone();
                                items.into_iter().enumerate().map(|(idx, s)| {
                                    let text = s.text;
                                    let kind_class = if s.kind == "title" { "search-ac-kind search-ac-kind--title" } else if s.kind == "keyword" { "search-ac-kind search-ac-kind--keyword" } else { "search-ac-kind" };
                                    let kind_label = if s.kind == "title" { "文章" } else if s.kind == "keyword" { "关键词" } else { "标签" };
                                    rsx! {
                                        div {
                                            class: "search-ac-item",
                                            key: "{text}-{idx}",
                                            onclick: {
                                                let text = text.clone();
                                                move |_| {
                                                    suggestion_clicked.set(true);
                                                    query.set(text.clone());
                                                    show_suggestions.set(false);
                                                }
                                            },
                                            span { class: "search-ac-text", "{text}" }
                                            span { class: "{kind_class}", "{kind_label}" }
                                        }
                                    }
                                })
                            }
                        }
                    }
                }
            }

            // 只有真正触发加载（输入/URL 带 q）时才显示 loading；
            // 刚进入空搜索页应展示「输入关键词开始搜索」。
            if loading_index && query_str.is_empty() {
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

                    // 在知识图谱中查看搜索结果
                    span { class: "search-meta-sep", "·" }
                    button {
                        class: "search-kg-link",
                        onclick: {
                            let slugs: Vec<String> = result_list.iter().take(20).map(|r| r.slug.clone()).collect();
                            move |_| {
                                let highlight = slugs.join(",");
                                // 先导航到 KG 页面，再通过 replaceState 设置查询参数
                                nav.push(Route::KnowledgeGraphView);
                                if let Some(window) = web_sys::window() {
                                    let _ = window.history().unwrap()
                                        .replace_state_with_url(
                                            &wasm_bindgen::JsValue::NULL, "",
                                            Some(&format!("/knowledge-graph?highlight={}", &highlight)),
                                        );
                                }
                            }
                        },
                        svg {
                            width: "14", height: "14", view_box: "0 0 24 24",
                            fill: "none", stroke: "currentColor", stroke_width: "2",
                            stroke_linecap: "round", stroke_linejoin: "round",
                            circle { cx: "12", cy: "12", r: "3" }
                            path { d: "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 1 1-2.83 2.83l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-4 0v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 1 1-2.83-2.83l.06-.06A1.65 1.65 0 0 0 4.68 15a1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1 0-4h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 1 1 2.83-2.83l.06.06A1.65 1.65 0 0 0 9 4.68a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 4 0v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 1 1 2.83 2.83l-.06.06A1.65 1.65 0 0 0 19.4 9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 0 4h-.09a1.65 1.65 0 0 0-1.51 1z" }
                        }
                        "在知识图谱中查看"
                    }
                }
                div { class: "search-results",
                    {result_list.iter().take(visible_count()).map(|r| {
                        let slug = r.slug.clone();
                        let title = r.title.clone();
                        let summary = r.summary.clone();
                        let tags = r.tags.clone();
                        let category = r.category.clone();
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
            if !loading_index && query_str.is_empty() {
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
