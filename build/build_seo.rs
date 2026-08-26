use crate::build_common::*;
use std::path::Path;

const SITE_URL: &str = "https://ganto.me";

/// 静态页面列表（不含文章），用于 sitemap
const STATIC_PAGES: &[&str] = &[
    "/",
    "/about",
    "/tags",
    "/dev",
    "/search",
    "/knowledge-graph",
    "/ai-summary",
];

/// 生成 sitemap.xml 写入 static/ 目录（trunk 构建时会复制到 dist/static/）
pub fn generate_sitemap(posts: &[PostData]) {
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
    );
    xml.push_str(
        r#"<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">"#,
    );

    // 静态页面
    for page in STATIC_PAGES {
        xml.push_str("<url>");
        xml.push_str(&format!(
            "<loc>{}{}</loc>",
            SITE_URL,
            if *page == "/" { "/" } else { page }
        ));
        xml.push_str("<changefreq>weekly</changefreq>");
        xml.push_str("<priority>0.8</priority>");
        xml.push_str("</url>");
    }

    // 文章页面
    for post in posts {
        let loc = format!("{}/post/{}", SITE_URL, escape_xml(&post.slug));
        // 日期取前 10 位 YYYY-MM-DD（兼容 "2023-01-11 15:20:55" 格式）
        let lastmod = if post.date.len() >= 10 {
            post.date[..10].to_string()
        } else {
            post.date.clone()
        };
        xml.push_str("<url>");
        xml.push_str(&format!("<loc>{}</loc>", loc));
        xml.push_str(&format!("<lastmod>{}</lastmod>", escape_xml(&lastmod)));
        xml.push_str("<changefreq>monthly</changefreq>");
        xml.push_str("<priority>0.9</priority>");
        xml.push_str("</url>");
    }

    xml.push_str("</urlset>\n");

    let dest = Path::new("static/sitemap.xml");
    std::fs::write(dest, &xml).expect("Failed to write sitemap.xml");
    println!("sitemap.xml generated with {} urls", posts.len() + STATIC_PAGES.len());
}
