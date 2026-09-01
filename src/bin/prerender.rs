//! 文章静态预渲染工具（SEO 优化）
//!
//! 在 `trunk build` 之后运行，用法：
//!   cargo run --release --bin prerender
//!
//! 功能：
//! 1. 读取 dist/static/posts.json（含 content_html 全文），为每篇文章生成
//!    dist/post/<slug>/index.html —— 爬虫无需执行 JS 即可读取完整内容
//! 2. 为每个系列（series）文档生成 dist/series/<slug>/index.html 目录页
//! 3. 将 dist/index.html 复制为 dist/404.html（GitHub Pages SPA 兜底）

#[cfg(not(target_arch = "wasm32"))]
mod prerender_impl {
    use serde::Deserialize;
    use std::collections::BTreeSet;
    use std::fs;
    use std::path::Path;

    const SITE: &str = "https://ganto.me";
    const OG_IMAGE: &str = "https://ganto.me/static/images/og-image.png";
    const DIST: &str = "dist";

    #[derive(Deserialize)]
    struct PostJson {
        slug: String,
        title: String,
        date: String,
        #[serde(default)]
        author: String,
        #[serde(default)]
        tags: Vec<String>,
        #[serde(default)]
        summary: String,
        #[serde(default)]
        content_html: String,
        #[serde(default)]
        series: String,
        #[serde(default)]
        order: i32,
        #[serde(default)]
        catalog: String,
    }

    /// HTML 属性/文本转义
    fn esc(s: &str) -> String {
        s.replace('&', "&amp;")
            .replace('<', "&lt;")
            .replace('>', "&gt;")
            .replace('"', "&quot;")
            .replace('\'', "&#39;")
    }

    /// JSON 字符串转义
    fn json_escape(s: &str) -> String {
        let mut out = String::with_capacity(s.len() + 8);
        out.push('"');
        for c in s.chars() {
            match c {
                '"' => out.push_str("\\\""),
                '\\' => out.push_str("\\\\"),
                '\n' => out.push_str("\\n"),
                '\r' => out.push_str("\\r"),
                '\t' => out.push_str("\\t"),
                c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                c => out.push(c),
            }
        }
        out.push('"');
        out
    }

    /// 移除内容中的 <script>（content_html 以 unsafe 模式渲染，可能包含）
    fn strip_scripts(html: &str) -> String {
        let mut result = String::with_capacity(html.len());
        let mut rest = html;
        while let Some(start) = rest.find("<script") {
            result.push_str(&rest[..start]);
            let after = &rest[start..];
            let end = after.find("</script>").map(|i| i + "</script>".len());
            match end {
                Some(end) => rest = &after[end..],
                None => {
                    rest = "";
                    break;
                }
            }
        }
        result.push_str(rest);
        result
    }

