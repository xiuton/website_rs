use crate::build_common::*;
use std::collections::HashMap;
use std::path::Path;

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
pub fn generate_rake_keywords(posts: &[PostData]) {
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

