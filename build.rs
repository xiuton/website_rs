use std::env;
use std::fs;
use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=posts");
    
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("blog_posts.rs");
    
    let posts_dir = Path::new("posts");
    if !posts_dir.exists() {
        fs::write(&dest_path, "pub const BLOG_POSTS: &[BlogPost] = &[];").unwrap();
        return;
    }
    
    let mut posts = Vec::new();
    
    if let Ok(entries) = fs::read_dir(posts_dir) {
        for entry in entries.flatten() {
            if let Ok(file_type) = entry.file_type() {
                if file_type.is_file() {
                    if let Some(ext) = entry.path().extension() {
                        if ext == "md" {
                            if let Ok(content) = fs::read_to_string(entry.path()) {
                                let mut lines = content.lines();
                                let mut front_matter = String::new();
                                let mut in_front_matter = false;
                                let mut post_content = String::new();
                                
                                // 解析 front matter
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
                                
                                // 解析内容
                                for line in lines {
                                    post_content.push_str(line);
                                    post_content.push('\n');
                                }
                                
                                // 解析 front matter
                                let title = front_matter
                                    .lines()
                                    .find(|l| l.starts_with("title:"))
                                    .map(|l| l.replace("title:", "").trim().trim_matches('"').to_string())
                                    .unwrap_or_default();
                                
                                let date = front_matter
                                    .lines()
                                    .find(|l| l.starts_with("date:"))
                                    .map(|l| l.replace("date:", "").trim().trim_matches('"').to_string())
                                    .unwrap_or_default();
                                
                                let author = front_matter
                                    .lines()
                                    .find(|l| l.starts_with("author:"))
                                    .map(|l| l.replace("author:", "").trim().trim_matches('"').to_string())
                                    .unwrap_or_default();
                                
                                let tags = front_matter
                                    .lines()
                                    .find(|l| l.starts_with("tags:"))
                                    .map(|l| {
                                        l.replace("tags:", "")
                                            .trim()
                                            .trim_matches(|c| c == '[' || c == ']')
                                            .split(',')
                                            .map(|s| s.trim().trim_matches('"').to_string())
                                            .collect::<Vec<_>>()
                                    })
                                    .unwrap_or_default();
                                
                                posts.push((title, date, author, tags, post_content));
                            }
                        }
                    }
                }
            }
        }
    }
    
    // 按日期排序（最新的在前）
    posts.sort_by(|a, b| b.1.cmp(&a.1));
    
    let mut output = String::from("pub const BLOG_POSTS: &[BlogPost] = &[\n");
    
    for (title, date, author, tags, content) in posts {
        output.push_str(&format!(
            "    BlogPost {{\n        title: \"{}\",\n        date: \"{}\",\n        author: \"{}\",\n        tags: &[{}],\n        content: r#\"{}\"#,\n    }},\n",
            title,
            date,
            author,
            tags.iter()
                .map(|t| format!("\"{}\"", t))
                .collect::<Vec<_>>()
                .join(", "),
            content
        ));
    }
    
    output.push_str("];\n");
    
    fs::write(&dest_path, output).unwrap();
} 