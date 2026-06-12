use std::env;
use std::fs;
use std::path::Path;
use std::collections::{HashMap, HashSet};

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

    // 生成语义索引（词共现 + 自动补全建议）
    generate_semantic_index(&posts);

    // 生成知识图谱
    generate_knowledge_graph(&posts);

    // 生成 AI 文章摘要（TextRank）
    generate_ai_summaries(&posts);

    // 生成马尔可夫链续写数据
    generate_markov_chain(&posts);

    // RAKE 关键词提取
    generate_rake_keywords(&posts);

    // LDA 主题模型
    generate_lda_topics(&posts);

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

/// 安全截断 UTF-8 字符串（不会在多字节字符中间切断）
fn truncate_utf8_safe(s: &str, max_bytes: usize) -> &str {
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
    if posts.is_empty() {
        let dest = Path::new("static/search-index.json");
        std::fs::write(dest, "[]").expect("Failed to write search index");
        return;
    }

    let n = posts.len() as f64;

    // ── Pass 1: 收集每篇文章的 token 计数 & 文档频率 ──
    struct DocTokens {
        counts: HashMap<String, f64>,   // token → 原始计数（含标题/标签加权）
    }

    let mut doc_tokens_list: Vec<DocTokens> = Vec::with_capacity(posts.len());
    let mut doc_freq: HashMap<String, f64> = HashMap::new(); // 出现某 token 的文章数

    for post in posts {
        let source = format!(
            "{} {} {} {} {}",
            post.title, post.summary, post.tags.join(" "), post.category,
            truncate_utf8_safe(&post.content, 2000)
        );
        let tokens = tokenize(&source);

        let mut counts: HashMap<String, f64> = HashMap::new();
        let mut seen_in_doc: HashSet<&str> = HashSet::new();

        for token in &tokens {
            *counts.entry(token.clone()).or_insert(0.0) += 1.0;
            if seen_in_doc.insert(token.as_str()) {
                *doc_freq.entry(token.clone()).or_insert(0.0) += 1.0;
            }
        }

        // 标题词额外加权
        for t in tokenize(&post.title) {
            *counts.entry(t).or_insert(0.0) += 0.5;
        }
        // 标签词额外加权
        for tag in &post.tags {
            for t in tokenize(tag) {
                *counts.entry(t).or_insert(0.0) += 1.5;
            }
        }

        doc_tokens_list.push(DocTokens { counts });
    }

    // ── Pass 2: 计算 IDF 并构建 TF-IDF 稀疏向量 ──
    // IDF(t) = ln(N / df(t)) + 1 （平滑，避免 df=N 时 IDF=0）
    let idf: HashMap<String, f64> = doc_freq
        .iter()
        .map(|(token, df)| (token.clone(), (n / df).ln() + 1.0))
        .collect();

    struct TfidfVector {
        // 只保留 TF-IDF 值最高的前 120 个词（稀疏向量）
        pairs: Vec<(String, f64)>,
        magnitude: f64,
    }

    let mut vectors: Vec<TfidfVector> = Vec::with_capacity(posts.len());

    for dt in &doc_tokens_list {
        let total: f64 = dt.counts.values().sum();
        if total == 0.0 {
            vectors.push(TfidfVector {
                pairs: Vec::new(),
                magnitude: 0.0,
            });
            continue;
        }

        let mut tfidf_map: HashMap<String, f64> = HashMap::new();
        for (token, count) in &dt.counts {
            let tf = count / total;
            let idf_val = idf.get(token).copied().unwrap_or(1.0);
            tfidf_map.insert(token.clone(), tf * idf_val);
        }

        // 排序取 top-120
        let mut pairs: Vec<(String, f64)> = tfidf_map.into_iter().collect();
        pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        pairs.truncate(120);

        let magnitude: f64 = pairs.iter().map(|(_, v)| v * v).sum::<f64>().sqrt();

        vectors.push(TfidfVector { pairs, magnitude });
    }

    // ── Pass 3: 计算文章间余弦相似度，取 top-5 相关 ──
    let mut related_slugs: Vec<Vec<String>> = Vec::with_capacity(posts.len());

    for i in 0..posts.len() {
        let mut score_list: Vec<(usize, f64)> = Vec::new();

        for j in 0..posts.len() {
            if i == j {
                continue;
            }

            let vi = &vectors[i];
            let vj = &vectors[j];

            // 稀疏向量点积（只遍历较短的那一个）
            let (shorter, longer) = if vi.pairs.len() < vj.pairs.len() {
                (&vi.pairs, &vj.pairs)
            } else {
                (&vj.pairs, &vi.pairs)
            };

            // 构建 longer 的快速查找表（token → value）
            let long_map: HashMap<&str, f64> = longer
                .iter()
                .map(|(k, v)| (k.as_str(), *v))
                .collect();

            let mut dot = 0.0;
            for (token, val_i) in shorter {
                if let Some(val_j) = long_map.get(token.as_str()) {
                    dot += val_i * val_j;
                }
            }

            let mag = vi.magnitude * vj.magnitude;
            let similarity = if mag > 1e-10 { dot / mag } else { 0.0 };

            if similarity > 0.01 {
                score_list.push((j, similarity));
            }
        }

        score_list.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let related: Vec<String> = score_list
            .into_iter()
            .take(5)
            .map(|(idx, _)| posts[idx].slug.clone())
            .collect();

        related_slugs.push(related);
    }

    // ── Pass 4: 输出 JSON ──
    let mut index = String::from("[\n");

    for (idx, post) in posts.iter().enumerate() {
        // keywords: 存 TF-IDF 值（取代原来的纯 TF）
        let keywords_json: Vec<String> = vectors[idx]
            .pairs
            .iter()
            .map(|(k, v)| format!(r#"["{}",{:.6}]"#, k, v))
            .collect();

        // related: ["slug1", "slug2", ...]
        let related_json = related_slugs[idx]
            .iter()
            .map(|s| format!(r#""{}""#, s))
            .collect::<Vec<_>>()
            .join(",");

        index.push_str(&format!(
            r#"  {{"slug":"{}","title":"{}","summary":"{}","tags":["{}"],"category":"{}","keywords":[{}],"related":[{}]}}"#,
            post.slug,
            post.title.replace('\\', "\\\\").replace('"', "\\\""),
            post.summary.replace('\\', "\\\\").replace('"', "\\\""),
            post.tags.join(r#"",""#),
            post.category,
            keywords_json.join(","),
            related_json,
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

// ══════════════════════════════════════════════════════════
// 知识图谱构建（零外部依赖：纯 std + 手动 JSON）
// ══════════════════════════════════════════════════════════

/// 简单 LCG 伪随机数生成器
struct Rng {
    state: u64,
}

impl Rng {
    fn new() -> Self {
        // 基于内存地址 + 当前时间做种子
        let addr = &0u64 as *const u64 as u64;
        Self { state: addr.wrapping_mul(6364136223846793005).wrapping_add(1) }
    }

    /// 返回 [0.0, 1.0) 的 f64
    fn next_f64(&mut self) -> f64 {
        self.state = self.state.wrapping_mul(6364136223846793005).wrapping_add(1);
        let bits = (self.state >> 11) & 0xFFFFFFFFFFFFF;
        bits as f64 / 0xFFFFFFFFFFFFFu64 as f64
    }

    /// 返回 [0, n) 的随机 usize
    fn next_usize(&mut self, n: usize) -> usize {
        (self.next_f64() * n as f64) as usize
    }
}

// ══════════════════════════════════════════════════════════
// 语义索引生成：词共现矩阵 + 自动补全建议
// ══════════════════════════════════════════════════════════
fn generate_semantic_index(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/semantic.json");
        std::fs::write(dest, r#"{"cooccur":[],"suggestions":[]}"#)
            .expect("Failed to write semantic index");
        return;
    }

    let n = posts.len() as f64;

    // ── 与 search_index 共享的 TF-IDF 计算 ──
    let mut doc_tokens_list: Vec<HashMap<String, f64>> = Vec::with_capacity(posts.len());
    let mut doc_freq: HashMap<String, f64> = HashMap::new();

    for post in posts {
        let source = format!(
            "{} {} {} {} {}",
            post.title, post.summary, post.tags.join(" "), post.category,
            truncate_utf8_safe(&post.content, 2000)
        );
        let tokens = tokenize(&source);
        let mut counts: HashMap<String, f64> = HashMap::new();
        let mut seen: HashSet<String> = HashSet::new();
        for token in &tokens {
            *counts.entry(token.clone()).or_insert(0.0) += 1.0;
            if seen.insert(token.clone()) {
                *doc_freq.entry(token.clone()).or_insert(0.0) += 1.0;
            }
        }
        for t in tokenize(&post.title) {
            *counts.entry(t).or_insert(0.0) += 0.5;
        }
        for tag in &post.tags {
            for t in tokenize(tag) {
                *counts.entry(t).or_insert(0.0) += 1.5;
            }
        }
        doc_tokens_list.push(counts);
    }

    let idf: HashMap<String, f64> = doc_freq
        .iter()
        .map(|(token, df)| (token.clone(), (n / df).ln() + 1.0))
        .collect();

    // ── 提取每篇文章的 top-30 关键词 ──
    let mut article_keywords: Vec<Vec<(String, f64)>> = Vec::with_capacity(posts.len());
    for (_i, dt) in doc_tokens_list.iter().enumerate() {
        let total: f64 = dt.values().sum();
        if total == 0.0 {
            article_keywords.push(Vec::new());
            continue;
        }
        let mut tfidf: Vec<(String, f64)> = dt
            .iter()
            .map(|(token, count)| {
                let tf = count / total;
                let idf_val = idf.get(token).copied().unwrap_or(1.0);
                (token.clone(), tf * idf_val)
            })
            .collect();
        tfidf.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        tfidf.truncate(30);
        // 归一化
        let mag: f64 = tfidf.iter().map(|(_, v)| v * v).sum::<f64>().sqrt();
        if mag > 1e-10 {
            for (_, v) in &mut tfidf {
                *v /= mag;
            }
        }
        article_keywords.push(tfidf);
    }

    // ── 构建词共现矩阵 ──
    // cooccur[w1][w2] = 两词共同出现在同一篇文章中的加权分数之和
    let mut cooccur: HashMap<String, HashMap<String, f64>> = HashMap::new();

    for kws in &article_keywords {
        for (i, (wi, si)) in kws.iter().enumerate() {
            for (j, (wj, sj)) in kws.iter().enumerate() {
                if i >= j {
                    continue;
                }
                let strength = si.min(*sj); // 最小分数作为共现强度

                let entry = cooccur.entry(wi.clone()).or_default();
                *entry.entry(wj.clone()).or_insert(0.0) += strength;

                let entry = cooccur.entry(wj.clone()).or_default();
                *entry.entry(wi.clone()).or_insert(0.0) += strength;
            }
        }
    }

    // 为每个词保留 top-15 共现词
    let mut cooccur_top: HashMap<String, Vec<(String, f64)>> = HashMap::new();
    for (word, neighbors) in &cooccur {
        let mut sorted: Vec<(String, f64)> = neighbors
            .iter()
            .map(|(w, s)| (w.clone(), *s))
            .collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted.truncate(15);
        cooccur_top.insert(word.clone(), sorted);
    }

    // ── 构建自动补全建议列表 ──
    // 收集所有标题 + 标签（去重）
    let mut suggestions: Vec<String> = Vec::new();
    let mut seen: HashSet<String> = HashSet::new();

    for post in posts {
        if seen.insert(post.title.clone()) {
            suggestions.push(post.title.clone());
        }
        for tag in &post.tags {
            if seen.insert(tag.clone()) {
                suggestions.push(tag.clone());
            }
        }
    }
    // 按短到长排序（简短的关键词优先）
    suggestions.sort_by_key(|s| s.chars().count());

    // 收集知识图谱 tag 关联信息
    // 对于有共现的 tag，生成 tag→related_tags
    let mut tag_relations: HashMap<String, Vec<String>> = HashMap::new();
    for post in posts {
        for (i, t1) in post.tags.iter().enumerate() {
            if tag_relations.contains_key(t1) {
                continue;
            }
            for (j, t2) in post.tags.iter().enumerate() {
                if i == j {
                    continue;
                }
                tag_relations
                    .entry(t1.clone())
                    .or_default()
                    .push(t2.clone());
            }
        }
    }

    // ── 输出 JSON ──
    let mut json = String::from("{\n");

    // cooccur
    json.push_str(r#"  "cooccur": {"#);
    let mut first = true;
    for (word, neighbors) in &cooccur_top {
        if !first {
            json.push(',');
        }
        first = false;
        json.push_str(&format!(
            r#""{}":{{"#,
            escape_json_string(word),
        ));
        let mut nfirst = true;
        for (nw, ns) in neighbors {
            if !nfirst {
                json.push(',');
            }
            nfirst = false;
            json.push_str(&format!(r#""{}":{:.6}"#, escape_json_string(nw), ns));
        }
        json.push('}');
    }
    json.push_str("},\n");

    // suggestions
    json.push_str(r#"  "suggestions": ["#);
    for (i, s) in suggestions.iter().enumerate() {
        if i > 0 {
            json.push(',');
        }
        json.push('"');
        json.push_str(&escape_json_string(s));
        json.push('"');
    }
    json.push_str("],\n");

    // tag_relations (for smarter autocomplete)
    json.push_str(r#"  "tag_relations": {"#);
    let mut tfirst = true;
    for (tag, related) in &tag_relations {
        if !tfirst {
            json.push(',');
        }
        tfirst = false;
        json.push_str(&format!(r#""{}":["#, escape_json_string(tag)));
        for (i, rt) in related.iter().enumerate() {
            if i > 0 {
                json.push(',');
            }
            json.push('"');
            json.push_str(&escape_json_string(rt));
            json.push('"');
        }
        json.push(']');
    }
    json.push_str("}\n");

    json.push_str("}\n");

    let dest = Path::new("static/semantic.json");
    std::fs::write(dest, &json).expect("Failed to write semantic index");
}

fn generate_knowledge_graph(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/knowledge-graph.json");
        std::fs::write(dest, "{}").expect("Failed to write knowledge graph");
        return;
    }

    // ── 1. 构建节点索引 ──
    let mut node_ids: Vec<String> = Vec::new();
    let mut node_index: HashMap<String, usize> = HashMap::new();

    for post in posts {
        let aid = format!("article:{}", post.slug);
        if !node_index.contains_key(&aid) {
            node_index.insert(aid.clone(), node_ids.len());
            node_ids.push(aid);
        }
    }

    let mut all_tags: HashSet<String> = HashSet::new();
    let mut all_categories: HashSet<String> = HashSet::new();
    for post in posts {
        for tag in &post.tags {
            all_tags.insert(tag.clone());
        }
        all_categories.insert(post.category.clone());
    }

    for tag in &all_tags {
        let tid = format!("tag:{}", tag);
        node_index.insert(tid.clone(), node_ids.len());
        node_ids.push(tid);
    }
    for cat in &all_categories {
        let cid = format!("category:{}", cat);
        node_index.insert(cid.clone(), node_ids.len());
        node_ids.push(cid);
    }

    let n = node_ids.len();

    // ── 2. 构建邻接矩阵（稀疏） ──
    let mut adj: Vec<Vec<(usize, f64)>> = vec![Vec::new(); n];

    let article_indices: HashMap<&str, usize> = posts
        .iter()
        .map(|p| (p.slug.as_str(), node_index[&format!("article:{}", p.slug)]))
        .collect();

    fn add_edge(adj: &mut Vec<Vec<(usize, f64)>>, a: usize, b: usize, w: f64) {
        adj[a].push((b, w));
        adj[b].push((a, w));
    }

    // Tag 共现边
    {
        let mut tag_cooccur: HashMap<(usize, usize), f64> = HashMap::new();
        for post in posts {
            let tag_idxs: Vec<usize> = post
                .tags
                .iter()
                .map(|t| node_index[&format!("tag:{}", t)])
                .collect();
            for &ti in &tag_idxs {
                for &tj in &tag_idxs {
                    if ti < tj {
                        *tag_cooccur.entry((ti, tj)).or_insert(0.0) += 1.0;
                    }
                }
            }
        }
        for ((ti, tj), count) in &tag_cooccur {
            add_edge(&mut adj, *ti, *tj, *count);
        }
    }

    // Article ↔ Tag
    for post in posts {
        let ai = article_indices[post.slug.as_str()];
        for tag in &post.tags {
            let ti = node_index[&format!("tag:{}", tag)];
            add_edge(&mut adj, ai, ti, 2.0);
        }
    }

    // Article ↔ Category
    for post in posts {
        let ai = article_indices[post.slug.as_str()];
        let ci = node_index[&format!("category:{}", post.category)];
        add_edge(&mut adj, ai, ci, 1.0);
    }

    // Article ↔ Article（TF-IDF 余弦相似度 > 阈值）
    {
        let mut doc_tokens: Vec<HashMap<String, f64>> = Vec::new();
        let mut doc_freq: HashMap<String, f64> = HashMap::new();

        for post in posts {
            let source = format!(
                "{} {} {} {} {}",
                post.title,
                post.summary,
                post.tags.join(" "),
                post.category,
                truncate_utf8_safe(&post.content, 2000)
            );
            let tokens = tokenize(&source);
            let mut counts: HashMap<String, f64> = HashMap::new();
            let mut seen: HashSet<String> = HashSet::new();
            for token in &tokens {
                *counts.entry(token.clone()).or_insert(0.0) += 1.0;
                if seen.insert(token.clone()) {
                    *doc_freq.entry(token.clone()).or_insert(0.0) += 1.0;
                }
            }
            for t in tokenize(&post.title) {
                *counts.entry(t).or_insert(0.0) += 1.5;
            }
            for tag in &post.tags {
                for t in tokenize(tag) {
                    *counts.entry(t).or_insert(0.0) += 2.0;
                }
            }
            doc_tokens.push(counts);
        }

        let np = posts.len() as f64;
        let idf: HashMap<String, f64> = doc_freq
            .iter()
            .map(|(t, df)| (t.clone(), (np / df).ln() + 1.0))
            .collect();

        for i in 0..posts.len() {
            for j in (i + 1)..posts.len() {
                let dot = doc_tokens[i]
                    .iter()
                    .filter_map(|(t, c)| {
                        doc_tokens[j]
                            .get(t)
                            .map(|cj| c * cj * idf.get(t).copied().unwrap_or(1.0))
                    })
                    .sum::<f64>();

                let mag_i = doc_tokens[i]
                    .iter()
                    .map(|(t, c)| (c * idf.get(t).copied().unwrap_or(1.0)).powi(2))
                    .sum::<f64>()
                    .sqrt();
                let mag_j = doc_tokens[j]
                    .iter()
                    .map(|(t, c)| (c * idf.get(t).copied().unwrap_or(1.0)).powi(2))
                    .sum::<f64>()
                    .sqrt();

                let sim = if mag_i * mag_j > 1e-10 {
                    dot / (mag_i * mag_j)
                } else {
                    0.0
                };

                if sim > 0.08 {
                    let ai = article_indices[posts[i].slug.as_str()];
                    let aj = article_indices[posts[j].slug.as_str()];
                    add_edge(&mut adj, ai, aj, sim * 3.0);
                }
            }
        }
    }

    // ── 3. PageRank ──
    let damping = 0.85;
    let max_iter = 100;
    let tol = 1e-6;
    let mut pr = vec![1.0 / n as f64; n];
    let mut pr_new = vec![0.0; n];

    for _ in 0..max_iter {
        let sink_sum: f64 = pr
            .iter()
            .enumerate()
            .filter(|(i, _)| adj[*i].is_empty())
            .map(|(_, v)| v)
            .sum();

        for i in 0..n {
            let mut rank = 0.0;
            for j in 0..n {
                if adj[j].is_empty() {
                    rank += pr[j] / n as f64;
                } else {
                    let out_sum: f64 = adj[j].iter().map(|(_, w)| w).sum();
                    for (k, w) in &adj[j] {
                        if *k == i {
                            rank += pr[j] * w / out_sum;
                        }
                    }
                }
            }
            pr_new[i] = (1.0 - damping) / n as f64 + damping * (rank + sink_sum / n as f64);
        }

        let diff: f64 = pr.iter().zip(&pr_new).map(|(a, b)| (a - b).abs()).sum();
        pr.copy_from_slice(&pr_new);
        if diff < tol {
            break;
        }
    }

    let pr_max = pr.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let pr_min = pr.iter().cloned().fold(f64::INFINITY, f64::min);
    let pr_normalized: Vec<f64> = if (pr_max - pr_min).abs() < 1e-10 {
        vec![0.5; n]
    } else {
        pr.iter().map(|v| (v - pr_min) / (pr_max - pr_min)).collect()
    };

    // ── 4. 社区发现 ──
    let communities = label_propagation(&adj, n);

    // ── 5. 手动构建 JSON ──
    let mut json = String::from("{\n  \"articles\":{");
    let mut first_article = true;

    let mut rng = Rng::new();

    for post in posts {
        let ai = article_indices[post.slug.as_str()];

        // RWR
        let rwr_scores = random_walk_with_restart(&adj, ai, 0.3, 30, 20, &mut rng);

        let mut rel_arts = Vec::new();
        let mut rel_tags = Vec::new();
        let mut rel_cats = Vec::new();

        for &(node_idx, score) in &rwr_scores {
            if score < 0.01 {
                continue;
            }
            let node_id = &node_ids[node_idx];
            if node_idx != ai && node_id.starts_with("article:") {
                let slug = &node_id[8..];
                let reason = if adj[ai].iter().any(|(k, _)| *k == node_idx) { "内容相似" } else { "标签关联" };
                rel_arts.push((slug.to_string(), score, reason));
            } else if node_id.starts_with("tag:") {
                rel_tags.push((node_id[4..].to_string(), score));
            } else if node_id.starts_with("category:") {
                let cat = &node_id[9..];
                if cat != post.category {
                    rel_cats.push((cat.to_string(), score));
                }
            }
        }

        rel_arts.truncate(8);
        rel_tags.truncate(6);
        rel_cats.truncate(3);

        // BFS
        let (hop1, hop2) = bfs_explore(&adj, &node_ids, ai, 2);

        // 社区名称
        let community_idx = communities[ai];
        let community_name = {
            let mut tag_counts: HashMap<&str, usize> = HashMap::new();
            for (idx, &c) in communities.iter().enumerate() {
                if c == community_idx {
                    let nid = &node_ids[idx];
                    if nid.starts_with("tag:") {
                        *tag_counts.entry(&nid[4..]).or_insert(0) += 1;
                    }
                }
            }
            let mut sorted: Vec<_> = tag_counts.into_iter().collect();
            sorted.sort_by(|a, b| b.1.cmp(&a.1));
            sorted
                .first()
                .map(|(n, _)| format!("{} 生态", n))
                .unwrap_or_else(|| format!("社区 {}", community_idx))
        };

        if !first_article { json.push(','); }
        first_article = false;

        json.push_str(&format!("\n    \"{}\":{{", post.slug));
        json.push_str(&format!("\"pagerank\":\"{:.4}\",", pr_normalized[ai]));
        json.push_str(&format!("\"community\":\"{}\",", community_name));

        // related
        json.push_str("\"related\":{");

        json.push_str("\"articles\":[");
        for (idx, (slug, score, reason)) in rel_arts.iter().enumerate() {
            if idx > 0 { json.push(','); }
            json.push_str(&format!("{{\"slug\":\"{}\",\"score\":\"{:.4}\",\"reason\":\"{}\"}}", slug, score, reason));
        }
        json.push_str("],");

        json.push_str("\"tags\":[");
        for (idx, (name, score)) in rel_tags.iter().enumerate() {
            if idx > 0 { json.push(','); }
            json.push_str(&format!("{{\"name\":\"{}\",\"score\":\"{:.4}\"}}", name, score));
        }
        json.push_str("],");

        json.push_str("\"categories\":[");
        for (idx, (name, score)) in rel_cats.iter().enumerate() {
            if idx > 0 { json.push(','); }
            json.push_str(&format!("{{\"name\":\"{}\",\"score\":\"{:.4}\"}}", name, score));
        }
        json.push(']');
        json.push('}'); // close related

        // explore
        json.push_str(",\"explore\":{");

        json.push_str("\"hop1\":{");
        write_explore_group(&mut json, &hop1);
        json.push_str("},\"hop2\":{");
        write_explore_group(&mut json, &hop2);
        json.push('}');

        json.push('}'); // close explore
        json.push('}'); // close article
    }

    json.push_str("\n  },\n  \"tags\":{},\n  \"node_count\":");
    json.push_str(&n.to_string());
    json.push_str("\n}\n");

    let dest = Path::new("static/knowledge-graph.json");
    std::fs::write(dest, &json).expect("Failed to write knowledge graph");
}

fn write_explore_group(json: &mut String, group: &ExploreGroup) {
    json.push_str("\"articles\":[");
    for (i, s) in group.articles.iter().enumerate() {
        if i > 0 { json.push(','); }
        json.push('"');
        json.push_str(s);
        json.push('"');
    }
    json.push_str("],\"tags\":[");
    for (i, s) in group.tags.iter().enumerate() {
        if i > 0 { json.push(','); }
        json.push('"');
        json.push_str(s);
        json.push('"');
    }
    json.push_str("],\"categories\":[");
    for (i, s) in group.categories.iter().enumerate() {
        if i > 0 { json.push(','); }
        json.push('"');
        json.push_str(s);
        json.push('"');
    }
    json.push(']');
}

struct ExploreGroup {
    articles: Vec<String>,
    tags: Vec<String>,
    categories: Vec<String>,
}

fn bfs_explore(
    adj: &[Vec<(usize, f64)>],
    node_ids: &[String],
    start: usize,
    max_depth: usize,
) -> (ExploreGroup, ExploreGroup) {
    use std::collections::VecDeque;

    let mut visited = vec![false; adj.len()];
    let mut depth = vec![usize::MAX; adj.len()];
    let mut queue = VecDeque::new();

    visited[start] = true;
    depth[start] = 0;
    queue.push_back(start);

    while let Some(u) = queue.pop_front() {
        if depth[u] >= max_depth {
            continue;
        }
        for &(v, _) in &adj[u] {
            if !visited[v] {
                visited[v] = true;
                depth[v] = depth[u] + 1;
                queue.push_back(v);
            }
        }
    }

    fn group_by_type(node_ids: &[String], depth: &[usize], target_depth: usize) -> ExploreGroup {
        let mut articles = Vec::new();
        let mut tags = Vec::new();
        let mut categories = Vec::new();

        for (i, d) in depth.iter().enumerate() {
            if *d != target_depth {
                continue;
            }
            let node = &node_ids[i];
            if node.starts_with("article:") {
                articles.push(node[8..].to_string());
            } else if node.starts_with("tag:") {
                tags.push(node[4..].to_string());
            } else if node.starts_with("category:") {
                categories.push(node[9..].to_string());
            }
        }

        ExploreGroup { articles, tags, categories }
    }

    let mut hop1 = group_by_type(node_ids, &depth, 1);
    let mut hop2 = group_by_type(node_ids, &depth, 2);

    // 限制每跳数量，避免一篇文章关联过多节点
    hop1.articles.truncate(15);
    hop1.tags.truncate(10);
    hop1.categories.truncate(5);
    hop2.articles.truncate(10);
    hop2.tags.truncate(8);
    hop2.categories.truncate(5);

    (hop1, hop2)
}

fn random_walk_with_restart(
    adj: &[Vec<(usize, f64)>],
    start: usize,
    restart_prob: f64,
    max_steps: usize,
    top_k: usize,
    rng: &mut Rng,
) -> Vec<(usize, f64)> {
    let n = adj.len();
    let mut scores = vec![0.0; n];
    let mut current = start;

    for _ in 0..max_steps {
        scores[current] += 1.0 / max_steps as f64;

        if rng.next_f64() < restart_prob {
            current = start;
            continue;
        }

        if adj[current].is_empty() {
            current = start;
        } else {
            let total_w: f64 = adj[current].iter().map(|(_, w)| w).sum();
            let mut r = rng.next_f64() * total_w;
            for &(neighbor, w) in &adj[current] {
                r -= w;
                if r <= 0.0 {
                    current = neighbor;
                    break;
                }
            }
            if r > 0.0 {
                current = adj[current].last().unwrap().0;
            }
        }
    }

    let mut result: Vec<(usize, f64)> = scores.into_iter().enumerate().collect();
    result.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    result.truncate(top_k);
    result
}

fn label_propagation(adj: &[Vec<(usize, f64)>], n: usize) -> Vec<usize> {
    let mut rng = Rng::new();
    let mut labels: Vec<usize> = (0..n).collect();
    let mut changed = true;
    let max_iter = 20;

    for _ in 0..max_iter {
        if !changed {
            break;
        }
        changed = false;

        let mut order: Vec<usize> = (0..n).collect();
        for i in (0..n).rev() {
            let j = rng.next_usize(i + 1);
            order.swap(i, j);
        }

        for &u in &order {
            let mut label_counts: HashMap<usize, f64> = HashMap::new();
            for &(v, w) in &adj[u] {
                *label_counts.entry(labels[v]).or_insert(0.0) += w;
            }
            *label_counts.entry(labels[u]).or_insert(0.0) += 0.1;

            let best_label = label_counts
                .into_iter()
                .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(l, _)| l)
                .unwrap_or(labels[u]);

            if best_label != labels[u] {
                labels[u] = best_label;
                changed = true;
            }
        }
    }

    let mut remap: HashMap<usize, usize> = HashMap::new();
    let mut next_id = 0;
    labels
        .iter()
        .map(|&l| {
            *remap.entry(l).or_insert_with(|| {
                let id = next_id;
                next_id += 1;
                id
            })
        })
        .collect()
}

// ══════════════════════════════════════════════════════════
// TextRank 文章摘要（零外部依赖：纯 std + 手动 JSON）
// ══════════════════════════════════════════════════════════

/// 将文章内容拆分为句子（中英文混合）
fn split_sentences(text: &str) -> Vec<String> {
    let mut sentences: Vec<String> = Vec::new();
    let mut current = String::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];
        current.push(c);

        // 句子结束符：中文标点 或 英文句号/问号/感叹号后跟空格/换行
        let is_end = match c {
            '。' | '！' | '？' | '；' => true,
            '.' | '!' | '?' | ';' => {
                // 英文标点：检查是否可能为缩写（如 e.g. Mr. vs.）
                let next_is_end = i + 1 >= chars.len()
                    || chars[i + 1].is_whitespace()
                    || chars[i + 1] == '\n';
                // 简单判断：前面是数字则可能是小数点，不是句子结束
                let prev_is_digit = i > 0 && chars[i - 1].is_ascii_digit();
                next_is_end && !prev_is_digit
            }
            '\n' => {
                // 连续空行视为段落分隔 = 句子结束
                let mut peek = i + 1;
                while peek < chars.len() && chars[peek] == '\n' {
                    peek += 1;
                }
                if peek - i >= 2 {
                    true
                } else {
                    // 单换行在 markdown 中常见，不强制切句
                    false
                }
            }
            _ => false,
        };

        if is_end {
            let trimmed = current.trim().to_string();
            if !trimmed.is_empty() {
                sentences.push(trimmed);
            }
            current.clear();
        }

        i += 1;
    }

    // 处理最后一段
    let trimmed = current.trim().to_string();
    if !trimmed.is_empty() {
        sentences.push(trimmed);
    }

    sentences
}

/// 计算两个句子的相似度（基于共同 token 的 Jaccard + TF 加权）
fn sentence_similarity(a: &HashMap<String, f64>, b: &HashMap<String, f64>) -> f64 {
    if a.is_empty() || b.is_empty() {
        return 0.0;
    }

    let mut intersection = 0.0;
    let mut union = 0.0;

    // 使用小集合遍历以优化性能
    let (small, large) = if a.len() < b.len() { (a, b) } else { (b, a) };

    for (token, tf_a) in small {
        if let Some(tf_b) = large.get(token) {
            intersection += tf_a.min(*tf_b);
            union += tf_a.max(*tf_b);
        } else {
            union += *tf_a;
        }
    }

    // 加上 large 中不在 small 里的 token 权重
    for (token, tf_b) in large {
        if !small.contains_key(token) {
            union += *tf_b;
        }
    }

    if union < 1e-10 {
        0.0
    } else {
        intersection / union
    }
}

/// TextRank 核心：对句子图运行 PageRank
fn text_rank(
    sentences: &[String],
    damping: f64,
    max_iter: usize,
    convergence_threshold: f64,
) -> Vec<(usize, f64)> {
    let n = sentences.len();
    if n == 0 {
        return Vec::new();
    }
    if n == 1 {
        return vec![(0, 1.0)];
    }

    // 1. 对每个句子分词，计算 TF
    let mut sentence_tokens: Vec<HashMap<String, f64>> = Vec::with_capacity(n);
    for s in sentences {
        let tokens = tokenize(s);
        let mut tf: HashMap<String, f64> = HashMap::new();
        let total = tokens.len() as f64;
        if total > 0.0 {
            for t in tokens {
                *tf.entry(t).or_insert(0.0) += 1.0 / total;
            }
        }
        sentence_tokens.push(tf);
    }

    // 2. 构建相似度矩阵
    let mut sim: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        for j in i + 1..n {
            let s = sentence_similarity(&sentence_tokens[i], &sentence_tokens[j]);
            sim[i][j] = s;
            sim[j][i] = s;
        }
    }

    // 3. 对每行归一化（出边权重和为 1）
    let mut trans: Vec<Vec<f64>> = vec![vec![0.0; n]; n];
    for i in 0..n {
        let row_sum: f64 = sim[i].iter().sum();
        if row_sum > 1e-10 {
            for j in 0..n {
                trans[i][j] = sim[i][j] / row_sum;
            }
        }
    }

    // 4. PageRank 迭代
    let mut scores: Vec<f64> = vec![1.0 / n as f64; n];
    let base = (1.0 - damping) / n as f64;

    for _ in 0..max_iter {
        let mut new_scores = vec![base; n];

        for i in 0..n {
            for j in 0..n {
                if j != i {
                    new_scores[i] += damping * trans[j][i] * scores[j];
                }
            }
        }

        // 检查收敛
        let mut max_diff = 0.0;
        for i in 0..n {
            let diff = (new_scores[i] - scores[i]).abs();
            if diff > max_diff {
                max_diff = diff;
            }
        }

        scores = new_scores;

        if max_diff < convergence_threshold {
            break;
        }
    }

    // 5. 排序并返回索引+分数
    let mut ranked: Vec<(usize, f64)> = (0..n).map(|i| (i, scores[i])).collect();
    ranked.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    ranked
}

/// 生成 AI 摘要：TextRank 提取 Top-N 句子，保留原文顺序
fn extractive_summary(text: &str, top_n: usize) -> Option<String> {
    if text.trim().is_empty() {
        return None;
    }

    let sentences = split_sentences(text);
    if sentences.is_empty() {
        return None;
    }

    // 对太短的文章不缩减
    if sentences.len() <= top_n {
        return Some(text.trim().to_string());
    }

    let ranked = text_rank(&sentences, 0.85, 100, 1e-6);

    // 取 top-N，按原文顺序排列
    let top_indices: Vec<usize> = ranked
        .iter()
        .take(top_n)
        .map(|(idx, _)| *idx)
        .collect();

    let mut ordered: Vec<(usize, &String)> = top_indices
        .iter()
        .map(|&idx| (idx, &sentences[idx]))
        .collect();
    ordered.sort_by_key(|(idx, _)| *idx);

    let summary: String = ordered
        .into_iter()
        .map(|(_, s)| s.as_str())
        .collect::<Vec<_>>()
        .join("");

    if summary == text.trim() {
        Some(summary)
    } else {
        Some(summary)
    }
}

/// 转义 JSON 字符串中的特殊字符（手动实现，免额外依赖）
fn escape_json_string(s: &str) -> String {
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

fn generate_ai_summaries(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/summaries.json");
        std::fs::write(dest, "[]").expect("Failed to write summaries");
        return;
    }

    let mut json = String::from("[\n");

    for (idx, post) in posts.iter().enumerate() {
        // 对每篇文章运行 TextRank，取 top-5 句子
        let summary = extractive_summary(&post.content, 5)
            .unwrap_or_else(|| if post.summary.is_empty() {
                post.content.chars().take(200).collect()
            } else {
                post.summary.clone()
            });

        let escaped_summary = escape_json_string(&summary);
        let escaped_title = escape_json_string(&post.title);

        json.push_str(&format!(
            r#"  {{"slug":"{}","title":"{}","summary":"{}","date":"{}","tags":["{}"],"category":"{}"}}"#,
            post.slug,
            escaped_title,
            escaped_summary,
            post.date,
            post.tags.join(r#"",""#),
            post.category,
        ));

        if idx < posts.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push('\n');
        }
    }

    json.push_str("]\n");

    let dest = Path::new("static/summaries.json");
    std::fs::write(dest, &json).expect("Failed to write summaries json");
}

/// 为马尔可夫链对文本进行分词（保留标点，中文逐字）
fn markov_tokenize(text: &str) -> Vec<String> {
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

/// 生成马尔可夫链续写数据（trigram 模型）
fn generate_markov_chain(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/markov.json");
        std::fs::write(dest, "{}").expect("Failed to write markov json");
        return;
    }

    let mut entries: Vec<String> = Vec::new();

    for post in posts {
        let tokens = markov_tokenize(&post.content);
        if tokens.len() < 10 {  // 太短的文章跳过
            continue;
        }

        // 构建 trigram 模型
        let mut starters: Vec<String> = Vec::new();
        let mut chain: Vec<(String, Vec<(String, u32)>)> = Vec::new();

        if tokens.len() >= 2 {
            starters.push(format!("{}||{}", tokens[0], tokens[1]));
        }

        for i in 0..tokens.len().saturating_sub(2) {
            let key = format!("{}||{}", tokens[i], tokens[i + 1]);
            let next = tokens[i + 2].clone();

            // 找或插入 chain 条目
            if let Some(pos) = chain.iter().position(|(k, _)| *k == key) {
                let nexts = &mut chain[pos].1;
                if let Some(npos) = nexts.iter().position(|(n, _)| *n == next) {
                    nexts[npos].1 += 1;
                } else if nexts.len() < 10 {  // 最多保留 10 个后继
                    nexts.push((next, 1));
                }
            } else {
                chain.push((key, vec![(next, 1)]));
            }
        }

        // 只保留至少出现 2 次的条目（去噪声）
        chain.retain(|(_, nexts)| {
            let total: u32 = nexts.iter().map(|(_, c)| c).sum();
            total >= 2
        });

        if chain.is_empty() {
            continue;
        }

        // 序列化 JSON
        let starters_json: Vec<String> = starters.iter()
            .map(|s| format!(r#""{}""#, escape_json_string(s)))
            .collect();

        let chain_json: Vec<String> = chain.iter()
            .map(|(key, nexts)| {
                let nexts_json: Vec<String> = nexts.iter()
                    .map(|(n, c)| format!(r#""{}":{}"#, escape_json_string(n), c))
                    .collect();
                format!(r#""{}":{{{}}}"#, escape_json_string(key), nexts_json.join(","))
            })
            .collect();

        let article_json = format!(
            r#"{{"s":[{}],"c":{{{}}}}}"#,
            starters_json.join(","),
            chain_json.join(",")
        );

        entries.push(format!(
            r#""{}":{}"#,
            escape_json_string(&post.slug),
            article_json
        ));
    }

    let json = format!("{{{}}}", entries.join(","));
    let dest = Path::new("static/markov.json");
    std::fs::write(dest, &json).expect("Failed to write markov json");
    println!("markov.json written with {} articles", entries.len());
}

// ══════════════════════════════════════════════════════════
// RAKE 关键词提取
// ══════════════════════════════════════════════════════════

/// RAKE 短语分隔符（英文标点 + 中文句读 + 常见停用词）
const RAKE_DELIMITERS: &[&str] = &[
    ",", ".", ":", ";", "!", "?", "(", ")", "[", "]", "{", "}", "\"", "'",
    "，", "。", "：", "；", "！", "？", "（", "）", "【", "】", "《", "》",
    "—", "…", "、", "”", "“", "‘", "’",
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
    "also", "what", "which", "who", "whom", "的", "了", "在", "是",
    "我", "有", "和", "就", "不", "人", "都", "一", "一个", "上",
    "也", "很", "到", "说", "要", "去", "你", "会", "着", "没有",
    "看", "好", "自己", "这", "他", "她", "它", "们", "那", "些",
    "所", "被", "把", "让", "用", "对", "与", "或", "及", "但",
    "而", "且", "因为", "所以", "如果", "虽然", "然而", "因此",
    "然后", "可以", "已经", "还是", "比较", "非常", "之后", "之前",
    "这个", "那个", "这些", "那些", "什么", "怎么", "怎样", "如何",
    "为什么", "是不是", "这样", "那样", "一样", "时候", "现在",
    "一种", "其中", "其他", "很多", "需要", "可能", "一定", "必须",
    "应该", "能够", "不能", "不会", "不断", "通过", "进行", "使用",
    "实现", "问题", "方式", "情况", "方法", "过程", "结果", "不同",
    "主要", "基本", "重要", "一般", "目前", "我们", "他们", "表示",
    "处理", "提供", "支持", "包括", "开发", "运行", "相关", "存在",
    "直接", "得到", "发生", "成为", "开始", "继续", "作用", "利用",
    "考虑", "完成", "工作", "系统", "技术", "内容", "数据", "信息",
    "产生", "具有", "这里", "觉得", "知道", "真的", "喜欢", "帮助",
    "影响", "来说", "东西", "全部", "完全", "变化", "理解", "还有",
];

/// 检查一个 token 是否为 RAKE 分隔符
fn is_rake_delimiter(token: &str) -> bool {
    RAKE_DELIMITERS.contains(&token.to_lowercase().as_str())
}

/// RAKE 关键词提取
/// 返回 (关键词, 分数) 列表，按分数降序排列
fn rake_extract(text: &str, max_keywords: usize) -> Vec<(String, f64)> {
    if text.trim().is_empty() {
        return Vec::new();
    }

    // 1. 分词
    let tokens = markov_tokenize(text);

    // 2. 按分隔符切分成候选短语
    let mut phrases: Vec<Vec<String>> = Vec::new();
    let mut current_phrase: Vec<String> = Vec::new();

    for token in &tokens {
        if is_rake_delimiter(token) {
            if !current_phrase.is_empty() {
                // 过滤掉只有一个字的短语（对中文）
                let phrase_text: String = current_phrase.iter().map(|s| s.as_str()).collect();
                if phrase_text.chars().count() >= 2 {
                    phrases.push(std::mem::take(&mut current_phrase));
                } else {
                    current_phrase.clear();
                }
            }
        } else {
            current_phrase.push(token.clone());
        }
    }
    if !current_phrase.is_empty() {
        let phrase_text: String = current_phrase.iter().map(|s| s.as_str()).collect();
        if phrase_text.chars().count() >= 2 {
            phrases.push(current_phrase);
        }
    }

    if phrases.is_empty() {
        return Vec::new();
    }

    // 3. 构建词频和词共现度
    let mut word_freq: HashMap<String, f64> = HashMap::new();
    let mut word_degree: HashMap<String, f64> = HashMap::new();

    for phrase in &phrases {
        let len = phrase.len() as f64;
        for word in phrase {
            *word_freq.entry(word.clone()).or_insert(0.0) += 1.0;
            *word_degree.entry(word.clone()).or_insert(0.0) += len - 1.0; // 与其他词的共现
        }
    }

    // 4. 计算词分数：degree / frequency
    let mut word_score: HashMap<String, f64> = HashMap::new();
    for (word, &freq) in &word_freq {
        let degree = word_degree.get(word).copied().unwrap_or(0.0);
        word_score.insert(word.clone(), degree / freq.max(1.0));
    }

    // 5. 计算候选短语分数：sum of word scores
    let mut phrase_scores: Vec<(String, f64)> = phrases
        .iter()
        .map(|phrase| {
            let text: String = phrase.join("");
            let score: f64 = phrase.iter().map(|w| word_score.get(w).copied().unwrap_or(0.0)).sum();
            // 短短语有额外加分（避免长短语过于占优）
            let len_bonus = 1.0 / (phrase.len() as f64).sqrt();
            (text, score * len_bonus)
        })
        .collect();

    // 6. 去重，保留最高分
    phrase_scores.sort_by(|(t1, s1), (t2, s2)| {
        t1.cmp(t2).then_with(|| s2.partial_cmp(s1).unwrap_or(std::cmp::Ordering::Equal))
    });
    phrase_scores.dedup_by(|(t1, _), (t2, _)| t1 == t2);

    // 7. 按分数排序取 top-N
    phrase_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    phrase_scores.truncate(max_keywords);

    // 过滤掉常见噪声词（长度太短且分数低的）
    phrase_scores.retain(|(t, s)| {
        let char_count = t.chars().count();
        char_count >= 2 || (char_count == 1 && *s > 2.0)
    });

    phrase_scores
}

/// 生成 RAKE 关键词 JSON
fn generate_rake_keywords(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/rake-keywords.json");
        std::fs::write(dest, "{}").expect("Failed to write rake keywords");
        return;
    }

    let mut json = String::from("{\n");

    for (idx, post) in posts.iter().enumerate() {
        let source = format!(
            "{} {} {}",
            post.title,
            truncate_utf8_safe(&post.content, 3000),
            post.tags.join(" ")
        );

        let keywords = rake_extract(&source, 15);

        let kw_json: Vec<String> = keywords
            .iter()
            .map(|(k, s)| format!(r#"["{}",{:.4}]"#, escape_json_string(k), s))
            .collect();

        json.push_str(&format!(
            r#"  "{}": [{}]"#,
            post.slug,
            kw_json.join(",")
        ));

        if idx < posts.len() - 1 {
            json.push_str(",\n");
        } else {
            json.push('\n');
        }
    }

    json.push_str("}\n");

    let dest = Path::new("static/rake-keywords.json");
    std::fs::write(dest, &json).expect("Failed to write rake keywords");
    println!("rake-keywords.json generated");
}

// ══════════════════════════════════════════════════════════
// LDA 主题模型（Collapsed Gibbs Sampling）
// ══════════════════════════════════════════════════════════

/// 对文章进行中文分词（用于 LDA 输入）
fn lda_tokenize(text: &str) -> Vec<String> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = text.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let c = chars[i];

        if c.is_whitespace() || c.is_ascii_punctuation() {
            i += 1;
            continue;
        }

        // 英文单词
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
            // CJK bigram
            if i + 1 < chars.len() && !chars[i + 1].is_ascii_punctuation() && !chars[i + 1].is_whitespace() {
                let bigram: String = [c, chars[i + 1]].iter().collect();
                tokens.push(bigram);
            }
            tokens.push(c.to_string());
            i += 1;
        }
    }

    tokens
}

/// LDA Collapsed Gibbs Sampling
struct LdaModel {
    /// 主题数
    topics: usize,
    /// 词汇表 (word → index)
    vocab: HashMap<String, usize>,
    /// 词汇表逆映射
    idx_to_word: Vec<String>,
    /// 每篇文档的 token 列表（词索引）
    doc_tokens: Vec<Vec<usize>>,
    /// 每个 token 当前分配的主题
    topic_assignments: Vec<Vec<usize>>,
    /// n_dt[d][t]: 文档 d 中分配给主题 t 的 token 数
    n_dt: Vec<Vec<f64>>,
    /// n_wt[w][t]: 词 w 分配给主题 t 的次数
    n_wt: Vec<Vec<f64>>,
    /// n_t[t]: 主题 t 的总 token 数
    n_t: Vec<f64>,
    alpha: f64,
    beta: f64,
}

impl LdaModel {
    fn new(
        docs: &[Vec<String>],
        topics: usize,
        alpha: f64,
        beta: f64,
    ) -> Self {
        // 构建词汇表
        let mut vocab: HashMap<String, usize> = HashMap::new();
        let mut idx_to_word: Vec<String> = Vec::new();
        for doc in docs {
            for word in doc {
                if !vocab.contains_key(word) {
                    vocab.insert(word.clone(), idx_to_word.len());
                    idx_to_word.push(word.clone());
                }
            }
        }

        let vocab_size = vocab.len();
        let n_docs = docs.len();
        let mut rng = Rng::new();

        // 将文档 token 转为词索引
        let doc_tokens: Vec<Vec<usize>> = docs
            .iter()
            .map(|doc| {
                doc.iter()
                    .filter_map(|w| vocab.get(w).copied())
                    .collect()
            })
            .collect();

        // 初始化随机主题分配
        let mut topic_assignments: Vec<Vec<usize>> = Vec::with_capacity(n_docs);
        let mut n_dt: Vec<Vec<f64>> = vec![vec![0.0; topics]; n_docs];
        let mut n_wt: Vec<Vec<f64>> = vec![vec![0.0; topics]; vocab_size];
        let mut n_t = vec![0.0; topics];

        for (d, tokens) in doc_tokens.iter().enumerate() {
            let mut assigns = Vec::with_capacity(tokens.len());
            for &w in tokens {
                let t = rng.next_usize(topics);
                assigns.push(t);
                n_dt[d][t] += 1.0;
                n_wt[w][t] += 1.0;
                n_t[t] += 1.0;
            }
            topic_assignments.push(assigns);
        }

        LdaModel {
            topics,
            vocab,
            idx_to_word,
            doc_tokens,
            topic_assignments,
            n_dt,
            n_wt,
            n_t,
            alpha,
            beta,
        }
    }

    /// 运行 Gibbs Sampling
    fn train(&mut self, iterations: usize) {
        let vocab_size = self.vocab.len();
        let mut rng = Rng::new();

        for _ in 0..iterations {
            for d in 0..self.doc_tokens.len() {
                for (i, &w) in self.doc_tokens[d].iter().enumerate() {
                    let old_t = self.topic_assignments[d][i];

                    // 移除当前 token 的计数
                    self.n_dt[d][old_t] -= 1.0;
                    self.n_wt[w][old_t] -= 1.0;
                    self.n_t[old_t] -= 1.0;

                    // 计算每个主题的条件概率
                    let mut probs = vec![0.0; self.topics];
                    let mut total = 0.0;
                    for t in 0..self.topics {
                        let p_dt = (self.n_dt[d][t] + self.alpha)
                            / (self.doc_tokens[d].len() as f64 + self.alpha * self.topics as f64);
                        let p_wt = (self.n_wt[w][t] + self.beta)
                            / (self.n_t[t] + self.beta * vocab_size as f64);
                        probs[t] = p_dt * p_wt;
                        total += probs[t];
                    }

                    // 采样新主题
                    let mut r = rng.next_f64() * total;
                    let mut new_t = 0;
                    for t in 0..self.topics {
                        r -= probs[t];
                        if r <= 0.0 {
                            new_t = t;
                            break;
                        }
                    }

                    // 分配新主题
                    self.topic_assignments[d][i] = new_t;
                    self.n_dt[d][new_t] += 1.0;
                    self.n_wt[w][new_t] += 1.0;
                    self.n_t[new_t] += 1.0;
                }
            }
        }
    }

    /// 获取每篇文档的主题分布
    fn doc_topic_distribution(&self, d: usize) -> Vec<(usize, f64)> {
        let total = self.n_dt[d].iter().sum::<f64>() + self.alpha * self.topics as f64;
        let mut dist: Vec<(usize, f64)> = self.n_dt[d]
            .iter()
            .enumerate()
            .map(|(t, &count)| (t, (count + self.alpha) / total))
            .collect();
        dist.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        dist
    }

    /// 获取每个主题的 top-N 词汇
    fn topic_words(&self, t: usize, n: usize) -> Vec<(String, f64)> {
        let vocab_size = self.vocab.len();
        let total = self.n_t[t] + self.beta * vocab_size as f64;

        let mut word_probs: Vec<(usize, f64)> = (0..vocab_size)
            .map(|w| (w, (self.n_wt[w][t] + self.beta) / total))
            .collect();
        word_probs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        word_probs.truncate(n);

        word_probs
            .into_iter()
            .map(|(w, p)| (self.idx_to_word[w].clone(), p))
            .collect()
    }

    /// 为主题命名（基于 top meaningful words）
    fn topic_name(&self, t: usize) -> String {
        let top_words = self.topic_words(t, 10);
        // 过滤掉标点、单字虚词等无意义的 token，取前 3 个有意义的词
        let meaningful: Vec<&str> = top_words
            .iter()
            .map(|(w, _)| w.as_str())
            .filter(|w| {
                let is_single_char = w.chars().count() == 1;
                if is_single_char {
                    let c = w.chars().next().unwrap();
                    let cjk_punct = ['，', '。', '、', '！', '？', '；', '：', '（', '）', '【', '】', '《', '》', '—', '…', '·', '～'];
                    if cjk_punct.contains(&c) { return false; }
                    if c.is_ascii_punctuation() { return false; }
                }
                let stop: &[&str] = &["的", "了", "是", "在", "有", "和", "与", "这", "那", "上", "下", "中", "个", "以", "就", "不", "也", "都", "而", "之", "其", "中", "或", "将", "被", "能"];
                !stop.contains(w)
            })
            .take(3)
            .collect();
        if meaningful.is_empty() {
            // fallback: 直接用 top 2 词
            top_words
                .iter()
                .take(2)
                .map(|(w, _)| w.as_str())
                .collect::<Vec<_>>()
                .join("")
        } else {
            meaningful.join(" / ")
        }
    }
}

/// 生成 LDA 主题模型 JSON
fn generate_lda_topics(posts: &[PostData]) {
    if posts.len() < 3 {
        let dest = Path::new("static/lda-topics.json");
        std::fs::write(dest, "{}").expect("Failed to write lda topics");
        return;
    }

    // 为每篇文章分词
    let docs: Vec<Vec<String>> = posts
        .iter()
        .map(|post| {
            let source = format!(
                "{} {} {} {}",
                post.title,
                post.title, // 标题加权
                post.tags.join(" "),
                truncate_utf8_safe(&post.content, 3000)
            );
            lda_tokenize(&source)
        })
        .collect();

    // 主题数：取文章数 / 2 和 8 的较小值，至少 3
    let num_topics = (posts.len() / 2).max(3).min(8);

    let mut lda = LdaModel::new(&docs, num_topics, 0.1, 0.01);
    lda.train(200);

    // 输出 JSON
    let mut json = String::from("{\n");

    // 主题 → 词汇
    json.push_str(r#"  "topics": {"#);
    for t in 0..num_topics {
        let words = lda.topic_words(t, 8);
        let words_json: Vec<String> = words
            .iter()
            .map(|(w, p)| format!(r#"["{}",{:.4}]"#, escape_json_string(w), p))
            .collect();
        json.push_str(&format!(r#""{}": [{}]"#, t, words_json.join(",")));
        if t < num_topics - 1 {
            json.push(',');
        }
    }
    json.push_str("},\n");

    // 主题名称
    json.push_str(r#"  "topic_names": ["#);
    for t in 0..num_topics {
        if t > 0 { json.push(','); }
        json.push('"');
        json.push_str(&escape_json_string(&lda.topic_name(t)));
        json.push('"');
    }
    json.push_str("],\n");

    // 每篇文章的主题分布
    json.push_str(r#"  "articles": {"#);
    for (idx, post) in posts.iter().enumerate() {
        let dist = lda.doc_topic_distribution(idx);
        let dist_json: Vec<String> = dist
            .iter()
            .map(|(t, p)| format!(r#""{}":{:.4}"#, t, p))
            .collect();
        json.push_str(&format!(
            r#""{}":{{{}}}"#,
            post.slug,
            dist_json.join(",")
        ));
        if idx < posts.len() - 1 {
            json.push(',');
        }
    }
    json.push_str("}\n");

    json.push_str("}\n");

    let dest = Path::new("static/lda-topics.json");
    std::fs::write(dest, &json).expect("Failed to write lda topics");
    println!("lda-topics.json generated with {} topics", num_topics);
}