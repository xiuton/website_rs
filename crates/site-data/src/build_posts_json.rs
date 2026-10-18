use comrak::Options;
use regex::Regex;
use site_content::*;
use site_model::{PostMeta, PostsMeta};
use std::path::Path;

fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_id_prefix = Some(String::from("user-content-"));
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.render.r#unsafe = true;
    comrak::markdown_to_html(markdown, &options)
}

/// 扫描 `<pre lang="XXX">`（comrak 输出），注入 `data-lang="XXX"`。
/// 无 `lang` 属性的 `<pre>`（plaintext 块）不设标签。
fn inject_data_lang(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut rest = html;

    while let Some(pos) = rest.find("<pre lang=\"") {
        result.push_str(&rest[..pos]);
        let after = &rest[pos + "<pre lang=\"".len()..];
        let end = after.find('"').unwrap_or(0);
        let lang = &after[..end];
        use std::fmt::Write;
        let _ = write!(result, "<pre lang=\"{lang}\" data-lang=\"{lang}\"");
        rest = &after[end + 1..];
    }
    result.push_str(rest);
    result
}

/// 为 `<img>` 标签添加 loading="lazy" 属性（如尚未设置）
fn add_lazy_loading(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut last_end = 0;

    for (start, _) in html.match_indices("<img ") {
        let tag_start = start;
        let tag_end = if let Some(end) = html[tag_start..].find('>') {
            tag_start + end
        } else {
            continue;
        };

        result.push_str(&html[last_end..tag_start]);

        let tag = &html[tag_start..tag_end];
        if !tag.contains("loading=") {
            result.push_str(&html[tag_start..tag_start + 5]); // "<img "
            result.push_str("loading=\"lazy\" ");
            result.push_str(&html[tag_start + 5..tag_end]);
        } else {
            result.push_str(tag);
        }

        last_end = tag_end;
    }

    result.push_str(&html[last_end..]);
    result
}

/// 构建期完成正文 HTML 的全部加工（语言标记、图片懒加载）。
///
/// 以前这一步在浏览器里做（`prepare_blog_html`），意味着 wasm 必须打包 comrak
/// 和全部文章原文；现在只保留一份构建期结果，预渲染页和 SPA 看到的 HTML 完全一致。
pub fn prepare_content_html(markdown: &str) -> String {
    let html = markdown_to_html(markdown);
    let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
    let html = inject_data_lang(&html);
    add_lazy_loading(&html)
}

/// 从 Markdown 生成纯文本摘要（front matter 没写 summary 时的兜底）。
pub fn excerpt_from_markdown(markdown: &str, max_chars: usize) -> String {
    let html = markdown_to_html(markdown);
    let tag = Regex::new(r"<[^>]*>").expect("valid tag regex");
    let text = tag.replace_all(&html, " ");
    let text = text
        .replace("&amp;", "&")
        .replace("&lt;", "<")
        .replace("&gt;", ">")
        .replace("&quot;", "\"")
        .replace("&#39;", "'")
        .replace("&nbsp;", " ");
    let collapsed = text.split_whitespace().collect::<Vec<_>>().join(" ");
    collapsed.chars().take(max_chars).collect()
}

fn markdown_to_mini_program_html(markdown: &str) -> String {
    let mut options = Options::default();
    options.extension.autolink = true;
    options.extension.strikethrough = true;
    options.extension.table = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_id_prefix = None;
    options.extension.footnotes = false;
    options.extension.description_lists = false;
    options.render.r#unsafe = false;
    let html = comrak::markdown_to_html(markdown, &options);
    // 标签改写交给 tools/mini-html（lol_html 解析器），不再用正则拼接
    mini_html::sanitize_for_mini_program(&html)
}
/// 是否额外产出旧格式的 `posts.json`（含原始 Markdown / HTML / 小程序 HTML 三份正文）。
///
/// 默认关闭：站点自身已经改为「元数据 + 按需 HTML 片段」，不再需要这个 6 MB 的文件。
/// 如果站外还有小程序等消费者依赖 `/static/posts.json`，构建时设置
/// `WEBSITE_LEGACY_POSTS_JSON=1` 即可恢复原产物。
fn legacy_posts_json_enabled() -> bool {
    matches!(std::env::var("WEBSITE_LEGACY_POSTS_JSON").as_deref(), Ok("1") | Ok("true"))
}

