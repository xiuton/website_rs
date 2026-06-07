use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

struct PostData {
    title: String,
    date: String,
    author: String,
    tags: Vec<String>,
    content: String,
    slug: String,
    category: String,
    summary: String,
}

fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn escape_xml(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&apos;")
}

fn strip_yaml_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR must be set");
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");

    let posts_dir = Path::new("posts");
    if !posts_dir.exists() {
        let blog_posts = "pub const BLOG_POSTS: &[BlogPost] = &[];";
        fs::write(dest_path, blog_posts).expect("Failed to write empty blog posts");
        return;
    }

    let mut posts: Vec<PostData> = Vec::new();
    let mut date_count: HashMap<String, i32> = HashMap::new();

    scan_dir(posts_dir, posts_dir, "", &mut posts, &mut date_count);

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let mut output = String::from("pub const BLOG_POSTS: &[BlogPost] = &[\n");

    for post in &posts {
        output.push_str(&format!(
            "    BlogPost {{\n        title: r#####\"{}\"#####,\n        date: r#####\"{}\"#####,\n        author: r#####\"{}\"#####,\n        tags: &[{}],\n        content: r#####\"{}\"#####,\n        slug: r#####\"{}\"#####,\n        category: r#####\"{}\"#####,\n        summary: r#####\"{}\"#####,\n    }},\n",
            post.title,
            post.date,
            post.author,
            post.tags.iter()
                .map(|t| format!("\"{}\"", escape_rust_string(t)))
                .collect::<Vec<_>>()
                .join(", "),
            post.content,
            post.slug,
            post.category,
            post.summary,
        ));
    }

    output.push_str("];\n");

    fs::write(dest_path, output).expect("Failed to write blog posts");

    // 生成 RSS feed
    generate_rss_feed(&posts, out_dir.to_str().unwrap());

    // 生成搜索索引
    generate_search_index(&posts);

    println!("cargo:rerun-if-changed=build.rs");
}

fn generate_rss_feed(posts: &[PostData], out_dir: &str) {
    let mut xml = String::from(concat!(
        r#"<?xml version="1.0" encoding="UTF-8"?>"#,
        r#"<rss version="2.0" xmlns:atom="http://www.w3.org/2005/Atom">"#,
        r#"<channel>"#,
        r#"<title>干徒的博客</title>"#,
        r#"<link>https://ganto.cn</link>"#,
        r#"<description>干徒 (Ganto) 的个人技术博客，分享 Rust、前端、WebAssembly 等编程技术文章。</description>"#,
        r#"<language>zh-CN</language>"#,
        r#"<atom:link href="https://ganto.cn/feed.xml" rel="self" type="application/rss+xml"/>"#,
    ));

    for post in posts.iter().take(20) {
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

        // RSS pubDate 格式: RFC 2822
        let pub_date = if post.date.len() >= 10 {
            format!("{} 00:00:00 +0800", &post.date[..10])
        } else {
            post.date.clone()
        };
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

/// 简易中文/英文分词
fn tokenize(text: &str) -> Vec<String> {
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
const STOP_WORDS: &[&str] = &[
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

fn generate_search_index(posts: &[PostData]) {
    let mut index = String::from("[\n");

    for (idx, post) in posts.iter().enumerate() {
        let mut tf: HashMap<String, f64> = HashMap::new();

        // 从标题、摘要、标签中提取关键词
        let source = format!("{} {} {} {}",
            post.title, post.summary, post.tags.join(" "), post.category);
        let tokens = tokenize(&source);

        let total = tokens.len() as f64;
        for token in &tokens {
            *tf.entry(token.clone()).or_insert(0.0) += 1.0;
        }

        // 归一化 TF
        if total > 0.0 {
            for v in tf.values_mut() {
                *v /= total;
            }
        }

        // 标题词额外加权
        let title_tokens = tokenize(&post.title);
        for t in &title_tokens {
            *tf.entry(t.clone()).or_insert(0.0) += 0.3;
        }

        // 标签词额外加权
        for tag in &post.tags {
            let tag_tokens = tokenize(tag);
            for t in &tag_tokens {
                *tf.entry(t.clone()).or_insert(0.0) += 0.5;
            }
        }

        let keywords_json: Vec<String> = tf.iter()
            .map(|(k, v)| format!(r#"["{}",{:.4}]"#, k, v))
            .collect();

        index.push_str(&format!(
            r#"  {{"slug":"{}","title":"{}","summary":"{}","tags":["{}"],"category":"{}","keywords":[{}]}}"#,
            post.slug,
            post.title.replace('\\', "\\\\").replace('"', "\\\""),
            post.summary.replace('\\', "\\\\").replace('"', "\\\""),
            post.tags.join(r#"",""#),
            post.category.replace('\\', "\\\\").replace('"', "\\\""),
            keywords_json.join(","),
        ));

        if idx < posts.len() - 1 {
            index.push_str(",\n");
        } else {
            index.push('\n');
        }
    }

    index.push_str("]\n");

    let dest = Path::new("static/search-index.json");
    std::fs::write(dest, &index).expect("Failed to write search index");
}

fn scan_dir(
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

fn process_post(
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

    let custom_slug = front_matter
        .lines()
        .find(|l| l.starts_with("slug:"))
        .map(|l| l.replace("slug:", "").trim().to_string())
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
    });
}