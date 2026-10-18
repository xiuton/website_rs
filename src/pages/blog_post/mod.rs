use crate::components::icons::{BackArrowIcon, HomeIcon, ScrollTopIcon, TagIcon, WideModeIcon};
use crate::components::PageHead;
use crate::models::BlogPost;
use crate::routes::Route;
use crate::utils::knowledge_graph::{self, ArticleNode};
use crate::utils::search::SearchEngine;
use crate::utils::topics::{self, LdaData};
use crate::utils::{code_highlight, storage, title};
use crate::BLOG_POSTS;
use dioxus::prelude::*;

/// 预渲染页会把文章正文同时放进 `<template id="prerender-content">`。
/// 首屏直接复用这份 HTML：无需再发请求，也不会出现「预渲染内容被清空 → 加载中」的闪跳。
/// 客户端路由跳转（没有预渲染页）时返回 None，由调用方按需 fetch。
#[cfg(target_arch = "wasm32")]
fn take_prerendered_content(slug: &str) -> Option<String> {
    let document = web_sys::window()?.document()?;
    let template = document.get_element_by_id("prerender-content")?;
    if template.get_attribute("data-slug").as_deref() != Some(slug) {
        return None;
    }
    let html = template.inner_html();
    template.remove();
    Some(html)
}

/// 按 slug 拉取构建期生成的文章 HTML 片段（会话级缓存，重复访问不重复下载）。
async fn fetch_post_html(slug: &str) -> Option<String> {
    let encoded = js_sys::encode_uri_component(slug);
    let path = format!("/static/post-html/{encoded}.html");
    crate::utils::cache::fetch_text(&path).await
}

