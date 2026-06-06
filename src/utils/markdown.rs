use comrak::ComrakOptions;

pub fn full_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.autolink = true;
    options.extension.tasklist = true;
    options.extension.superscript = true;
    options.extension.header_ids = Some("".to_string());
    options.extension.footnotes = true;
    options.extension.description_lists = true;
    options.parse.smart = true;
    options.render.hardbreaks = true;
    options.render.github_pre_lang = true;
    options.render.unsafe_ = false;
    options.render.escape = true;
    options
}

pub fn preview_options() -> ComrakOptions {
    let mut options = ComrakOptions::default();
    options.extension.strikethrough = true;
    options.extension.table = true;
    options
}

pub fn markdown_to_html(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &full_options())
}

pub fn markdown_to_html_preview(markdown: &str) -> String {
    comrak::markdown_to_html(markdown, &preview_options())
}

pub fn clean_markdown_excerpt(markdown: &str, max_len: usize) -> String {
    let html = markdown_to_html_preview(markdown);
    let text = strip_html_tags(&html);

    let text: String = text
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");

    if text.chars().count() <= max_len {
        text
    } else {
        text.chars().take(max_len).collect::<String>() + "..."
    }
}

fn strip_html_tags(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result
}