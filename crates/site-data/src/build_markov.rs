use site_content::*;
use site_model::{ArticleChain, MarkovModel};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::Path;

type Pair = (u32, u32);
type ChainMap = BTreeMap<Pair, HashMap<u32, u32>>;
type ChainEntry = (Pair, Vec<(u32, u32)>);

/// 生成紧凑格式的 markov 模型（序列化由 serde 负责）：
///
/// - token 只存一份字典，转移里全部用 `u32` 下标，避免重复存字符串
/// - 只保留出现 ≥2 次的转移，模型体积比原始 trigram 小一个数量级
pub fn generate_markov_chain(posts: &[PostData]) {
    if posts.is_empty() {
        let dest = Path::new("static/markov.json");
        let model = MarkovModel::new(Vec::new(), HashMap::new());
        let json = serde_json::to_string(&model).expect("serialize empty markov model");
        std::fs::write(dest, json).expect("Failed to write markov json");
        return;
    }

    // 先分词并建立全局 token 字典（BTreeSet 保证顺序稳定）
    let tokenized: Vec<(&PostData, Vec<String>)> =
        posts.iter().map(|post| (post, markov_tokenize(&post.content))).collect();
    let mut vocab: BTreeSet<String> = BTreeSet::new();
    for (_, tokens) in &tokenized {
        for token in tokens {
            vocab.insert(token.clone());
        }
    }
    let tokens: Vec<String> = vocab.into_iter().collect();
    let token_ids: HashMap<&str, u32> =
        tokens.iter().enumerate().map(|(i, token)| (token.as_str(), i as u32)).collect();

    let mut articles: HashMap<String, ArticleChain> = HashMap::new();

    for (post, raw_tokens) in &tokenized {
        if raw_tokens.len() < 10 {
            continue;
        }
        let ids: Vec<u32> = raw_tokens.iter().map(|t| token_ids[t.as_str()]).collect();
        if ids.len() < 3 {
            continue;
        }

        // trigram 统计
        let mut chain: ChainMap = BTreeMap::new();
        for window in ids.windows(3) {
            let key = (window[0], window[1]);
            *chain.entry(key).or_default().entry(window[2]).or_insert(0) += 1;
        }

        // 剪枝：每个 key 最多 3 个后继，并且只保留至少出现 2 次的转移。
        // 绝大多数 trigram 只出现一次，全部丢弃后模型会小一个数量级；
        // 起始 pair 改为从存活下来的 key 中取，保证仍能生成。
        let mut chain_entries: Vec<ChainEntry> = Vec::with_capacity(chain.len());
        for (key, nexts) in chain {
            let mut nexts: Vec<(u32, u32)> = nexts.into_iter().collect();
            nexts.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
            nexts.truncate(3);
            if nexts.first().map(|(_, count)| *count >= 2).unwrap_or(false) {
                nexts.retain(|(_, count)| *count >= 2);
                chain_entries.push((key, nexts));
            }
        }
        if chain_entries.is_empty() {
            continue;
        }
        // 起点从存活 key 中取，最多 8 个
        let starters: Vec<[u32; 2]> = chain_entries.iter().take(8).map(|((a, b), _)| [*a, *b]).collect();
        let chain: Vec<([u32; 2], Vec<[u32; 2]>)> = chain_entries
            .into_iter()
            .map(|((a, b), nexts)| ([a, b], nexts.into_iter().map(|(token, count)| [token, count]).collect()))
            .collect();

        articles.insert(post.slug.clone(), ArticleChain { s: starters, c: chain });
    }

    let model = MarkovModel::new(tokens, articles);
    let json = serde_json::to_string(&model).expect("serialize markov model");
    let dest = Path::new("static/markov.json");
    std::fs::write(dest, &json).expect("Failed to write markov json");
    println!(
        "markov.json written: {} articles, {} tokens, {} KB",
        model.articles.len(),
        model.tokens.len(),
        json.len() / 1024
    );
}
