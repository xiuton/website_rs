use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::RuntimeBlogPost;
use crate::BLOG_POSTS;
use crate::utils::{title, code_highlight};
use crate::components::icons::{BackArrowIcon, HomeIcon, ScrollTopIcon, TagIcon};

fn prepare_blog_html(content: &str) -> String {
    let html = crate::utils::markdown::markdown_to_html(content);
    let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
    html.replace("<pre><code class=\"", "<pre><code class=\"language-")
}

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let mut is_wide_mode = use_signal(|| false);
    let post = use_memo(move || {
        BLOG_POSTS.iter()
            .find(|p| p.slug == slug)
            .map(|p| -> RuntimeBlogPost { RuntimeBlogPost::from_static(p) })
    });

    // Set page title
    use_effect(move || {
        if let Some(post) = post() {
            title::set_page_title(&format!("{} - 干徒", post.title));
        }
        
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
                                    if let Ok(history) = window.history() {
                                    let _ = history.back();
                                }
                                }
                            },
                            BackArrowIcon {}
                        }
                        Link { 
                            to: Route::Home, 
                            class: "back-button",
                            aria_label: "首页",
                            HomeIcon {}
                        }
                        span { class: "nav-divider" }
                        button { 
                            class: "function-button",
                            aria_label: "回顶部",
                            onclick: move |_| {
                                if let Some(window) = web_sys::window() {
                                window.scroll_to_with_x_and_y(0.0, 0.0);
                            }
                            },
                            ScrollTopIcon {}
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
                        dangerous_inner_html: prepare_blog_html(&post.content)
                    }
                    div { class: "blog-tags",
                        {post.tags.iter().map(|tag| rsx! {
                            span { class: "blog-tag",
                                TagIcon {}
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