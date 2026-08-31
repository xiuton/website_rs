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

    /// 从 dist/index.html 提取内置 CSS 文件名与 <style> 块（主题变量）
    fn extract_css_and_vars(index_html: &str) -> (String, String) {
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

        // 内置样式块：<style>...</style>
        let vars_style = if let Some(start) = index_html.find("<style>") {
            let after = &index_html[start + "<style>".len()..];
            if let Some(end) = after.find("</style>") {
                after[..end].to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        (css, vars_style)
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
    fn render_page(post: &PostJson, all_posts: &[PostJson], css: &str, vars_style: &str) -> String {
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
    <style>
{vars_style}
    </style>
    <style>
        .prerender-nav {{
            display: flex; align-items: center; justify-content: space-between;
            gap: 1rem; padding: 0.9rem 1.25rem; flex-wrap: wrap;
            background: var(--bg-elevated); border-bottom: 1px solid var(--border-default);
        }}
        .prerender-nav .site-title {{ font-weight: 700; color: var(--text-primary); text-decoration: none; font-size: 1.05rem; }}
        .prerender-nav .site-title:hover {{ color: var(--accent); }}
        .prerender-nav .nav-links {{ display: flex; gap: 1rem; }}
        .prerender-nav .nav-links a {{ color: var(--text-secondary); text-decoration: none; font-size: 0.9rem; }}
        .prerender-nav .nav-links a:hover {{ color: var(--accent); }}
        .prerender-article {{ max-width: 760px; margin: 0 auto; padding: 1rem 1.25rem 3rem; }}
        .prerender-article h1 {{ font-size: 1.8rem; margin: 1.25rem 0 0.5rem; color: var(--text-primary); }}
        .prerender-meta {{ display: flex; gap: 1rem; flex-wrap: wrap; color: var(--text-tertiary); font-size: 0.9rem; margin-bottom: 0.5rem; }}
        .prerender-footer {{ text-align: center; padding: 2rem 1rem; color: var(--text-tertiary); border-top: 1px solid var(--border-default); font-size: 0.85rem; }}
    </style>
    <script>
        (function() {{
            var theme = localStorage.getItem('theme');
            if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {{
                document.documentElement.classList.add('dark');
            }}
        }})();
    </script>
</head>
<body>
    <div class="app">
        <nav class="prerender-nav">
            <a class="site-title" href="/">干徒</a>
            <div class="nav-links">
                <a href="/">首页</a>
                <a href="/about">关于</a>
                <a href="/tags">书签</a>
                <a href="/search">搜索</a>
            </div>
        </nav>
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
        <footer class="prerender-footer">
            © 干徒 (Ganto) · <a href="/">返回首页</a>
        </footer>
    </div>
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
            vars_style = vars_style,
            date = esc(&date),
            tag_html = tag_html,
            content_html = content_html,
            series_html = series_html,
        )
    }

    /// 生成系列（多章节文档）目录页的预渲染 HTML
    fn series_page_html(
        entry: &PostJson,
        chapters: &[&PostJson],
        series_id: &str,
        css: &str,
        vars_style: &str,
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
    <style>
{vars_style}
    </style>
    <style>
        .prerender-nav {{
            display: flex; align-items: center; justify-content: space-between;
            gap: 1rem; padding: 0.9rem 1.25rem; flex-wrap: wrap;
            background: var(--bg-elevated); border-bottom: 1px solid var(--border-default);
        }}
        .prerender-nav .site-title {{ font-weight: 700; color: var(--text-primary); text-decoration: none; font-size: 1.05rem; }}
        .prerender-nav .site-title:hover {{ color: var(--accent); }}
        .prerender-nav .nav-links {{ display: flex; gap: 1rem; }}
        .prerender-nav .nav-links a {{ color: var(--text-secondary); text-decoration: none; font-size: 0.9rem; }}
        .prerender-nav .nav-links a:hover {{ color: var(--accent); }}
        .prerender-footer {{ text-align: center; padding: 2rem 1rem; color: var(--text-tertiary); border-top: 1px solid var(--border-default); font-size: 0.85rem; }}
    </style>
    <script>
        (function() {{
            var theme = localStorage.getItem('theme');
            if (theme === 'dark' || (!theme && window.matchMedia('(prefers-color-scheme: dark)').matches)) {{
                document.documentElement.classList.add('dark');
            }}
        }})();
    </script>
</head>
<body>
    <div class="app">
        <nav class="prerender-nav">
            <a class="site-title" href="/">干徒</a>
            <div class="nav-links">
                <a href="/">首页</a>
                <a href="/about">关于</a>
                <a href="/tags">书签</a>
                <a href="/search">搜索</a>
            </div>
        </nav>
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
        <footer class="prerender-footer">
            © 干徒 (Ganto) · <a href="/">返回首页</a>
        </footer>
    </div>
</body>
</html>
"#,
            series = esc(&entry.series),
            desc = esc(&desc),
            author = esc(author),
            url = url,
            og_image = OG_IMAGE,
            css = css,
            vars_style = vars_style,
            total = total,
            first_slug = first_slug,
            last_date = esc(last_date),
            list = list,
        )
    }

    pub fn run() -> Result<(), String> {
        let dist = Path::new(DIST);
        if !dist.is_dir() {
            return Err(format!("dist directory not found: {}", DIST));
        }

        let index_path = dist.join("index.html");
        let index_html = fs::read_to_string(&index_path).map_err(|e| e.to_string())?;
        let (css, vars_style) = extract_css_and_vars(&index_html);

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
            let page = render_page(post, &posts, &css, &vars_style);
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
            let page = series_page_html(entry, &chapters, &series_id, &css, &vars_style);
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
