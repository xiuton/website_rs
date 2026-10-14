use std::fs;
use std::path::Path;
use std::collections::HashMap;
use comrak::ComrakOptions;
use toml;

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
    /// 系列目录页自定义路径段（由文件夹名自动生成），
    /// 空串表示文章在根目录下（非系列文档）
    pub catalog: String,
    /// 原始文件名（不含扩展名），用于排序
    pub filename: String,
    /// 系列级别的摘要（来自配置文件），用于首页列表展示
    pub series_summary: String,
}

/// 章节级配置：`_config.toml` 中 `[chapters]` 表或 `[[chapters]]` 数组的一条
///
/// 每个字段都对应 front matter 中的同名字段，取值时 front matter 优先。
/// 有了它，章节 Markdown 就可以完全不写 front matter。
#[derive(Debug, Default, Clone)]
pub struct ChapterConfig {
    /// 匹配键：章节文件名（不含 `.md`）或系列内章节编号
    pub key: String,
    pub title: Option<String>,
    pub summary: Option<String>,
    pub author: Option<String>,
    pub date: Option<String>,
    pub series: Option<String>,
    pub tags: Vec<String>,
    pub order: Option<i32>,
    pub slug: Option<String>,
    pub catalog: Option<String>,
}

/// 系列文章的 slug 生成方式（由 `_config.toml` 的 `slug_style` 指定）
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum SlugStyle {
    /// 默认：`{slug_prefix}-{order:02}`（如 `rust-llm-guide-01`）
    #[default]
    Prefix,
    /// 与普通文章一致：用日期生成（如 `20240511`，同日期依次 `-2`、`-3`）
    Date,
}

/// 目录配置文件结构体
/// 从 _config.toml 读取的默认值，文章 front matter 中的值会覆盖这些默认值
#[derive(Debug, Default)]
pub struct DirConfig {
    pub author: Option<String>,
    pub series: Option<String>,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub slug_prefix: Option<String>,
    pub summary: Option<String>,
    /// 系列章节的 slug 生成方式
    pub slug_style: SlugStyle,
    /// 章节级配置，顺序即匹配优先级：`[[chapters]]` 在前，`[chapters]` 表在后
    pub chapters: Vec<ChapterConfig>,
    /// `[series]` 分组是否显式出现（用于校验与提示）
    pub has_series_block: bool,
}

/// 从 toml 的字符串/整数键中取出章节匹配键
fn chapter_key_of(key: &str, value: &toml::Value, key_hint: Option<&str>) -> String {
    // 表写法：键本身就是匹配键
    if let Some(hint) = key_hint {
        return hint.to_string();
    }
    // 数组写法：条目内显式给 file / key / id
    if let Some(v) = value.get("file").and_then(|v| v.as_str()) {
        return v.trim().to_string();
    }
    if let Some(v) = value.get("key").and_then(|v| v.as_str()) {
        return v.trim().to_string();
    }
    if let Some(v) = value.get("id").and_then(|v| v.as_str()) {
        return v.trim().to_string();
    }
    key.trim().to_string()
}

/// 解析一个章节条目（`[chapters]` 表值与 `[[chapters]]` 数组元素共用）
fn parse_chapter_entry(key: &str, value: &toml::Value, key_hint: Option<&str>) -> ChapterConfig {
    let mut chapter = ChapterConfig {
        key: chapter_key_of(key, value, key_hint),
        ..Default::default()
    };
    if chapter.key.is_empty() {
        return chapter;
    }

    if let Some(v) = value.get("title").and_then(|v| v.as_str()) {
        chapter.title = Some(v.to_string());
    }
    if let Some(v) = value.get("summary").and_then(|v| v.as_str()) {
        chapter.summary = Some(v.to_string());
    }
    if let Some(v) = value.get("author").and_then(|v| v.as_str()) {
        chapter.author = Some(v.to_string());
    }
    if let Some(v) = value.get("date").and_then(|v| v.as_str()) {
        chapter.date = Some(v.to_string());
    }
    if let Some(v) = value.get("series").and_then(|v| v.as_str()) {
        chapter.series = Some(v.to_string());
    }
    if let Some(v) = value.get("slug").and_then(|v| v.as_str()) {
        let s = v.trim();
        if !s.is_empty() {
            chapter.slug = Some(s.to_string());
        }
    }
    if let Some(v) = value.get("catalog").and_then(|v| v.as_str()) {
        let s = v.trim();
        if !s.is_empty() {
            chapter.catalog = Some(s.to_string());
        }
    }
    // order 允许整数或数字字符串
    if let Some(v) = value.get("order") {
        if let Some(n) = v.as_integer() {
            chapter.order = Some(n as i32);
        } else if let Some(s) = v.as_str() {
            chapter.order = s.trim().parse::<i32>().ok();
        }
    }
    if let Some(tags) = value.get("tags").and_then(|v| v.as_array()) {
        chapter.tags = tags
            .iter()
            .filter_map(|t| t.as_str().map(|s| s.to_string()))
            .collect();
    }

    chapter
}

