//! 文章静态预渲染工具（SEO 优化）
//!
//! 在 `trunk build` 之后运行，用法：
//!   cargo run --release --bin prerender
//!
//! 功能：
//! 1. 读取 dist/static/posts.json（含 content_html 全文），为每篇文章生成
//!    dist/post/<slug>/index.html —— 爬虫无需执行 JS 即可读取完整内容
//! 2. 将 dist/index.html 复制为 dist/404.html（GitHub Pages SPA 兜底）

#[cfg(not(target_arch = "wasm32"))]
mod prerender_impl {
    use serde::Deserialize;
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

    /// 生成单篇文章的预渲染 HTML
    fn render_page(post: &PostJson, css: &str, vars_style: &str) -> String {
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
            let page = render_page(post, &css, &vars_style);
            fs::write(out_dir.join("index.html"), page).map_err(|e| e.to_string())?;
            count += 1;
        }

        // GitHub Pages SPA 兜底：未知路径加载 404.html（内容与 index.html 一致）
        fs::copy(&index_path, dist.join("404.html")).map_err(|e| e.to_string())?;

        println!("[prerender] generated {} static post pages + 404.html, css={}", count, css);
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
