use dioxus::prelude::*;
use dioxus_router::prelude::{Link, use_route};
use web_sys::UrlSearchParams;
use std::collections::BTreeSet;

use crate::models::BlogPost;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::title;
use crate::utils::storage;
use crate::components::icons::{TagIcon, ChevronLeftIcon, ChevronRightIcon};

const ALL_CATEGORY: &str = "全部";

fn get_page_size(size_from_url: Option<usize>) -> usize {
    if let Some(size) = size_from_url {
        return size;
    }
    storage::get_blog_page_size()
}

#[component]
pub fn Home() -> Element {
    let posts = use_signal(|| BLOG_POSTS.iter().collect::<Vec<&BlogPost>>());
    let mut selected_category = use_signal(|| ALL_CATEGORY.to_string());

    title::set_page_title("首页 - 干徒");

    let mut current_page = use_signal(|| 1);
    let mut page_size = use_signal(|| 10);

    // 监听路由导航（包括同路由切换），从 URL 同步分页、每页条数和分类
    use_effect(move || {
        let _route = use_route::<Route>();
        let search = web_sys::window()
            .map(|w| w.location().search().unwrap_or_default())
            .unwrap_or_default();
        if let Ok(params) = UrlSearchParams::new_with_str(&search) {
            current_page.set(params.get("page").and_then(|v| v.parse().ok()).unwrap_or(1));
            page_size.set(get_page_size(params.get("size").and_then(|v| v.parse().ok())));
            selected_category.set(params.get("category").unwrap_or_else(|| ALL_CATEGORY.to_string()));
        }
    });

    let update_url = move |page: usize, size: usize, cat: &str| {
        if let Some(window) = web_sys::window() {
            let href = window.location().href().unwrap_or_default();
            if let Ok(url) = web_sys::Url::new(&href) {
                let search_params = url.search_params();
                search_params.set("page", &page.to_string());
                search_params.set("size", &size.to_string());
                if cat == ALL_CATEGORY {
                    search_params.delete("category");
                } else {
                    search_params.set("category", cat);
                }
                let new_url = format!("/?{}", search_params.to_string());
                if let Ok(history) = window.history() {
                    let _ = history.replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&new_url),
                    );
                }
            }
        }
    };

    use_effect(move || {
        update_url(current_page(), page_size(), &selected_category());
    });

    let categories: BTreeSet<String> = {
        let mut cats = BTreeSet::new();
        for post in posts.read().iter() {
            if !post.category.is_empty() {
                cats.insert(post.category.to_string());
            }
        }
        cats
    };

    // 合并系列文章：同一 series 的章节在列表中只显示文档入口（order 最小的一章），
    // 并记录章节数，避免首页被同一文档的所有章节刷屏
    let display_posts = use_memo(move || {
        let all = posts.read();
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut result: Vec<(&BlogPost, usize)> = Vec::new();
        for post in all.iter() {
            if post.series.is_empty() {
                result.push((post, 0));
            } else if !seen.contains(post.series) {
                seen.insert(post.series);
                let chapters: Vec<&BlogPost> = all
                    .iter()
                    .copied()
                    .filter(|p| p.series == post.series)
                    .collect();
                let entry = *chapters
                    .iter()
                    .min_by(|a, b| a.order.cmp(&b.order).then_with(|| a.date.cmp(&b.date)))
                    .unwrap();
                result.push((entry, chapters.len()));
            }
        }
        // 按日期倒序排序（文档入口使用其发布时间）
        result.sort_by(|a, b| b.0.date.cmp(&a.0.date));
        result
    });

    let filtered_posts = use_memo(move || {
        let cat = selected_category();
        if cat == ALL_CATEGORY {
            display_posts.read().clone()
        } else {
            display_posts.read().iter()
                .filter(|(p, _)| p.category == cat)
                .copied()
                .collect::<Vec<_>>()
        }
    });

    let total_pages = use_memo(move || {
        let total = filtered_posts.read().len();
        if total == 0 { 1 } else { ((total as f64) / (page_size() as f64)).ceil() as usize }
    });

    let current_page_posts = use_memo(move || {
        let posts = filtered_posts.read();
        let start = (current_page() - 1) * page_size();
        let end = start + page_size();
        posts[start.min(posts.len())..end.min(posts.len())].to_vec()
    });

    rsx! {
        div { class: "blog-container",
            div { class: "blog-list",
                if !categories.is_empty() {
                    div { class: "category-filter",
                        {
                            let all_cats = {
                                let mut c = vec![ALL_CATEGORY.to_string()];
                                c.extend(categories.iter().cloned());
                                c
                            };
                            all_cats.into_iter().map(|cat| {
                                let is_active = selected_category() == cat;
                                rsx! {
                                    button {
                                        class: if is_active { "category-btn active" } else { "category-btn" },
                                        onclick: {
                                            let cat = cat.clone();
                                            move |_| {
                                                selected_category.set(cat.clone());
                                                current_page.set(1);
                                            }
                                        },
                                        {cat.clone()}
                                    }
                                }
                            })
                        }
                    }
                }
                if posts.read().is_empty() {
                    div { class: "loading", "加载中..." }
                } else {
                    div { class: "blog-posts",
                        {current_page_posts().iter().map(|(post, chapter_count)| {
                            rsx! {
                                div { class: "blog-preview",
                                    Link { to: Route::BlogPostView { slug: post.slug.to_string() },
                                        div { class: "preview-header",
                                            h2 { class: "preview-title", {post.title} }
                                            if *chapter_count > 0 {
                                                span { class: "preview-series-badge", "系列 · 共 {chapter_count} 章" }
                                            }
                                            if !post.category.is_empty() {
                                                span { class: "preview-category", {post.category} }
                                            }
                                        }
                                        div { class: "preview-meta",
                                            span { class: "preview-date", {post.date} }
                                            span { class: "preview-author", {post.author} }
                                        }
                                        div { class: "preview-content",
                                            p { class: "preview-excerpt",
                                                {if !post.summary.is_empty() {
                                                    post.summary.to_string()
                                                } else {
                                                    crate::utils::markdown::clean_markdown_excerpt(post.content, 150)
                                                }}
                                            }
                                        }
                                        div { class: "preview-tags",
                                            {post.tags.iter().map(|tag| {
                                                rsx! {
                                                    span { class: "preview-tag",
                                                        TagIcon {}
                                                        {tag}
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                            }
                        })}
                    }
                    div { class: "pagination",
                        button {
                            class: "pagination-btn",
                            disabled: current_page() <= 1,
                            onclick: move |_| {
                                if current_page() > 1 {
                                    current_page.set(current_page() - 1);
                                }
                            },
                            ChevronLeftIcon {}
                        }
                        span { class: "pagination-info",
                            "{current_page()}/{total_pages()}"
                        }
                        button {
                            class: "pagination-btn",
                            disabled: current_page() >= total_pages(),
                            onclick: move |_| {
                                if current_page() < total_pages() {
                                    current_page.set(current_page() + 1);
                                }
                            },
                            ChevronRightIcon {}
                        }
                        select {
                            class: "page-size-select",
                            name: "page-size-select",
                            value: page_size.to_string(),
                            onchange: move |evt: Event<FormData>| {
                                let value = evt.data.value();
                                if let Ok(new_size) = value.parse::<usize>() {
                                    page_size.set(new_size);
                                    current_page.set(1);
                                    storage::set_blog_page_size(new_size);
                                }
                            },
                            option { value: "5", "5条/页" }
                            option { value: "10", "10条/页" }
                            option { value: "20", "20条/页" }
                            option { value: "50", "50条/页" }
                        }
                    }
                }
            }
        }
    }
}