    /// 从 dist/index.html 提取内置 CSS 文件名、主题变量块（:root / .dark）与 wasm 启动脚本
    /// 注意：只提取变量块，不复制 index.html 中的骨架屏等其他内联样式，
    /// 否则其中的 .navbar-links 等规则会覆盖外部 styles.css，导致导航样式错乱
    fn extract_css_and_vars(index_html: &str) -> (String, String, String, String) {
        // CSS 文件名：href="xxx.css"
        let css = if let Some(pos) = index_html.find(".css") {
            let head = &index_html[..pos];
            if let Some(href) = head.rfind("href=\"") {
                let name = &index_html[href + "href=\"".len()..pos + 4];
                if name.starts_with('/') {
                    name.to_string()
                } else {
                    format!("/{}", name)
                }
            } else {
                "/styles.css".to_string()
            }
        } else {
            "/styles.css".to_string()
        };

        // 仅提取 :root { ... } 与 .dark { ... } 两个主题变量块
        let mut vars_style = String::new();
        for selector in [":root", ".dark"] {
            if let Some(start) = index_html.find(selector) {
                if let Some(open_rel) = index_html[start..].find('{') {
                    let open = start + open_rel;
                    if let Some(close_rel) = index_html[open + 1..].find('}') {
                        let close = open + 1 + close_rel;
                        vars_style.push_str(&index_html[start..=close]);
                        vars_style.push('\n');
                    }
                }
            }
        }

        // wasm 启动脚本：<script type="module">...</script>
        // 浏览器加载静态页后由它启动 WASM，无缝升级为完整 SPA
        let wasm_script = if let Some(start) = index_html.find("<script type=\"module\">") {
            let after = &index_html[start..];
            if let Some(end) = after.find("</script>") {
                after[..end + "</script>".len()].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        // preload/modulepreload 链接标签：提前下载 wasm，缩短挂载前清空静态内容后的空白
        let mut preloads = String::new();
        let mut search_from = 0;
        while let Some(start) = index_html[search_from..].find("<link") {
            let abs = search_from + start;
            let segment = &index_html[abs..];
            let end_rel = segment.find('>').unwrap_or(segment.len());
            let tag = &segment[..end_rel + 1];
            if tag.contains("modulepreload") || tag.contains("preload") {
                preloads.push_str(tag);
                preloads.push('\n');
            }
            search_from = abs + end_rel + 1;
        }

        (css, vars_style, wasm_script, preloads)
    }

    /// 提取文章描述：优先 summary，否则从正文 HTML 取纯文本
    fn extract_description(post: &PostJson) -> String {
        let summary = post.summary.trim();
        if !summary.is_empty() {
            return summary.chars().take(160).collect();
        }
        let text = strip_scripts(&post.content_html);
        // 去标签
        let mut text = text;
        while let Some(start) = text.find('<') {
            if let Some(end) = text[start..].find('>') {
                text.replace_range(start..start + end + 1, " ");
            } else {
                text = text[..start].to_string();
                break;
            }
        }
        let text = text
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
            .trim()
            .to_string();
        text.chars().take(160).collect()
    }

    /// SPA 导航栏的静态 HTML（复用同一套 .navbar-* 样式类，外观与 SPA 完全一致）
    /// 交互降级：sticky/涟漪依赖 JS 不可用；主题切换由下方内联脚本处理
    const NAVBAR_HTML: &str = r#"<div class="navbar-content">
    <div class="navbar-ui">
        <div class="navbar-title-wrapper">
            <h1 class="navbar-title">干徒</h1>
            <div class="navbar-glow"></div>
        </div>
        <div class="navbar-subtitle">这很酷</div>
    </div>
</div>
<div class="navbar-sticky-wrap">
    <div class="navbar-links">
        <a class="nav-link nav-active" href="/">首页</a>
        <a class="nav-link" href="/about">关于</a>
        <a class="nav-link" href="/tags">书签</a>
        <a class="nav-link" href="/search">搜索</a>
        <a class="nav-link" href="/knowledge-graph">图谱</a>
        <a class="nav-link" href="/ai-summary">AI摘要</a>
        <div class="nav-item-with-sub">
            <a class="nav-link" href="/dev">开发</a>
            <div class="nav-submenu">
                <a class="nav-sub-link" href="/circle-generator">圆形生成器</a>
            </div>
        </div>
        <a class="nav-icon-btn rss-toggle" href="/rss" title="RSS 订阅" aria-label="RSS 订阅">
            <svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" width="16" height="16"><path d="M6.18 15.64a2.18 2.18 0 0 1 2.18 2.18C8.36 19 7.38 20 6.18 20C5 20 4 19 4 17.82a2.18 2.18 0 0 1 2.18-2.18M4 4.44A15.56 15.56 0 0 1 19.56 20h-2.83A12.73 12.73 0 0 0 4 7.27zm0 5.66a9.9 9.9 0 0 1 9.9 9.9h-2.83A7.07 7.07 0 0 0 4 12.93z"/></svg>
        </a>
        <button class="nav-icon-btn theme-toggle" id="prerender-theme-toggle" aria-label="切换主题" title="切换主题">
            <svg class="icon-moon" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" width="16" height="16" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round"><path d="M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"/></svg>
            <svg class="icon-sun" xmlns="http://www.w3.org/2000/svg" viewBox="0 0 1024 1024" width="16" height="16" fill="currentColor"><path d="M512 704a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512m0-704a32 32 0 0 1 32 32v64a32 32 0 0 1-64 0V96a32 32 0 0 1 32-32m0 768a32 32 0 0 1 32 32v64a32 32 0 1 1-64 0v-64a32 32 0 0 1 32-32M195.2 195.2a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 1 1-45.248 45.248L195.2 240.448a32 32 0 0 1 0-45.248zm543.104 543.104a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 0 1-45.248 45.248l-45.248-45.248a32 32 0 0 1 0-45.248M64 512a32 32 0 0 1 32-32h64a32 32 0 0 1 0 64H96a32 32 0 0 1-32-32m768 0a32 32 0 0 1 32-32h64a32 32 0 1 1 0 64h-64a32 32 0 0 1-32-32M195.2 828.8a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248L240.448 828.8a32 32 0 0 1-45.248 0zm543.104-543.104a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248l-45.248 45.248a32 32 0 0 1-45.248 0"/></svg>
        </button>
    </div>
</div>"#;

    /// SPA 页脚的静态 HTML（复用 .footer-content 样式）
    const FOOTER_HTML: &str = r#"<footer class="footer-content">
    <div class="copyright">
        <span>2019-2026 </span>
        <svg xmlns="http://www.w3.org/2000/svg" width="14" height="14" viewBox="0 0 512 512" fill="rgb(161, 98, 7)" style="margin: 0 2px; position: relative; top: 2px;"><path d="M256 48a208 208 0 1 1 0 416 208 208 0 1 1 0-416zm0 464A256 256 0 1 0 256 0a256 256 0 1 0 0 512zM199.4 312.6c-31.2-31.2-31.2-81.9 0-113.1s81.9-31.2 113.1 0c9.4 9.4 24.6 9.4 33.9 0s9.4-24.6 0-33.9c-50-50-131-50-181 0s-50 131 0 181s131 50 181 0c9.4-9.4 9.4-24.6 0-33.9s-24.6-9.4-33.9 0c-31.2 31.2-81.9 31.2-113.1 0z"/></svg>
        <span> 干徒 / Ganto</span>
    </div>
</footer>"#;

    /// 主题初始化 + 静态页主题切换按钮（无涟漪动画）+ URL 规范化
    /// URL 规范化：去掉路径尾斜杠（刷新时静态服务器会 301 到带斜杠地址），
    /// 避免 WASM 启动后 SPA 路由把 slug 解析成带斜杠导致匹配失败
    const THEME_SCRIPT: &str = r#"(function() {
    if (location.pathname.length > 1 && location.pathname.endsWith('/')) {
        history.replaceState(null, '', location.pathname.replace(/\/+$/, '') + location.search + location.hash);
    }
    var theme = localStorage.getItem('theme');
    if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {
        document.documentElement.classList.add('dark');
    }
    var tbtn = document.getElementById('prerender-theme-toggle');
    if (tbtn) {
        tbtn.addEventListener('click', function() {
            var d = document.documentElement;
            var dark = d.classList.toggle('dark');
            localStorage.setItem('theme', dark ? 'dark' : 'light');
        });
    }
})();"#;