fn post_meta(post: &PostData) -> PostMeta {
    PostMeta {
        slug: post.slug.clone(),
        title: post.title.clone(),
        date: post.date.clone(),
        author: post.author.clone(),
        category: post.category.clone(),
        tags: post.tags.clone(),
        summary: post.summary.clone(),
        series: post.series.clone(),
        order: post.order,
        sort_key: post.sort_key.clone(),
        catalog: post.catalog.clone(),
        filename: post.filename.clone(),
        series_summary: post.series_summary.clone(),
    }
}

pub fn generate_posts_json(posts: &[PostData]) {
    // 每篇文章一份 HTML 片段：SPA 按需加载，避免把全部正文塞进 wasm。
    // 先清空目录，防止已删除文章留下孤儿文件。
    let html_dir = Path::new("static/post-html");
    if html_dir.exists() {
        if let Err(e) = std::fs::remove_dir_all(html_dir) {
            println!("cargo:warning=清理 {} 失败: {e}", html_dir.display());
        }
    }
    if let Err(e) = std::fs::create_dir_all(html_dir) {
        println!("cargo:warning=创建 {} 失败: {e}", html_dir.display());
    }

    let legacy = legacy_posts_json_enabled();

    if posts.is_empty() {
        let empty = serde_json::to_string(&PostsMeta::new(Vec::new())).expect("serialize empty posts meta");
        std::fs::write("static/posts-meta.json", empty).expect("Failed to write posts meta");
        if legacy {
            std::fs::write("static/posts.json", "[]").expect("Failed to write posts json");
        } else {
            let _ = std::fs::remove_file("static/posts.json");
        }
        return;
    }

    let mut meta_posts: Vec<PostMeta> = Vec::with_capacity(posts.len());
    let mut legacy_entries: Vec<String> = Vec::with_capacity(posts.len());

    for post in posts {
        let content_html = prepare_content_html(&post.content);
        let file_name = post.slug.replace(['/', '\\'], "-");
        std::fs::write(html_dir.join(format!("{file_name}.html")), &content_html)
            .unwrap_or_else(|e| panic!("写入 {file_name}.html 失败: {e}"));

        meta_posts.push(post_meta(post));

        if legacy {
            let tags_json: Vec<String> = post.tags.iter().map(|t| format!(r#""{}""#, escape_json_string(t))).collect();
            legacy_entries.push(format!(
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
    "sort_key": "{}",
    "catalog": "{}",
    "filename": "{}",
    "series_summary": "{}",
    "content": "{}",
    "content_html": "{}",
    "content_mini": "{}"
  }}"#,
                post.slug,
                escape_json_string(&post.title),
                post.date,
                escape_json_string(&post.author),
                escape_json_string(&post.category),
                tags_json.join(", "),
                escape_json_string(&post.summary),
                escape_json_string(&post.series),
                post.order,
                escape_json_string(&post.sort_key),
                escape_json_string(&post.catalog),
                escape_json_string(&post.filename),
                escape_json_string(&post.series_summary),
                escape_json_string(&post.content),
                escape_json_string(&content_html),
                escape_json_string(&markdown_to_mini_program_html(&post.content)),
            ));
        }
    }

    let meta_json = serde_json::to_string(&PostsMeta::new(meta_posts)).expect("serialize posts meta");
    std::fs::write("static/posts-meta.json", meta_json).expect("Failed to write posts meta");

    if legacy {
        let legacy_json = format!("[\n{}\n]\n", legacy_entries.join(",\n"));
        std::fs::write("static/posts.json", legacy_json).expect("Failed to write posts json");
    } else {
        // 清理上一次构建残留的旧产物，避免它继续被复制进 dist/
        let _ = std::fs::remove_file("static/posts.json");
    }

    println!(
        "posts-meta.json generated with {} articles (legacy posts.json: {})",
        posts.len(),
        if legacy { "on" } else { "off" }
    );
}
