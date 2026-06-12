use crate::build_common::*;
use std::path::Path;

pub fn generate_rss_feed(posts: &[PostData], out_dir: &str) {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">"#,
        r#"<channel>"#,
        r#"<title>干徒的博客</title>"#,
        r#"<link>https://ganto.cn</link>"#,
        r#"<description>干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。</description>"#,
        r#"<language>zh-CN</language>"#,
        r#"<atom:link href="https://ganto.cn/feed.xml" rel="self" type="application/rss+xml"/>"#,
    ));

    for post in posts.iter().take(20) {
        xml.push_str("<item>\n");
        xml.push_str(&format!(
            "  <title>{}</title>\n",
            escape_xml(&post.title)
        ));
        xml.push_str(&format!(
            "  <link>https://ganto.cn/post/{}</link>\n",
            escape_xml(&post.slug)
        ));
        xml.push_str("  <guid isPermaLink=\"true\">");
        xml.push_str(&format!(
            "https://ganto.cn/post/{}",
            escape_xml(&post.slug)
        ));
        xml.push_str("</guid>\n");

        // RSS pubDate 格式: RFC 2822
        let pub_date = if post.date.len() >= 10 {
            format!("{} 00:00:00 +0800", &post.date[..10])
        } else {
            post.date.clone()
        };
        xml.push_str(&format!("  <pubDate>{}</pubDate>\n", escape_xml(&pub_date)));

        xml.push_str(&format!(
            "  <author>{}</author>\n",
            escape_xml(&post.author)
        ));

        if !post.summary.is_empty() {
            xml.push_str(&format!(
                "  <description>{}</description>\n",
                escape_xml(&post.summary)
            ));
        } else {
            // 截取正文前 300 字作摘要
            let excerpt: String = post.content
                .chars()
                .take(300)
                .collect();
            xml.push_str(&format!(
                "  <description>{}</description>\n",
                escape_xml(&excerpt)
            ));
        }

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

