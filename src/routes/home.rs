use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use wasm_bindgen_futures::spawn_local;
use web_sys::UrlSearchParams;

use crate::models::RuntimeBlogPost;
use crate::routes::Route;
use crate::BLOG_POSTS;

#[component]
pub fn Home() -> Element {
    let posts = use_signal(|| Vec::<RuntimeBlogPost>::new());
    
    // 获取当前 URL 的查询参数
    let location = web_sys::window()
        .unwrap()
        .location();
    let search = location.search().unwrap_or_default();
    let query_params: UrlSearchParams = UrlSearchParams::new_with_str(&search).unwrap();
    
    // 从 URL 参数中获取页码和每页数量
    let page_from_url = query_params.get("page")
        .and_then(|v| v.parse::<usize>().ok());
    let size_from_url = query_params.get("size")
        .and_then(|v| v.parse::<usize>().ok());

    let mut current_page = use_signal(|| page_from_url.unwrap_or(1));
    let mut page_size = use_signal(|| {
        // 优先使用 URL 参数
        if let Some(size) = size_from_url {
            return size;
        }
        // 其次使用 localStorage 保存的值
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                if let Ok(Some(size)) = storage.get_item("blog_page_size") {
                    if let Ok(size) = size.parse::<usize>() {
                        return size;
                    }
                }
            }
        }
        10  // 默认值
    });

    let total_pages = use_memo(move || {
        let total = posts.read().len();
        ((total as f64) / (page_size() as f64)).ceil() as usize
    });

    // 更新 URL 的函数
    let update_url = move |page: usize, size: usize| {
        if let Some(window) = web_sys::window() {
            if let Ok(url) = web_sys::Url::new(&window.location().href().unwrap()) {
                let search_params = url.search_params();
                search_params.set("page", &page.to_string());
                search_params.set("size", &size.to_string());
                let new_url = format!("/?{}", search_params.to_string());
                let _ = window.history().unwrap()
                    .replace_state_with_url(
                        &wasm_bindgen::JsValue::NULL,
                        "",
                        Some(&new_url),
                    );
            }
        }
    };

    // 监听页码和每页数量变化，更新 URL
    use_effect(move || {
        update_url(current_page(), page_size());
    });

    // 加载博客文章列表
    use_effect(move || {
        let mut posts = posts.clone();
        spawn_local(async move {
            // 使用构建脚本生成的文章列表
            let loaded_posts = BLOG_POSTS.iter().map(|post| RuntimeBlogPost {
                    title: post.title.to_string(),
                    date: post.date.to_string(),
                    author: post.author.to_string(),
                    tags: post.tags.iter().map(|&s| s.to_string()).collect(),
                    content: post.content.to_string(),
                    slug: post.slug.to_string(),
                }).collect();
            posts.set(loaded_posts);
        });
    });

    // 获取当前页的文章
    let current_page_posts = use_memo(move || {
        let posts = posts.read();
        let start = (current_page() - 1) * page_size();
        let end = start + page_size();
        posts[start.min(posts.len())..end.min(posts.len())].to_vec()
    });

    rsx! {
        div { class: "blog-container",
            div { class: "blog-list",
                if posts.read().is_empty() {
                    div { class: "loading", "加载中..." }
                } else {
                    div { class: "blog-posts",
                        {current_page_posts().iter().map(|post| {
                            let post = post.clone();
                            rsx! {
                                div { class: "blog-preview",
                                    Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                        h2 { class: "preview-title", {post.title.clone()} }
                                        div { class: "preview-meta",
                                            span { class: "preview-date", {post.date.clone()} }
                                            span { class: "preview-author", {post.author.clone()} }
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
                    // 分页控制
                    div { class: "pagination",
                        // 上一页按钮
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
                        // 页码显示
                        span { class: "pagination-info",
                            "{current_page()}/{total_pages()}"
                        }
                        // 下一页按钮
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
                        // 每页显示数量选择
                        select {
                            class: "page-size-select",
                            name: "page-size-select",
                            value: page_size.to_string(),
                            onchange: move |evt: Event<FormData>| {
                                let value = evt.data.value();
                                if let Ok(new_size) = value.parse::<usize>() {
                                    page_size.set(new_size);
                                    current_page.set(1);  // 重置到第一页
                                    // 保存选择到 localStorage
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