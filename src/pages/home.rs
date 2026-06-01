use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;
use web_sys::UrlSearchParams;
use std::collections::BTreeSet;

use crate::models::RuntimeBlogPost;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::title;

#[component]
pub fn Home() -> Element {
    let posts = use_signal(|| Vec::<RuntimeBlogPost>::new());
    let mut selected_category = use_signal(|| "全部".to_string());

    use_effect(move || {
        title::set_page_title("首页 - 干徒");
        ()
    });

    let location = web_sys::window()
        .expect("Failed to get window")
        .location();
    let search = location.search().unwrap_or_default();
    let query_params: UrlSearchParams = UrlSearchParams::new_with_str(&search).expect("Failed to create URLSearchParams");

    let page_from_url = query_params.get("page")
        .and_then(|v| v.parse::<usize>().ok());
    let size_from_url = query_params.get("size")
        .and_then(|v| v.parse::<usize>().ok());
    let cat_from_url = query_params.get("category");

    let mut current_page = use_signal(|| page_from_url.unwrap_or(1));
    let mut page_size = use_signal(|| {
        if let Some(size) = size_from_url {
            return size;
        }
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(size)) = storage.get_item("blog_page_size") {
                    if let Ok(size) = size.parse::<usize>() {
                        return size;
                    }
                }
            }
        }
        10
    });

    use_effect(move || {
        if let Some(cat) = cat_from_url.as_ref() {
            selected_category.set(cat.clone());
        }
    });

    let update_url = move |page: usize, size: usize, cat: &str| {
        if let Some(window) = web_sys::window() {
            if let Ok(url) = web_sys::Url::new(&window.location().href().expect("Failed to get href")) {
                let search_params = url.search_params();
                search_params.set("page", &page.to_string());
                search_params.set("size", &size.to_string());
                if cat == "全部" {
                    search_params.delete("category");
                } else {
                    search_params.set("category", cat);
                }
                let new_url = format!("/?{}", search_params.to_string());
                let _ = window.history().expect("Failed to get history")
                    .replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&new_url),
                    );
            }
        }
    };

    use_effect(move || {
        update_url(current_page(), page_size(), &selected_category());
    });

    use_effect(move || {
        let mut posts = posts.clone();
        spawn_local(async move {
            let loaded_posts = BLOG_POSTS.iter().map(|post| RuntimeBlogPost {
                    title: post.title.to_string(),
                    date: post.date.to_string(),
                    author: post.author.to_string(),
                    tags: post.tags.iter().map(|&s| s.to_string()).collect(),
                    content: post.content.to_string(),
                    slug: post.slug.to_string(),
                    category: post.category.to_string(),
                    summary: post.summary.to_string(),
                }).collect();
            posts.set(loaded_posts);
        });
    });

    let categories: BTreeSet<String> = {
        let mut cats = BTreeSet::new();
        for post in posts.read().iter() {
            if !post.category.is_empty() {
                cats.insert(post.category.clone());
            }
        }
        cats
    };

    let filtered_posts = use_memo(move || {
        let cat = selected_category();
        if cat == "全部" {
            posts.read().clone()
        } else {
            posts.read().iter()
                .filter(|p| p.category == cat)
                .cloned()
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
                                let mut c = vec!["全部".to_string()];
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
                        {current_page_posts().iter().map(|post| {
                            let post = post.clone();
                            rsx! {
                                div { class: "blog-preview",
                                    Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                        div { class: "preview-header",
                                            h2 { class: "preview-title", {post.title.clone()} }
                                            if !post.category.is_empty() {
                                                span { class: "preview-category", {post.category.clone()} }
                                            }
                                        }
                                        div { class: "preview-meta",
                                            span { class: "preview-date", {post.date.clone()} }
                                            span { class: "preview-author", {post.author.clone()} }
                                        }
                                        div { class: "preview-content",
                                            p { class: "preview-excerpt",
                                                {if !post.summary.is_empty() {
                                                    post.summary.clone()
                                                } else {
                                                    crate::utils::markdown::clean_markdown_excerpt(&post.content, 150)
                                                }}
                                            }
                                        }
                                        div { class: "preview-tags",
                                            {post.tags.iter().map(|tag| {
                                                rsx! {
                                                    span { class: "preview-tag",
                                                        svg {
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            fill: "none",
                                                            view_box: "0 0 24 24",
                                                            stroke_width: "1.5",
                                                            stroke: "currentColor",
                                                            class: "size-6",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                d: "M5.25 8.25h15m-16.5 7.5h15m-1.8-13.5-3.9 19.5m-2.1-19.5-3.9 19.5"
                                                            }
                                                        }
                                                        {tag.clone()}
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
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                class: "size-5",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M15.75 19.5L8.25 12l7.5-7.5"
                                }
                            }
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
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                fill: "none",
                                view_box: "0 0 24 24",
                                stroke_width: "2",
                                stroke: "currentColor",
                                class: "size-5",
                                path {
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    d: "M8.25 4.5l7.5 7.5-7.5 7.5"
                                }
                            }
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
                                    if let Some(window) = web_sys::window() {
                                        if let Some(storage) = window.local_storage().ok().flatten() {
                                            let _ = storage.set_item("blog_page_size", &new_size.to_string());
                                        }
                                    }
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