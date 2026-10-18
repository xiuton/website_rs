//! 文章静态预渲染工具（SEO 优化）
//!
//! 在 `trunk build` 之后运行，用法：
//!   cargo run --release -p prerender
//!
//! 功能：
//! 1. 读取 static/posts-meta.json（元数据）与 static/post-html/*.html（正文），为每篇文章生成
//!    dist/post/<slug>/index.html —— 爬虫无需执行 JS 即可读取完整内容
//! 2. 为每个系列（series）文档生成 dist/series/<slug>/index.html 目录页
//! 3. 将 dist/index.html 复制为 dist/404.html（GitHub Pages SPA 兜底）

#[cfg(not(target_arch = "wasm32"))]
mod prerender_impl {
    use dioxus::prelude::*;
    use dioxus_history::{History, MemoryHistory};
    use dioxus_router::components::HistoryProvider;
    use serde::Deserialize;
    use site_model::{PostMeta, PostsMeta};
    use std::collections::BTreeSet;
    use std::fs;
    use std::ops::Deref;
    use std::path::{Path, PathBuf};
    use std::rc::Rc;
    use std::time::Duration;

    #[derive(Props, Clone, PartialEq)]
    struct SsrAppProps {
        at: String,
    }

    #[component]
    fn SsrApp(props: SsrAppProps) -> Element {
        rsx! {
            HistoryProvider {
                history: move |_| Rc::new(MemoryHistory::with_initial_path(props.at.clone())) as Rc<dyn History>,
                Router::<web_rs::Route> {}
            }
        }
    }

    /// 用真实 Dioxus 组件渲染一个路由，供静态页复用，避免再维护一套手写 HTML。
    fn render_route(path: &str) -> Result<String, String> {
        let mut vdom = VirtualDom::new_with_props(SsrApp, SsrAppProps { at: path.to_string() });
        vdom.rebuild_in_place();
        Ok(dioxus_ssr::render(&vdom))
    }

    const SITE: &str = "https://ganto.me";
    const OG_IMAGE: &str = "https://ganto.me/static/images/og-image.png";
    const DIST: &str = "dist";

    /// 共享的 PostMeta + 从 post-html 读取的正文。
    #[derive(Deserialize)]
    struct PostData {
        #[serde(flatten)]
        meta: PostMeta,
        #[serde(skip)]
        content_html: String,
    }

    impl Deref for PostData {
        type Target = PostMeta;

        fn deref(&self) -> &Self::Target {
            &self.meta
        }
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
    fn extract_description(post: &PostData) -> String {
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
        let text = text.split_whitespace().collect::<Vec<_>>().join(" ").trim().to_string();
        text.chars().take(160).collect()
    }

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

    /// 文章页专用：先把可见正文搬进 `<template>` 供 SPA 复用，再清空 #main。
    ///
    /// 这样 HTML 里只保留一份正文：爬虫/禁用 JS 时看到可见的 `.blog-content`，
    /// SPA 启动时从 template 同步取用，不需要重新请求正文。
    fn make_article_clear_script(slug: &str) -> String {
        format!(
            r#"<script>/* 把预渲染正文交给 SPA 后清空 #main，避免重复渲染 */
try{{
  var main=document.getElementById('main');
  var content=main?main.querySelector('.blog-content'):null;
  if(content){{
    var t=document.createElement('template');
    t.id='prerender-content';
    t.setAttribute('data-slug', {slug});
    t.innerHTML=content.innerHTML;
    document.body.appendChild(t);
  }}
  if(main) main.innerHTML='';
}}catch(e){{}}
</script>"#,
            slug = json_escape(slug)
        )
    }

