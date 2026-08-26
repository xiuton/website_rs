use web_sys::window;

/// 站点根 URL（OG / canonical 使用）
const SITE_URL: &str = "https://ganto.me";
/// 默认分享图
const OG_IMAGE: &str = "https://ganto.me/static/images/og-image.png";

pub fn set_page_title(title: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            document.set_title(title);
        }
    }
}

/// 更新或创建 meta 标签的 content 属性
fn set_meta(attr_name: &str, attr_value: &str, content: &str) {
    let Some(document) = window().and_then(|w| w.document()) else { return };
    // 优先查找已有标签并更新
    let selector = match attr_name {
        "name" => format!("meta[name=\"{}\"]", attr_value),
        "property" => format!("meta[property=\"{}\"]", attr_value),
        _ => return,
    };
    if let Some(el) = document.query_selector(&selector).ok().flatten() {
        let _ = el.set_attribute("content", content);
        return;
    }
    // 不存在则创建新标签
    if let Ok(el) = document.create_element("meta") {
        let _ = el.set_attribute(attr_name, attr_value);
        let _ = el.set_attribute("content", content);
        if let Some(head) = document.head() {
            let _ = head.append_child(&el);
        }
    }
}

/// 更新或创建 canonical link 标签
fn set_canonical(url: &str) {
    let Some(document) = window().and_then(|w| w.document()) else { return };
    if let Some(el) = document.query_selector("link[rel=\"canonical\"]").ok().flatten() {
        let _ = el.set_attribute("href", url);
        return;
    }
    if let Ok(el) = document.create_element("link") {
        let _ = el.set_attribute("rel", "canonical");
        let _ = el.set_attribute("href", url);
        if let Some(head) = document.head() {
            let _ = head.append_child(&el);
        }
    }
}

/// 仅同步 canonical 与 og:url 为当前页面路径（供 Layout 全局调用）。
/// 只接受相对路径，例如 "/about"；不会覆盖页面级 title/description。
pub fn set_page_canonical(path: &str) {
    let path = if path.is_empty() || path == "/" { "/".to_string() } else { path.to_string() };
    let full_url = format!("{}{}", SITE_URL, path);
    set_canonical(&full_url);
    set_meta("property", "og:url", &full_url);
}

/// 更新指定 id 的 JSON-LD 结构化数据（不存在则创建）
fn set_jsonld(id: &str, json: &str) {
    let Some(document) = window().and_then(|w| w.document()) else { return };
    let selector = format!("script#{}", id);
    if let Some(el) = document.query_selector(&selector).ok().flatten() {
        let _ = el.set_text_content(Some(json));
        return;
    }
    if let Ok(el) = document.create_element("script") {
        let _ = el.set_attribute("type", "application/ld+json");
        let _ = el.set_attribute("id", id);
        let _ = el.set_text_content(Some(json));
        if let Some(head) = document.head() {
            let _ = head.append_child(&el);
        }
    }
}

/// 转义 JSON 字符串值
fn json_str(s: &str) -> String {
    serde_json::to_string(s).unwrap_or_else(|_| "\"\"".to_string())
}

/// 设置所有 SEO 相关 meta 标签 + canonical + 通用 WebPage 结构化数据
pub fn set_seo(title: &str, description: &str, url: &str) {
    let full_url = if url.starts_with("http") { url.to_string() } else { format!("{}{}", SITE_URL, url) };
    let doc_title = format!("{} - 干徒", title);

    set_page_title(&doc_title);
    set_canonical(&full_url);

    // 标准 meta
    set_meta("name", "description", description);

    // Open Graph
    set_meta("property", "og:title", &doc_title);
    set_meta("property", "og:description", description);
    set_meta("property", "og:type", "article");
    set_meta("property", "og:url", &full_url);
    set_meta("property", "og:image", OG_IMAGE);
    set_meta("property", "og:image:alt", title);

    // Twitter Card
    set_meta("name", "twitter:card", "summary_large_image");
    set_meta("name", "twitter:title", &doc_title);
    set_meta("name", "twitter:description", description);
    set_meta("name", "twitter:image", OG_IMAGE);

    // 通用 WebPage 结构化数据
    let jsonld = format!(
        r#"{{"@context":"https://schema.org","@type":"WebPage","name":{},"description":{},"url":{},"inLanguage":"zh-CN"}}"#,
        json_str(&doc_title),
        json_str(description),
        json_str(&full_url),
    );
    set_jsonld("seo-page-jsonld", &jsonld);
}

/// 设置文章页的 BlogPosting 结构化数据
pub fn set_article_jsonld(
    title: &str,
    description: &str,
    url: &str,
    date: &str,
    author: &str,
    tags: &[&str],
) {
    let full_url = if url.starts_with("http") { url.to_string() } else { format!("{}{}", SITE_URL, url) };
    let jsonld = format!(
        r#"{{"@context":"https://schema.org","@type":"BlogPosting","headline":{},"description":{},"url":{},"datePublished":{},"dateModified":{},"inLanguage":"zh-CN","image":{},"author":{{"@type":"Person","name":{}}},"keywords":{},"publisher":{{"@type":"Organization","name":"干徒的博客","url":"https://ganto.me/"}}}}"#,
        json_str(title),
        json_str(description),
        json_str(&full_url),
        json_str(date),
        json_str(date),
        json_str(OG_IMAGE),
        json_str(author),
        json_str(&tags.join(",")),
    );
    set_jsonld("seo-article-jsonld", &jsonld);
}

/// 重置 SEO 为网站默认值（首页）
pub fn reset_seo_default() {
    let title = "干徒 - 开发爱好者";
    let desc = "干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。";
    let home_url = format!("{}/", SITE_URL);

    set_page_title(title);
    set_canonical(&home_url);
    set_meta("name", "description", desc);
    set_meta("property", "og:title", title);
    set_meta("property", "og:description", desc);
    set_meta("property", "og:type", "website");
    set_meta("property", "og:url", &home_url);
    set_meta("property", "og:image", OG_IMAGE);
    set_meta("property", "og:image:alt", "干徒 Ganto 的个人技术博客");
    set_meta("name", "twitter:card", "summary_large_image");
    set_meta("name", "twitter:title", title);
    set_meta("name", "twitter:description", desc);
    set_meta("name", "twitter:image", OG_IMAGE);

    let jsonld = format!(
        r#"{{"@context":"https://schema.org","@type":"WebPage","name":{},"description":{},"url":{},"inLanguage":"zh-CN"}}"#,
        json_str(title),
        json_str(desc),
        json_str(&home_url),
    );
    set_jsonld("seo-page-jsonld", &jsonld);
}
