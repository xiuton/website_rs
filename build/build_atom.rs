use crate::build_common::*;
use std::path::Path;

/// YYYY-MM-DD → RFC 3339（Atom 1.0 要求）
fn format_rfc3339(date_str: &str) -> String {
    if date_str.len() < 10 {
        return date_str.to_string();
    }
    format!("{}T00:00:00+08:00", &date_str[..10])
}

pub fn generate_atom_feed(posts: &[PostData], out_dir: &str) {
    let updated = if let Some(latest) = posts.first() {
        format_rfc3339(&latest.date)
    } else {
        format_rfc3339("2024-01-01")
    };

    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="utf-8"?>"#,
        r#"<feed xmlns="http://www.w3.org/2005/Atom">"#,
        r#"<title>干徒的博客</title>"#,
        r#"<link href="https://ganto.me"/>"#,
        r#"<id>https://ganto.me</id>"#,
    ));

    xml.push_str(&format!(
        "<link rel=\"self\" href=\"https://ganto.me/static/atom.xml\"/>\n"
    ));
    xml.push_str(&format!(
        "<updated>{}</updated>\n",
        escape_xml(&updated)
    ));
    xml.push_str(concat!(
        "<author><name>干徒</name></author>\n",
        "<generator uri=\"https://ganto.me\">website-rs</generator>\n",
    ));

    let total = posts.len().min(50);
    for post in posts.iter().take(total) {
        xml.push_str("<entry>\n");

        xml.push_str(&format!(
            "  <title>{}</title>\n",
            escape_xml(&post.title)
        ));

        xml.push_str(&format!(
            "  <link href=\"https://ganto.me/post/{}\"/>\n",
            escape_xml(&post.slug)
        ));

        xml.push_str(&format!(
            "  <id>https://ganto.me/post/{}</id>\n",
            escape_xml(&post.slug)
        ));

        xml.push_str(&format!(
            "  <updated>{}</updated>\n",
            escape_xml(&format_rfc3339(&post.date))
        ));

        xml.push_str(&format!(
            "  <published>{}</published>\n",
            escape_xml(&format_rfc3339(&post.date))
        ));

        xml.push_str(&format!(
            "  <author><name>{}</name></author>\n",
            escape_xml(&post.author)
        ));

        // summary: 优先使用 summary，否则转 HTML 后提取纯文本前 500 字符
        let summary_text = if !post.summary.is_empty() {
            post.summary.clone()
        } else {
            let html = md_to_html(&post.content);
            let text = strip_html_tags(&html);
            text.chars().take(500).collect()
        };
        xml.push_str(&format!(
            "  <summary><![CDATA[{}]]></summary>\n",
            summary_text
        ));

        // content: 全文 HTML，CDATA 包裹
        let html = md_to_html(&post.content);
        xml.push_str(&format!(
            "  <content type=\"html\"><![CDATA[{}]]></content>\n",
            html
        ));

        for tag in &post.tags {
            xml.push_str(&format!(
                "  <category term=\"{}\"/>\n",
                escape_xml(tag)
            ));
        }

        xml.push_str("</entry>\n");
    }

    xml.push_str("</feed>\n");

    let feed_path = std::path::Path::new(out_dir).join("atom.xml");
    std::fs::write(&feed_path, &xml).expect("Failed to write Atom feed");

    // 复制到 static 目录供 Trunk 打包
    let dest_feed = Path::new("static/atom.xml");
    std::fs::write(dest_feed, &xml).expect("Failed to write atom.xml to static/");
}
