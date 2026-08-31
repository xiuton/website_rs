use std::fs;
use std::path::Path;
use std::collections::HashMap;
use comrak::ComrakOptions;

pub struct PostData {
    pub title: String,
    pub date: String,
    pub author: String,
    pub tags: Vec<String>,
    pub content: String,
    pub slug: String,
    pub category: String,
    pub summary: String,
    /// 所属系列名（多章节文档），空串表示独立文章
    pub series: String,
    /// 章节在系列中的顺序，缺省为 0
    pub order: i32,
}

pub fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

pub fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

// ---- 以下为 RSS / Atom 共享的 Markdown → HTML 处理函数 ----

/// 从字节数组当前位置解码一个 UTF-8 字符，返回 (char, 字节数)
fn char_at(bytes: &[u8], i: usize) -> (char, usize) {
    let c = bytes[i];
    match c {
        _ if c < 0x80 => (c as char, 1),
        _ if c < 0xE0 => {
            if i + 1 < bytes.len() && (bytes[i+1] & 0xC0) == 0x80 {
                if let Ok(s) = std::str::from_utf8(&bytes[i..i+2]) {
                    if let Some(ch) = s.chars().next() { return (ch, 2); }
                }
            }
            ('\u{FFFD}', 1)
        }
        _ if c < 0xF0 => {
            let end = std::cmp::min(i + 3, bytes.len());
            if let Ok(s) = std::str::from_utf8(&bytes[i..end]) {
                if let Some(ch) = s.chars().next() { return (ch, s.len()); }
            }
            ('\u{FFFD}', 1)
        }
        _ => {
            let end = std::cmp::min(i + 4, bytes.len());
            if let Ok(s) = std::str::from_utf8(&bytes[i..end]) {
                if let Some(ch) = s.chars().next() { return (ch, s.len()); }
            }
            ('\u{FFFD}', 1)
        }
    }
}

/// 解码一个 UTF-8 字符并追加到 String，返回消耗的字节数
fn push_utf8(result: &mut String, bytes: &[u8], i: usize) -> usize {
    let (ch, len) = char_at(bytes, i);
    result.push(ch);
    len
}

/// 转义 <code>...</code> 内的 < > &，防止代码片段被 RSS / Atom 校验器误判为 HTML 标签
fn escape_html_code(html: &str) -> String {
    let mut result = String::with_capacity(html.len());
    let mut in_code = false;
    let bytes = html.as_bytes();
    let mut i = 0;

    while i < bytes.len() {
        if !in_code {
            if i + 5 < bytes.len() && &bytes[i..i+5] == b"<code" {
                // 把开标签 <code ...> 完整复制出来，跳过其内部 > 再开始转义
                in_code = true;
                i += push_utf8(&mut result, bytes, i);
                while i < bytes.len() && bytes[i] != b'>' {
                    i += push_utf8(&mut result, bytes, i);
                }
                if i < bytes.len() {
                    result.push('>');
                    i += 1;
                }
                continue;
            }
        } else {
            if i + 7 < bytes.len() && &bytes[i..i+7] == b"</code>" {
                result.push_str("</code>");
                i += 7;
                in_code = false;
                continue;
            } else {
                match bytes[i] {
                    b'<' => { result.push_str("&lt;"); i += 1; continue; }
                    b'>' => { result.push_str("&gt;"); i += 1; continue; }
                    b'&' => {
                        let rest = &bytes[i..];
                        let min_len = rest.len().min(6);
                        let slice = std::str::from_utf8(&rest[..min_len]).unwrap_or("");
                        if slice.starts_with("&lt;") || slice.starts_with("&gt;")
                            || slice.starts_with("&amp;") || slice.starts_with("&quot;")
                            || slice.starts_with("&apos;") || slice.starts_with("&#")
                        {
                            // 已有实体，原样保留
                        } else {
                            result.push_str("&amp;");
                            i += 1;
                            continue;
                        }
                    }
                    _ => {}
                }
            }
        }
        let (ch, len) = char_at(bytes, i);
        result.push(ch);
        i += len;
    }
    result
}

/// Markdown → HTML，代码块内 < > & 自动转义
pub fn md_to_html(md: &str) -> String {
    let mut options = ComrakOptions::default();
    options.extension.table = true;
    options.extension.strikethrough = true;
    options.extension.tasklist = true;
    options.extension.autolink = true;
    options.extension.footnotes = true;
    let raw = comrak::markdown_to_html(md, &options);
    escape_html_code(&raw)
}

