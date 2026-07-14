use crate::build_common::*;
use std::path::Path;

pub fn generate_posts_json(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/posts.json");
        std::fs::write(dest, "[]").expect("Failed to write posts json");
        return;
    }

    let mut json = String::from("[\n");

    for (idx, post) in posts.iter().enumerate() {
        let escaped_title = escape_json_string(&post.title);
        let escaped_content = escape_json_string(&post.content);
        let escaped_summary = escape_json_string(&post.summary);
        let escaped_author = escape_json_string(&post.author);
        let escaped_category = escape_json_string(&post.category);

        let tags_json: Vec<String> = post
            .tags
            .iter()
            .map(|t| format!(r#""{}""#, escape_json_string(t)))
            .collect();

        json.push_str(&format!(
            r#"  {{
    "slug": "{}",
    "title": "{}",
    "date": "{}",
    "author": "{}",
    "category": "{}",
    "tags": [{}],
    "summary": "{}",
    "content": "{}"
  }}"#,
            post.slug,
            escaped_title,
            post.date,
            escaped_author,
            escaped_category,
            tags_json.join(", "),
            escaped_summary,
            escaped_content,
        ));

        if idx < posts.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push('\n');
        }
    }

    json.push_str("]\n");

    let dest = Path::new("static/posts.json");
    std::fs::write(dest, &json).expect("Failed to write posts json");
    println!("posts.json generated with {} articles", posts.len());
}
