use dioxus::prelude::*;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::utils::{title, code_highlight, storage};
use crate::components::icons::{BackArrowIcon, HomeIcon, ScrollTopIcon, TagIcon, WideModeIcon};
use crate::utils::search::SearchEngine;
use crate::utils::knowledge_graph::{self, ArticleNode};
use dioxus_router::prelude::{Link, use_route};

fn prepare_blog_html(content: &str) -> String {
    let html = crate::utils::markdown::markdown_to_html(content);
    // 给无语言标记的代码块加上 plaintext
    let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
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
    // 用信号包裹 slug，通过 use_route() 监听路由变化（prop 本身不触发 effect 重新执行）
    let mut current_slug = use_signal(|| slug.clone());
    use_effect(move || {
        // use_route() 是响应式的，路由变化时 effect 会重新执行
        if let Route::BlogPostView { slug } = use_route::<Route>() {
            if current_slug() != slug {
                current_slug.set(slug);
            }
        }
    });

    let mut is_wide_mode = use_signal(|| false);

    let post = use_memo(move || {
        let s = current_slug();
        BLOG_POSTS.iter()
            .find(|p| p.slug == s)
    });

    // 加载搜索引擎以获取相关文章推荐
    let mut related_results = use_signal(|| Vec::<(String, String, String)>::new()); // (slug, title, category)

    use_effect(move || {
        let slug = current_slug();
        spawn(async move {
            // 重置相关文章
            related_results.set(Vec::new());
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

    // 加载知识图谱
    let mut article_node = use_signal(|| Option::<ArticleNode>::None);

    use_effect(move || {
        let slug = current_slug();
        spawn(async move {
            article_node.set(None);
            if let Some(graph) = knowledge_graph::load_graph().await {
                if let Some(node) = graph.articles.get(&slug) {
                    article_node.set(Some(node.clone()));
                }
            }
        });
    });

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
                            h3 { class: "related-title",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "16", height: "16",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                class: "section-icon",
                                path { d: "M14.5 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V7.5L14.5 2z" }
                                polyline { points: "14 2 14 8 20 8" }
                                line { x1: "16", y1: "13", x2: "8", y2: "13" }
                                line { x1: "16", y1: "17", x2: "8", y2: "17" }
                                line { x1: "10", y1: "9", x2: "8", y2: "9" }
                            }
                            "相关文章"
                        }
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
                    // 知识图谱关联面板
                    {article_node.read().as_ref().map(|node| {
                        let community = node.community.clone();
                        let pr = knowledge_graph::pct_str(&node.pagerank);
                        let related = node.related.clone();
                        let exp = node.explore_data.clone();

                        rsx! {
                            div { class: "kg-panel",
                                h3 { class: "kg-title", "知识关联" }
                                // 图统计栏
                                div { class: "kg-stats",
                                    div { class: "kg-stat",
                                        span { class: "kg-stat-label", "社区" }
                                        span { class: "kg-stat-value", "{community}" }
                                    }
                                    div { class: "kg-stat",
                                        span { class: "kg-stat-label", "PageRank" }
                                        span { class: "kg-stat-value", "{pr}" }
                                    }
                                }
                                // RWR 关联文章
                                if !related.articles.is_empty() {
                                    div { class: "kg-section",
                                        div { class: "kg-section-header",
                                            span { class: "kg-section-icon",
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        view_box: "0 0 24 24",
                                                        width: "16", height: "16",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        path { d: "M9 17H7A5 5 0 0 1 7 7h2" }
                                                        path { d: "M15 7h2a5 5 0 1 1 0 10h-2" }
                                                        line { x1: "8", y1: "12", x2: "16", y2: "12" }
                                                    }
                                                }
                                                span { class: "kg-section-title", "关联文章" }
                                            span { class: "kg-section-count", "({related.articles.len()})" }
                                        }
                                        div { class: "kg-link-list",
                                            {related.articles.iter().map(|a| {
                                                let to = Route::BlogPostView { slug: a.slug.clone() };
                                                let score = knowledge_graph::pct_str(&a.score);
                                                let reason = a.reason.clone();
                                                rsx! {
                                                    Link { to, class: "kg-link-item",
                                                        span { class: "kg-link-score", "{score} · {reason}" }
                                                        span { class: "kg-link-text",
                                                            "{BLOG_POSTS.iter().find(|p| p.slug == a.slug).map(|p| p.title).unwrap_or(&a.slug)}"
                                                        }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                                // 关联标签
                                if !related.tags.is_empty() {
                                    div { class: "kg-section",
                                        div { class: "kg-section-header",
                                            span { class: "kg-section-icon",
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        view_box: "0 0 24 24",
                                                        width: "16", height: "16",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        path { d: "M12 2H2v10l9.28 9.29a2 2 0 0 0 2.83 0l6.89-6.89a2 2 0 0 0 0-2.83L12 2z" }
                                                        path { d: "M7 7h.01" }
                                                    }
                                                }
                                                span { class: "kg-section-title", "关联标签" }
                                        }
                                        div { class: "kg-tag-cloud",
                                            {related.tags.iter().map(|t| {
                                                let score = knowledge_graph::pct_str(&t.score);
                                                rsx! {
                                                    span { class: "kg-tag",
                                                        span { class: "kg-tag-name", "{t.name}" }
                                                        span { class: "kg-tag-score", "{score}" }
                                                    }
                                                }
                                            })}
                                        }
                                    }
                                }
                                // 探索式导航: 1-hop
                                if !exp.hop1.tags.is_empty() {
                                    div { class: "kg-section",
                                        div { class: "kg-section-header",
                                            span { class: "kg-section-icon",
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        view_box: "0 0 24 24",
                                                        width: "16", height: "16",
                                                        fill: "none",
                                                        stroke: "currentColor",
                                                        stroke_width: "2",
                                                        stroke_linecap: "round",
                                                        stroke_linejoin: "round",
                                                        circle { cx: "12", cy: "12", r: "10" }
                                                        polygon { points: "16.24 7.76 14 14 7.76 16.24 10 10 16.24 7.76" }
                                                    }
                                                }
                                                span { class: "kg-section-title", "探索发现" }
                                        }
                                        div { class: "kg-explore",
                                            if !exp.hop1.tags.is_empty() {
                                                div { class: "kg-explore-row",
                                                    span { class: "kg-explore-label", "直达标签" }
                                                    div { class: "kg-tag-cloud",
                                                        {exp.hop1.tags.iter().map(|t| {
                                                            rsx! { span { class: "kg-tag kg-tag-plain", "{t}" } }
                                                        })}
                                                    }
                                                }
                                            }
                                            if !exp.hop2.articles.is_empty() {
                                                div { class: "kg-explore-row",
                                                    span { class: "kg-explore-label",
                                                        svg {
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            view_box: "0 0 24 24",
                                                            width: "14", height: "14",
                                                            fill: "none",
                                                            stroke: "currentColor",
                                                            stroke_width: "2",
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            line { x1: "6", y1: "3", x2: "6", y2: "15" }
                                                            circle { cx: "18", cy: "6", r: "3" }
                                                            circle { cx: "6", cy: "18", r: "3" }
                                                            path { d: "M18 9a9 9 0 0 1-9 9" }
                                                        }
                                                        "二度关联"
                                                    }
                                                    div { class: "kg-link-list",
                                                        {exp.hop2.articles.iter().map(|s| {
                                                            let to = Route::BlogPostView { slug: s.clone() };
                                                            rsx! {
                                                                Link { to, class: "kg-link-item kg-link-sub",
                                                                    span { class: "kg-link-text",
                                                                        "{BLOG_POSTS.iter().find(|p| p.slug == s.as_str()).map(|p| p.title).unwrap_or(s)}"
                                                                    }
                                                                }
                                                            }
                                                        })}
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
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