#[component]
pub fn BlogPostView(slug: String) -> Element {
    // 容忍 URL 尾斜杠（预渲染静态页链接带 "/"），避免 slug 匹配失败
    let slug = slug.trim_end_matches('/').to_string();
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

    // 正文下方的推荐/主题/图谱数据在滚动到可见区域后再加载，
    // 避免和正文、字体争抢首屏带宽。
    let mut details_visible = use_signal(|| false);

    let post = use_memo(move || {
        let s = current_slug();
        BLOG_POSTS.iter().find(|p| p.slug == s)
    });

    // 首屏直接同步取预渲染模板里的正文，避免「先渲染加载中、再替换」的闪动；
    // 客户端路由跳转时才按需拉取构建期生成的 HTML 片段。
    // wasm 里因此不再需要打包全部文章正文，也不需要 comrak。
    #[cfg(target_arch = "wasm32")]
    let initial_content = take_prerendered_content(&slug);
    #[cfg(not(target_arch = "wasm32"))]
    let initial_content = crate::ssr::take_content(&slug);
    let has_initial_content = initial_content.is_some();
    let mut content_html = use_signal(move || initial_content.unwrap_or_default());
    let mut content_loading = use_signal(|| !has_initial_content);
    let mut restored_initial = use_signal(|| has_initial_content);

    use_effect(move || {
        // 订阅路由变化（必须在提前返回之前读取）
        let slug = current_slug();

        if *restored_initial.peek() {
            restored_initial.set(false);
            return;
        }

        content_loading.set(true);
        content_html.set(String::new());

        let current = current_slug;
        spawn(async move {
            let html = fetch_post_html(&slug).await;
            // 快速切换文章时丢弃过期响应
            if current.peek().as_str() != slug.as_str() {
                return;
            }
            match html {
                Some(html) => content_html.set(html),
                None => content_html.set("<p class=\"blog-content-error\">正文加载失败，请刷新重试。</p>".into()),
            }
            content_loading.set(false);
        });
    });

    // 加载搜索引擎（关键词匹配相关文章）
    let mut related_results = use_signal(Vec::<(String, String, String)>::new);

    use_effect(move || {
        if !details_visible() {
            return;
        }
        let slug = current_slug();
        spawn(async move {
            related_results.set(Vec::new());
            if let Some(text) = crate::utils::cache::fetch_text("/static/search-index.json").await {
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
        });
    });

    // 加载知识图谱
    let mut article_node = use_signal(|| Option::<ArticleNode>::None);

    use_effect(move || {
        if !details_visible() {
            return;
        }
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

    // 加载 LDA 主题数据
    let mut lda_data = use_signal(|| Option::<LdaData>::None);

    use_effect(move || {
        if !details_visible() {
            return;
        }
        spawn(async move {
            lda_data.set(None);
            if let Some(data) = topics::load_lda().await {
                lda_data.set(Some((*data).clone()));
            }
        });
    });

    use_effect(move || {
        // 设置文章级结构化数据 + 恢复宽屏模式；标题/描述/canonical 由 PageHead 负责
        if let Some(post) = post() {
            let description: String = post.summary.chars().take(160).collect();
            let post_url = format!("https://ganto.me/post/{}", post.slug);
            title::set_article_jsonld(post.title, &description, &post_url, post.date, post.author, post.tags);
        }
        if post().is_some() {
            is_wide_mode.set(storage::get_blog_wide_mode());
        }
    });

    use_effect(move || {
        // 只有正文包含代码块时才加载 highlight.js（约 40 KB gzip）
        if content_html().contains("<pre") {
            code_highlight::init_highlight();
        }
    });

    use_effect(move || {
        // 只有正文包含代码块时才重新高亮，避免无代码时反复轮询 hljs
        let html = content_html();
        if post().is_some() && html.contains("<pre") {
            code_highlight::reapply_highlight();
        }
    });

    let article_label = post().map(|p| format!("文章：{}", p.title));
    // 保存 slug 备用（避免在 if-let 内部被 post 绑定屏蔽）
    let post_slug = current_slug();

    // 系列聚合信息：(系列名, 章节列表, 当前章节索引)
    // 同一 series 字段的文章属于同一个多章节文档，按 order（其次日期）排序
    let series_info: Option<(String, Vec<&'static BlogPost>, usize)> = post().and_then(|p| {
        if p.series.is_empty() {
            return None;
        }
        let series_key = if !p.series.is_empty() && !p.catalog.is_empty() { p.catalog } else { p.series };
        let series = p.series.to_string();
        let mut chapters: Vec<&'static BlogPost> = BLOG_POSTS
            .iter()
            .filter(|q| {
                let q_key = if !q.series.is_empty() && !q.catalog.is_empty() { q.catalog } else { q.series };
                q_key == series_key
            })
            .collect();
        // 章节顺序由 build.rs 预计算的 sort_key 决定（显式 order → 时间 → 标题）
        chapters.sort_by(|a, b| a.sort_key.cmp(b.sort_key).then_with(|| a.filename.cmp(b.filename)));
        let idx = chapters.iter().position(|q| q.slug == p.slug)?;
        Some((series, chapters, idx))
    });

    rsx! {
        div { class: "blog-container",
            if let Some(post) = post() {
                PageHead {
                    key: "{post.slug}",
                    title: post.title.to_string(),
                    description: post.summary.chars().take(160).collect::<String>(),
                    canonical: format!("/post/{}", post.slug),
                }
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
                        if content_loading() {
                            div { class: "blog-content-loading", "正文加载中…" }
                        } else {
                            div { dangerous_inner_html: content_html() }
                        }
                    }
                    // 系列章节导航（多章节文档）
                    {series_info.as_ref().map(|(series, chapters, idx)| rsx! {
                        SeriesNav {
                            series: series.clone(),
                            chapters: chapters.clone(),
                            idx: *idx,
                        }
                    })}
                    div { class: "blog-tags",
                        {post.tags.iter().map(|tag| rsx! {
                            span { class: "blog-tag",
                                TagIcon {}
                                { *tag }
                            }
                        })}
                    }
                    // 滚动到这里才加载下方的推荐/主题/图谱数据
                    div {
                        class: "details-sentinel",
                        style: "height:1px;",
                        onvisible: move |evt| {
                            if evt.is_intersecting().unwrap_or(false) {
                                details_visible.set(true);
                            }
                        },
                    }
                    // 相关文章推荐（关键词匹配）
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
                            "关键词相关"
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
                    // 马尔可夫链 AI 续写（紧挨文章正文底部）
                    MarkovContinuation { slug: post_slug.clone() }
                    // 相关文章推荐（语义相似度）
                    RelatedArticles { slug: post_slug.clone(), visible: details_visible }
                    // 主题雷达图
                    {lda_data.read().as_ref().map(|lda| {
                        let topics = lda.article_topics(&post_slug, 6);
                        if topics.is_empty() {
                            return rsx! {};
                        }
                        let n = topics.len();
                        let cx = 100.0;
                        let cy = 100.0;
                        let r = 80.0;
                        let levels = 4;

                        // 生成网格线
                        let mut grid_lines = String::new();
                        for level in 1..=levels {
                            let level_r = r * level as f64 / levels as f64;
                            let level_points: Vec<String> = topics.iter().enumerate().map(|(i, _)| {
                                let angle = -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                                let x = cx + level_r * angle.cos();
                                let y = cy + level_r * angle.sin();
                                format!("{:.1},{:.1}", x, y)
                            }).collect();
                            grid_lines.push_str(&format!("<polygon points=\"{}\" fill=\"none\" stroke=\"var(--border-default)\" stroke-width=\"0.6\"/>", level_points.join(" ")));
                        }

                        // 生成轴线
                        let mut axis_lines = String::new();
                        for (i, _) in topics.iter().enumerate() {
                            let angle = -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                            let x = cx + r * angle.cos();
                            let y = cy + r * angle.sin();
                            axis_lines.push_str(&format!("<line x1=\"{:.1}\" y1=\"{:.1}\" x2=\"{:.1}\" y2=\"{:.1}\" stroke=\"var(--border-default)\" stroke-width=\"0.5\"/>", cx, cy, x, y));
                        }

                        // 生成数据区域
                        let data_points: Vec<String> = topics.iter().enumerate().map(|(i, t)| {
                            let angle = -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                            let data_r = r * t.probability;
                            let x = cx + data_r * angle.cos();
                            let y = cy + data_r * angle.sin();
                            format!("{:.1},{:.1}", x, y)
                        }).collect();

                        let data_polygon = format!("<polygon points=\"{}\" fill=\"var(--accent-soft)\" fill-opacity=\"0.35\" stroke=\"var(--accent)\" stroke-width=\"1.5\" stroke-linejoin=\"round\"/>", data_points.join(" "));

                        // 生成数据点
                        let mut data_dots = String::new();
                        for (i, t) in topics.iter().enumerate() {
                            let angle = -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                            let data_r = r * t.probability;
                            let x = cx + data_r * angle.cos();
                            let y = cy + data_r * angle.sin();
                            data_dots.push_str(&format!("<circle cx=\"{:.1}\" cy=\"{:.1}\" r=\"3\" fill=\"var(--accent)\"/>", x, y));
                        }

                        // 生成标签
                        let mut labels = String::new();
                        for (i, t) in topics.iter().enumerate() {
                            let angle = -std::f64::consts::PI / 2.0 + 2.0 * std::f64::consts::PI * i as f64 / n as f64;
                            let label_r = r + 14.0;
                            let x = cx + label_r * angle.cos();
                            let y = cy + label_r * angle.sin();
                            let anchor = if x > cx + 2.0 { "start" } else if x < cx - 2.0 { "end" } else { "middle" };
                            let pct = format!("{:.0}%", t.probability * 100.0);
                            labels.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" dominant-baseline=\"middle\" font-size=\"9\" fill=\"var(--text-secondary)\" font-family=\"system-ui, sans-serif\">{}</text>", x, y, anchor, t.topic_name));
                            labels.push_str(&format!("<text x=\"{:.1}\" y=\"{:.1}\" text-anchor=\"{}\" dominant-baseline=\"middle\" font-size=\"7\" fill=\"var(--text-tertiary)\" font-family=\"system-ui, sans-serif\">{}</text>", x, y + 11.0, anchor, pct));
                        }

                        let svg_content = format!(
                            "{}{}{}{}{}",
                            grid_lines, axis_lines, data_polygon, data_dots, labels
                        );

                        rsx! {
                            div { class: "topic-radar",
                                h3 { class: "topic-radar-title",
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
                                        circle { cx: "12", cy: "12", r: "10" }
                                        path { d: "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z" }
                                        path { d: "M2 12h20" }
                                    }
                                    "主题分布"
                                }
                                div { class: "topic-radar-chart",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "-30 -30 260 260",
                                        width: "100%",
                                        style: "max-width: 360px; overflow: visible; aspect-ratio: 1;",
                                        dangerous_inner_html: svg_content
                                    }
                                }
                                div { class: "topic-radar-legend",
                                    {topics.iter().map(|t| {
                                        let pct = format!("{:.0}%", t.probability * 100.0);
                                        rsx! {
                                            span { class: "topic-radar-legend-item",
                                                span { class: "topic-radar-legend-dot", style: "background: var(--accent);" }
                                                span { class: "topic-radar-legend-name", "{t.topic_name}" }
                                                span { class: "topic-radar-legend-pct", "{pct}" }
                                            }
                                        }
                                    })}
                                }
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
                                h3 { class: "kg-title",
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
                                        path { d: "M12 2L2 7l10 5 10-5-10-5z" }
                                        path { d: "M2 17l10 5 10-5" }
                                        path { d: "M2 12l10 5 10-5" }
                                    }
                                    "知识关联"
                                }
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
                                                                    svg {
                                                                        width: "12", height: "12", view_box: "0 0 24 24",
                                                                        fill: "none", stroke: "currentColor", stroke_width: "2",
                                                                        stroke_linecap: "round", stroke_linejoin: "round",
                                                                        path { d: "M5 12h14" }
                                                                        path { d: "M12 5l7 7-7 7" }
                                                                    }
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

mod markov;
mod related;
mod series_nav;

use markov::MarkovContinuation;
use related::RelatedArticles;
use series_nav::SeriesNav;
