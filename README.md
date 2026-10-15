# 干徒 - Ganto's Website

基于 **Rust + Dioxus + WebAssembly** 构建的个人网站，提供博客、书签管理、代码实验等功能。

## 功能特点

- 🌓 **亮色/暗色主题切换** — 支持 View Transition API 平滑过渡，记忆用户偏好
- 📱 **响应式设计** — 适配桌面端与移动端，工具栏自动调整位置
- ⚡ **WebAssembly 驱动** — 基于 Rust 编译为 Wasm，接近原生的运行性能
- 📝 **Markdown 博客** — Front Matter 元数据，comrak 渲染，highlight.js 代码高亮
- 📚 **多章节文档支持** — 同一 `series` + 同一文件夹的文章自动聚合为文档，文章页底部显示**可折叠的章节目录**（默认折叠，只露标题行与上下篇）与「目录页」入口；首页列表合并显示为一条文档入口；子目录下无 `series` 字段的文章作为分类归总独立显示
- ⚙️ **目录配置文件** — 系列文档支持 `_config.toml` 配置文件，统一管理重复元数据（author、series、tags、date、summary）
- 🗂️ **章节元数据集中配置** — `_config.toml` 的 `[chapters]` / `[[chapters]]` 可逐章配置 `title`/`summary`/`tags`，章节 Markdown **可以完全不写 front matter**；两种写法可逐文件混用
- 🔗 **slug 生成策略可选** — `slug_prefix`（`rust-llm-guide-01`）或 `slug_style = "date"`（`20240511`，同日期依次 `-2`/`-3`）；构建期检查全局 slug 唯一性，撞车直接告警
- 🏷️ **标签系统** — 文章标签展示；首页卡片标签超过 10 个时折叠为「+N」，点击展开/收起
- 🔖 **书签管理** — TOML 配置的书签页面，支持搜索筛选
- 🎮 **操场 (Playground)** — 代码实验环境
- 🔧 **开发工具页** — 内置开发辅助功能
- 🧪 **测试页面** — 组件功能验证
- 🔄 **宽屏模式** — 博客文章支持一键切换宽屏布局
- 📦 **GitHub Actions CI/CD** — 自动构建部署到 GitHub Pages
- ☁️ **Netlify 部署** — SPA 路由重定向配置
- 🔍 **SEO 优化** — sitemap.xml / robots.txt、JSON-LD 结构化数据（WebSite / BlogPosting）、OG 分享图、文章静态预渲染（爬虫可直接读取全文）

## 技术栈

