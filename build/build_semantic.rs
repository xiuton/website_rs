use crate::build_common::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn generate_semantic_index(posts: &[PostData]) {
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