/// 去掉 HTML 标签，保留实体编码（如 &lt;），不解码，用于生成纯文本摘要
pub fn strip_html_tags(html: &str) -> String {
    let mut result = String::new();
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' => in_tag = false,
            _ if !in_tag => result.push(c),
            _ => {}
        }
    }
    result.split_whitespace().collect::<Vec<_>>().join(" ")
}

pub fn strip_yaml_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

/// 安全截断 UTF-8 字符串（不会在多字节字符中间切断）
pub fn truncate_utf8_safe(s: &str, max_bytes: usize) -> &str {
    if s.len() <= max_bytes {
        return s;
    }
    let mut end = max_bytes;
    while end > 0 && !s.is_char_boundary(end) {
        end -= 1;
    }
    &s[..end]
}

/// 简易中文/英文分词
pub fn tokenize(text: &str) -> Vec<String> {
    let mut tokens: Vec<String> = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // 跳过标点和空白
        if c.is_ascii_punctuation() || c.is_whitespace() {
            i += 1;
            continue;
        }

        // 英文单词：连续字母数字
        if c.is_ascii_alphanumeric() {
            let mut word = String::new();
            while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                word.push(chars[i].to_ascii_lowercase());
                i += 1;
            }
            if word.len() >= 2 && !STOP_WORDS.contains(&word.as_str()) {
                tokens.push(word);
            }
        } else {
            // CJK 字符：取 bigram
            if i + 1 < chars.len() && !chars[i + 1].is_ascii_punctuation() && !chars[i + 1].is_whitespace() {
                let bigram: String = [c, chars[i + 1]].iter().collect();
                tokens.push(bigram);
            }
            // 单字也保留
            tokens.push(c.to_string());
            i += 1;
        }
    }

    tokens
}

/// 停用词（常见无意义词）
pub const STOP_WORDS: &[&str] = &[
    "the", "a", "an", "is", "are", "was", "were", "be", "been", "being",
    "have", "has", "had", "do", "does", "did", "will", "would", "could",
    "should", "may", "might", "can", "shall", "to", "of", "in", "for",
    "on", "with", "at", "by", "from", "as", "into", "through", "during",
    "before", "after", "above", "below", "between", "out", "off", "over",
    "under", "again", "further", "then", "once", "here", "there", "when",
    "where", "why", "how", "all", "both", "each", "few", "more", "most",
    "other", "some", "such", "no", "nor", "not", "only", "own", "same",
    "so", "than", "too", "very", "and", "but", "or", "if", "this", "that",
    "it", "its", "we", "you", "he", "she", "they", "my", "your", "our",
    "their", "me", "him", "her", "us", "them", "i", "just", "about",
    "also", "what", "which", "who", "whom",
];

pub fn scan_dir(
    dir: &Path,
    _base_dir: &Path,
    category: &str,
    posts: &mut Vec<PostData>,
    date_count: &mut HashMap<String, i32>,
) {
    match fs::read_dir(dir) {
        Ok(entries) => {
            for entry in entries.flatten() {
                let path = entry.path();
                if let Ok(file_type) = entry.file_type() {
                    if file_type.is_dir() {
                        let dir_name = path
                            .file_name()
                            .and_then(|n| n.to_str())
                            .unwrap_or("");
                        scan_dir(&path, _base_dir, dir_name, posts, date_count);
                    } else if file_type.is_file() {
                        if let Some(ext) = path.extension() {
                            if ext == "md" {
                                if let Ok(content) = fs::read_to_string(&path) {
                                    process_post(
                                        &content,
                                        category,
                                        posts,
                                        date_count,
                                    );
                                }
                            }
                        }
                    }
                }
            }
        }
        Err(e) => {
            eprintln!("cargo:warning=Failed to read directory {:?}: {}", dir, e);
        }
    }
}

