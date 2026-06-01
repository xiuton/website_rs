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
    let mut result = String::with_capacity(markdown.len());
    let mut in_code_block = false;

    for line in markdown.lines() {
        let trimmed = line.trim();

        if trimmed.starts_with("```") {
            in_code_block = !in_code_block;
            continue;
        }

        if in_code_block {
            continue;
        }

        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || trimmed == "---"
            || trimmed == "***"
            || trimmed == "___"
        {
            continue;
        }

        let cleaned = clean_inline_markdown(trimmed);
        if !cleaned.is_empty() {
            if !result.is_empty() {
                result.push(' ');
            }
            result.push_str(&cleaned);

            if result.chars().count() >= max_len {
                break;
            }
        }
    }

    let truncated: String = result.chars().take(max_len).collect();
    if result.chars().count() > max_len {
        truncated + "..."
    } else {
        truncated
    }
}

fn clean_inline_markdown(text: &str) -> String {
    let mut cleaned = String::with_capacity(text.len());
    let chars: Vec<char> = text.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let c = chars[i];

        if c == '!' && i + 1 < len && chars[i + 1] == '[' {
            i = skip_to_closing(&chars, i + 2, '[', ']');
            if i < len && chars[i] == ']' {
                i += 1;
            }
            if i < len && chars[i] == '(' {
                i = skip_to_closing(&chars, i + 1, '(', ')');
                if i < len && chars[i] == ')' {
                    i += 1;
                }
            }
            continue;
        }

        if c == '[' {
            let start = i;
            i = skip_to_closing(&chars, i + 1, '[', ']');
            if i < len && chars[i] == ']' {
                i += 1;
                if i < len && chars[i] == '(' {
                    let link_text: String = chars[start + 1..i - 1].iter().collect();
                    cleaned.push_str(&link_text);
                    i = skip_to_closing(&chars, i + 1, '(', ')');
                    if i < len && chars[i] == ')' {
                        i += 1;
                    }
                    continue;
                }
            }
            cleaned.push(c);
            i += 1;
            continue;
        }

        if c == '`' {
            let mut j = i + 1;
            while j < len && chars[j] != '`' {
                j += 1;
            }
            if j < len {
                let code_text: String = chars[i + 1..j].iter().collect();
                cleaned.push_str(&code_text);
                i = j + 1;
                continue;
            }
        }

        if c == '*' && i + 1 < len && chars[i + 1] == '*' {
            i += 2;
            while i < len && !(chars[i] == '*' && i + 1 < len && chars[i + 1] == '*') {
                cleaned.push(chars[i]);
                i += 1;
            }
            i += 2;
            continue;
        }

        if (c == '*' || c == '_') && i + 1 < len && chars[i + 1] != ' ' {
            let marker = c;
            i += 1;
            while i < len && chars[i] != marker {
                cleaned.push(chars[i]);
                i += 1;
            }
            i += 1;
            continue;
        }

        if c == '>' && (i == 0 || chars[i - 1] == '\n') {
            i += 1;
            while i < len && chars[i] == ' ' {
                i += 1;
            }
            continue;
        }

        if c == '-' || c == '*' || c == '+' {
            if i == 0 || (i > 0 && chars[i - 1] == '\n') {
                let j = i + 1;
                if j < len && chars[j] == ' ' {
                    i = j + 1;
                    continue;
                }
            }
        }

        if c.is_ascii_digit() {
            if i == 0 || (i > 0 && chars[i - 1] == '\n') {
                let mut j = i;
                while j < len && chars[j].is_ascii_digit() {
                    j += 1;
                }
                if j < len && chars[j] == '.' {
                    let k = j + 1;
                    if k < len && chars[k] == ' ' {
                        i = k + 1;
                        continue;
                    }
                }
            }
        }

        cleaned.push(c);
        i += 1;
    }

    cleaned.trim().to_string()
}

fn skip_to_closing(chars: &[char], start: usize, open: char, close: char) -> usize {
    let mut depth = 1;
    let mut i = start;
    while i < chars.len() && depth > 0 {
        if chars[i] == open {
            depth += 1;
        } else if chars[i] == close {
            depth -= 1;
        }
        if depth > 0 {
            i += 1;
        }
    }
    i
}