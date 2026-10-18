use crate::build_common::*;
use std::path::Path;

pub fn generate_markov_chain(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/markov.json");
        std::fs::write(dest, "{}").expect("Failed to write markov json");
        return;
    }

    let mut entries: Vec<String> = Vec::new();

    for post in posts {
        let tokens = markov_tokenize(&post.content);
        if tokens.len() < 10 {
            // 太短的文章跳过
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
                } else if nexts.len() < 20 {
                    // 最多保留 20 个后继
                    nexts.push((next, 1));
                }
            } else {
                chain.push((key, vec![(next, 1)]));
            }
        }

        // 剪枝：trigram 表里绝大多数转移只出现一次，是体积的主要来源。
        // 每个 key 最多保留 5 个后继；只要存在出现 ≥2 次的转移，就丢掉只出现一次的噪声。
        // 这样既能把 markov.json 压下去，又保留常见搭配的生成能力。
        for (_, nexts) in &mut chain {
            nexts.sort_by(|a, b| b.1.cmp(&a.1));
            nexts.truncate(5);
            if nexts.iter().any(|(_, count)| *count >= 2) {
                nexts.retain(|(_, count)| *count >= 2);
            }
        }
        chain.retain(|(_, nexts)| !nexts.is_empty());

        if chain.is_empty() {
            continue;
        }

        // 序列化 JSON
        let starters_json: Vec<String> = starters.iter().map(|s| format!(r#""{}""#, escape_json_string(s))).collect();

        let chain_json: Vec<String> = chain
            .iter()
            .map(|(key, nexts)| {
                let nexts_json: Vec<String> = nexts
                    .iter()
                    .map(|(n, c)| format!(r#""{}":{}"#, escape_json_string(n), c))
                    .collect();
                format!(r#""{}":{{{}}}"#, escape_json_string(key), nexts_json.join(","))
            })
            .collect();

        let article_json = format!(r#"{{"s":[{}],"c":{{{}}}}}"#, starters_json.join(","), chain_json.join(","));

        entries.push(format!(r#""{}":{}"#, escape_json_string(&post.slug), article_json));
    }

    let json = format!("{{{}}}", entries.join(","));
    let dest = Path::new("static/markov.json");
    std::fs::write(dest, &json).expect("Failed to write markov json");
    println!("markov.json written with {} articles", entries.len());
}