pub fn process_post(
    content: &str,
    category: &str,
    posts: &mut Vec<PostData>,
    date_count: &mut HashMap<String, i32>,
) {
    let mut in_front_matter = false;
    let mut front_matter = String::new();
    let mut post_content = String::new();
    let mut in_front_matter_content = false;

    for line in content.lines() {
        if line == "---" {
            if !in_front_matter {
                in_front_matter = true;
                in_front_matter_content = true;
                continue;
            } else {
                in_front_matter_content = false;
                continue;
            }
        }
        if in_front_matter_content {
            front_matter.push_str(line);
            front_matter.push('\n');
        } else if in_front_matter {
            post_content.push_str(line);
            post_content.push('\n');
        }
    }

    let title = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("title:"))
            .map(|l| l.replace("title:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let date = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("date:"))
            .map(|l| l.replace("date:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let author = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("author:"))
            .map(|l| l.replace("author:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let tags = front_matter
        .lines()
        .find(|l| l.starts_with("tags:"))
        .map(|l| {
            l.replace("tags:", "")
                .trim()
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| strip_yaml_quotes(s.trim()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let summary = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("summary:"))
            .map(|l| l.replace("summary:", "").trim().to_string())
            .unwrap_or_default(),
    );

    // 系列字段：同一 series 名的文章属于同一个多章节文档
    let series = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("series:"))
            .map(|l| l.replace("series:", "").trim().to_string())
            .unwrap_or_default(),
    );

    // 章节顺序：缺失时默认 0（按日期排序兜底）
    let order = front_matter
        .lines()
        .find(|l| l.starts_with("order:"))
        .and_then(|l| l.replace("order:", "").trim().parse::<i32>().ok())
        .unwrap_or(0);

    let custom_slug = front_matter
        .lines()
        .find(|l| l.starts_with("slug:"))
        .map(|l| strip_yaml_quotes(&l.replace("slug:", "").trim().to_string()))
        .filter(|s| !s.is_empty());

    let slug = if let Some(slug) = custom_slug {
        slug
    } else {
        let date_parts: Vec<&str> = date.split(' ').collect();
        let date_str = if !date_parts.is_empty() {
            date_parts[0].replace('-', "")
        } else {
            "00000000".to_string()
        };
        let count = date_count.entry(date_str.clone()).or_insert(0);
        *count += 1;
        if *count > 1 {
            format!("{}-{}", date_str, *count)
        } else {
            date_str
        }
    };

    posts.push(PostData {
        title,
        date,
        author,
        tags,
        content: post_content,
        slug,
        category: category.to_string(),
        summary,
        series,
        order,
    });
}

/// 简单 LCG 伪随机数生成器

/// 为马尔可夫链对文本进行分词（保留标点，中文逐字）
pub fn markov_tokenize(text: &str) -> Vec<String> {
    let disallowed = ['#', '`', '~', '|', '>', '*'];
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        // 跳过空白
        if c.is_whitespace() {
            i += 1;
            continue;
        }

        // 跳过纯 markdown 语法符号（不是正文内容）
        if disallowed.contains(&c) {
            i += 1;
            continue;
        }

        // 英文单词：连续字母数字
        if c.is_ascii_alphanumeric() {
            let mut word = String::new();
            while i < chars.len() && chars[i].is_ascii_alphanumeric() {
                word.push(chars[i].to_ascii_lowercase());
                i += 1;
            }
            tokens.push(word);
            continue;
        }

        // 其他字符（中文、标点等）每个独立成 token
        tokens.push(c.to_string());
        i += 1;
    }

    tokens
}

/// 简单 LCG 伪随机数生成器
pub struct Rng {
    state: u64,
}

impl Rng {
    pub fn new() -> Self {
        // 基于内存地址 + 当前时间做种子
        let addr = &0u64 as *const u64 as u64;
        Self { state: addr.wrapping_mul(6364136223846793005).wrapping_add(1) }
    }

    /// 返回 [0.0, 1.0) 的 f64
    pub fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = (self.state >> 11) & 0xFFFFFFFFFFFFF;
        bits as f64 / 0xFFFFFFFFFFFFFu64 as f64
    }

    /// 返回 [0, n) 的随机 usize
    pub fn next_usize(&mut self, n: usize) -> usize {
        (self.next_f64() * n as f64) as usize
    }
}

/// 转义 JSON 字符串中的特殊字符（手动实现，免额外依赖）
pub fn escape_json_string(s: &str) -> String {
    let mut result = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            // 处理其他 ASCII 控制字符 (0x00-0x1F)
            c if (c as u32) < 0x20 => {
                result.push_str(&format!("\\u{:04x}", c as u32));
            }
            _ => result.push(c),
        }
    }
    result
}