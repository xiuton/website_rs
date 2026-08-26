# 干徒 - Ganto's Website

基于 **Rust + Dioxus + WebAssembly** 构建的个人网站，提供博客、书签管理、代码实验等功能。

## 功能特点

- 🌓 **亮色/暗色主题切换** — 支持 View Transition API 平滑过渡，记忆用户偏好
- 📱 **响应式设计** — 适配桌面端与移动端，工具栏自动调整位置
- ⚡ **WebAssembly 驱动** — 基于 Rust 编译为 Wasm，接近原生的运行性能
- 📝 **Markdown 博客** — Front Matter 元数据，comrak 渲染，highlight.js 代码高亮
- 🏷️ **标签系统** — 文章标签展示
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
| Markdown 解析 | pulldown-cmark (首页预览) |
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

### 构建生产版本

```bash
trunk build --release
```

构建产物输出到 `dist/` 目录。

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

> `build.rs` 会在编译时扫描 `posts/` 目录，解析所有 `.md` 文件的 Front Matter，
> 生成包含文章元数据和内容的 Rust 数组，嵌入到最终 Wasm 产物中。

## 项目结构

```
.
├── .github/
│   └── workflows/
│       └── build.yml        # GitHub Actions CI/CD（含 SEO 预渲染步骤）
├── build/                   # 构建期数据生成模块（build.rs 调用）
│   ├── build_common.rs      # 共享工具
│   ├── build_posts_json.rs  # 生成 posts.json
│   ├── build_rss.rs         # RSS 订阅
│   ├── build_atom.rs        # Atom 订阅
│   ├── build_search.rs      # 搜索索引
│   ├── build_summaries.rs   # AI 摘要分类
│   ├── build_seo.rs         # sitemap.xml 生成
│   └── ...                  # 其余 build_*.rs 模块
├── data/                    # 应用数据
│   └── bookmarks.toml       # 书签配置
├── posts/                   # 博客 Markdown 文章
│   ├── Rust所有权.md
│   ├── Dioxus基于Rust的多平台开发框架.md
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
│   │   ├── home.rs          # 首页（博客列表 + 分页）
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
├── Trunk.toml               # Trunk 构建配置（Wasm 优化）
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
dioxus-web = "0.6.3"
dioxus-hooks = "0.6.2"
dioxus-router = "0.6.3"
wasm-bindgen = "0.2.89"
web-sys = "0.3.66"        # DOM / Storage / History API
comrak = "0.20.0"          # Markdown → HTML (GFM)
pulldown-cmark = "0.13.0"  # 首页文章预览
toml = "0.8.8"            # 书签配置解析
serde = "1.0"             # 序列化
chrono = "0.4"            # 日期时间
gloo-timers = "0.3.0"     # 异步定时器

[build-dependencies]
walkdir = "2.4.0"         # 构建时遍历 posts 目录
comrak = "0.20.0"
```

## 部署说明

### 静态文件部署

```bash
trunk build --release
```

将 `dist/` 目录部署到任意静态 Web 服务器即可。

### GitHub Pages（Actions 自动部署）

推送代码到 `mod` 分支后，GitHub Actions 自动：

1. 安装 Rust wasm 工具链
2. 运行 `trunk build --release`
3. 编译并运行 `prerender` 工具生成文章静态页与 `404.html`
4. 将 `dist/` 部署到 `gh-pages` 分支

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
