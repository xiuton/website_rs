---
title: "windows终端命令受限"
date: 2024-03-15 09:18:00
author: 干徒
link: https://www.cnblogs.com/ganto/articles/18074472
tags: ["Windows", "技术"]
---

# windows终端命令受限

在windows系统上，通过以下命令安装pnpm包管理工具，然后在终端执行`pnpm -v`会报错

```sh
npm install -g pnpm

pnpm -v # 报错

```

运行get-ExecutionPolicy，显示Restricted（受限的）

```sh
get-ExecutionPolicy
Restricted

```

运行set-ExecutionPolicy RemoteSigned，设置RemoteSigned

```sh
set-ExecutionPolicy RemoteSigned

```

再次查看

```sh
get-ExecutionPolicy
RemoteSigned

```