    /// WASM 挂载前清空 #main 中的预渲染内容
    /// dioxus-web 挂载容器时不会清空已有 DOM，若不清理，静态页会与 SPA 渲染结果上下重复
    const CLEAR_SCRIPT: &str = r#"<script>/* 挂载前清空预渲染内容，避免与 WASM 渲染结果重复 */
try{document.getElementById('main').innerHTML='';}catch(e){}</script>"#;

    /// 生成系列章节导航 HTML（与 SPA 中的 .series-nav 结构一致）
    fn series_nav_html(post: &PostJson, all_posts: &[PostJson]) -> String {
        if post.series.is_empty() {
            return String::new();
        }
        let mut chapters: Vec<&PostJson> = all_posts
            .iter()
            .filter(|q| q.series == post.series)
            .collect();
        chapters.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| b.date.cmp(&a.date)));
        let idx = chapters.iter().position(|q| q.slug == post.slug).unwrap_or(0);
        let total = chapters.len();

        let mut list = String::new();
        for (i, ch) in chapters.iter().enumerate() {
            let active = i == idx;
            let current_badge = if active {
                "<span class=\"series-nav-item-now\">本篇</span>".to_string()
            } else {
                String::new()
            };
            list.push_str(&format!(
                "<a class=\"series-nav-item{}\" href=\"/post/{}/\"><span class=\"series-nav-item-index\">{}</span><span class=\"series-nav-item-title\">{}</span>{}</a>",
                if active { " active" } else { "" },
                ch.slug,
                i + 1,
                esc(&ch.title),
                current_badge,
            ));
        }

        let prev_html = if idx > 0 {
            let prev = chapters[idx - 1];
            format!(
                "<a class=\"series-nav-page prev\" href=\"/post/{}/\"><span class=\"series-nav-page-label\">上一篇</span><span class=\"series-nav-page-title\">{}</span></a>",
                prev.slug,
                esc(&prev.title),
            )
        } else {
            "<span class=\"series-nav-page disabled\">已经是第一章</span>".to_string()
        };

        let next_html = if let Some(next) = chapters.get(idx + 1) {
            format!(
                "<a class=\"series-nav-page next\" href=\"/post/{}/\"><span class=\"series-nav-page-label\">下一篇</span><span class=\"series-nav-page-title\">{}</span></a>",
                next.slug,
                esc(&next.title),
            )
        } else {
            "<span class=\"series-nav-page disabled\">已经是最后一章</span>".to_string()
        };

        format!(
            "<div class=\"series-nav\"><div class=\"series-nav-header\"><svg xmlns=\"http://www.w3.org/2000/svg\" view_box=\"0 0 24 24\" width=\"16\" height=\"16\" fill=\"none\" stroke=\"currentColor\" stroke_width=\"2\" stroke_linecap=\"round\" stroke_linejoin=\"round\" class=\"section-icon\"><path d=\"M4 19.5A2.5 2.5 0 0 1 6.5 17H20\"/><path d=\"M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z\"/></svg><div class=\"series-nav-heading\"><span class=\"series-nav-title\">{}</span><span class=\"series-nav-count\">共 {} 章</span></div></div><div class=\"series-nav-list\">{}</div><div class=\"series-nav-pager\">{}{}</div></div>",
            esc(&post.series),
            total,
            list,
            prev_html,
            next_html,
        )
    }

    /// 生成单篇文章的预渲染 HTML
    fn render_page(
        post: &PostJson,
        all_posts: &[PostJson],
        css: &str,
        preloads: &str,
        vars_style: &str,
        wasm_script: &str,
    ) -> String {
        let title = &post.title;
        let date: String = post.date.chars().take(10).collect();
        let author = if post.author.trim().is_empty() { "干徒" } else { &post.author };
        let desc = extract_description(post);
        let url = format!("{}/post/{}", SITE, post.slug);
        let keywords = post.tags.join(", ");

        let tag_html: String = post
            .tags
            .iter()
            .map(|t| format!("<span class=\"blog-tag\">{}</span>", esc(t)))
            .collect();

        let series_html = series_nav_html(post, all_posts);

        let jsonld = format!(
            r#"{{"@context":"https://schema.org","@type":"BlogPosting","headline":{},"description":{},"url":{},"datePublished":{},"dateModified":{},"inLanguage":"zh-CN","image":{},"author":{{"@type":"Person","name":{}}},"keywords":{},"publisher":{{"@type":"Organization","name":"干徒的博客","url":"https://ganto.me/"}}}}"#,
            json_escape(title),
            json_escape(&desc),
            json_escape(&url),
            json_escape(&date),
            json_escape(&date),
            json_escape(OG_IMAGE),
            json_escape(author),
            json_escape(&keywords),
        );

        let content_html = strip_scripts(&post.content_html);

        format!(
            r#"<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{title} - 干徒</title>
    <meta name="description" content="{desc}">
    <meta name="author" content="{author}">
    <meta name="keywords" content="{keywords}">
    <link rel="canonical" href="{url}">
    <meta property="og:title" content="{title} - 干徒">
    <meta property="og:description" content="{desc}">
    <meta property="og:type" content="article">
    <meta property="og:url" content="{url}">
    <meta property="og:site_name" content="干徒的博客">
    <meta property="og:locale" content="zh_CN">
    <meta property="og:image" content="{og_image}">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta property="og:image:alt" content="{title} - 干徒">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="{title} - 干徒">
    <meta name="twitter:description" content="{desc}">
    <meta name="twitter:image" content="{og_image}">
    <script type="application/ld+json">
{jsonld}
    </script>
    <link rel="stylesheet" href="{css}">
{preloads}
    <style>
{vars_style}
    </style>
    <style>
        .prerender-article {{ max-width: 760px; margin: 0 auto; padding: 1rem 1.25rem 3rem; }}
        .prerender-meta {{ display: flex; gap: 1rem; flex-wrap: wrap; color: var(--text-tertiary); font-size: 0.9rem; margin-bottom: 0.5rem; }}
        .theme-toggle .icon-sun {{ display: none; }}
        .dark .theme-toggle .icon-sun {{ display: inline; }}
        .dark .theme-toggle .icon-moon {{ display: none; }}
    </style>
    <script>
{theme_script}
    </script>
</head>
<body>
    <div id="main">
        <div class="app">
{navbar}
        <main class="main-content">
            <article class="blog-post prerender-article">
                <h1>{title}</h1>
                <div class="prerender-meta">
                    <span>{date}</span>
                    <span>{author}</span>
                </div>
                <div class="blog-tags">{tag_html}</div>
                <div class="blog-content">
{content_html}
                </div>
                {series_html}
            </article>
        </main>
{footer}
        </div>
    </div>
{clear_script}
{wasm_script}
</body>
</html>
"#,
            title = esc(title),
            desc = esc(&desc),
            author = esc(author),
            keywords = esc(&keywords),
            url = url,
            og_image = OG_IMAGE,
            jsonld = jsonld,
            css = css,
            preloads = preloads,
            vars_style = vars_style,
            theme_script = THEME_SCRIPT,
            navbar = NAVBAR_HTML,
            footer = FOOTER_HTML,
            date = esc(&date),
            tag_html = tag_html,
            content_html = content_html,
            series_html = series_html,
            clear_script = CLEAR_SCRIPT,
            wasm_script = wasm_script,
        )
    }

    /// 生成系列（多章节文档）目录页的预渲染 HTML
    fn series_page_html(
        entry: &PostJson,
        chapters: &[&PostJson],
        series_id: &str,
        css: &str,
        preloads: &str,
        vars_style: &str,
        wasm_script: &str,
    ) -> String {
        let total = chapters.len();
        let author = if entry.author.trim().is_empty() { "干徒" } else { &entry.author };
        let desc = if entry.summary.trim().is_empty() {
            format!("{} 系列文档，共 {} 章。", entry.series, total)
        } else {
            entry.summary.clone()
        };
        let url = format!("{}/series/{}", SITE, series_id);
        let first_slug = &chapters[0].slug;
        let last_date = chapters
            .iter()
            .map(|c| c.date.as_str())
            .max()
            .unwrap_or(&entry.date);

        let mut list = String::new();
        for (i, ch) in chapters.iter().enumerate() {
            list.push_str(&format!(
                "<a class=\"series-chapter-card\" href=\"/post/{}/\"><span class=\"series-chapter-index\">{}</span><div class=\"series-chapter-body\"><h3 class=\"series-chapter-title\">{}</h3><p class=\"series-chapter-summary\">{}</p><span class=\"series-chapter-date\">{}</span></div></a>",
                ch.slug,
                i + 1,
                esc(&ch.title),
                if ch.summary.trim().is_empty() { String::new() } else { esc(&ch.summary) },
                esc(&ch.date),
            ));
        }

        format!(
            r#"<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{series} - 干徒</title>
    <meta name="description" content="{desc}">
    <meta name="author" content="{author}">
    <link rel="canonical" href="{url}">
    <meta property="og:title" content="{series} - 干徒">
    <meta property="og:description" content="{desc}">
    <meta property="og:type" content="website">
    <meta property="og:url" content="{url}">
    <meta property="og:site_name" content="干徒的博客">
    <meta property="og:locale" content="zh_CN">
    <meta property="og:image" content="{og_image}">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="{series} - 干徒">
    <meta name="twitter:description" content="{desc}">
    <link rel="stylesheet" href="{css}">
{preloads}
    <style>
{vars_style}
    </style>
    <style>
        .theme-toggle .icon-sun {{ display: none; }}
        .dark .theme-toggle .icon-sun {{ display: inline; }}
        .dark .theme-toggle .icon-moon {{ display: none; }}
    </style>
    <script>
{theme_script}
    </script>
</head>
<body>
    <div id="main">
        <div class="app">
{navbar}
        <main class="main-content">
            <div class="series-container">
                <div class="series-hero">
                    <div class="series-hero-tag">系列文档</div>
                    <h1 class="series-hero-title">{series}</h1>
                    <p class="series-hero-desc">{desc}</p>
                    <div class="series-hero-meta">
                        <span>共 {total} 章</span>
                        <span>作者：{author}</span>
                        <span>最后更新：{last_date}</span>
                    </div>
                    <a class="series-start-btn" href="/post/{first_slug}/">开始阅读 →</a>
                </div>
                <div class="series-chapter-list">{list}</div>
            </div>
        </main>
{footer}
        </div>
    </div>
{clear_script}
{wasm_script}
</body>
</html>
"#,
            series = esc(&entry.series),
            desc = esc(&desc),
            author = esc(author),
            url = url,
            og_image = OG_IMAGE,
            css = css,
            preloads = preloads,
            vars_style = vars_style,
            theme_script = THEME_SCRIPT,
            navbar = NAVBAR_HTML,
            footer = FOOTER_HTML,
            total = total,
            first_slug = first_slug,
            last_date = esc(last_date),
            list = list,
            clear_script = CLEAR_SCRIPT,
            wasm_script = wasm_script,
        )
    }

    pub fn run() -> Result<(), String> {
        let dist = Path::new(DIST);
        if !dist.is_dir() {
            return Err(format!("dist directory not found: {}", DIST));
        }

        let index_path = dist.join("index.html");
        let index_html = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
        let (css, vars_style, wasm_script, preloads) = extract_css_and_vars(&index_html);

        let posts_path = dist.join("static").join("posts.json");
        let posts_json = fs::read_to_string(&posts_path).map_err(|e| e.to_string())?;
        let posts: Vec<PostJson> =
            serde_json::from_str(&posts_json).map_err(|e| e.to_string())?;

        let mut count = 0usize;
        for post in &posts {
            if post.slug.is_empty() {
                continue;
            }
            let out_dir = dist.join("post").join(&post.slug);
            fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
            let page = render_page(post, &posts, &css, &preloads, &vars_style, &wasm_script);
            fs::write(out_dir.join("index.html"), page).map_err(|e| e.to_string())?;
            count += 1;
        }

        // 系列文档目录页：每个 series 生成一个目录页（入口章 = order 最小的一章）
        let mut series_count = 0usize;
        let mut seen_series: BTreeSet<&String> = BTreeSet::new();
        for post in &posts {
            if post.series.is_empty() || post.slug.is_empty() || !seen_series.insert(&post.series) {
                continue;
            }
            let mut chapters: Vec<&PostJson> =
                posts.iter().filter(|q| q.series == post.series).collect();
            chapters.sort_by(|a, b| a.order.cmp(&b.order).then_with(|| b.date.cmp(&a.date)));
            let entry = chapters[0];
            // 目录页标识：优先用 catalog 字段，未配置则回退到入口章的 slug
            let series_id = if entry.catalog.is_empty() {
                entry.slug.clone()
            } else {
                entry.catalog.clone()
            };
            let out_dir = dist.join("series").join(&series_id);
            fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
            let page = series_page_html(entry, &chapters, &series_id, &css, &preloads, &vars_style, &wasm_script);
            fs::write(out_dir.join("index.html"), page).map_err(|e| e.to_string())?;
            series_count += 1;
        }

        // GitHub Pages SPA 兜底：未知路径加载 404.html（内容与 index.html 一致）
        fs::copy(&index_path, dist.join("404.html")).map_err(|e| e.to_string())?;

        println!(
            "[prerender] generated {} post pages + {} series pages + 404.html, css={}",
            count, series_count, css
        );
        Ok(())
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(e) = prerender_impl::run() {
        eprintln!("[prerender] error: {}", e);
        std::process::exit(1);
    }
}
