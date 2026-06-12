use crate::build_common::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn generate_search_index(posts: &[PostData]) {
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
