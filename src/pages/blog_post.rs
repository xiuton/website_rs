use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::RuntimeBlogPost;
use crate::BLOG_POSTS;
use crate::utils::{title, code_highlight};

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let mut is_wide_mode = use_signal(|| false);
    let post = use_memo(move || {
        BLOG_POSTS.iter()
            .find(|p| p.slug == slug)
            .map(|p| RuntimeBlogPost {
                title: p.title.to_string(),
                date: p.date.to_string(),
                author: p.author.to_string(),
                tags: p.tags.iter().map(|&s| s.to_string()).collect(),
                content: p.content.to_string(),
                slug: p.slug.to_string(),
                category: p.category.to_string(),
                summary: p.summary.to_string(),
            })
    });

    // Set page title
    use_effect(move || {
        if let Some(post) = post() {
            title::set_page_title(&format!("{} - 干徒", post.title));
        }
        ()
    });

    // 在页面加载时，读取 localStorage 恢复宽屏状态
    use_effect(move || {
        if post().is_some() {
            if let Some(window) = web_sys::window() {
                if let Some(storage) = window.local_storage().ok().flatten() {
                    if let Ok(Some(wide_mode)) = storage.get_item("blog_wide_mode") {
                        is_wide_mode.set(wide_mode == "true");
                    }
                }
            }
        }
        ()
    });

    // 初始化代码高亮
    use_effect(move || {
        code_highlight::init_highlight();
    });

    // 监听文章内容变化，重新应用代码高亮
    use_effect(move || {
        let post = post();
        if post.is_some() {
            code_highlight::reapply_highlight();
        }
    });

    rsx! {
        div { class: "blog-container",
            if let Some(post) = post() {
                div { 
                    class: if is_wide_mode() { "blog-post wide-mode" } else { "blog-post" },
                    div { class: "blog-nav",
                        button { 
                            class: "back-button",
                            aria_label: "后退",
                            onclick: move |_| {
                                if let Some(window) = web_sys::window() {
                                    let _ = window.history().expect("Failed to get history").back();
                                }
                            },
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "M15.75 19.5 8.25 12l7.5-7.5" }
                            }
                        }
                        Link { 
                            to: Route::Home, 
                            class: "back-button",
                            aria_label: "首页",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "m2.25 12 8.954-8.955c.44-.439 1.152-.439 1.591 0L21.75 12M4.5 9.75v10.125c0 .621.504 1.125 1.125 1.125H9.75v-4.875c0-.621.504-1.125 1.125-1.125h2.25c.621 0 1.125.504 1.125 1.125V21h4.125c.621 0 1.125-.504 1.125-1.125V9.75M8.25 21h8.25" }
                            }
                        }
                        span { class: "nav-divider" }
                        button { 
                            class: "function-button",
                            aria_label: "回顶部",
                            onclick: move |_| {
                                let window = web_sys::window().expect("Failed to get window");
                                let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                            },
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path { d: "m4.5 15.75 7.5-7.5 7.5 7.5" }
                            }
                        }
                        button { 
                            class: if is_wide_mode() { "function-button active" } else { "function-button" },
                            aria_label: if is_wide_mode() { "关闭宽屏" } else { "宽屏模式" },
                            onclick: move |_| {
                                let new_mode = !is_wide_mode();
                                is_wide_mode.set(new_mode);
                                if let Some(window) = web_sys::window() {
                                    if let Some(storage) = window.local_storage().ok().flatten() {
                                        let _ = storage.set_item("blog_wide_mode", if new_mode { "true" } else { "false" });
                                    }
                                }
                            },
                            {
                                rsx! {
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        width: "24",
                                        height: "24",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M8 3h8m-8 18h8M4 12h16M4 12l3-3m-3 3l3 3m13-3l-3-3m3 3l-3 3" }
                                    }
                                }
                            }
                        }
                    }
                    div { class: "blog-title-wrapper",
                        div { class: "blog-title",
                            h2 { {post.title.clone()} }
                            if !post.category.is_empty() {
                                span { class: "blog-category", {post.category.clone()} }
                            }
                        }
                        div { class: "blog-meta",
                            span { class: "blog-date", {post.date.clone()} }
                            span { class: "blog-author", {post.author.clone()} }
                        }
                    }
                    div { 
                        class: "blog-content",
                        dangerous_inner_html: {
                            let html = crate::utils::markdown::markdown_to_html(&post.content);
                            let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
                            let html = html.replace("<pre><code class=\"", "<pre><code class=\"language-");
                            html
                        }
                    }
                    div { class: "blog-tags",
                        {post.tags.iter().map(|tag| rsx! {
                            span { class: "blog-tag",
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
                        })}
                    }
                }
            } else {
                div { class: "not-found",
                    h2 { "文章未找到" }
                    p { "抱歉，找不到请求的文章。" }
                }
            }
        }
    }
} 