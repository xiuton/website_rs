use std::env;
use std::fs;
use std::path::Path;
use std::collections::HashMap;

fn escape_rust_string(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"")
}

fn strip_yaml_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

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

    let mut posts: Vec<(String, String, String, Vec<String>, String, String, String, String)> = Vec::new();
    let mut date_count: HashMap<String, i32> = HashMap::new();

    scan_dir(posts_dir, posts_dir, "", &mut posts, &mut date_count);

    posts.sort_by(|a, b| b.1.cmp(&a.1));

    let mut output = String::from("pub const BLOG_POSTS: &[BlogPost] = &[\n");

    for (title, date, author, tags, content, slug, category, summary) in posts {
        output.push_str(&format!(
            "    BlogPost {{\n        title: r#####\"{}\"#####,\n        date: r#####\"{}\"#####,\n        author: r#####\"{}\"#####,\n        tags: &[{}],\n        content: r#####\"{}\"#####,\n        slug: r#####\"{}\"#####,\n        category: r#####\"{}\"#####,\n        summary: r#####\"{}\"#####,\n    }},\n",
            title,
            date,
            author,
            tags.iter()
                .map(|t| format!("\"{}\"", escape_rust_string(t)))
                .collect::<Vec<_>>()
                .join(", "),
            content,
            slug,
            category,
            summary,
        ));
    }

    output.push_str("];\n");

    fs::write(dest_path, output).expect("Failed to write blog posts");
    println!("cargo:rerun-if-changed=build.rs");
}

fn scan_dir(
    dir: &Path,
    base_dir: &Path,
    category: &str,
    posts: &mut Vec<(String, String, String, Vec<String>, String, String, String, String)>,
    date_count: &mut HashMap<String, i32>,
) {
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_dir() {
                    let dir_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("");
                    scan_dir(&path, base_dir, dir_name, posts, date_count);
                } else if file_type.is_file() {
                    if let Some(ext) = path.extension() {
                        if ext == "md" {
                            if let Ok(content) = fs::read_to_string(&path) {
                                process_post(
                                    &content,
                                    category,
                                    posts,
                                    date_count,
                                );
                            }
                        }
                    }
                }
            }
        }
    }
}

fn process_post(
    content: &str,
    category: &str,
    posts: &mut Vec<(String, String, String, Vec<String>, String, String, String, String)>,
    date_count: &mut HashMap<String, i32>,
) {
    let mut lines = content.lines();
    let mut front_matter = String::new();
    let mut in_front_matter = false;
    let mut post_content = String::new();

    while let Some(line) = lines.next() {
        if line == "---" {
            if !in_front_matter {
                in_front_matter = true;
                continue;
            } else {
                break;
            }
        }
        if in_front_matter {
            front_matter.push_str(line);
            front_matter.push('\n');
        }
    }

    for line in lines {
        post_content.push_str(line);
        post_content.push('\n');
    }

    let title = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("title:"))
            .map(|l| l.replace("title:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let date = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("date:"))
            .map(|l| l.replace("date:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let author = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("author:"))
            .map(|l| l.replace("author:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let tags = front_matter
        .lines()
        .find(|l| l.starts_with("tags:"))
        .map(|l| {
            l.replace("tags:", "")
                .trim()
                .trim_matches(|c| c == '[' || c == ']')
                .split(',')
                .map(|s| strip_yaml_quotes(s.trim()))
                .collect::<Vec<_>>()
        })
        .unwrap_or_default();

    let summary = strip_yaml_quotes(
        &front_matter
            .lines()
            .find(|l| l.starts_with("summary:"))
            .map(|l| l.replace("summary:", "").trim().to_string())
            .unwrap_or_default(),
    );

    let date_parts: Vec<&str> = date.split(' ').collect();
    let date_str = if date_parts.len() >= 1 {
        date_parts[0].replace('-', "")
    } else {
        "00000000".to_string()
    };

    let count = date_count.entry(date_str.clone()).or_insert(0);
    *count += 1;
    let base_slug = if *count > 1 {
        format!("{}-{}", date_str, *count)
    } else {
        date_str
    };

    let slug = base_slug;

    posts.push((
        title,
        date,
        author,
        tags,
        post_content,
        slug,
        category.to_string(),
        summary,
    ));
}