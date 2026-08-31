use crate::build_common::*;
use comrak::ComrakOptions;
use regex::Regex;
use std::path::Path;

fn markdown_to_html(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some(String::from("user-content-"));
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.render.unsafe_ = true;
    comrak::markdown_to_html(markdown, &options)
}

fn markdown_to_mini_program_html(markdown: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.extension.tagfilter = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = None;
    options.extension.footnotes = false;
    options.extension.description_lists = false;
    options.render.unsafe_ = false;
    let html = comrak::markdown_to_html(markdown, &options);
    sanitize_for_mini_program(&html)
}

fn sanitize_for_mini_program(html: &str) -> String {
    let mut result = html.to_string();
    result = result.replace("aria-hidden=\"true\"", "");
    result = result.replace("class=\"anchor\"", "");
    result = result.replace("class=\"footnote-ref\"", "");
    result = result.replace("class=\"footnote-definition\"", "");
    result = result.replace("class=\"footnote-body\"", "");
    result = result.replace("class=\"footnote-backref\"", "");
    result = result.replace("data-footnote-ref", "");
    result = result.replace("id=\"user-content-", "id=\"");

    let empty_anchor = Regex::new(r#"<a\s*name="[^"]*"\s*/?>"#).unwrap();
    result = empty_anchor.replace_all(&result, "").to_string();

    let script_tag = Regex::new(r#"<script[^>]*>.*?</script>"#).unwrap();
    result = script_tag.replace_all(&result, "").to_string();

    let style_tag = Regex::new(r#"<style[^>]*>.*?</style>"#).unwrap();
    result = style_tag.replace_all(&result, "").to_string();

    let iframe_tag = Regex::new(r#"<iframe[^>]*>.*?</iframe>"#).unwrap();
    result = iframe_tag.replace_all(&result, "").to_string();

    let svg_tag = Regex::new(r#"<svg[^>]*>.*?</svg>"#).unwrap();
    result = svg_tag.replace_all(&result, "").to_string();

    let canvas_tag = Regex::new(r#"<canvas[^>]*>.*?</canvas>"#).unwrap();
    result = canvas_tag.replace_all(&result, "").to_string();

    let video_tag = Regex::new(r#"<video[^>]*>.*?</video>"#).unwrap();
    result = video_tag.replace_all(&result, "").to_string();

    let audio_tag = Regex::new(r#"<audio[^>]*>.*?>"#).unwrap();
    result = audio_tag.replace_all(&result, "").to_string();

    let embed_tag = Regex::new(r#"<embed[^>]*>"#).unwrap();
    result = embed_tag.replace_all(&result, "").to_string();

    let object_tag = Regex::new(r#"<object[^>]*>.*?</object>"#).unwrap();
    result = object_tag.replace_all(&result, "").to_string();

    let param_tag = Regex::new(r#"<param[^>]*>"#).unwrap();
    result = param_tag.replace_all(&result, "").to_string();

    let applet_tag = Regex::new(r#"<applet[^>]*>.*?</applet>"#).unwrap();
    result = applet_tag.replace_all(&result, "").to_string();

    let frame_tag = Regex::new(r#"<frame[^>]*>"#).unwrap();
    result = frame_tag.replace_all(&result, "").to_string();

    let frameset_tag = Regex::new(r#"<frameset[^>]*>.*?</frameset>"#).unwrap();
    result = frameset_tag.replace_all(&result, "").to_string();

    result = result.replace("<hr>", "<hr/>");
    result = result.replace("<br>", "<br/>");

    let a_tag = Regex::new(r#"<a([^>]*)>(.*?)</a>"#).unwrap();
    result = a_tag.replace_all(&result, |caps: &regex::Captures| {
        let attrs = &caps[1];
        let content = &caps[2];
        if attrs.contains("href=") {
            format!("<navigator{}>{}</navigator>", attrs.replace("href=", "url="), content)
        } else {
            format!("<span>{}</span>", content)
        }
    }).to_string();

    let img_tag = Regex::new(r#"<img([^>]*)>"#).unwrap();
    result = img_tag.replace_all(&result, |caps: &regex::Captures| {
        let attrs = &caps[1];
        if attrs.contains("src=") {
            format!("<img{} mode=\"widthFix\"/>", attrs)
        } else {
            "".to_string()
        }
    }).to_string();

    let table_tag = Regex::new(r#"<table([^>]*)>"#).unwrap();
    result = table_tag.replace_all(&result, "<table$1 style=\"width:100%;border-collapse:collapse;\">").to_string();

    let td_tag = Regex::new(r#"<td([^>]*)>"#).unwrap();
    result = td_tag.replace_all(&result, "<td$1 style=\"border:1px solid #e0e0e0;padding:8px;box-sizing:border-box;\">").to_string();

    let th_tag = Regex::new(r#"<th([^>]*)>"#).unwrap();
    result = th_tag.replace_all(&result, "<th$1 style=\"border:1px solid #e0e0e0;padding:8px;box-sizing:border-box;background:#f5f5f5;\">").to_string();

    result = result.replace("<div", "<view");
    result = result.replace("</div>", "</view>");
    result = result.replace("<span", "<text");
    result = result.replace("</span>", "</text>");

    result = result.replace("<strong>", "<text style=\"font-weight:bold;\">");
    result = result.replace("</strong>", "</text>");
    result = result.replace("<b>", "<text style=\"font-weight:bold;\">");
    result = result.replace("</b>", "</text>");
    result = result.replace("<em>", "<text style=\"font-style:italic;\">");
    result = result.replace("</em>", "</text>");
    result = result.replace("<i>", "<text style=\"font-style:italic;\">");
    result = result.replace("</i>", "</text>");
    result = result.replace("<del>", "<text style=\"text-decoration:line-through;\">");
    result = result.replace("</del>", "</text>");
    result = result.replace("<s>", "<text style=\"text-decoration:line-through;\">");
    result = result.replace("</s>", "</text>");

    let code_tag = Regex::new(r#"<code([^>]*)>(.*?)</code>"#).unwrap();
    result = code_tag.replace_all(&result, |caps: &regex::Captures| {
        let attrs = &caps[1];
        let content = &caps[2];
        format!("<text{} style=\"font-family:monospace;background:#f4f4f4;padding:2px 4px;border-radius:3px;\">{}</text>", attrs, content)
    }).to_string();

    let pre_tag = Regex::new(r#"<pre([^>]*)>(.*?)</pre>"#).unwrap();
    result = pre_tag.replace_all(&result, |caps: &regex::Captures| {
        let attrs = &caps[1];
        let content = &caps[2];
        format!("<view{} style=\"font-family:monospace;background:#f4f4f4;padding:12px;border-radius:8px;overflow-x:auto;\">{}</view>", attrs, content)
    }).to_string();

    let blockquote_tag = Regex::new(r#"<blockquote([^>]*)>(.*?)</blockquote>"#).unwrap();
    result = blockquote_tag.replace_all(&result, |caps: &regex::Captures| {
        let attrs = &caps[1];
        let content = &caps[2];
        format!("<view{} style=\"border-left:4px solid #6a9b50;padding-left:16px;background:#f5f5f5;padding:12px 16px;margin:16px 0;border-radius:0 8px 8px 0;\">{}</view>", attrs, content)
    }).to_string();

    result
}

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
        let escaped_content_html = escape_json_string(&markdown_to_html(&post.content));
        let escaped_content_mini = escape_json_string(&markdown_to_mini_program_html(&post.content));
        let escaped_summary = escape_json_string(&post.summary);
        let escaped_author = escape_json_string(&post.author);
        let escaped_category = escape_json_string(&post.category);
        let escaped_series = escape_json_string(&post.series);
        let escaped_catalog = escape_json_string(&post.catalog);

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
    "series": "{}",
    "order": {},
    "catalog": "{}",
    "content": "{}",
    "content_html": "{}",
    "content_mini": "{}"
  }}"#,
            post.slug,
            escaped_title,
            post.date,
            escaped_author,
            escaped_category,
            tags_json.join(", "),
            escaped_summary,
            escaped_series,
            post.order,
            escaped_catalog,
            escaped_content,
            escaped_content_html,
            escaped_content_mini,
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
