use crate::build_common::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn generate_article_embeddings(posts: &[PostData]) {
    if posts.is_empty() {
        return;
    }

    let n = posts.len();

    // 1. 计算每篇文章的 TF-IDF 向量
    let mut doc_tokens_list: Vec<HashMap<String, f64>> = Vec::with_capacity(n);
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
        // 标题和标签加权
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
        .map(|(token, df)| (token.clone(), (n as f64 / df).ln() + 1.0))
        .collect();

    // 2. 构建 L2 归一化的 TF-IDF 向量
    let mut vectors: Vec<HashMap<String, f64>> = Vec::with_capacity(n);
    for dt in &doc_tokens_list {
        let total: f64 = dt.values().sum();
        if total == 0.0 {
            vectors.push(HashMap::new());
            continue;
        }
        let mut tfidf: HashMap<String, f64> = HashMap::new();
        for (token, count) in dt {
            let tf = count / total;
            let idf_val = idf.get(token).copied().unwrap_or(1.0);
            tfidf.insert(token.clone(), tf * idf_val);
        }
        // L2 归一化
        let mag: f64 = tfidf.values().map(|v| v * v).sum::<f64>().sqrt();
        if mag > 1e-10 {
            for v in tfidf.values_mut() {
                *v /= mag;
            }
        }
        vectors.push(tfidf);
    }

    // 3. 计算两两余弦相似度，取 top-10
    let threshold = 0.05;
    let mut all_related: Vec<Vec<(usize, f64)>> = Vec::with_capacity(n);

    for i in 0..n {
        let mut sims: Vec<(usize, f64)> = Vec::new();
        for j in 0..n {
            if i == j {
                continue;
            }
            // 余弦相似度（向量已 L2 归一化，点积即余弦相似度）
            let mut dot = 0.0_f64;
            for (key, va) in &vectors[i] {
                if let Some(vb) = vectors[j].get(key) {
                    dot += va * vb;
                }
            }
            if dot > threshold {
                sims.push((j, dot));
            }
        }
        sims.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sims.truncate(10);
        all_related.push(sims);
    }

    // 4. 输出 JSON: { "slug": [{"slug":"...","score":0.95}, ...], ... }
    let dest = Path::new("static/embeddings.json");
    let mut json = String::from("{");
    for (i, related) in all_related.iter().enumerate() {
        if related.is_empty() {
            continue;
        }
        if json.len() > 1 {
            json.push(',');
        }
        json.push_str(&format!(
            r#""{}":["#,
            escape_json_string(&posts[i].slug)
        ));
        for (k, (j, score)) in related.iter().enumerate() {
            if k > 0 {
                json.push(',');
            }
            json.push_str(&format!(
                r#"{{"slug":"{}","score":{:.4}}}"#,
                escape_json_string(&posts[*j].slug),
                score
            ));
        }
        json.push(']');
    }
    json.push('}');
    std::fs::write(dest, &json).expect("Failed to write embeddings.json");
    println!("embeddings.json written with {} articles", n);
}
