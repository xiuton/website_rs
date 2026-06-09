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

    // 生成知识图谱
    generate_knowledge_graph(&posts);

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