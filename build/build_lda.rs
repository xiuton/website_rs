use crate::build_common::*;
use std::collections::HashMap;
use std::path::Path;

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
pub fn generate_lda_topics(posts: &[PostData]) {
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
