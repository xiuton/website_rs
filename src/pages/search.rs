use dioxus::prelude::*;
use crate::utils::title;
use crate::utils::search::{SearchEngine, SearchResult};
use dioxus_router::prelude::use_navigator;
use crate::routes::Route;

#[component]
pub fn Search() -> Element {
    title::set_page_title("搜索 - 干徒");

    let index_loaded = use_signal(|| false);
    let engine_ref = use_signal(|| SearchEngine::new());

    // 加载索引
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

    let mut query = use_signal(|| String::new());
    let mut results = use_signal(|| Vec::<SearchResult>::new());

    let mut do_search = move |q: &str| {
        let engine = engine_ref.read();
        results.set(engine.search(q));
    };

    let nav = use_navigator();

    let result_list = results.read().clone();
    let result_count = result_list.len();
    let has_query = !query.read().is_empty();
    let is_loaded = index_loaded();

    rsx! {
        div { class: "search-container",
            div { class: "search-header",
                h1 { class: "search-title", "搜索文章" }
                p { class: "search-subtitle", "输入关键词，搜索博客中的所有文章" }
            }

            div { class: "search-input-wrapper",
                input {
                    class: "search-input",
                    r#type: "search",
                    placeholder: "搜索文章...",
                    aria_label: "搜索文章",
                    autofocus: true,
                    oninput: move |evt| {
                        let q = evt.data.value();
                        query.set(q.clone());
                        do_search(&q);
                    },
                    value: "{query}"
                }
                if !query.read().is_empty() {
                    button {
                        class: "search-clear",
                        aria_label: "清除搜索",
                        onclick: move |_| {
                            query.set(String::new());
                            results.set(Vec::new());
                        },
                        "✕"
                    }
                }
            }

            if !is_loaded {
                div { class: "search-status", "正在加载搜索索引..." }
            } else if has_query {
                div { class: "search-result-count",
                    if result_count > 0 {
                        "找到 {result_count} 篇相关文章"
                    } else {
                        "未找到相关文章，试试其他关键词"
                    }
                }
                div { class: "search-results",
                    {result_list.iter().enumerate().map(|(_i, r)| {
                        let slug = r.slug.clone();
                        let title = r.title.clone();
                        let summary = r.summary.clone();
                        let tags = r.tags.clone();
                        let category = r.category.clone();
                        let nav = nav.clone();
                        let key = slug.clone();
                        rsx! {
                            div {
                                class: "search-result-item",
                                key: "{key}",
                                onclick: move |_| { nav.push(Route::BlogPostView { slug: slug.clone() }); },
                                div { class: "search-result-title", "{title}" }
                                if !summary.is_empty() {
                                    div { class: "search-result-summary", "{summary}" }
                                }
                                div { class: "search-result-meta",
                                    if !tags.is_empty() {
                                        span { class: "search-result-tags",
                                            for tag in &tags {
                                                span { class: "search-result-tag", "#{tag}" }
                                            }
                                        }
                                    }
                                    span { class: "search-result-category", "{category}" }
                                }
                            }
                        }
                    })}
                }
            } else {
                div { class: "search-empty",
                    div { class: "search-empty-icon", "🔍" }
                    p { "输入关键词开始搜索" }
                    p { class: "search-hint", "支持中英文关键词，例如：Rust、WASM、性能优化" }
                }
            }
        }
    }
}