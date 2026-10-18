#[path = "build/build_atom.rs"]
mod build_atom;
#[path = "build/build_common.rs"]
mod build_common;
#[path = "build/build_embeddings.rs"]
mod build_embeddings;
#[path = "build/build_knowledge_graph.rs"]
mod build_knowledge_graph;
#[path = "build/build_lda.rs"]
mod build_lda;
#[path = "build/build_markov.rs"]
mod build_markov;
#[path = "build/build_posts_json.rs"]
mod build_posts_json;
#[path = "build/build_rake.rs"]
mod build_rake;
#[path = "build/build_rss.rs"]
mod build_rss;
#[path = "build/build_search.rs"]
mod build_search;
#[path = "build/build_semantic.rs"]
mod build_semantic;
#[path = "build/build_seo.rs"]
mod build_seo;
#[path = "build/build_summaries.rs"]
mod build_summaries;

use build_atom::generate_atom_feed;
use build_common::*;
use build_embeddings::generate_article_embeddings;
use build_knowledge_graph::generate_knowledge_graph;
use build_lda::generate_lda_topics;
use build_markov::generate_markov_chain;
use build_posts_json::generate_posts_json;
use build_rake::generate_rake_keywords;
use build_rss::generate_rss_feed;
use build_search::generate_search_index;
use build_semantic::generate_semantic_index;
use build_seo::generate_sitemap;
use build_summaries::generate_ai_summaries;

use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=posts");
    // build/ 下的生成逻辑改动也要触发重跑，否则会出现「代码改了、JSON 没变」
    println!("cargo:rerun-if-changed=build");
    println!("cargo:rerun-if-changed=build.rs");

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR must be set");
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");

    let posts_dir = Path::new("posts");
    if !posts_dir.exists() {
        let blog_posts = "pub const BUILD_ID: &str = \"empty\";\n\npub const BLOG_POSTS: &[BlogPost] = &[];";
        fs::write(dest_path, blog_posts).expect("Failed to write empty blog posts");
        return;
    }

    let mut posts: Vec<PostData> = Vec::new();
    let mut date_count: HashMap<String, i32> = HashMap::new();

    scan_dir(posts_dir, posts_dir, "", &mut posts, &mut date_count);

    // 统一分配日期型 slug（普通文章 + slug_style = "date" 的系列章节），并检查 slug 唯一性
    assign_slugs(&mut posts);

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    // front matter 没写 summary 时，构建期生成纯文本摘要作为兜底。
    // 客户端不再需要为此运行 comrak（见 build_posts_json::excerpt_from_markdown）。
    for post in &mut posts {
        if post.summary.trim().is_empty() {
            let excerpt = build_posts_json::excerpt_from_markdown(&post.content, 150);
            post.summary = if excerpt.trim().is_empty() { post.series_summary.clone() } else { excerpt };
        }
    }

    // 构建指纹：文章内容变化时改变，用于给 /static/*.json 与 post-html 加缓存版本号
    let build_id = {
        use std::hash::{Hash, Hasher};
        let mut hasher = std::collections::hash_map::DefaultHasher::new();
        for post in &posts {
            post.slug.hash(&mut hasher);
            post.date.hash(&mut hasher);
            post.content.hash(&mut hasher);
        }
        format!("{:x}", hasher.finish())
    };

    let mut output = format!("pub const BUILD_ID: &str = \"{build_id}\";\n\npub const BLOG_POSTS: &[BlogPost] = &[\n");

    for post in &posts {
        output.push_str(&format!(
            "    BlogPost {{\n        title: r#####\"{}\"#####,\n        date: r#####\"{}\"#####,\n        author: r#####\"{}\"#####,\n        tags: &[{}],\n        slug: r#####\"{}\"#####,\n        category: r#####\"{}\"#####,\n        summary: r#####\"{}\"#####,\n        series: r#####\"{}\"#####,\n        order: {},\n        sort_key: r#####\"{}\"#####,\n        catalog: r#####\"{}\"#####,\n        filename: r#####\"{}\"#####,\n        series_summary: r#####\"{}\"#####,\n    }},\n",
            post.title,
            post.date,
            post.author,
            post.tags.iter()
                .map(|t| format!("\"{}\"", escape_rust_string(t)))
                .collect::<Vec<_>>()
                .join(", "),
            post.slug,
            post.category,
            post.summary,
            post.series,
            post.order,
            escape_rust_string(&post.sort_key),
            post.catalog,
            post.filename,
            post.series_summary,
        ));
    }

    output.push_str("];\n");

    fs::write(dest_path, output).expect("Failed to write blog posts");

    generate_rss_feed(&posts, out_dir.to_str().unwrap());
    generate_atom_feed(&posts, out_dir.to_str().unwrap());
    generate_search_index(&posts);
    generate_semantic_index(&posts);
    generate_article_embeddings(&posts);
    generate_knowledge_graph(&posts);
    generate_ai_summaries(&posts);
    generate_markov_chain(&posts);
    generate_rake_keywords(&posts);
    generate_lda_topics(&posts);
    generate_sitemap(&posts);
    generate_posts_json(&posts);
}