| 类别 | 技术 |
|------|------|
| 框架 | [Dioxus 0.6.3](https://dioxuslabs.com/) |
| 语言 | Rust (edition 2021) |
| 编译目标 | wasm32-unknown-unknown |
| 构建工具 | [Trunk 0.21](https://trunkrs.dev/) |
| 样式 | SCSS (Sass) |
| Markdown 渲染 | [comrak](https://docs.rs/comrak) (GFM 兼容) |
| 代码高亮 | [highlight.js 11](https://highlightjs.org/) |
| HTTP 客户端 | gloo-net（加载 posts.json / markov.json 等预生成数据） |
| 路由 | dioxus-router 0.6.3 |
| 构建脚本 | `build.rs` 编译期解析博客文章 |
| 数据生成 | `build/` 目录下多个 `build_*.rs` 模块（posts.json、rss、atom、搜索索引、摘要、SEO 文件等） |
| 预渲染工具 | `src/bin/prerender.rs`（trunk 构建后生成文章静态页与 404.html） |
| 字体 | Inter, JetBrains Mono, MiSans, 得意黑 |

## 环境要求

- **Rust** 1.70.0+
- **wasm32-unknown-unknown** 目标
- **Trunk** (Rust → Wasm 构建工具)
- **Sass** (SCSS 编译)

## 安装步骤

### 1. 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### 2. 添加 Wasm 编译目标

```bash
rustup target add wasm32-unknown-unknown
```

### 3. 安装 Trunk

```bash
cargo install trunk
```

### 4. 安装 Sass

```bash
npm install -g sass
```

### 5. 克隆项目

```bash
git clone https://github.com/xiuton/website_rs.git
cd website_rs
```

## 开发指南

### 启动开发服务器

```bash
trunk serve
```

默认在 `http://localhost:8080` 访问。

自定义端口：

```bash
trunk serve --port 8081
```

> `trunk serve` 只提供 **SPA**（页面由 Wasm 在浏览器里渲染），不带 SEO 预渲染页；
> 而且它在**每次源码/文章变动时会重建整个 `dist/`**，之前生成的 `dist/post/**` 会被清掉。
> 这是正常的：开发时看 SPA 就够了，预渲染页只在打包发布前生成。

### 构建生产版本

**顺序不能颠倒**：

```bash
cargo build --release --bin web-rs   # 1. 触发 build.rs，生成 static/*.json
trunk build --release                # 2. 编译 Wasm，并把 static/ 复制进 dist/
cargo run --release --bin prerender  # 3. 依赖 dist/static/posts.json，逐篇生成静态页
```

构建产物输出到 `dist/` 目录。

> ⚠️ **顺序为什么重要**：`build.rs` 负责把 `posts/` 解析成 `static/posts.json`、
> `markov.json`、`search-index.json`、RSS/Atom、sitemap 等数据文件；`trunk build`
> 再通过 `<link data-trunk rel="copy-dir" href="static">` 把它们复制进 `dist/static/`。
> `prerender` 读的是 `dist/static/posts.json`，所以第 1、2 步必须走在它前面。
> 如果跳过第 1 步（或改完文章没重新构建），页面会因缺少数据文件而报
> `加载 xxx.json 失败`。
>
> 另外：`trunk build` / `trunk serve` 会**重建整个 `dist/`**，所以预渲染页在每次
> trunk 构建后都需要重新生成（即重跑上面的第 3 步）。

**关于 SRI：** [Trunk.toml](Trunk.toml) 中设置了 `no_sri = true`。Trunk 默认会把
integrity 摘要写进 `index.html`，但它热重载时会替换资源文件，浏览器如果缓存了旧的
`index.html` 就会因摘要不匹配而拒绝加载资源（`Failed to find a valid digest ...`）。
静态托管下文件名本身已带内容指纹，SRI 收益很低，因此统一关闭。

生成文章静态页（SEO 预渲染，爬虫无需执行 JS 即可读取全文）：

```bash
cargo run --release --bin prerender
```

该命令读取 `dist/static/posts.json`，为每篇文章生成 `dist/post/<slug>/index.html`，并将 `dist/index.html` 复制为 `dist/404.html`（GitHub Pages SPA 兜底）。CI 部署流程会自动执行此步骤。

### 清理并重新构建

```bash
cargo clean ; trunk clean ; trunk build ; trunk serve
```

### 新建博客文章

```bash
cargo run --bin new 文章标题
```

将在 `posts/` 目录下生成带 Front Matter 模板的 Markdown 文件。

查看帮助：

```bash
cargo run --bin new
```

**Front Matter 格式示例：**

```yaml
---
title: 我的第一篇文章
date: 2026-04-19 20:30:00
author: 干徒
tags: [rust, dioxus]
---

# 一级标题

正文内容...
```

**多章节文档（系列）：** 一篇长文档拆成多个章节文件时，将这些章节文件放在一个独立子目录中（如 `posts/rust-guide/`），目录名会自动作为系列目录页的路径标识。

**目录配置文件（推荐）：** 在系列目录下创建 `_config.toml` 文件，统一管理重复元数据：

```toml
# posts/rust-guide/_config.toml
author = "干徒"
series = "Rust 学习指南"
summary = "系列文档摘要"
tags = ["Rust"]
date = "2026-09-01 12:00:00"
slug_prefix = "rust-guide"
# 章节 slug 生成方式，可选（默认 prefix）：
#   "prefix" → rust-guide-01、rust-guide-02…
#   "date"   → 20240511、20240511-2…（与普通文章一致）
slug_style = "prefix"
```

第一种用法：文章只写独有字段，配置文件的值自动填充：

```yaml
---
title: "第三章 所有权"
tags: ["所有权"]
summary: "Rust 的所有权系统是其核心特性之一..."
---
```

第二种用法（章节元数据集中配置）：**章节文件里一个字段都不写**，全部放进 `_config.toml` 的
`[chapters]` 表，键是章节文件名（不含 `.md`）：

```toml
# posts/rust-guide/_config.toml（在顶层字段之后追加）
[chapters."01-所有权"]
title   = "第一章 所有权"
summary = "Rust 的所有权系统是其核心特性之一..."
tags    = ["所有权"]

# 键也可以写成纯编号（模糊匹配文件名前缀）
[chapters."02"]
title = "第二章 借用与生命周期"
```

也可以使用等价的数组写法（优先级高于表，适合字段较多的条目）：

```toml
[[chapters]]
file  = "03-生命周期"
title = "第三章 生命周期"
order = 3
```

对应的章节文件只剩正文：

```markdown
# 第一章 所有权

正文直接开始……
```

> ⚠️ 正文顶部的 `---` 水平分割线不要写在文件最前面（前面至少要有一行标题或正文），
> 否则会被当成 front matter 的起始标记。

**合并规则：**

| 字段 | 合并方式 | 说明 |
|------|---------|------|
| `title` | 覆盖 | front matter → `[chapters]` → （无 front matter 时）文件名派生 |
| `author`、`series`、`date` | 覆盖 | front matter → `[chapters]` → 配置文件顶层 |
| `summary` | 覆盖 | front matter → `[chapters]` → 配置文件顶层（顶层 `summary` 同时作为首页入口的系列摘要） |
| `tags` | 合并 | 顶层 + `[chapters]` + front matter，三处合并去重 |
| `slug` | 自动生成 | front matter / `[chapters]` 自定义优先，否则按 `slug_style` 生成（见下） |
| `order` | 优先 front matter | 其次 `[chapters]`，最后从文件名数字提取（如 `03-xxx.md` → `3`） |
| `catalog` | 目录名 | 默认取文件夹名，`[chapters]` 可覆盖 |

**slug 生成规则：**

| `slug_style` | 系列章节的 slug | 例子 |
|--------------|----------------|------|
| `"prefix"`（默认） | `{slug_prefix}-{order:02}` | `rust-guide-01`、`rust-guide-02` |
| `"date"` | 用日期（与普通文章相同），同日期依次加 `-2`、`-3` | `20240511`、`20240511-2` |

- 没有 `slug_prefix` 时，系列章节回退为文件名，避免多章都落到同一个 slug 上。
- 日期编号**完全确定**，按 **`完整日期时间` → `order` → `文件名` → 原始顺序** 排序后依次分配：
  - 同一天里**时间更早的一定拿到基础 slug**。例如同日的两篇
    `2025-02-20 22:14:00` 与 `2025-02-20 22:27:00`，前者是 `20250220`，后者是 `20250220-2`
    —— 不会因为构建顺序不同而互换。
  - 时间完全相同（如共用一个 `date` 的系列章节）时才轮到 `order` 决定先后，
    所以给系列章节配 `order`（或文件名 `NN-` 前缀）能让 URL 顺序符合章节顺序。
  - 最后两级（文件名、原始顺序）只用于兜底，保证**同样输入永远得到同样结果**。
- 没有数字序号的章节（如 `vue one.md`）必须靠 `[chapters]` 里的 `order`、
  或改用 `slug_style = "date"` 来区分，否则会撞成同一个 slug。

**slug 唯一性校验：** 构建时会全量检查一遍，发现重复立刻告警并指出是哪两篇
（重复的 slug 会让后一篇永远无法通过 URL 访问）：

```
cargo:warning=[slug 重复] "2023-docs-00" 同时属于 5分钟快速入门Taro.md 与 css常用场景.md：
后一篇将无法通过 URL 访问。请在 _config.toml 中为章节设置 order、或使用 slug_style = "date"、或直接指定 slug
```

**章节键匹配规则（依次尝试）：**

1. 与文件名去掉扩展名后完全相等（`[chapters."01-所有权"]`）
2. 键是数字编号，与文件名首个 `-`/`_` 前的片段相等（`[chapters."01"]` ↔ `01-所有权.md`）
3. 键是数字编号，与章节 `order` 相等（`[chapters."1"]`）

**编译期校验：** 配置中的章节条目若没有匹配到任何章节文件、或同一个条目匹配到多个文件、
或章节最终标题为空，构建时都会输出 `cargo:warning` 提示，格式说明见
`posts/rust-llm/_README.md`。

**首页列表展示：** 系列文档在首页列表中合并展示，使用以下规则：
- 标题：使用配置文件的 `series` 字段
- 摘要：使用配置文件的 `summary` 字段（如果配置了）
- 作者：使用配置文件的 `author` 字段
- 日期：使用所有章节中最晚的日期
- 标签：合并所有文章的标签（配置文件 + 所有章节），去重

**系列章节的排序规则：** 按以下三级优先级，由 `build.rs` 预计算成 `sort_key` 后统一比较：

| 优先级 | 依据 | 什么时候生效 |
|--------|------|-------------|
| 1 | **显式 `order`** | front matter `order`、`[chapters]` 的 `order`、或文件名 `NN-` 前缀（如 `01-所有权.md`）。**只要有一个来源给了序号就用序号**，值 0 也是有效序号 |
| 2 | **时间倒序**（近的在前） | 完全没有序号可参考时——按完整 `YYYY-MM-DD HH:MM:SS` 排序，**时间越晚越靠前** |
| 3 | **标题**（字典序） | 时间也相同时——按标题首字母顺序 |

规则要点：

- 有 `order` 的章节**永远排在没 `order` 的章节前面**（`sort_key` 用 `0`/`1` 前缀区分两个层级）。
  想严格控制章节阅读顺序，请用 `NN-` 前缀或配 `order`；不写序号时会退化为「新的在前」。
- 时间倒序通过对日期时间**逐位取 9 的补数**实现（`invert_datetime`）：越晚的时间得到的 key 越小，
  字符串比较即可完成倒序，调用方无需反向比较。
- 三级依据都相同（极少见）时才退化为文件名比较，保证结果完全确定。

**首页「文档入口」取哪一章：** 取排序最靠前的那一章作为入口（标题/摘要用入口章 + 系列配置）。

**文件过滤：** 以 `_` 开头的文件（如 `_README.md`、`_config.toml`）会被自动跳过，不会被识别为文章。

**手动指定（不用配置文件时）：** 直接在 front matter 中添加 `series` 和 `order` 字段：

```yaml
---
title: 第三章 所有权
date: 2026-09-03 09:00:00
series: "Rust 学习指南"
order: 3
slug: "rust-guide-03"
---
```

**`[series]` 分组（可选）：** 若想把系列元数据与章节表在视觉上分开，可以改用 `[series]` 分组，
其中字段与顶层同名字段等价，且优先于顶层写法：

```toml
[series]
name        = "Rust 学习指南"   # 等价于顶层 series
summary     = "系列文档摘要"     # 等价于顶层 summary
author      = "干徒"
tags        = ["Rust"]
slug_prefix = "rust-guide"
```

**参考实现：** `posts/rust-llm/` 就是「章节零 front matter」的完整示例，
其 [_config.toml](posts/rust-llm/_config.toml) 集中维护 51 章元数据，
细节见 [配置说明](posts/rust-llm/_README.md)。

**目录页路径：** 由文件夹名自动决定，无需在 front matter 中配置。例如 `posts/rust-guide/` 下的系列文章，目录页地址为 `/series/rust-guide`。

**文章分类归总：** 如果子目录下的文章没有 `series` 字段，该目录仅作为分类归总（如 `posts/old-docs/`），文章会像普通文章一样独立显示，文件夹名作为分类标签。

同一 `series` 的文章会被自动聚合（以文件夹为单位区分，同名 `series` 在不同文件夹中视为不同系列）：

- 文章页底部显示**可折叠的章节目录**（按 `order` 排序、当前章高亮、默认折叠）与**上一篇/下一篇**导航
  - 折叠/展开由标题行控制：点标题行任意位置切换，箭头随之翻转
  - 「目录页」按钮位于列表上方，折叠时也可点击
  - 折叠状态保持在 DOM 上（`<details>`），无 JS 时同样可用
- 首页列表只显示一条**文档入口**（带"系列 · 共 N 章"标记），不会列出每个章节
- 首页卡片的标签超过 10 个时折叠为「+N」按钮，点击展开全部、再点收起
- 点击文档入口进入**目录页**（`/series/<目录名>`），从目录页再进入各章节

> `build.rs` 会在编译时扫描 `posts/` 目录，解析所有 `.md` 文件的 Front Matter，
> 生成包含文章元数据和内容的 Rust 数组，嵌入到最终 Wasm 产物中。

## 项目结构

```
.
├── .github/
│   └── workflows/
│       └── build.yml        # GitHub Actions CI/CD（含 SEO 预渲染步骤）
├── build/                   # 构建期数据生成模块（build.rs 调用）
│   ├── build_common.rs      # 共享工具：扫描 posts、front matter 解析、_config.toml、slug 分配
│   ├── build_posts_json.rs  # 生成 posts.json
│   ├── build_rss.rs         # RSS 订阅
│   ├── build_atom.rs        # Atom 订阅
│   ├── build_search.rs      # 搜索索引
│   ├── build_summaries.rs   # AI 摘要分类
│   ├── build_seo.rs         # sitemap.xml 生成
│   └── ...                  # 其余 build_*.rs 模块
├── data/                    # 应用数据
│   └── bookmarks.toml       # 书签配置
├── docs/                    # 设计文档
│   └── optimization-analysis.md
├── posts/                   # 博客 Markdown 文章
│   ├── Rust所有权.md         # 根目录文章：slug 用日期，如 20230111
│   ├── old-docs/            # 分类归总目录（无 series 字段，文章独立显示）
│   │   ├── _config.toml     # slug_style = "date"
│   │   └── ...
│   ├── rust-llm/            # 多章节文档（有 series 字段，自动聚合为系列）
│   │   ├── _config.toml     # 系列配置 + [chapters] 逐章元数据
│   │   ├── _README.md       # 配置文件使用说明
│   │   ├── 00-学习计划.md     # 章节文件无 front matter，正文直接开始
│   │   └── ...
│   └── ...
├── src/
│   ├── bin/
│   │   ├── new.rs           # CLI 工具：新建博客文章
│   │   └── prerender.rs     # CLI 工具：文章静态预渲染（SEO）
│   ├── assets/
│   │   └── playground.css   # 操场页面专用样式
│   ├── components/          # 可复用 UI 组件
│   │   ├── mod.rs
│   │   ├── footer.rs        # 页脚（版权、ICP 备案）
│   │   ├── layout.rs        # 主布局（导航栏 + 内容 + 页脚）
│   │   ├── navbar.rs        # 导航栏（路由、主题切换）
│   │   └── test_layout.rs   # 测试页面专用布局
│   ├── models/
│   │   └── mod.rs           # 数据模型：BlogPost, RuntimeBlogPost
│   ├── pages/               # 页面组件（对应路由）
│   │   ├── mod.rs
│   │   ├── about.rs         # 关于页面
│   │   ├── blog_post.rs     # 博客文章详情页
│   │   ├── dev.rs           # 开发工具页面
│   │   ├── home.rs          # 首页（博客列表 + 分页 + 系列文档合并）
│   │   ├── not_found.rs     # 404 页面
│   │   ├── playground.rs    # 操场（代码实验）
│   │   ├── tags.rs          # 书签页（搜索 + 筛选）
│   │   └── test.rs          # 测试页面
│   ├── routes/
│   │   └── mod.rs           # 路由定义（Dioxus Router）
│   ├── utils/
│   │   ├── mod.rs
│   │   ├── dark_mode.rs     # 暗色模式状态管理
│   │   └── title.rs         # 页面标题设置
│   ├── app.rs               # 应用根组件
│   ├── lib.rs               # 库入口（模块声明 + 公开 API）
│   ├── main.rs              # 程序入口
│   └── styles.scss          # 全局样式（Sass）
├── static/                  # 静态资源
│   ├── blog-images/         # 博客文章配图
│   ├── fonts/               # 字体文件
│   │   ├── IBMPlexSansSC-Medium.woff2
│   │   ├── JetBrainsMonoNLNerdFont-Regular.ttf
│   │   ├── MiSans-Regular.otf
│   │   ├── NeueMachina-Bold.woff2
│   │   └── SmileySans-Oblique.otf
│   ├── images/              # 通用图片
│   │   └── og-image.png     # 社交分享图（1200×630）
│   ├── robots.txt           # 爬虫协议（指向 sitemap.xml）
│   └── favicon.svg          # 网站图标
├── .gitignore
├── Cargo.toml               # Rust 依赖配置
├── Cargo.lock
├── LICENSE                  # MIT 许可证
├── README.md
├── Trunk.toml               # Trunk 构建配置（release + 关闭 SRI）
├── build.rs                 # 构建脚本（编译期解析文章）
├── index.html               # HTML 入口模板
└── netlify.toml             # Netlify 部署配置（SPA 路由）
```

## 路由一览

| 路由 | 页面 | 组件 | 布局 |
|------|------|------|------|
| `/` | 首页 | `Home` | `Layout` |
| `/about` | 关于 | `About` | `Layout` |
| `/tags` | 书签 | `Tags` | `Layout` |
| `/dev` | 开发 | `Dev` | `Layout` |
| `/post/:slug` | 博客详情 | `BlogPostView` | `Layout` |
| `/playground` | 操场 | `Playground` | `Layout` |
| `/test` | 测试 | `Test` | `TestLayout` |
| `/:..route` | 404 | `NotFound` | `Layout` |

## 核心依赖

```toml
[dependencies]
dioxus = { version = "0.6.3", features = ["web", "router"] }
wasm-bindgen = "0.2.89"
wasm-bindgen-futures = "0.4.39"
web-sys = "0.3.66"          # DOM / Storage / History / Canvas
js-sys = "0.3.66"           # Promise 等（异步延时等）
serde = "1.0"               # 序列化
serde_json = "1.0"          # 解析 posts.json / markov.json 等预生成数据
toml = "0.8.8"              # 书签配置解析
comrak = "0.20.0"           # Markdown → HTML (GFM)
gloo-net = "0.5.0"          # 浏览器端 HTTP 请求
futures-channel = "0.3.30"
wee_alloc = "0.4.5"         # Wasm 内存分配器
console_error_panic_hook = "0.1.7"

[target.'cfg(not(target_arch = "wasm32"))'.dependencies]
chrono = "0.4"              # 仅 CLI 工具（new / prerender）用

[build-dependencies]
comrak = "0.20.0"           # front matter 之外的正文渲染
regex = "1.11.0"
toml = "0.8.8"              # 解析 _config.toml 配置文件
```

## 部署说明

### 静态文件部署

```bash
cargo build --release --bin web-rs   # 1. 生成 static/*.json
trunk build --release                # 2. 编译 Wasm 并复制 static/ 进 dist/
cargo run --release --bin prerender  # 3. 生成文章静态页与 404.html
```

将 `dist/` 目录部署到任意静态 Web 服务器即可。

> ⚠️ 只跑 `trunk build` 会得到「没有预渲染页」的产物：SPA 能正常跑，但爬虫看到的是空壳。
> 完整发布必须补上第 1 步（生成数据）和第 3 步（预渲染）。

### GitHub Pages（Actions 自动部署）

推送代码到 `mod` 分支后，GitHub Actions 自动：

1. 安装 Rust wasm 工具链
2. `cargo build --release --target wasm32-unknown-unknown`（触发 `build.rs` 生成 `static/*.json`）
3. 运行 `trunk build --release`（编译 Wasm，并把 `static/` 复制进 `dist/`）
4. 编译并运行 `prerender` 工具生成文章静态页与 `404.html`
5. 将 `dist/` 部署到 `gh-pages` 分支

配置文件：[.github/workflows/build.yml](.github/workflows/build.yml)

### Netlify

项目包含 `netlify.toml`，配置了 SPA 路由重定向：

```toml
[[redirects]]
  from = "/*"
  to = "/index.html"
  status = 200
```

## 主题定制

主题通过 CSS 变量实现，在 `src/styles.scss` 中定义：

```scss
:root {
  --bg-color: #f7f7f7;
  --text-color: #222;
  --card-bg: #fff;
  --primary-color: #...;
  --accent-color: #...;
  --border-color: #...;
  /* ... */
}

.dark {
  --bg-color: #18181c;
  --text-color: #f7f7f7;
  --card-bg: #1a1a20;
  /* ... */
}
```

主题切换支持 [View Transition API](https://developer.chrome.com/docs/web-platform/view-transitions/)，提供圆形扩散动画效果。用户选择会持久化到 `localStorage`。

## 代码高亮主题

博客文章使用 highlight.js 渲染代码块，支持切换主题：

**浅色主题：** github、atom-one-light、vs、solarized-light、xcode

**深色主题：** atom-one-dark、vs2015、monokai、dracula、solarized-dark、night-owl、tokyo-night-dark、github-dark

**特色主题：** gradient-dark、gradient-light、rainbow、brown-paper

## Windows 开发环境补充

### 安装 wasm-opt

Trunk 的 `--release` 构建需要 wasm-opt 来优化 Wasm 产物。

**方法一：手动安装**

1. 创建目录 `C:\Users\{用户名}\.cache\trunk\bin`
2. 下载 wasm-opt：
   ```powershell
   $url = "https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-windows.tar.gz"
   Invoke-WebRequest -Uri $url -OutFile "$env:USERPROFILE\.cache\trunk\bin\binaryen.tar.gz"
   ```
3. 解压并将 `binaryen-version_123\bin\wasm-opt.exe` 移动到 `C:\Users\{用户名}\.cache\trunk\bin\`

**方法二：脚本自动化**

```powershell
# 创建目标目录
$targetDir = "$env:USERPROFILE\.cache\trunk\bin"
New-Item -ItemType Directory -Force -Path $targetDir

# 下载
$url = "https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-windows.tar.gz"
Invoke-WebRequest -Uri $url -OutFile "$targetDir\binaryen.tar.gz"

# 解压并移动
tar -xf "$targetDir\binaryen.tar.gz" -C $targetDir
Move-Item "$targetDir\binaryen-version_123\bin\wasm-opt.exe" $targetDir -Force

# 清理
Remove-Item "$targetDir\binaryen-version_123" -Recurse -Force
Remove-Item "$targetDir\binaryen.tar.gz" -Force

Write-Host "wasm-opt setup completed!"
```

> 开发模式 (`trunk serve`) 不需要 wasm-opt，只有 `trunk build --release` 才需要。

### 关闭 Release 优化

如果不需要 wasm-opt，可在 `Trunk.toml` 中设置：

```toml
[wasm]
opt = false
```

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/amazing-feature`)
3. 提交更改 (`git commit -m 'feat: add amazing feature'`)
4. 推送到分支 (`git push origin feature/amazing-feature`)
5. 创建 Pull Request

## 许可证

MIT License — 详见 [LICENSE](LICENSE)
