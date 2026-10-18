//! 站点数据生成器集合。
//!
//! 这些模块原来直接挂在 `build.rs` 的 `#[path]` 模块里，导致：
//! - 无法单独测试
//! - `build.rs` 变成 130 行的调度中心
//! - 生成逻辑与编译脚本生命周期绑定
//!
//! 现在统一放到独立 crate，`build.rs` 只负责调用。

pub mod build_atom;
pub mod build_embeddings;
pub mod build_knowledge_graph;
pub mod build_lda;
pub mod build_markov;
pub mod build_posts_json;
pub mod build_rake;
pub mod build_rss;
pub mod build_search;
pub mod build_semantic;
pub mod build_seo;
pub mod build_summaries;

pub use build_atom::generate_atom_feed;
pub use build_embeddings::generate_article_embeddings;
pub use build_knowledge_graph::generate_knowledge_graph;
pub use build_lda::generate_lda_topics;
pub use build_markov::generate_markov_chain;
pub use build_posts_json::generate_posts_json;
pub use build_rake::generate_rake_keywords;
pub use build_rss::generate_rss_feed;
pub use build_search::generate_search_index;
pub use build_semantic::generate_semantic_index;
pub use build_seo::generate_sitemap;
pub use build_summaries::generate_ai_summaries;

/// 一次性生成全部站点数据。
///
/// 若文章内容与生成器源码都没变，并且上一轮产物仍在，则跳过生成；
/// 缓存戳写到 `OUT_DIR`，不会进入 `static/`/`dist`。
pub fn generate_all(posts: &[site_content::PostData], out_dir: &str) {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::path::Path;

    let mut hasher = DefaultHasher::new();
    env!("SITE_DATA_HASH").hash(&mut hasher);
    for post in posts {
        post.slug.hash(&mut hasher);
        post.title.hash(&mut hasher);
        post.date.hash(&mut hasher);
        post.summary.hash(&mut hasher);
        post.series.hash(&mut hasher);
        post.content.hash(&mut hasher);
    }
    let stamp = format!("{:x}", hasher.finish());
    let cache_path = Path::new(out_dir).join(".site-data-cache");
    let cache_hit = std::fs::read_to_string(&cache_path).ok().as_deref() == Some(stamp.as_str())
        && Path::new("static/post-html").is_dir()
        && [
            "static/posts-meta.json",
            "static/markov.json",
            "static/search-index.json",
            "static/semantic.json",
            "static/embeddings.json",
            "static/knowledge-graph.json",
            "static/summaries.json",
            "static/lda-topics.json",
            "static/rake-keywords.json",
            "static/sitemap.xml",
            "static/feed.xml",
            "static/atom.xml",
        ]
        .iter()
        .all(|path| Path::new(path).is_file());
    if cache_hit {
        println!("site-data: unchanged, reuse previous generated files");
        return;
    }

    generate_rss_feed(posts, out_dir);
    generate_atom_feed(posts, out_dir);
    generate_search_index(posts);
    generate_semantic_index(posts);
    generate_article_embeddings(posts);
    generate_knowledge_graph(posts);
    generate_ai_summaries(posts);
    generate_markov_chain(posts);
    generate_rake_keywords(posts);
    generate_lda_topics(posts);
    generate_sitemap(posts);
    generate_posts_json(posts);
    let _ = std::fs::write(cache_path, stamp);
}
