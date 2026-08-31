use dioxus::prelude::*;
use crate::routes::Route;
use crate::BLOG_POSTS;
use crate::models::BlogPost;
use crate::utils::{title, code_highlight, storage};
use crate::components::icons::{BackArrowIcon, HomeIcon, ScrollTopIcon, TagIcon, WideModeIcon};
use crate::utils::search::SearchEngine;
use crate::utils::knowledge_graph::{self, ArticleNode};
use crate::utils::topics::{self, LdaData};
use dioxus_router::prelude::{Link, use_route};
use std::collections::HashMap;
use serde::Deserialize;

fn prepare_blog_html(content: &str) -> String {
    let html = crate::utils::markdown::markdown_to_html(content);
    // 给无语言标记的代码块加上 plaintext
    let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
    // 从 language-* 类名提取语言标识，写入 data-lang 到 <pre>
    let html = inject_data_lang(&html);
    add_lazy_loading(&html)
}

/// 扫描 `<pre lang="XXX">`（comrak 输出），注入 `data-lang="XXX"`。
/// 无 `lang` 属性的 `<pre>`（plaintext 块）不设标签。
fn inject_data_lang(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(pos) = rest.find("<pre lang=\"") {
        result.push_str(&rest[..pos]);
        let after = &rest[pos + "<pre lang=\"".len()..];
        let end = after.find('"').unwrap_or(0);
        let lang = &after[..end];
        use std::fmt::Write;
        let _ = write!(result, "<pre lang=\"{lang}\" data-lang=\"{lang}\"");
        rest = &after[end + 1..];
    }
    result.push_str(rest);
    result
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

    // 加载搜索引擎（关键词匹配相关文章）
    let mut related_results = use_signal(|| Vec::<(String, String, String)>::new());

    use_effect(move || {
        let slug = current_slug();
        spawn(async move {
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

    // 加载 LDA 主题数据
    let mut lda_data = use_signal(|| Option::<LdaData>::None);

    use_effect(move || {
        spawn(async move {
            lda_data.set(None);
            if let Some(data) = topics::load_lda().await {
                lda_data.set(Some(data));
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
            let post_url = format!("https://ganto.me/post/{}", post.slug);
            title::set_seo(&post.title, &description, &post_url);
            // 文章级 BlogPosting 结构化数据
            title::set_article_jsonld(
                &post.title,
                &description,
                &post_url,
                &post.date,
                &post.author,
                post.tags,
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
    // 保存 slug 备用（避免在 if-let 内部被 post 绑定屏蔽）
    let post_slug = current_slug();

    // 系列聚合信息：(系列名, 章节列表, 当前章节索引)
    // 同一 series 字段的文章属于同一个多章节文档，按 order（其次日期）排序
    let series_info: Option<(String, Vec<&'static BlogPost>, usize)> = post().and_then(|p| {
        if p.series.is_empty() {
            return None;
        }
        let series = p.series.to_string();
        let mut chapters: Vec<&'static BlogPost> = BLOG_POSTS
            .iter()
            .filter(|q| q.series == p.series)
            .collect();
        chapters.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| b.date.cmp(&a.date)));
        let idx = chapters.iter().position(|q| q.slug == p.slug)?;
        Some((series, chapters, idx))
    });

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
                    // 系列章节导航（多章节文档）
                    {series_info.as_ref().map(|(series, chapters, idx)| {
                        let total = chapters.len();
                        let prev = if *idx > 0 { Some(chapters[*idx - 1]) } else { None };
                        let next = chapters.get(*idx + 1);
                        let series_slug = {
                            let entry = chapters[0];
                            if entry.catalog.is_empty() { entry.slug } else { entry.catalog }
                        };
                        rsx! {
                            div { class: "series-nav",
                                div { class: "series-nav-header",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        view_box: "0 0 24 24",
                                        width: "16", height: "16",
                                        fill: "none", stroke: "currentColor",
                                        stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                                        class: "section-icon",
                                        path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                                        path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                                    }
                                    div { class: "series-nav-heading",
                                        Link {
                                            to: Route::SeriesView { slug: series_slug.to_string() },
                                            class: "series-nav-title-link",
                                            span { class: "series-nav-title", "{series}" }
                                        }
                                        span { class: "series-nav-count", "共 {total} 章" }
                                    }
                                }
                                div { class: "series-nav-list",
                                    {chapters.iter().enumerate().map(|(i, ch)| {
                                        let to = Route::BlogPostView { slug: ch.slug.to_string() };
                                        let is_current = i == *idx;
                                        rsx! {
                                            Link {
                                                to,
                                                class: if is_current { "series-nav-item active" } else { "series-nav-item" },
                                                span { class: "series-nav-item-index", "{i + 1}" }
                                                span { class: "series-nav-item-title", "{ch.title}" }
                                                if is_current {
                                                    span { class: "series-nav-item-now", "本篇" }
                                                }
                                            }
                                        }
                                    })}
                                }
                                div { class: "series-nav-pager",
                                    if let Some(prev) = prev {
                                        Link {
                                            to: Route::BlogPostView { slug: prev.slug.to_string() },
                                            class: "series-nav-page prev",
                                            span { class: "series-nav-page-label", "上一篇" }
                                            span { class: "series-nav-page-title", "{prev.title}" }
                                        }
                                    } else {
                                        span { class: "series-nav-page disabled", "已经是第一章" }
                                    }
                                    if let Some(next) = next {
                                        Link {
                                            to: Route::BlogPostView { slug: next.slug.to_string() },
                                            class: "series-nav-page next",
                                            span { class: "series-nav-page-label", "下一篇" }
                                            span { class: "series-nav-page-title", "{next.title}" }
                                        }
                                    } else {
                                        span { class: "series-nav-page disabled", "已经是最后一章" }
                                    }
                                }
                            }
                        }
                    })}
                    div { class: "blog-tags",
                        {post.tags.iter().map(|tag| rsx! {
                            span { class: "blog-tag",
                                TagIcon {}
                                {tag}
                            }
                        })}
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
                    RelatedArticles { slug: post_slug.clone() }
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

/// 马尔可夫链 AI 续写组件
#[component]
fn MarkovContinuation(slug: String) -> Element {
    let mut data_loaded = use_signal(|| false);
    let mut generator = use_signal(|| None::<crate::utils::markov::MarkovGenerator>);
    let generating = use_signal(|| false);
    let output = use_signal(|| String::new());

    // 只加载一次（slug 在组件声明期 clone 后使用）
    let slug_for_load = slug.clone();
    use_effect(move || {
        if *data_loaded.read() { return; }
        let s = slug_for_load.clone();
        spawn(async move {
            match crate::utils::markov::load_markov().await {
                Ok(data) => {
                    if data.contains_key(&s) {
                        let gen = crate::utils::markov::MarkovGenerator::new(data);
                        generator.set(Some(gen));
                    }
                }
                Err(e) => {
                    web_sys::console::error_1(&format!("加载 markov.json 失败: {:?}", e).into());
                }
            }
            data_loaded.set(true);
        });
    });

    let gen = generator.clone();
    let out = output;
    let mut gen_state = generating;

    let handle_generate = move |_| {
        if *gen_state.read() { return; }
        gen_state.set(true);
        let g = gen.cloned();
        let mut out_w = out.clone();
        let mut gs = gen_state.clone();
        let s = slug.clone();
        spawn(async move {
            if let Some(ref gen) = g {
                let text = gen.generate(&s, Some(""), 300);
                out_w.set(text);
            } else {
                out_w.set("暂无该文章的续写模型。".into());
            }
            gs.set(false);
        });
    };

    rsx! {
        div { class: "markov-section",
            div { class: "markov-header",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    width: "16", height: "16",
                    fill: "none", stroke: "currentColor",
                    stroke_width: "2", stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M12 2a10 10 0 1 0 10 10" }
                    polyline { points: "12 12 18 12 18 6" }
                }
                span { "AI 续写" }
                span { class: "markov-badge", "马尔可夫链" }
            }
            p { class: "markov-desc", "基于本文内容训练的 tri-gram 语言模型，自动生成下文片段。" }
            div { class: "markov-actions",
                button {
                    class: "markov-btn",
                    disabled: *generating.read() || !*data_loaded.read(),
                    onclick: handle_generate,
                    if *generating.read() { "生成中…" } else { "生成续写" }
                }
            }
            if !output.read().is_empty() {
                div { class: "markov-output",
                    {output.read().as_str()}
                }
            }
        }
    }
}

/// 嵌入数据结构
#[derive(Debug, Clone, Deserialize)]
struct RelatedEntry {
    slug: String,
    score: f64,
}

/// 相关文章推荐组件（基于 TF-IDF 余弦相似度）
#[component]
fn RelatedArticles(slug: String) -> Element {
    let mut loaded = use_signal(|| false);
    let mut related = use_signal(|| Vec::<(String, f64)>::new());

    use_effect(move || {
        if *loaded.read() {
            return;
        }
        let s = slug.clone();
        spawn(async move {
            let resp = gloo_net::http::Request::get("/static/embeddings.json")
                .send()
                .await;
            if let Ok(resp) = resp {
                if let Ok(text) = resp.text().await {
                    if let Ok(data) = serde_json::from_str::<HashMap<String, Vec<RelatedEntry>>>(&text) {
                        if let Some(entries) = data.get(&s) {
                            let mut items: Vec<(String, f64)> = entries
                                .iter()
                                .take(5)
                                .map(|e| (e.slug.clone(), e.score))
                                .collect();
                            items.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
                            related.set(items);
                        }
                    }
                }
            }
            loaded.set(true);
        });
    });

    let items = related.read().clone();

    if items.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "related-articles-semantic",
            h3 { class: "related-articles-semantic-title",
                svg {
                    xmlns: "http://www.w3.org/2000/svg",
                    view_box: "0 0 24 24",
                    width: "18", height: "18",
                    fill: "none", stroke: "currentColor",
                    stroke_width: "2",
                    stroke_linecap: "round", stroke_linejoin: "round",
                    path { d: "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" }
                    path { d: "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" }
                }
                "相关文章"
            }
            div { class: "related-articles-semantic-list",
                {items.iter().map(|(related_slug, score)| {
                    let title = BLOG_POSTS.iter()
                        .find(|p| p.slug == *related_slug)
                        .map(|p| &p.title[..])
                        .unwrap_or(related_slug);
                    let pct = (score * 100.0) as u8;
                    let to = Route::BlogPostView { slug: related_slug.clone() };
                    rsx! {
                        Link {
                            to,
                            class: "related-articles-semantic-item",
                            span { class: "related-articles-semantic-item-title", "{title}" }
                            span { class: "related-articles-semantic-item-score", "{pct}%" }
                        }
                    }
                })}
            }
        }
    }
} 