/// 从目录下的 _config.toml 读取配置
/// 如果文件不存在或解析失败，返回默认的空配置
pub fn load_dir_config(dir: &Path) -> DirConfig {
    let config_path = dir.join("_config.toml");
    if !config_path.exists() {
        return DirConfig::default();
    }

    match fs::read_to_string(&config_path) {
        Ok(content) => {
            match toml::from_str::<toml::Value>(&content) {
                Ok(value) => {
                    let mut config = DirConfig::default();

                    // ── 顶层字段（原有语义，保持不变）──
                    if let Some(author) = value.get("author").and_then(|v| v.as_str()) {
                        config.author = Some(author.to_string());
                    }
                    
                    if let Some(series) = value.get("series").and_then(|v| v.as_str()) {
                        config.series = Some(series.to_string());
                    }
                    
                    if let Some(tags) = value.get("tags").and_then(|v| v.as_array()) {
                        config.tags = tags
                            .iter()
                            .filter_map(|t| t.as_str().map(|s| s.to_string()))
                            .collect();
                    }
                    
                    if let Some(date) = value.get("date").and_then(|v| v.as_str()) {
                        config.date = Some(date.to_string());
                    }
                    
                    if let Some(slug_prefix) = value.get("slug_prefix").and_then(|v| v.as_str()) {
                        config.slug_prefix = Some(slug_prefix.to_string());
                    }
                    
                    if let Some(summary) = value.get("summary").and_then(|v| v.as_str()) {
                        config.summary = Some(summary.to_string());
                    }

                    // ── [series] 分组（可选）：同名字段覆盖顶层写法 ──
                    if let Some(series_tbl) = value.get("series").and_then(|v| v.as_table()) {
                        config.has_series_block = true;
                        if let Some(v) = series_tbl.get("author").and_then(|v| v.as_str()) {
                            config.author = Some(v.to_string());
                        }
                        if let Some(v) = series_tbl.get("name").and_then(|v| v.as_str()) {
                            config.series = Some(v.to_string());
                        }
                        if let Some(v) = series_tbl.get("series").and_then(|v| v.as_str()) {
                            config.series = Some(v.to_string());
                        }
                        if let Some(v) = series_tbl.get("summary").and_then(|v| v.as_str()) {
                            config.summary = Some(v.to_string());
                        }
                        if let Some(v) = series_tbl.get("date").and_then(|v| v.as_str()) {
                            config.date = Some(v.to_string());
                        }
                        if let Some(v) = series_tbl.get("slug_prefix").and_then(|v| v.as_str()) {
                            config.slug_prefix = Some(v.to_string());
                        }
                        if let Some(tags) = series_tbl.get("tags").and_then(|v| v.as_array()) {
                            config.tags = tags
                                .iter()
                                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                                .collect();
                        }
                    }

                    // ── slug_style：系列章节的 slug 生成方式 ──
                    if let Some(style) = value.get("slug_style").and_then(|v| v.as_str()) {
                        match style.trim().to_ascii_lowercase().as_str() {
                            "date" | "日期" => config.slug_style = SlugStyle::Date,
                            "prefix" | "前缀" => config.slug_style = SlugStyle::Prefix,
                            other => eprintln!(
                                "cargo:warning=[_config.toml] {:?} 的 slug_style = {:?} 无法识别，可选值：\"prefix\"（默认）或 \"date\"",
                                config_path, other
                            ),
                        }
                    }

                    // ── [[chapters]] 数组（优先）──
                    if let Some(arr) = value.get("chapters").and_then(|v| v.as_array()) {
                        for entry in arr {
                            let key = entry
                                .get("file")
                                .or_else(|| entry.get("key"))
                                .or_else(|| entry.get("id"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string();
                            if key.trim().is_empty() {
                                eprintln!(
                                    "cargo:warning=[_config.toml] {:?} 的 [[chapters]] 条目缺少 file/key/id 字段，已跳过",
                                    config_path
                                );
                                continue;
                            }
                            config.chapters.push(parse_chapter_entry(&key, entry, Some(&key)));
                        }
                    }

                    // ── [chapters] 表 ──
                    if let Some(tbl) = value.get("chapters").and_then(|v| v.as_table()) {
                        for (key, entry) in tbl {
                            config.chapters.push(parse_chapter_entry(key, entry, Some(key)));
                        }
                    }

                    config
                }
                Err(e) => {
                    eprintln!("cargo:warning=Failed to parse {:?}: {}", config_path, e);
                    DirConfig::default()
                }
            }
        }
        Err(e) => {
            eprintln!("cargo:warning=Failed to read {:?}: {}", config_path, e);
            DirConfig::default()
        }
    }
}

/// 章节条目的匹配优先级（数字越小越优先）
///
/// 1. 文件名词干完全相等
/// 2. 条目是数字编号，等于文件名首个 `-`/`_` 前的片段（`01` ↔ `01-张量.md`）
/// 3. 条目是数字编号，等于章节 order（`1` ↔ `01-张量.md` 的 order=1）
fn chapter_match_rank(key: &str, filename: &str, order: i32) -> Option<u8> {
    let key = key.trim();
    if key.is_empty() {
        return None;
    }
    if key == filename {
        return Some(0);
    }

    let key_num = key.parse::<i32>().ok();

    let prefix = filename
        .split(|c| c == '-' || c == '_')
        .next()
        .unwrap_or("");
    if let Some(n) = key_num {
        if prefix == key || prefix.parse::<i32>().map(|p| p == n).unwrap_or(false) {
            return Some(1);
        }
        if n == order {
            return Some(2);
        }
    }
    None
}

/// 在目录配置中查找当前章节的章节级配置
#[allow(dead_code)]
pub fn find_chapter<'a>(cfg: &'a DirConfig, filename: &str, order: i32) -> Option<&'a ChapterConfig> {
    let mut best: Option<(u8, usize, &ChapterConfig)> = None;
    for (idx, chapter) in cfg.chapters.iter().enumerate() {
        if let Some(rank) = chapter_match_rank(&chapter.key, filename, order) {
            let better = match best {
                None => true,
                Some((best_rank, best_idx, _)) => (rank, idx) < (best_rank, best_idx),
            };
            if better {
                best = Some((rank, idx, chapter));
            }
        }
    }
    best.map(|(_, _, c)| c)
}

