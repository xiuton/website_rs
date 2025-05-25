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

    // 确保目标目录存在
    let posts_dir = Path::new("posts");
    fs::create_dir_all(posts_dir).expect("无法创建posts目录");

    // 生成最终文件名
    let final_filename = format!("{}.md", title);
    let display_path = format!("posts/{}", final_filename);

    // 写入文件
    let file_path = posts_dir.join(&final_filename);
    fs::write(&file_path, content).expect("无法写入文件");

    println!("已创建: {}", display_path);
} 