use crate::build_common::*;
use std::path::Path;
use comrak::ComrakOptions;

fn format_rfc2822(date_str: &str) -> String {
    // 尝试解析 YYYY-MM-DD 格式的日期，转为 RFC 2822
    if date_str.len() < 10 {
        return date_str.to_string();
    }

    let parts: Vec<&str> = date_str[..10].split('-').collect();
    if parts.len() != 3 {
        return date_str.to_string();
    }

    let year: i32 = match parts[0].parse() { Ok(y) => y, Err(_) => return date_str.to_string() };
    let month: u32 = match parts[1].parse() { Ok(m) => m, Err(_) => return date_str.to_string() };
    let day: u32 = match parts[2].parse() { Ok(d) => d, Err(_) => return date_str.to_string() };

    let month_names = ["Jan", "Feb", "Mar", "Apr", "May", "Jun",
                       "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    let month_abbr = if month >= 1 && month <= 12 {
        month_names[(month - 1) as usize]
    } else {
        return date_str.to_string();
    };

    // 计算星期几（Zeller 公式简化版）
    let (m, y) = if month <= 2 { (month + 12, year - 1) } else { (month, year) };
    let q = day;
    let k = (y % 100) as u32;
    let j = (y / 100) as u32;
    let h = (q + (13 * (m + 1)) / 5 + k + k / 4 + j / 4 + 5 * j) % 7;
    let weekdays = ["Sat", "Sun", "Mon", "Tue", "Wed", "Thu", "Fri"];

    format!("{}, {:02} {} {:04} 00:00:00 +0800", weekdays[h as usize], day, month_abbr, year)
}

fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    comrak::markdown_to_html(md, &options)
}

pub fn generate_rss_feed(posts: &[PostData], out_dir: &str) {
    let last_build_date = if let Some(latest) = posts.first() {
        format_rfc2822(&latest.date)
    } else {
        format_rfc2822("2024-01-01")
    };

    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">"#,
        r#"<channel>"#,
        r#"<title>干徒的博客</title>"#,
        r#"<link>https://ganto.me</link>"#,
        r#"<description>干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。</description>"#,
        r#"<language>zh-CN</language>"#,
        r#"<docs>https://www.rssboard.org/rss-specification</docs>"#,
        r#"<generator>website-rs</generator>"#,
    ));

    xml.push_str(&format!(
        "  <lastBuildDate>{}</lastBuildDate>\n",
        escape_xml(&last_build_date)
    ));
    xml.push_str(&format!(
        "  <pubDate>{}</pubDate>\n",
        escape_xml(&last_build_date)
    ));

    xml.push_str(concat!(
        r#"<atom:link href="https://ganto.me/static/feed.xml" rel="self" type="application/rss+xml"/>"#,
    ));

    let total = posts.len().min(50);
    for post in posts.iter().take(total) {
        xml.push_str("<item>\n");
        xml.push_str(&format!(
            "  <title>{}</title>\n",
            escape_xml(&post.title)
        ));
        xml.push_str(&format!(
            "  <link>https://ganto.me/post/{}</link>\n",
            escape_xml(&post.slug)
        ));
        xml.push_str("  <guid isPermaLink=\"true\">");
        xml.push_str(&format!(
            "https://ganto.me/post/{}",
            escape_xml(&post.slug)
        ));
        xml.push_str("</guid>\n");

        // RSS pubDate 格式: RFC 2822（如 Mon, 29 Jan 2026 00:00:00 +0800）
        let pub_date = format_rfc2822(&post.date);
        xml.push_str(&format!("  <pubDate>{}</pubDate>\n", escape_xml(&pub_date)));

        // RSS 2.0 规范要求 author 使用 email 格式: email@domain (Name)
        xml.push_str(&format!(
            "  <author>i@ganto.me ({})</author>\n",
            escape_xml(&post.author)
        ));

        // description: 优先使用 summary，否则取正文前 500 字符
        let description_text = if !post.summary.is_empty() {
            post.summary.clone()
        } else {
            post.content.chars().take(500).collect()
        };
        xml.push_str(&format!(
            "  <description>{}</description>\n",
            escape_xml(&description_text)
        ));

        // content:encoded: 全文 HTML，XML 实体编码（不用 CDATA，避免校验器误判代码片段为 HTML 标签）
        let html = md_to_html(&post.content);
        xml.push_str(&format!(
            "  <content:encoded>{}</content:encoded>\n",
            escape_xml(&html)
        ));

        for tag in &post.tags {
            xml.push_str(&format!(
                "  <category>{}</category>\n",
                escape_xml(tag)
            ));
        }
        xml.push_str("</item>\n");
    }

    xml.push_str("</channel>\n</rss>\n");

    let feed_path = std::path::Path::new(out_dir).join("feed.xml");
    std::fs::write(&feed_path, &xml).expect("Failed to write RSS feed");

    // 复制到 static 目录供 Trunk 打包
    let dest_feed = Path::new("static/feed.xml");
    std::fs::write(dest_feed, &xml).expect("Failed to write feed.xml to static/");
}