/// 章节条目的匹配排名（供构建期校验重复匹配使用）
pub fn chapter_rank(key: &str, filename: &str, order: i32) -> Option<u8> {
    chapter_match_rank(key, filename, order)
}

/// 由文件名/章节键派生标题：去掉 `NN-` / `NN_` 前缀，`-` / `_` 换成空格
pub fn derive_title(key: &str) -> String {
    let trimmed = key.trim();
    let body = match trimmed.split_once(['-', '_']) {
        Some((head, tail)) if !head.is_empty() && head.chars().all(|c| c.is_ascii_digit()) => tail,
        _ => trimmed,
    };
    body.replace(['-', '_'], " ").trim().to_string()
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
    // 读取当前目录的配置文件
    let dir_config = load_dir_config(dir);

    // 记录每个 [chapters] 条目命中的章节文件（构建期校验用）
    let mut matched: Vec<Option<String>> = vec![None; dir_config.chapters.len()];
    
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
                                let filename = path
                                    .file_stem()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("");
                                
                                // 跳过以 _ 开头的文件（如 _README.md）
                                if filename.starts_with('_') {
                                    continue;
                                }
                                
                                if let Ok(content) = fs::read_to_string(&path) {
                                    process_post(
                                        &content,
                                        category,
                                        filename,
                                        posts,
                                        date_count,
                                        &dir_config,
                                        Some(&mut matched),
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

    // ── 校验：配置里写了但没有任何章节文件命中的条目 ──
    for (idx, chapter) in dir_config.chapters.iter().enumerate() {
        if matched.get(idx).map(|m| m.is_none()).unwrap_or(true) {
            eprintln!(
                "cargo:warning=[_config.toml] [chapters] 条目 \"{}\" 未匹配到任何章节文件（目录 {:?}）",
                chapter.key, dir
            );
        }
    }
}

pub fn process_post(
    content: &str,
    category: &str,
    filename: &str,
    posts: &mut Vec<PostData>,
    // 日期型 slug 统一在 assign_slugs 里分配（需要全局视角），这里不再使用
    _date_count: &mut HashMap<String, i32>,
    dir_config: &DirConfig,
    mut matched_chapters: Option<&mut Vec<Option<String>>>,
) {
    // 解析 front matter 是否构成一个完整块（用于决定是否启用「由文件名派生标题」兜底）
    //
    // 规则：front matter 必须位于文件**开头**（允许前面有 UTF-8 BOM 或空行），
    // 否则第一个 `---` 只是正文里的水平分割线，不能被当成元数据起点。
    // 这样也顺带避免「正文顶部的分割线把整篇内容吞掉」这类事故。
    let mut front_matter_started = false;
    let mut front_matter_closed = false;
    let mut front_matter = String::new();
    let mut post_content = String::new();
    let mut in_front_matter_content = false;
    let mut seen_content = false;

    for line in content.lines() {
        let stripped = line.trim_start_matches('\u{feff}').trim();
        if stripped == "---" && !seen_content {
            if !front_matter_started {
                front_matter_started = true;
                in_front_matter_content = true;
                continue;
            } else if in_front_matter_content {
                front_matter_closed = true;
                in_front_matter_content = false;
                continue;
            }
        }
        if in_front_matter_content {
            front_matter.push_str(line);
            front_matter.push('\n');
        } else {
            post_content.push_str(line);
            post_content.push('\n');
        }
        if !in_front_matter_content {
            seen_content = true;
        }
    }

    // 出现开头的 `---` 却没有收尾的 `---`：整篇会被当成元数据，必须提示
    if front_matter_started && !front_matter_closed {
        eprintln!(
            "cargo:warning=[front matter 未闭合] {}：文件开头的 `---` 没有对应的收尾 `---`，正文会被当成元数据吞掉",
            filename
        );
    }

    // 完全没有 front matter 块 → 允许从 _config.toml / 文件名派生元数据
    let no_front_matter = !(front_matter_started && front_matter_closed);

    /// 取一个 front matter 字段（去引号），空串视为「未设置」
    fn fm_field(front_matter: &str, key: &str) -> Option<String> {
        let prefix = format!("{}:", key);
        let raw = front_matter
            .lines()
            .find(|l| l.starts_with(prefix.as_str()))
            .map(|l| l[prefix.len()..].trim().to_string())?;
        let value = strip_yaml_quotes(&raw);
        if value.is_empty() { None } else { Some(value) }
    }

    /// front matter → 兜底（章节配置 / 目录配置）
    fn pick(fm: Option<String>, fallback: impl FnOnce() -> String) -> String {
        fm.unwrap_or_else(fallback)
    }

    let title_from_fm = fm_field(&front_matter, "title");
    let date_from_fm = fm_field(&front_matter, "date");
    let author_from_fm = fm_field(&front_matter, "author");
    let summary_from_fm = fm_field(&front_matter, "summary");
    let series_from_fm = fm_field(&front_matter, "series");
    let slug_from_fm = fm_field(&front_matter, "slug");

    let tags_from_fm: Vec<String> = front_matter
        .lines()
        .find(|l| l.starts_with("tags:"))
        .map(|l| {
            l.replace("tags:", "")
                .trim()
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| strip_yaml_quotes(s.trim()))
                .filter(|s| !s.is_empty())
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    // 章节顺序：front matter → 章节配置 → 文件名首个数字段
    let order_from_front_matter = front_matter
        .lines()
        .find(|l| l.starts_with("order:"))
        .and_then(|l| l.replace("order:", "").trim().parse::<i32>().ok());
    
    let order_from_filename = filename
        .split('-')
        .next()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(0);

    // 章节级配置查找需要用到 order，因此先算文件名 order 作为匹配依据
    let order_for_match = order_from_front_matter.unwrap_or(order_from_filename);
    let chapter_idx = dir_config
        .chapters
        .iter()
        .position(|c| chapter_rank(&c.key, filename, order_for_match).is_some());
    let chapter = chapter_idx.map(|idx| &dir_config.chapters[idx]);

    // 校验：一个数字编号条目同时被多个文件「非精确」命中，说明配置有歧义
    if let (Some(idx), Some(rank), Some(matched)) = (
        chapter_idx,
        chapter.and_then(|c| chapter_rank(&c.key, filename, order_for_match)),
        matched_chapters.as_deref_mut(),
    ) {
        if let Some(slot) = matched.get_mut(idx) {
            if let Some(prev) = slot.as_ref() {
                eprintln!(
                    "cargo:warning=[_config.toml] [chapters] 条目 \"{}\" 同时匹配了 {}.md 与 {}.md，建议改写成完整文件名",
                    dir_config.chapters[idx].key, prev, filename
                );
            } else if rank > 0 {
                eprintln!(
                    "cargo:warning=[_config.toml] [chapters] 条目 \"{}\" 是按编号匹配到 {}.md 的，建议改写成完整文件名以免歧义",
                    dir_config.chapters[idx].key, filename
                );
            }
            *slot = Some(filename.to_string());
        }
    }

    let chapter_order = chapter.and_then(|c| c.order);

    let order = order_from_front_matter
        .or(chapter_order)
        .unwrap_or(order_from_filename);

    // ── 标题：front matter → 章节配置 → （无 front matter 时）文件名派生 ──
    let title_from_config = chapter
        .and_then(|c| c.title.clone())
        .filter(|s| !s.is_empty());
    let title = title_from_fm
        .or_else(|| title_from_config.clone())
        .unwrap_or_else(|| {
            if no_front_matter {
                let key = chapter.map(|c| c.key.as_str()).unwrap_or(filename);
                derive_title(key)
            } else {
                String::new()
            }
        });
    if title.is_empty() {
        eprintln!(
            "cargo:warning=[标题为空] posts/{}/{}：请在 front matter 或 _config.toml 的 [chapters] 中配置 title",
            category, filename
        );
    }

    // ── 日期：front matter → 章节配置 → 目录配置 ──
    let date = pick(date_from_fm, || {
        chapter
            .and_then(|c| c.date.clone())
            .or_else(|| dir_config.date.clone())
            .unwrap_or_default()
    });

    // ── 作者：front matter → 章节配置 → 目录配置 ──
    let author = pick(author_from_fm, || {
        chapter
            .and_then(|c| c.author.clone())
            .or_else(|| dir_config.author.clone())
            .unwrap_or_default()
    });

    // ── 标签：目录配置 + 章节配置 + front matter，依次去重合并 ──
    let mut tags = dir_config.tags.clone();
    if let Some(chapter) = chapter {
        for tag in &chapter.tags {
            if !tags.contains(tag) {
                tags.push(tag.clone());
            }
        }
    }
    for tag in tags_from_fm {
        if !tags.contains(&tag) {
            tags.push(tag);
        }
    }

    // ── 摘要：front matter → 章节配置 → 目录配置 ──
    // 章节配置存在时（即便同值），也要优先于目录配置，否则每章都会拿到系列摘要
    let summary = pick(summary_from_fm, || {
        if let Some(s) = chapter.and_then(|c| c.summary.clone()) {
            return s;
        }
        if chapter.is_some() && no_front_matter {
            return String::new();
        }
        dir_config.summary.clone().unwrap_or_default()
    });

    // ── 系列：front matter → 章节配置 → 目录配置 ──
    let series = pick(series_from_fm, || {
        chapter
            .and_then(|c| c.series.clone())
            .or_else(|| dir_config.series.clone())
            .unwrap_or_default()
    });

    // 目录页 slug：默认使用文档文件夹名称（category），章节配置可覆盖
    let catalog = chapter
        .and_then(|c| c.catalog.clone())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| category.to_string());

    // 文章 slug：优先使用 front matter 中自定义的 slug，其次章节配置；
    // 都没有时按目录配置的 slug_style 决定——
    //   prefix（默认）：系列用 `{slug_prefix}-{order:02}`；无 prefix 则用文件名
    //   date          ：与普通文章一致，用日期（如 `20240511`），
    //                   因为有跨目录去重计数，这里先留空，待全部文章收集完再统一分配
    // 普通文章始终按日期生成，同样延后到 assign_slugs 统一分配（保证计数确定性）
    let custom_slug = slug_from_fm.or_else(|| chapter.and_then(|c| c.slug.clone()));

    let slug = if let Some(slug) = custom_slug {
        slug
    } else if !series.is_empty() && dir_config.slug_style == SlugStyle::Prefix {
        match &dir_config.slug_prefix {
            Some(prefix) => format!("{}-{:02}", prefix, order),
            None => filename.to_string(),
        }
    } else if !series.is_empty() && dir_config.slug_prefix.is_none() {
        // 系列但既没有 slug_prefix、又选了日期风格：回退为文件名，避免撞成同一个 slug
        filename.to_string()
    } else {
        // 待分配：普通文章 / date 风格的系列章节
        String::new()
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
        catalog,
        filename: filename.to_string(),
        series_summary: dir_config.summary.clone().unwrap_or_default(),
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

/// 从日期字符串取出紧凑日期（`2024-05-11 12:00` → `20240511`）
pub fn date_key(date: &str) -> String {
    let digits: String = date
        .split(' ')
        .next()
        .unwrap_or("")
        .chars()
        .filter(|c| c.is_ascii_digit())
        .collect();
    if digits.is_empty() {
        "00000000".to_string()
    } else {
        digits
    }
}

/// 统一分配「日期型 slug」，并检查全局 slug 唯一性
///
/// 需要延后到全部文章收集完再做的原因：
/// 1. 系列章节可以共用同一个日期（如 `_config.toml` 里只写了一个 date），
///    必须按日期统一编号才不会撞车；
/// 2. 编号顺序必须稳定（按 日期 → order → 文件名 排序后分配），
///    否则 `fs::read_dir` 的遍历顺序会让 `-2`/`-3` 在不同机器上落到不同章节。
pub fn assign_slugs(posts: &mut [PostData]) {
    // 收集所有待分配的文章，并按「日期 → order → 文件名」稳定排序：
    // 同一个日期的多篇（典型是共用 date 的系列章节）会依次拿到
    // `20240511`、`20240511-2`、`20240511-3`…
    let mut pending: Vec<(String, i32, String, usize)> = posts
        .iter()
        .enumerate()
        .filter(|(_, p)| p.slug.is_empty())
        .map(|(i, p)| (date_key(&p.date), p.order, p.filename.clone(), i))
        .collect();
    pending.sort();

    let mut used: HashMap<String, usize> = HashMap::new();
    for (key, _, _, i) in pending {
        let slug = used.entry(key.clone()).or_insert(0);
        *slug += 1;
        posts[i].slug = if *slug > 1 {
            format!("{}-{}", key, *slug)
        } else {
            key
        };
    }

    // ── 全局 slug 唯一性检查：重复会让某篇文章永远访问不到 ──
    let mut seen: HashMap<String, String> = HashMap::new();
    let mut reported: std::collections::HashSet<String> = std::collections::HashSet::new();
    let mut duplicates: Vec<(String, String, String)> = Vec::new();
    for post in posts.iter() {
        if post.slug.is_empty() {
            continue;
        }
        match seen.get(&post.slug) {
            Some(prev) => {
                if reported.insert(post.slug.clone()) {
                    duplicates.push((post.slug.clone(), prev.clone(), post.filename.clone()));
                }
            }
            None => {
                seen.insert(post.slug.clone(), post.filename.clone());
            }
        }
    }
    for (slug, a, b) in duplicates {
        eprintln!(
            "cargo:warning=[slug 重复] \"{}\" 同时属于 {}.md 与 {}.md：后一篇将无法通过 URL 访问。\
             请在 _config.toml 中为章节设置 order、或使用 slug_style = \"date\"、或直接指定 slug",
            slug, a, b
        );
    }
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