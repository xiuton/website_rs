use dioxus::prelude::*;
use dioxus_router::prelude::{Link, use_route};
use web_sys::UrlSearchParams;
use std::collections::BTreeSet;

use crate::models::RuntimeBlogPost;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::title;
use crate::components::icons::{TagIcon, ChevronLeftIcon, ChevronRightIcon};

const ALL_CATEGORY: &str = "全部";

fn get_page_size(size_from_url: Option<usize>) -> usize {
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
}

#[component]
pub fn Home() -> Element {
    let mut posts = use_signal(Vec::<RuntimeBlogPost>::new);
    let mut selected_category = use_signal(|| ALL_CATEGORY.to_string());

    use_effect(move || {
        title::set_page_title("首页 - 干徒");
        
    });

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

    use_effect(move || {
        let loaded_posts = BLOG_POSTS.iter().map(RuntimeBlogPost::from_static).collect();
        posts.set(loaded_posts);
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
        if cat == ALL_CATEGORY {
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
                                                        TagIcon {}
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
