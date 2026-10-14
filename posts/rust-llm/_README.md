# rust-llm 系列文章配置说明

本系列采用**「元数据集中在 `_config.toml`」**的写法：章节 Markdown 里**不写 front matter**，
所有标题、摘要、标签都在 [_config.toml](_config.toml) 里维护。

## 一、配置字段

### 1. 系列级默认值（顶层字段）

```toml
series      = "Rust 大语言模型 学习指南"   # 系列名称（首页入口标题、文章页系列导航）
author      = "干徒"                      # 默认作者
date        = "2026-09-01 12:00:00"       # 默认日期
tags        = ["Rust", "LLM"]             # 默认标签（与章节标签合并）
slug_prefix = "rust-llm-guide"            # slug 前缀（slug_style = "prefix" 时使用）
summary     = "系列摘要"                   # 只用于首页「文档入口」卡片
slug_style  = "prefix"                    # 章节 slug 生成方式，可选 "prefix" | "date"
```

`slug_style` 的两种取值：

| 取值 | 章节 slug | 例子 |
|------|----------|------|
| `"prefix"`（默认，本系列在用） | `{slug_prefix}-{order:02}` | `rust-llm-guide-01` |
| `"date"` | 用日期，与普通文章一致；同日依次 `-2`、`-3` | `20240511`、`20240511-2` |

日期风格的编号顺序是**确定的**，按 `完整日期时间 → order → 文件名` 排序后分配：

- 同一天里时间更早的拿基础 slug（`22:14` → `20250220`，`22:27` → `20250220-2`），不会随机互换；
- 时间完全相同时（本系列 51 章共用同一个 `date`）才由 `order` 决定先后，
  因此本系列即使改用 `slug_style = "date"`，也会按章节顺序得到 `20260901`、`20260901-2`…

> 没有数字序号的章节（例如 `vue one.md`）必须靠 `order` 或 `slug_style = "date"` 区分，
> 否则多章会撞成同一个 slug —— 构建时会给出 `cargo:warning=[slug 重复]` 告警。

### 2. 章节级元数据（`[chapters]` 表）

键 = **章节文件名去掉 `.md`**：

```toml
[chapters."01-张量与基本运算"]
title   = "第 1 课：张量 Tensor —— 一切的基础"
summary = "从零实现张量的多维数组表示、内存布局与矩阵乘法。"
tags    = ["张量"]
```

每个条目支持的字段与 front matter 完全一致：

| 字段 | 说明 |
|------|------|
| `title` | 章节标题（**不写则自动从文件名派生**） |
| `summary` | 章节摘要（SEO description、列表展示、搜索索引） |
| `tags` | 章节标签（与顶层 `tags`、front matter 合并去重） |
| `date` / `author` / `series` | 覆盖系列级默认值 |
| `order` | 章节序号（不写则从文件名 `NN-` 前缀提取） |
| `slug` | 自定义 URL（不写则按 `slug_style` 生成） |
| `catalog` | 覆盖系列目录页路径段（默认取文件夹名 `rust-llm`） |

键也可以写成**纯编号**（模糊匹配文件名前缀）：

```toml
[chapters."02"]
title = "第 2 课：自动微分 Autograd"
```

也可以用数组写法 `[[chapters]]`（优先级高于表，适合字段较多的条目）：

```toml
[[chapters]]
file    = "03-张量运算扩展"
title   = "第 3 课：张量运算扩展"
tags    = ["张量"]
order   = 3
```

## 二、章节 Markdown 长什么样

只有正文，**没有 `---` 元数据块**：

```markdown
# 第 1 课：张量 Tensor —— 一切的基础

正文从这里直接开始……
```

> ⚠️ 注意：正文顶部的 `---` 水平分割线请勿写在文件最前面（前面至少要有一个标题或正文行），
> 否则会被当成 front matter 的起始标记。

## 三、取值优先级

按「具体 → 笼统」排列，前者有值则胜出：

```
front matter  >  [chapters] 章节配置  >  顶层系列默认值  >  派生兜底（文件名）
```

`tags` 是唯一例外，三个来源**合并去重**：

```
顶层 tags  →  章节 tags  →  front matter tags
```

派生兜底只在「文件完全没有 front matter」时启用：

| 字段 | 派生规则 |
|------|---------|
| `title` | 章节键去掉 `NN-` 前缀，`-`/`_` 换成空格（如 `01-张量与基本运算` → `张量与基本运算`） |
| `order` | 文件名首个 `${数字}-` 段（如 `25-KV-Cache.md` → `25`） |

## 四、编译期校验

`cargo build` 时会对 `_config.toml` 做一致性检查，问题以 `cargo:warning` 输出：

| 提示 | 含义 |
|------|------|
| `[chapters] 条目 "xx" 未匹配到任何章节文件` | 键拼错了，或章节文件被改名/删除 |
| `[chapters] 条目 "xx" 同时匹配了 a.md 与 b.md` | 编号键产生了歧义，请改写成完整文件名 |
| `[chapters] 条目 "xx" 是按编号匹配到 yy.md 的` | 模糊匹配成功但建议写成完整文件名 |
| `[front matter 未闭合]` | 文件开头有 `---` 却没有收尾的 `---`，正文会被当成元数据吞掉 |
| `[标题为空]` | 既没配 `title` 也无法派生，请补配置 |
| `[slug 重复]` | 多篇文章生成了同一个 URL（重复的那篇永远访问不到），见下 |

**slug 重复**是唯一的全站级检查（不限于本目录）：只要有两条数据拿到同一个 slug 就会告警，
并指出是哪两个文件：

```
cargo:warning=[slug 重复] "2023-docs-00" 同时属于 5分钟快速入门Taro.md 与 css常用场景.md：
后一篇将无法通过 URL 访问。请在 _config.toml 中为章节设置 order、或使用 slug_style = "date"、或直接指定 slug
```

常见成因与修法：

| 成因 | 修法 |
|------|------|
| 章节文件名没有数字前缀（`vue one.md`），`order` 全部兜底为 0 | 改名为 `01-vue-one.md`，或在 `[chapters]` 里写 `order`，或改用 `slug_style = "date"` |
| 多篇文章 `date` 相同、且都用日期型 slug | 用 `slug_style = "date"`（会自动加 `-2`/`-3`）或给文件加数字前缀 |
| 同一 `slug_prefix` + 相同 `order` | 检查 `order` 是否有重复 |

> 说明：`cargo` 只在构建脚本**重新执行**时打印这些警告；如果只是想立刻看一遍全部提示，
> 可以先 `touch build.rs`（或改动任意 `posts/` 下的文件）再执行 `cargo build`。

## 五、排序规则

系列目录页与章节导航按以下优先级排序：

1. `order` 字段（升序）
2. 文件名字典序（升序）
3. `date` 字段（降序）

## 六、覆盖配置

某一章需要「破例」时，直接在章节 Markdown 里加 front matter 即可，它会覆盖 `_config.toml`
（`tags` 仍然合并）：

```markdown
---
title: "特别篇：从零手写调度器"
summary: "本篇不跟随系列摘要"
date: "2026-10-01 09:00:00"
tags: ["特别"]
---
```

这种写法可以和「只写配置」的章节混用，互不影响。
