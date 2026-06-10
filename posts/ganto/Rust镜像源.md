---
title: "Rust镜像源"
date: 2025-03-13 09:27:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18769264
tags: ["Rust", "技术"]
---
## 配置Rust镜像源

配置文件位置  
`~/.cargo/config.toml`

修改 config.toml

```toml
[source.crates-io]
replace-with = 'aliyun'
[source.aliyun]
registry = "sparse+https://mirrors.aliyun.com/crates.io-index/"

```