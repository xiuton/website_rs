#[path = "build/build_common.rs"]
mod build_common;
#[path = "build/build_rss.rs"]
mod build_rss;
#[path = "build/build_search.rs"]
mod build_search;
#[path = "build/build_semantic.rs"]
mod build_semantic;
#[path = "build/build_embeddings.rs"]
mod build_embeddings;
#[path = "build/build_knowledge_graph.rs"]
mod build_knowledge_graph;
#[path = "build/build_summaries.rs"]
mod build_summaries;
#[path = "build/build_markov.rs"]
mod build_markov;
#[path = "build/build_rake.rs"]
mod build_rake;
#[path = "build/build_lda.rs"]
mod build_lda;

use build_common::*;
use build_rss::generate_rss_feed;
use build_search::generate_search_index;
use build_semantic::generate_semantic_index;
use build_embeddings::generate_article_embeddings;
use build_knowledge_graph::generate_knowledge_graph;
use build_summaries::generate_ai_summaries;
use build_markov::generate_markov_chain;
use build_rake::generate_rake_keywords;
use build_lda::generate_lda_topics;

use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn main() {
    println!("cargo:rerun-if-changed=posts");

    let out_dir = env::var_os("OUT_DIR").expect("OUT_DIR must be set");
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");

    let posts_dir = Path::new("posts");
    if !posts_dir.exists() {
        let blog_posts = "pub const BLOG_POSTS: &[BlogPost] = &[];";
        fs::write(dest_path, blog_posts).expect("Failed to write empty blog posts");
        return;
    }

    let mut posts: Vec<PostData> = Vec::new();
    let mut date_count: HashMap<String, i32> = HashMap::new();

    scan_dir(posts_dir, posts_dir, "", &mut posts, &mut date_count);

    posts.sort_by(|a, b| b.date.cmp(&a.date));

    let mut output = String::from("pub const BLOG_POSTS: &[BlogPost] = &[\n");

    for post in &posts {
        output.push_str(&format!(
            "    BlogPost {{\n        title: r#####\"{}\"#####,\n        date: r#####\"{}\"#####,\n        author: r#####\"{}\"#####,\n        tags: &[{}],\n        content: r#####\"{}\"#####,\n        slug: r#####\"{}\"#####,\n        category: r#####\"{}\"#####,\n        summary: r#####\"{}\"#####,\n    }},\n",
            post.title,
            post.date,
            post.author,
            post.tags.iter()
                .map(|t| format!("\"{}\"", escape_rust_string(t)))
                .collect::<Vec<_>>()
                .join(", "),
            post.content,
            post.slug,
            post.category,
            post.summary,
        ));
    }

    output.push_str("];\n");

    fs::write(dest_path, output).expect("Failed to write blog posts");

    generate_rss_feed(&posts, out_dir.to_str().unwrap());
    generate_search_index(&posts);
    generate_semantic_index(&posts);
    generate_article_embeddings(&posts);
    generate_knowledge_graph(&posts);
    generate_ai_summaries(&posts);
    generate_markov_chain(&posts);
    generate_rake_keywords(&posts);
    generate_lda_topics(&posts);

    println!("cargo:rerun-if-changed=build.rs");
}