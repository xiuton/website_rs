use crate::build_common::*;
use std::collections::{HashMap, HashSet};
use std::path::Path;

pub fn generate_knowledge_graph(posts: &[PostData]) {
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
