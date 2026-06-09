use dioxus::prelude::*;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::{title, code_highlight, storage};
use crate::components::icons::{BackArrowIcon, HomeIcon, ScrollTopIcon, TagIcon, WideModeIcon};
use crate::utils::search::SearchEngine;
use dioxus_router::prelude::Link;

fn prepare_blog_html(content: &str) -> String {
    let html = crate::utils::markdown::markdown_to_html(content);
    let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
    let html = html.replace("<pre><code class=\"", "<pre><code class=\"language-");
    // 为所有图片注入懒加载属性
    add_lazy_loading(&html)
}

/// 为 <img> 标签添加 loading="lazy" 属性（如尚未设置）
fn add_lazy_loading(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut last_end = 0;

    for (start, _) in html.match_indices("<img ") {
        // 找到 img 标签的结束 >
        let tag_start = start;
        let tag_end = if let Some(end) = html[tag_start..].find('>') {
            tag_start + end
        } else {
            continue;
        };

        result.push_str(&html[last_end..tag_start]);

        let tag = &html[tag_start..tag_end];
        // 只有尚未包含 loading 属性的才注入
        if !tag.contains("loading=") {
            result.push_str(&html[tag_start..tag_start + 5]); // "<img "
            result.push_str("loading=\"lazy\" ");
            result.push_str(&html[tag_start + 5..tag_end]);
        } else {
            result.push_str(tag);
        }

        last_end = tag_end;
    }

    result.push_str(&html[last_end..]);
    result
}

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let slug_for_memo = slug.clone();
    let mut is_wide_mode = use_signal(|| false);
    let post = use_memo(move || {
        BLOG_POSTS.iter()
            .find(|p| p.slug == slug_for_memo)
    });

    // 加载搜索引擎以获取相关文章推荐
    let mut related_results = use_signal(|| Vec::<(String, String, String)>::new()); // (slug, title, category)

    {
        let slug = slug.clone();
        use_effect(move || {
            let slug = slug.clone();
            spawn(async move {
                if let Ok(resp) = gloo_net::http::Request::get("/static/search-index.json")
                    .send()
                    .await
                {
                    if let Ok(text) = resp.text().await {
                        let mut engine = SearchEngine::new();
                        if engine.load(&text).is_ok() {
                            let related = engine.get_related(&slug, 5);
                            let items: Vec<_> = related
                                .iter()
                                .map(|r| (r.slug.clone(), r.title.clone(), r.category.clone()))
                                .collect();
                            related_results.set(items);
                        }
                    }
                }
            });
        });
    }

    use_effect(move || {
        // 设置页面标题 + 恢复宽屏模式 + SEO meta
        if let Some(post) = post() {
            title::set_page_title(&format!("{} - 干徒", post.title));

            let description = if !post.summary.is_empty() {
                post.summary[..post.summary.len().min(160)].to_string()
            } else {
                crate::utils::markdown::clean_markdown_excerpt(post.content, 150)
            };
            title::set_seo(
                &post.title,
                &description,
                &format!("https://ganto.cn/post/{}", post.slug),
            );
        }
        if post().is_some() {
            is_wide_mode.set(storage::get_blog_wide_mode());
        }
    });

    use_effect(move || {
        code_highlight::init_highlight();
    });

    use_effect(move || {
        if post().is_some() {
            code_highlight::reapply_highlight();
        }
    });

    let article_label = post().map(|p| format!("文章：{}", p.title));

    rsx! {
        div { class: "blog-container",
            if let Some(post) = post() {
                article { 
                    class: if is_wide_mode() { "blog-post wide-mode" } else { "blog-post" },
                    aria_label: article_label.as_deref().unwrap_or("文章"),
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
                                storage::set_blog_wide_mode(new_mode);
                            },
                            {
                                rsx! { WideModeIcon {} }
                            }
                        }
                    }
                    div { class: "blog-title-wrapper",
                        div { class: "blog-title",
                            h2 { {post.title} }
                            if !post.category.is_empty() {
                                span { class: "blog-category", {post.category} }
                            }
                        }
                        div { class: "blog-meta",
                            span { class: "blog-date", {post.date} }
                            span { class: "blog-author", {post.author} }
                        }
                    }
                    div { 
                        class: "blog-content",
                        dangerous_inner_html: prepare_blog_html(post.content)
                    }
                    div { class: "blog-tags",
                        {post.tags.iter().map(|tag| rsx! {
                            span { class: "blog-tag",
                                TagIcon {}
                                {tag}
                            }
                        })}
                    }
                    // 相关文章推荐
                    {(!related_results.read().is_empty()).then(|| rsx! {
                        div { class: "related-articles",
                            h3 { class: "related-title", "相关文章" }
                            div { class: "related-list",
                                {related_results.read().iter().map(|(slug, title, category)| {
                                    let to = Route::BlogPostView { slug: slug.clone() };
                                    rsx! {
                                        Link { to, class: "related-item",
                                            span { class: "related-item-title", "{title}" }
                                            if !category.is_empty() {
                                                span { class: "related-item-category", "{category}" }
                                            }
                                        }
                                    }
                                })}
                            }
                        }
                    })}
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