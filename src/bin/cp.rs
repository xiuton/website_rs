use chrono::Local;
use std::env;
use std::fs;
use std::path::Path;

fn main() {
    // 获取命令行参数
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("用法: cp <文件名>");
        println!("示例: cp my-post");
        return;
    }

    // 获取文件名
    let filename = &args[1];
    let filename = if filename.ends_with(".md") {
        filename.to_string()
    } else {
        format!("{}.md", filename)
    };

    // 获取当前日期时间（精确到秒）
    let now = Local::now();
    let date = now.format("%Y-%m-%d %H:%M:%S").to_string();

    // 从文件名生成标题（移除扩展名，将连字符替换为空格，首字母大写）
    let title = filename
        .trim_end_matches(".md")
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().chain(chars).collect(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    // 创建模板内容
    let content = format!(
        r#"---
title: "{}"
date: "{}"
author: "干徒"
tags: ["tag"]
---

# Title One

Text Example

## Title Two

Text Example
"#,
        title, date
    );

    // 确保 posts 目录存在
    let posts_dir = Path::new("public/posts");
    if !posts_dir.exists() {
        fs::create_dir_all(posts_dir).expect("无法创建 posts 目录");
    }

    // 生成唯一的文件名
    let mut final_filename = filename.clone();
    let mut counter = 1;
    let mut is_renamed = false;
    while posts_dir.join(&final_filename).exists() {
        let name_without_ext = filename.trim_end_matches(".md");
        final_filename = format!("{}-{}.md", name_without_ext, counter);
        counter += 1;
        is_renamed = true;
    }

    // 写入文件
    let file_path = posts_dir.join(&final_filename);
    fs::write(&file_path, content).expect("无法写入文件");

    // 统一使用正斜杠显示路径
    let display_path = format!("public/posts/{}", final_filename);
    if is_renamed {
        println!("文件已重命名为: {}", display_path);
    } else {
        println!("已创建: {}", display_path);
    }
} 