    /// 生成单篇文章的预渲染 HTML（正文由 Dioxus SSR 渲染）
    fn render_page(
        post: &PostData,
        _all_posts: &[PostData],
        css: &str,
        preloads: &str,
        vars_style: &str,
        wasm_script: &str,
    ) -> Result<String, String> {
        let title = &post.title;
        let date: String = post.date.chars().take(10).collect();
        let author = if post.author.trim().is_empty() { "干徒" } else { &post.author };
        let desc = extract_description(post);
        let url = format!("{}/post/{}", SITE, post.slug);
        let keywords = post.tags.join(", ");

        let content_html = strip_scripts(&post.content_html);
        web_rs::ssr::set_content(&post.slug, &content_html);
        web_rs::components::head::reset_ssr_head();
        let ssr_html = render_route(&format!("/post/{}", post.slug))?;
        let head = web_rs::components::head::take_ssr_head();
        let page_title = head
            .as_ref()
            .map(|h| h.title.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| title.to_string());
        let page_desc = head
            .as_ref()
            .map(|h| h.description.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| desc.clone());
        let page_url = head
            .as_ref()
            .map(|h| h.canonical.clone())
            .filter(|s| !s.is_empty())
            .map(|s| if s.starts_with("http") { s } else { format!("{SITE}{s}") })
            .unwrap_or_else(|| url.clone());
        let article_clear_script = make_article_clear_script(&post.slug);

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

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{page_title} - 干徒</title>
    <meta name="description" content="{page_desc}">
    <meta name="author" content="{author}">
    <meta name="keywords" content="{keywords}">
    <link rel="canonical" href="{page_url}">
    <meta property="og:title" content="{page_title} - 干徒">
    <meta property="og:description" content="{page_desc}">
    <meta property="og:type" content="article">
    <meta property="og:url" content="{page_url}">
    <meta property="og:site_name" content="干徒的博客">
    <meta property="og:locale" content="zh_CN">
    <meta property="og:image" content="{og_image}">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta property="og:image:alt" content="{page_title} - 干徒">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="{page_title} - 干徒">
    <meta name="twitter:description" content="{page_desc}">
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
        .theme-toggle .icon-sun {{ display: none; }}
        .dark .theme-toggle .icon-sun {{ display: inline; }}
        .dark .theme-toggle .icon-moon {{ display: none; }}
    </style>
    <script>
{theme_script}
    </script>
</head>
<body>
    <div id="main">{ssr_html}</div>
{article_clear_script}
{wasm_script}
</body>
</html>
"#,
            page_title = esc(&page_title),
            page_desc = esc(&page_desc),
            author = esc(author),
            keywords = esc(&keywords),
            page_url = page_url,
            og_image = OG_IMAGE,
            jsonld = jsonld,
            css = css,
            preloads = preloads,
            vars_style = vars_style,
            theme_script = THEME_SCRIPT,
            article_clear_script = article_clear_script,
            ssr_html = ssr_html,
            wasm_script = wasm_script,
        ))
    }

    /// 生成系列（多章节文档）目录页的预渲染 HTML
    fn series_page_html(
        entry: &PostData,
        chapters: &[&PostData],
        series_id: &str,
        css: &str,
        preloads: &str,
        vars_style: &str,
        wasm_script: &str,
    ) -> Result<String, String> {
        let total = chapters.len();
        let author = if entry.author.trim().is_empty() { "干徒" } else { &entry.author };
        let desc = if entry.summary.trim().is_empty() {
            format!("{} 系列文档，共 {} 章。", entry.series, total)
        } else {
            entry.summary.clone()
        };
        let url = format!("{}/series/{}", SITE, series_id);
        web_rs::components::head::reset_ssr_head();
        let ssr_html = render_route(&format!("/series/{series_id}"))?;
        let head = web_rs::components::head::take_ssr_head();
        let page_title = head
            .as_ref()
            .map(|h| h.title.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| entry.series.clone());
        let page_desc = head
            .as_ref()
            .map(|h| h.description.clone())
            .filter(|s| !s.is_empty())
            .unwrap_or_else(|| desc.clone());
        let page_url = head
            .as_ref()
            .map(|h| h.canonical.clone())
            .filter(|s| !s.is_empty())
            .map(|s| if s.starts_with("http") { s } else { format!("{SITE}{s}") })
            .unwrap_or_else(|| url.clone());

        Ok(format!(
            r#"<!DOCTYPE html>
<html lang="zh">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>{page_title} - 干徒</title>
    <meta name="description" content="{page_desc}">
    <meta name="author" content="{author}">
    <link rel="canonical" href="{page_url}">
    <meta property="og:title" content="{page_title} - 干徒">
    <meta property="og:description" content="{page_desc}">
    <meta property="og:type" content="website">
    <meta property="og:url" content="{page_url}">
    <meta property="og:site_name" content="干徒的博客">
    <meta property="og:locale" content="zh_CN">
    <meta property="og:image" content="{og_image}">
    <meta property="og:image:width" content="1200">
    <meta property="og:image:height" content="630">
    <meta name="twitter:card" content="summary_large_image">
    <meta name="twitter:title" content="{page_title} - 干徒">
    <meta name="twitter:description" content="{page_desc}">
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
    <div id="main">{ssr_html}</div>
{clear_script}
{wasm_script}
</body>
</html>
"#,
            page_title = esc(&page_title),
            page_desc = esc(&page_desc),
            author = esc(author),
            page_url = page_url,
            og_image = OG_IMAGE,
            css = css,
            preloads = preloads,
            vars_style = vars_style,
            theme_script = THEME_SCRIPT,
            clear_script = CLEAR_SCRIPT,
            ssr_html = ssr_html,
            wasm_script = wasm_script,
        ))
    }

    /// 移除 Trunk 注入的 autoreload WebSocket 脚本。
    ///
    /// `trunk build` 在部分缓存/热重载组合下会把 `{{__TRUNK_ADDRESS__}}`
    /// 占位符原样留在 dist/index.html，生产环境会产生一条无意义的 WebSocket
    /// 连接错误。预渲染阶段统一剔除。
    fn strip_trunk_autoreload(html: &str) -> String {
        let mut out = String::with_capacity(html.len());
        let mut rest = html;
        while let Some(start) = rest.find("<script") {
            let Some(rel_end) = rest[start..].find("</script>") else {
                break;
            };
            let end = start + rel_end + "</script>".len();
            let block = &rest[start..end];
            if block.contains(".well-known/trunk/ws") {
                out.push_str(&rest[..start]);
                rest = &rest[end..];
            } else {
                out.push_str(&rest[..end]);
                rest = &rest[end..];
            }
        }
        out.push_str(rest);
        out
    }

    /// 给首页注入 `<noscript>` 文章列表与 ItemList JSON-LD。
    ///
    /// 只在 `/` 的 HTML 上做，SPA 渲染时 `<noscript>` 不显示，不影响交互；
    /// 未执行 JS 的爬虫仍能拿到最新文章的标题与链接。
    fn inject_home_seo(index_html: &str, posts: &[PostData]) -> String {
        const MARKER: &str = "id=\"noscript-posts\"";
        if index_html.contains(MARKER) {
            return index_html.to_string();
        }

        let latest: Vec<&PostData> = posts
            .iter()
            .filter(|p| !p.slug.is_empty() && !p.title.is_empty())
            .take(20)
            .collect();
        if latest.is_empty() {
            return index_html.to_string();
        }

        let mut list = String::new();
        let mut items = Vec::with_capacity(latest.len());
        for (i, post) in latest.iter().enumerate() {
            let date: String = post.date.chars().take(10).collect();
            list.push_str(&format!(
                "<li><a href=\"/post/{}/\">{}</a> <time datetime=\"{}\">{}</time></li>",
                esc(&post.slug),
                esc(&post.title),
                esc(&date),
                esc(&date)
            ));
            items.push(format!(
                r#"{{"@type":"ListItem","position":{},"url":"{}/post/{}","name":{}}}"#,
                i + 1,
                SITE,
                esc(&post.slug),
                json_escape(&post.title)
            ));
        }

        let block = format!(
            r#"<noscript id="noscript-posts"><section><h2>最新文章</h2><ul>{list}</ul></section></noscript>
<script type="application/ld+json">{{"@context":"https://schema.org","@type":"ItemList","itemListElement":[{items}]}}</script>"#,
            items = items.join(",")
        );

        index_html.replacen("</body>", &format!("{block}\n</body>"), 1)
    }

    /// Trunk 的 post_build hook 可能在资源 staging 完成前就启动。
    /// 现在 prerender 编译只要几秒，必须显式等待 `.stage/index.html` 出现。
    fn wait_for_dist() -> Result<PathBuf, String> {
        for _ in 0..600 {
            let stage = Path::new("dist/.stage");
            if stage.is_dir() {
                return Ok(stage.to_path_buf());
            }
            if Path::new(DIST).join("index.html").is_file() {
                return Ok(Path::new(DIST).to_path_buf());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Err("等待 dist/.stage 超时".to_string())
    }

    fn wait_for_index(dist: &Path) -> Result<(), String> {
        let index = dist.join("index.html");
        for _ in 0..600 {
            if index.is_file() {
                return Ok(());
            }
            std::thread::sleep(Duration::from_millis(100));
        }
        Err(format!("等待 {} 超时", index.display()))
    }

    pub fn run() -> Result<(), String> {
        // Trunk 的 post_build hook 在资源发布前触发：此时产物还在 dist/.stage/。
        // 写入 .stage 后由 Trunk 统一挪到 dist/，这样 hook 与手工执行都能正常工作。
        let dist = wait_for_dist()?;
        wait_for_index(&dist)?;

        let index_path = dist.join("index.html");
        let index_html =
            fs::read_to_string(&index_path).map_err(|e| format!("读取 {} 失败: {e}", index_path.display()))?;
        let (css, vars_style, wasm_script, preloads) = extract_css_and_vars(&index_html);

        // 元数据与正文都从源码树 static/ 读取（build.rs 刚生成），
        // 不依赖 Trunk 的复制顺序；只把生成结果写进 dist/。
        let posts_path = Path::new("static").join("posts-meta.json");
        let posts_json =
            fs::read_to_string(&posts_path).map_err(|e| format!("读取 {} 失败: {e}", posts_path.display()))?;
        let posts_file: PostsMeta = serde_json::from_str(&posts_json).map_err(|e| e.to_string())?;
        if posts_file.schema_version != site_model::SCHEMA_VERSION {
            eprintln!(
                "warning: posts-meta schema {} != expected {}",
                posts_file.schema_version,
                site_model::SCHEMA_VERSION
            );
        }
        let mut posts: Vec<PostData> = posts_file
            .posts
            .into_iter()
            .map(|meta| PostData { meta, content_html: String::new() })
            .collect();

        let html_dir = Path::new("static").join("post-html");
        let mut missing_html = 0usize;
        for post in &mut posts {
            if post.slug.is_empty() {
                continue;
            }
            match fs::read_to_string(html_dir.join(format!("{}.html", post.slug))) {
                Ok(html) => post.content_html = html,
                Err(_) => missing_html += 1,
            }
        }
        if missing_html > 0 {
            eprintln!("warning: {missing_html} posts have no post-html fragment");
        }

        let mut count = 0usize;
        for post in &posts {
            if post.slug.is_empty() {
                continue;
            }
            let out_dir = dist.join("post").join(&post.slug);
            fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
            let page = render_page(post, &posts, &css, &preloads, &vars_style, &wasm_script)?;
            fs::write(out_dir.join("index.html"), page).map_err(|e| e.to_string())?;
            count += 1;
        }

        // 系列文档目录页：每个 series 生成一个目录页（入口章 = order 最小的一章）
        let mut series_count = 0usize;
        let mut seen_series: BTreeSet<String> = BTreeSet::new();
        for post in &posts {
            let series_key = if !post.series.is_empty() && !post.catalog.is_empty() {
                post.catalog.clone()
            } else {
                post.series.clone()
            };
            if series_key.is_empty() || post.slug.is_empty() || !seen_series.insert(series_key.clone()) {
                continue;
            }
            let mut chapters: Vec<&PostData> = posts
                .iter()
                .filter(|q| {
                    let q_key = if !q.series.is_empty() && !q.catalog.is_empty() {
                        &q.catalog
                    } else {
                        &q.series
                    };
                    *q_key == series_key
                })
                .collect();
            chapters.sort_by(|a, b| a.sort_key.cmp(&b.sort_key).then_with(|| a.filename.cmp(&b.filename)));
            let entry = chapters[0];
            let series_id = series_key;
            let out_dir = dist.join("series").join(&series_id);
            fs::create_dir_all(&out_dir).map_err(|e| e.to_string())?;
            let page = series_page_html(entry, &chapters, &series_id, &css, &preloads, &vars_style, &wasm_script)?;
            fs::write(out_dir.join("index.html"), page).map_err(|e| e.to_string())?;
            series_count += 1;
        }

        // 首页 SEO：给 SPA 首页补一份 <noscript> 文章列表 + ItemList 结构化数据。
        // 这样禁用 JS 的爬虫/读者也能看到最新文章入口，同时不影响 SPA 渲染。
        let index_with_seo = inject_home_seo(&strip_trunk_autoreload(&index_html), &posts);
        fs::write(&index_path, index_with_seo).map_err(|e| e.to_string())?;

        // GitHub Pages SPA 兜底：未知路径加载 404.html（内容与 index.html 一致）
        fs::copy(&index_path, dist.join("404.html")).map_err(|e| e.to_string())?;

        // posts-meta 仅供构建期使用，SPA / 预渲染都不需要它，避免无谓部署
        let _ = fs::remove_file(dist.join("static").join("posts-meta.json"));

        // 生产构建不包含 Playground，dev 专用背景图无需发布
        if std::env::var("WEBSITE_PROD").as_deref() == Ok("1") {
            let _ = fs::remove_file(dist.join("static").join("images").join("playground-bg.jpg"));
        }

        println!("[prerender] generated {} post pages + {} series pages + 404.html, css={}", count, series_count, css);
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn ssr_about_uses_shared_layout() {
            let html = render_route("/about").expect("render /about");
            assert!(html.contains("navbar-content"));
            assert!(html.contains("footer-content"));
            assert!(html.contains("main-content"));
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn main() {
    if let Err(e) = prerender_impl::run() {
        eprintln!("[prerender] error: {}", e);
        std::process::exit(1);
    }
}
