use crate::build_common::*;
use std::collections::HashMap;
use std::path::Path;

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


pub fn generate_ai_summaries(posts: &[PostData]) {
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

