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