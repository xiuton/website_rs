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

fn date_key(date: &str) -> &str {
    if date.len() >= 10 { &date[..10] } else { date }
}

pub fn generate_rss_feed(posts: &[PostData], out_dir: &str) {
    // 找到第 3 个不同日期作为全文输出的分界线
    let mut distinct_dates: Vec<&str> = Vec::new();
    for post in posts.iter() {
        let dk = date_key(&post.date);
        if distinct_dates.last() != Some(&dk) {
            distinct_dates.push(dk);
        }
    }
    // 不足 3 个不同日期 → 全部全文；否则第 3 个日期及之后的都是全文
    let full_cutoff: Option<&str> = if distinct_dates.len() > 3 {
        Some(distinct_dates[2])
    } else {
        None
    };

    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom" xmlns:content="http://purl.org/rss/1.0/modules/content/">"#,
        r#"<channel>"#,
        r#"<title>干徒的博客</title>"#,
        r#"<link>https://ganto.cn</link>"#,
        r#"<description>干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。</description>"#,
        r#"<language>zh-CN</language>"#,
        r#"<atom:link href="https://ganto.cn/feed.xml" rel="self" type="application/rss+xml"/>"#,
    ));

    let total = posts.len().min(50);
    for post in posts.iter().take(total) {
        let is_full = match full_cutoff {
            Some(cutoff) => date_key(&post.date) >= cutoff,
            None => true,
        };

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

        // RSS pubDate 格式: RFC 2822（如 Mon, 29 Jan 2026 00:00:00 +0800）
        let pub_date = format_rfc2822(&post.date);
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

        // 全文输出：用 content:encoded 包裹 CDATA
        if is_full {
            let html = md_to_html(&post.content);
            xml.push_str(&format!(
                "  <content:encoded><![CDATA[{}]]></content:encoded>\n",
                html
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

