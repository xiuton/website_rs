use web_sys::window;

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

/// 设置所有 SEO 相关 meta 标签
pub fn set_seo(title: &str, description: &str, url: &str) {
    set_page_title(title);

    let doc_title = format!("{} - 干徒", title);
    // 标准 meta
    set_meta("name", "description", description);

    // Open Graph
    set_meta("property", "og:title", &doc_title);
    set_meta("property", "og:description", description);
    set_meta("property", "og:type", "article");
    set_meta("property", "og:url", url);

    // Twitter Card
    set_meta("name", "twitter:title", &doc_title);
    set_meta("name", "twitter:description", description);
}

/// 重置 SEO 为网站默认值
pub fn reset_seo_default() {
    let title = "干徒 - 开发爱好者";
    let desc = "干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。";
    set_page_title(title);
    set_meta("name", "description", desc);
    set_meta("property", "og:title", title);
    set_meta("property", "og:description", desc);
    set_meta("property", "og:type", "website");
    set_meta("name", "twitter:title", title);
    set_meta("name", "twitter:description", desc);
}