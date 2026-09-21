# rust-llm 系列文章配置说明

## 配置文件

[_config.toml](_config.toml) 存储本系列文章的默认元数据，避免每篇文章重复填写相同字段。

## 配置字段

```toml
author = "干徒"                        # 作者
series = "Rust 大语言模型 学习指南"      # 系列名称
summary = "系列文档摘要"               # 系列摘要（用于首页列表展示）
tags = ["Rust", "LLM"]                 # 默认标签（会与文章标签合并）
date = "2026-09-01 12:00:00"           # 默认日期
slug_prefix = "rust-llm-guide"         # slug 前缀，生成格式：{prefix}-{order}
```

## 文章 front matter

每篇文章只需填写**独有的字段**，配置文件中的字段会自动填充：

```yaml
---
title: "第 1 课：张量 Tensor —— 一切的基础"
tags: ["张量"]
summary: "张量是深度学习里所有数据的统一容器..."
---
```

## 合并规则

| 字段 | 合并方式 | 说明 |
|------|---------|------|
| `author` | 覆盖 | front matter 有值则用，否则用配置 |
| `series` | 覆盖 | front matter 有值则用，否则用配置 |
| `date` | 覆盖 | front matter 有值则用，否则用配置 |
| `summary` | 覆盖 | front matter 有值则用，否则用配置 |
| `tags` | 合并 | 配置 tags + 文章 tags，去重 |
| `slug` | 自动生成 | 格式：`{slug_prefix}-{order:02}` |
| `order` | 优先 front matter | front matter 有值则用，否则从文件名数字提取（如 `01-xxx.md` → `1`） |

## 首页列表展示

系列文档在首页列表中合并展示，使用以下规则：
- 标题：使用配置文件的 `series` 字段
- 摘要：使用配置文件的 `summary` 字段（如果配置了）
- 作者：使用配置文件的 `author` 字段
- 日期：使用所有章节中最晚的日期
- 标签：合并所有文章的标签（配置文件 + 所有章节），去重

## 排序规则

系列文档按以下优先级排序：
1. `order` 字段（升序）
2. 文件名字典序（升序）
3. `date` 字段（降序）

## 示例

**配置文件 `_config.toml`：**
```toml
author = "干徒"
series = "Rust 大语言模型 学习指南"
summary = "不使用任何深度学习框架，仅用 Rust 标准库从零手写张量、自动微分、Transformer，最终训练出一个能生成文本的小型 GPT 模型。"
tags = ["Rust", "LLM"]
date = "2026-09-01 12:00:00"
slug_prefix = "rust-llm-guide"
```

**文章 `01-张量与基本运算.md`：**
```yaml
---
title: "第 1 课：张量 Tensor —— 一切的基础"
tags: ["张量"]
summary: "张量是深度学习里所有数据的统一容器..."
---
```

**生成的最终元数据：**
```json
{
  "slug": "rust-llm-guide-01",
  "title": "第 1 课：张量 Tensor —— 一切的基础",
  "date": "2026-09-01 12:00:00",
  "author": "干徒",
  "tags": ["Rust", "LLM", "张量"],
  "series": "Rust 大语言模型 学习指南",
  "series_summary": "不使用任何深度学习框架，仅用 Rust 标准库从零手写张量、自动微分、Transformer，最终训练出一个能生成文本的小型 GPT 模型。",
  "order": 1
}
```

## 覆盖配置

如果某篇文章需要不同的值，直接在 front matter 中指定即可：

```yaml
---
title: "特别篇"
author: "其他作者"    # 覆盖配置文件的 author
date: "2026-10-01"   # 覆盖配置文件的 date
summary: "特别篇摘要" # 覆盖配置文件的 summary
tags: ["特别"]        # 会与配置的 ["Rust", "LLM"] 合并
---
```