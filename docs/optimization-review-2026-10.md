# 优化复审（2026-10）—— 从"代码整洁"转向"首屏体积与数据流"

> 前五轮优化（见 [optimization-analysis.md](optimization-analysis.md)）已经把静态质量问题基本清零：
> clippy 0 warning、死代码清理、SVG/图标复用、CSS 变量精简、CI action 更新……
>
> 本轮复审换一个角度：不看"代码写得漂不漂亮"，而是看**用户实际下载了什么、构建是否可靠**。
> 所有数字均为本机实测（`dist/` 现有产物 + gzip -9），不是估算。

---

## 0. 结论速览

1. 一个**纯文字博客**，首访要传 **7–10 MB**，其中字体占 8 MB、wasm 1.3 MB。
2. 文章页会**无条件**拉取 **4.6 MB 的 `markov.json`**（gzip 后 1.44 MB），只为页面底部一个"AI 续写"小玩意。
3. wasm 里**原样嵌入了全部 93 篇文章正文（1.29 MB）**，同时 `posts.json` 里又存了三份正文（6 MB）。
4. release 构建里 **wasm-opt 被显式关闭**（`data-wasm-opt="0"`），本机甚至没有安装 wasm-opt。
5. 构建需要**手工三步且顺序敏感**，CI 里还在 `cargo clean` 之后才用缓存——两件事互相抵消。
6. 顺手发现一个**潜在 panic**（按字节切中文 summary）和一处**已被"修复"过又复发的重复 `@font-face`**——说明当前缺少防回归手段。

优先级判断：**P0 全在"少传 5–9 MB"上，而不是继续重构代码。** 同样的时间投在字体和 markov 上，收益是重构的几十倍。

---

## 1. 实测基线

### 1.1 资源体积（gzip -9）

| 资源 | 原始 | gzip | 何时加载 | 评价 |
|---|---:|---:|---|---|
| `web-rs-*_bg.wasm` | 3,706 KB | **1,298 KB** | 每个页面 | 未过 wasm-opt；含全部正文 |
| `styles-*.css` | 133 KB | 17 KB | 每个页面 | ✅ 正常 |
| `web-rs-*.js` | 69 KB | 11 KB | 每个页面 | ✅ 正常 |
| `MiSans-Regular.otf` | 6,348 KB | **5,443 KB** | **每个页面正文** | ❌ OTF，gzip 几乎无效 |
| `JetBrainsMonoNLNerdFont-Regular.ttf` | 2,152 KB | 1,177 KB | 代码块页面 | ❌ 全量 Nerd Font |
| `SmileySans-Oblique.otf` | 1,957 KB | 1,601 KB | 标题 | ❌ 全量字形 |
| `markov.json` | 4,628 KB | **1,435 KB** | **每篇文章页，挂载即拉** | ❌ 只为"AI 续写"按钮 |
| `posts.json` | 5,986 KB | 1,259 KB | 仅 `prerender` 构建期 | ❌ 却被部署上线 |
| `semantic.json` | 452 KB | 127 KB | 搜索页 | ⚠️ 可延迟到首次输入 |
| `search-index.json` | 227 KB | 70 KB | 搜索页 + 文章页 | ⚠️ 文章页每次重新拉+解析 |
| `knowledge-graph.json` | 107 KB | 7 KB | 图谱页 + 文章页 | ✅ 可接受 |
| `summaries.json` | 87 KB | 33 KB | AI 摘要页 | ✅ |
| `rake-keywords.json` | 59 KB | 25 KB | 搜索页 | ⚠️ |
| `embeddings.json` | 37 KB | 5 KB | 文章页 | ✅ |
| `lda-topics.json` | 11 KB | 3 KB | 文章页 | ✅ |
| `feed.xml` / `atom.xml` | 423 KB ×2 | — | 订阅 | ⚠️ 两份全文重复 |

### 1.2 各页面首访传输预算（首次访问、无缓存）

| 页面 | 主要开销 | 合计（gzip） |
|---|---|---:|
| 首页 | wasm + js + css + 正文字体 MiSans | ≈ **6.8 MB** |
| 文章页 | 上一行 + markov 1,435 KB + search-index 70 KB（代码块还会触发 JetBrains Mono 1,177 KB） | ≈ **8.4–9.6 MB** |
| 搜索页 | wasm + 字体 + 4 个索引（≈ 230 KB） | ≈ **7.0 MB** |

> 字体按实际命中计算：正文 MiSans（5.4 MB）每页必下；SmileySans 只被 `/dev` 页使用；JetBrains Mono 仅在页面存在代码块时下载。

作为参照：一篇 40 KB 的 Markdown 文章，正文本身只占首访流量的 **0.5%**。

### 1.3 `posts.json` 内部构成（5,986 KB）

| 字段 | 大小 | 占比 | 谁在用 |
|---|---:|---:|---|
| `content_mini` | 2,685 KB | 44.9% | 仓库内无任何运行时消费方（疑似小程序产物） |
| `content_html` | 1,781 KB | 29.8% | 仅 `prerender` |
| `content`（原始 md） | 1,289 KB | 21.5% | 仅 `prerender`（wasm 里另有一份） |
| 元数据 | 230 KB | 3.8% | 搜索/相关推荐真正需要的部分 |

---

## 2. P0：用户可直接感知的问题

### P0-1 文章页无条件拉 4.6 MB `markov.json`

**证据**

- [blog_post.rs:428](../src/pages/blog_post.rs#L428) 无条件渲染 `<MarkovContinuation>`
- [blog_post.rs:754-771](../src/pages/blog_post.rs#L754-L771) 在 `use_effect` 里**挂载即** `load_markov()`
- [markov.rs:26](../src/utils/markov.rs#L26) 一次性下载整包（内含 93 篇文章的 trigram 转移表）

**为什么严重**：读者可能永远不会点"生成续写"，但每次打开文章都要为它付出 1.44 MB gzip + 解析开销（JSON → `HashMap<String, HashMap<String,u32>>`，在主线程上）。

**改法（三选一，可叠加）**

1. **点击时加载**（最小改动，收益 100%）：

   ```rust
   // 按钮 onclick 里再 spawn(load_markov(...))，而不是 use_effect
   let handle_generate = move |_| {
       if generator.peek().is_some() { /* 直接生成 */ return; }
       // 首次点击才下载 + 解析，同时置 loading 态
   };
   ```

2. **按 slug 拆文件**：构建期输出 `static/markov/{slug}.json`，平均 50 KB/篇（≈15 KB gzip），只取当前文章。
3. **剪枝**：只保留 `count > 1` 的转移、每个 key 最多保留 top-N 个 next token，通常可再降 50–70%。

预期：文章页 **-1.4 MB**，且省掉一次大 JSON 解析。

---

### P0-2 字体 10.4 MB，且重复的 `@font-face` 让 `font-display: swap` 失效

**证据** [\_fonts.scss:1-32](../src/styles/_fonts.scss#L1-L32)：三套字体被定义了**两遍**（第 2–16 行带 `format()`/`font-display: swap`，第 18–32 行不带）。同 family、同 style、同 weight 的多条 `@font-face`，**后定义的会覆盖先定义的**，于是：

- `MiSans` 实际生效的是第 25 行那条 —— **没有 `font-display: swap`** → 首访出现 FOIT（文字空白等待 6.3 MB OTF）；
- 正文 `font-family` 首选就是 MiSans（[\_base.scss:14](../src/styles/_base.scss#L14)），等于每个页面都在等 5.4 MB 的字体。

顺带说明：这一项在 [optimization-analysis.md](optimization-analysis.md) 的 P1-03 里被标记为"已完成"，现在又复发了 —— 说明"改完没有回归校验"。

**改法**

1. **删掉 18–32 行的重复块**（保留一组带 `format()` + `font-display: swap` 的定义即可）。
2. **正文不要用自定义中文字体**。中文正文用系统字体栈（`-apple-system, "PingFang SC", "Microsoft YaHei", sans-serif`），视觉差异极小、收益最大。自定义字体只留给标题/强调。
3. 真要保留，**子集化 + woff2**：

   ```bash
   # 用全站实际出现过的字符（从 posts/ 提取）做子集
   pyftsubset MiSans-Regular.otf --text-file=used-chars.txt \
       --flavor=woff2 --layout-features='*' --output-file=MiSans-subset.woff2
   # JetBrains Mono 同理，只保留 ASCII + 用到的 Nerd 图标
   ```

   再配合 `unicode-range` 分片（中文按常用/次常用切 2–3 片），首屏只需下载命中片。
4. `<link rel="preload" as="font" type="font/woff2" crossorigin>` 只 preload **首屏必用**的那一个子集。

预期：首访字体从 **8.2 MB gzip → 200–600 KB**，LCP 从"等字体"变成"等 wasm"。

---

### P0-3 wasm 1.3 MB gzip：正文整包嵌入 + wasm-opt 被关闭 + `wee_alloc`

**证据**

- [build.rs:70-95](../build.rs#L70-L95)：把 93 篇文章的 `content` 全文写进 `BLOG_POSTS`（约 1.29 MB 原始字符串）。
- [index.html:89](../index.html#L89)：`<link data-trunk rel="rust" data-bin="web-rs" data-wasm-opt="0" />` —— **优化级别 0**，等于不做 wasm-opt（本机也确实没装 wasm-opt）。
- [lib.rs:1-4](../src/lib.rs#L1-L4)：`wee_alloc`。该 crate 早已停止维护，在 Rust 1.72+ 的默认分配器下通常**更大也更慢**，官方 README 自己也不再推荐。
- `comrak` 被编进 wasm 用于运行时渲染 Markdown（[markdown.rs](../src/utils/markdown.rs)），而构建期其实已经渲染过一遍 `content_html`。

**改法**

1. **把正文从初始包移到按需加载**：wasm 只保留元数据（title/date/tags/slug/summary/series/order），正文用 `/static/post/{slug}.html`（`prerender` 已经生成）或 `{slug}.json`。文章页只需再拉 20–60 KB。
2. **打开 wasm-opt**：`data-wasm-opt="z"`（与 `opt-level="z"` 对齐），CI 里 Trunk 会自动拉 binaryen。
3. **删掉 `wee_alloc`**，用默认分配器。
4. （可选）正文外置后，运行时就不需要 comrak 了 → 从依赖里移除，wasm 再降一截。

**粗估**：正文 1.29 MB 占 wasm 的 ~35%，加上 wasm-opt 通常 8–15% 的缩减和分配器替换，初始 wasm 有希望从 **1.30 MB gzip → 0.6–0.8 MB**。建议改完用 `twiggy top -n 30` 看真实构成再决定是否继续砍 comrak。

---

### P0-4 `posts.json` 三份正文，只服务构建期，却被部署上线

**证据** [build_posts_json.rs:208-210](../build/build_posts_json.rs#L208-L210) 同时输出 `content` / `content_html` / `content_mini`；文件落在 `static/`，被 Trunk 整目录复制进 `dist/` 并发布。

**影响**：不影响首屏（SPA 不请求它），但会让每次部署多传 6 MB、`gh-pages` 分支膨胀、`prerender` 每次解析 6 MB。`content_mini` 在本仓库内**没有任何消费方**。

**改法**

- 拆成三个产物：`posts-meta.json`（230 KB，给搜索/相关推荐）、`post-html/{slug}.html`（给 prerender + SPA 按需）、`posts-mini.json`（如果小程序还在用，单独产出、单独发布）。
- `prerender` 改为直接读元数据 + 逐篇 HTML，不必整包反序列化。
- 把纯构建期产物放到 `dist/_build/` 之外（或构建后删除），不要进发布目录。

---

### P0-5 预渲染页被脚本清空，出现"闪一下再重绘"

**证据** [prerender.rs:259-260](../src/bin/prerender.rs#L259-L260)：wasm 挂载前执行 `document.getElementById('main').innerHTML=''`。

用户看到的是：静态正文出现 → 瞬间空白 → wasm 重新渲染 → 再出现。SSR/prerender 的收益（首屏可见、SEO）被这一下抵消了一半，观感上也不如不预渲染。

**改法（从小到大）**

1. 最小：不要清空，而是把预渲染内容包一层，wasm 就绪后再替换（或先 `visibility: hidden` 再切换）。
2. 正确：用 Dioxus 的 hydration（`dioxus-web` 的 hydrate 模式）复用已有 DOM，避免重复渲染。
3. 或者干脆接受"SPA + 骨架屏"，把 prerender 只当 SEO 产物（校验 UA/爬虫再决定是否清空）。

---

## 3. P1：构建与 CI 的可靠性

### P1-1 三步构建、顺序敏感、`dist/` 被整个重建

现状（[README.md](README.md)）：`cargo build` → `trunk build` → `cargo run --bin prerender`，且 `trunk build` 会清空 `dist/`，所以第 3 步必须重跑。本地、CI 各维护一遍顺序，漏一步就得到"没有预渲染页"或"缺静态数据"的产物。

**改法**：用 Trunk 的 hook 把第 3 步挂到构建流程里，只剩一条命令：

```toml
# Trunk.toml
[[hooks]]
stage = "post_build"
command = "cargo"
command_arguments = ["run", "--release", "--bin", "prerender"]
```

（`prerender` 里已有 `dist/` 存在性检查，Trunk 构建完再跑正好。）这样 `trunk build --release` 与 `trunk serve` 的开发体验都统一，README 也能砍掉一大段"顺序不能颠倒"的说明。

### P1-2 `build.rs` 直接往源码树 `static/` 写文件

- 生成物落在源码目录（虽然 `.gitignore` 覆盖了），会污染工作区、跨分支残留、并让"构建"变成"改仓库"。
- 更隐蔽的是 [build.rs:48](../build.rs#L48) 只声明了 `rerun-if-changed=posts`。**改了 `build/build_common.rs` 等模块不会触发重跑**，容易出现"代码改了、JSON 没变"的假象。

**改法**：生成到 `OUT_DIR`（或 `target/generated`），由 `post_build` hook 复制进 `dist/static/`；同时补 `println!("cargo:rerun-if-changed=build");` 和 `build.rs` 自身。

### P1-3 CI 有一半步骤在互相抵消

[\_github/workflows/build.yml](../.github/workflows/build.yml)：

| 问题 | 说明 |
|---|---|
| `cargo clean` + 缓存 | 每次清掉 `target/`，缓存只剩 registry，wasm 全量重编译 |
| 缺 `target/` 缓存 | 应缓存 `target/` 并用 `Swatinem/rust-cache@v2` |
| 没有 `cargo fmt --check` / `clippy -D warnings` / `test` | 质量门禁缺失，前几轮修的 clippy 问题没有防线 |
| Trunk 用 `latest` 下载 | 不可复现；`thedodd/trunk` 已迁移到 `trunk-rs/trunk`，建议固定版本 |
| 先 `cargo build --target wasm32` 再 `trunk build` | Trunk 自己会编译，属于重复调用（虽有缓存，日志也乱） |
| 未加 `--no-default-features` | `default = ["dev-pages"]`，生产包会带上 Playground/Test |
| Dev、CircleGenerator 未 feature gate | `routes/mod.rs` 里 `/dev`、`/circle-generator` 始终编入（`dev.rs` 还带外部图片 API 轮播逻辑） |
| 部署体积 | 把 `posts.json`、`markov.json`、10 MB 字体一起发布 |

**建议的 CI 骨架**

```yaml
- uses: Swatinem/rust-cache@v2
- run: cargo fmt --all -- --check
- run: cargo clippy --all-targets -- -D warnings
- run: cargo test
- run: cargo build --release --target wasm32-unknown-unknown --no-default-features
- run: trunk build --release          # post_build hook 自动 prerender
- uses: peaceiris/actions-gh-pages@v4
```

---

## 4. P2：架构与可维护性

### P2-1 同一件事有两套实现（Markdown 渲染）

构建期用 comrak 产出 `content_html`（[build_posts_json.rs:6-19](../build/build_posts_json.rs#L6-L19)），运行期又用 comrak 把 `post.content` 再渲染一遍（[markdown.rs](../src/utils/markdown.rs)）。两边的 options 并不完全一致，迟早出现"预渲染页和 SPA 页排版不同"。

**建议**：渲染只保留一处。要么正文外置后直接用构建期的 HTML，要么彻底改成运行期渲染并删掉 `content_html`。二者取其一，别同时维护。

### P2-2 `prerender` 里手写了一份导航栏/页脚

[prerender.rs:180-234](../src/bin/prerender.rs#L180-L234) 硬编码了 100 多行 `NAVBAR_HTML`/`FOOTER_HTML`，与 [navbar.rs](../src/components/navbar.rs)、[footer.rs](../src/components/footer.rs) 是两份真相。改导航要改两处，且容易漏。

**建议**：把"静态壳"抽成模板（`include_str!` + 占位符），或把导航/页脚数据抽成结构体，Dioxus 与 prerender 共用。至少加一条测试：比较 prerender 输出的 nav 链接与路由表是否一致。

### P2-3 复杂度热点

| 文件 | 体量 | 建议 |
|---|---:|---|
| [blog_post.rs](../src/pages/blog_post.rs) | 910 行 | 拆出 `markov.rs` / `related.rs` / `series_nav.rs` 三个组件模块 |
| [circle_generator.rs](../src/pages/circle_generator.rs) | 71 KB | 拆 UI / 渲染 / 配置持久化；它整体不进生产包更好 |
| [build_common.rs](../build/build_common.rs) | 42 KB | 拆 scan / front-matter / config / slug 四个模块 |
| [prerender.rs](../src/bin/prerender.rs) | 663 行 | 页面模板与数据读取分离 |

### P2-4 数据层没有缓存

[blog_post.rs:101-122](../src/pages/blog_post.rs#L101-L122)：每次进入文章页都重新 `GET /static/search-index.json`（227 KB）再 `SearchEngine::load` 解析；`knowledge_graph`、`lda`、`embeddings` 同理。同一个会话里翻 5 篇文章 = 下载并解析 5 次。

**建议**：用 `LazyLock<OnceCell<...>>` / 模块级缓存（`tags.rs` 的 `BOOKMARKS` 已经是这个模式），把"加载一次"变成全局一次。

### P2-5 潜在 panic：按字节切中文 summary

[blog_post.rs:157](../src/pages/blog_post.rs#L157)：

```rust
post.summary[..post.summary.len().min(160)].to_string()
```

`summary` 是 UTF-8。当前 93 篇里恰好只有 1 篇 ≥160 字节且 160 落在字符边界，所以**今天不崩**；但只要新写一篇中文 summary 超过 160 字节，`160 % 3 != 0`，这里必定 panic（`release` 下 `panic = "abort"`，整个 wasm 直接挂）。

```rust
post.summary.chars().take(160).collect::<String>()
```

`prerender.rs` 里用的就是 `chars().take(160)`，说明这是遗漏而非风格选择。

### P2-6 小程序 HTML 用正则"消毒"

[build_posts_json.rs:37-163](../build/build_posts_json.rs#L37-L163) 用 20 多条正则做标签替换/剥离（含 `<script>`、`<iframe>`、`<svg>`、`<a>`、`<img>`、`<table>`……）。正则处理 HTML 天然脆弱：嵌套、属性里的 `>`、大小写、自闭合写法都可能漏。若 `content_mini` 确实还在用，建议换成 HTML 解析器重写标签，或至少加一组快照测试。

### P2-7 静态 JSON 没有内容指纹

`/static/search-index.json`、`markov.json` 等文件名不带 hash，而 hashed 的 wasm/CSS 才有。部署后浏览器（或 CDN）可能继续用旧索引，表现为"新文章搜不到"。给请求加 `?v=<build_id>`，或对 `/static/*.json` 设置较短的 `Cache-Control`。

### P2-8 SEO 覆盖不全

`prerender` 只生成 `/post/*` 和 `/series/*`。首页、`/tags`、`/search`、`/knowledge-graph` 仍是空壳 HTML，爬虫/社交预览拿不到内容。至少把首页列表（最近 N 篇）预渲染出来。

### P2-9 依赖升级（长期，不必现在做）

`dioxus 0.6.3` → 0.7 是较大的迁移（router 合并进主 crate、渲染 API 变化）；`comrak 0.20` 已落后多个版本；`gloo-net 0.5` → 0.6。建议单独开分支做，不要和性能优化混在一个 PR。

---

## 5. 建议路线图

按"投入产出比"排序，前两批加起来大约半天到一天，能去掉 80% 的无效传输。

| 批次 | 动作 | 预期收益 | 成本 |
|---|---|---|---|
| **第一批（立刻）** | markov 改点击加载；删 `_fonts.scss` 重复块；正文改系统字体栈 | 文章页 **-1.4 MB**，首访 **-5 MB** 量级 | 1–2 h |
| **第二批（本周）** | 字体子集化 woff2 + preload；`data-wasm-opt="z"`；删 `wee_alloc` | 再 **-2~3 MB**，wasm **-10%** | 半天 |
| **第三批（本月）** | 正文外置 / 元数据与内容分离；`posts.json` 拆三份；Trunk post_build hook；修 `rerun-if-changed` | 初始 wasm 有望到 0.6–0.8 MB gz | 1–2 天 |
| **第四批（迭代）** | CI 质量门禁 + 缓存；数据层一次性缓存；修 `summary[..160]`；首页预渲染 | 防回归、翻页更快 | 1–2 天 |
| **第五批（可选）** | hydration 替代 CLEAR_SCRIPT；prerender 与组件共享模板；Dioxus 0.7 迁移 | 观感与长期维护 | 按需 |

**验收方式**：给 CI 加一条"体积预算"——`wasm.gz ≤ 800 KB`、首屏字体 ≤ 600 KB、单页 JSON 总量 ≤ 400 KB，超出即失败。没有预算的数字优化，过几个月一定回弹（P1-03 字体重复定义复发就是例子）。

---

## 6. 附录：复现测量

```powershell
# 1) gzip 后体积
Add-Type -AssemblyName System.IO.Compression
Get-ChildItem dist -Recurse -File -Include *.wasm,*.js,*.css |
  ForEach-Object {
    $b=[IO.File]::ReadAllBytes($_.FullName)
    $ms=[IO.MemoryStream]::new()
    $gz=[IO.Compression.GZipStream]::new($ms,[IO.Compression.CompressionMode]::Compress,$true)
    $gz.Write($b,0,$b.Length); $gz.Dispose()
    "{0,-40} {1,8:N0} KB -> {2,7:N0} KB gz" -f $_.Name,($b.Length/1KB),($ms.ToArray().Length/1KB)
  }

# 2) wasm 内部构成（需先 cargo install twiggy）
twiggy top -n 30 dist/*.wasm

# 3) 字体子集化（需 pip install fonttools brotli）
pyftsubset static/fonts/MiSans-Regular.otf --text-file=used-chars.txt --flavor=woff2 --output-file=MiSans-subset.woff2

# 4) 一键看 posts.json 各字段占比
$j = Get-Content static/posts.json -Raw -Encoding UTF8 | ConvertFrom-Json
$j | ForEach-Object { [pscustomobject]@{slug=$_.slug; md=$_.content.Length; html=$_.content_html.Length; mini=$_.content_mini.Length} }
```

---

# 7. 实施记录（2026-10-18）

本报告的 P0/P1/P2 项已按顺序落地，以下是实施后的实测结果与剩余项说明。

## 7.1 体积对比（同一台机器、`trunk build --release`）

| 指标 | 优化前 | 优化后 | 变化 |
|---|---:|---:|---:|
| wasm 原始 | 3,706 KB | 1,216 KB | **-67%** |
| wasm gzip | 1,298 KB | 480 KB | **-63%** |
| 字体资源合计 | 10,407 KB | 598 KB | **-94%** |
| `posts.json` | 5,986 KB | 0（拆为 55 KB meta + 1.8 MB 按需 HTML） | 部署 -6 MB |
| 文章页首访（gzip，含字体） | ≈ 8.4–9.6 MB | ≈ 0.9 MB | **-89%** |
| 首页首访（gzip，含字体） | ≈ 6.8 MB | ≈ 0.76 MB | **-89%** |
| `dist/` 总大小 | 27.8 MB | 16.7 MB | -40% |
| 文章页挂载时下载的 markov | 1,435 KB（每次） | 0（点击才加载） | -100% |
| `markov.json` 体积 | 4,628 KB | 4,323 KB | -7% |

## 7.2 各项落地情况

| 编号 | 状态 | 落地方式 |
|---|---|---|
| P0-1 markov 懒加载 | ✅ | 组件挂载不再请求，点击「生成续写」才加载并缓存生成器 |
| P0-2 字体 | ✅ | 删除重复 `@font-face`；`cargo run -p font-subset` 按站内用字裁剪为 woff2（纯 Rust） |
| P0-3 wasm 瘦身 | ✅ | 正文外置到 `static/post-html/`；移除运行时 comrak / `wee_alloc`；启用 `wasm-opt -Oz`（binaryen 133） |
| P0-4 posts.json 拆分 | ✅ | `posts-meta.json` + `post-html/`；旧 6 MB 文件默认不产出，`WEBSITE_LEGACY_POSTS_JSON=1` 可恢复 |
| P0-5 预渲染闪跳 | ✅ | 正文同时写入 `<template id="prerender-content">`，SPA 首屏同步复用；不再先清空再拉取 |
| P1-1 构建收敛 | ✅ | Trunk `post_build` hook 调用 prerender，写入 `dist/.stage`；一条 `trunk build --release` 完成 |
| P1-2 build.rs | ✅ | 补 `rerun-if-changed=build`；旧产物自动清理 |
| P1-3 CI | ✅ | 固定 Trunk 0.21.14；`Swatinem/rust-cache`；fmt/clippy/test 门禁；`--no-default-features` |
| P2-1 渲染统一 | ✅ | 渲染只在构建期发生一次，预渲染页与 SPA 使用同一份 HTML |
| P2-2 导航单一来源 | ✅ | 新增 `src/nav.rs`，Dioxus 导航栏与 prerender 共用链接数据（页脚仍为静态 HTML） |
| P2-3 文件拆分 | ✅（部分） | `blog_post` 拆为 `mod.rs` / `markov.rs` / `related.rs` / `series_nav.rs`；`build_common`、`circle_generator` 未拆 |
| P2-4 数据层缓存 | ✅ | 新增 `utils/cache.rs`，静态 JSON 会话级缓存 + `BUILD_ID` 版本号 |
| P2-5 潜在 panic | ✅ | `summary[..160]` 改为 `chars().take(160)` |
| P2-6 小程序 HTML 消毒 | ✅ | 新增 `tools/mini-html`（lol_html 解析器）替换 20+ 条正则；含 5 个单元测试 |
| P2-7 静态 JSON 指纹 | ✅ | `cache::versioned()` 统一追加 `?v=BUILD_ID` |
| P2-8 首页预渲染 | ✅ | prerender 向 `dist/index.html` 注入 `<noscript>` 最新文章列表 + ItemList JSON-LD |
| P2-9 依赖大版本升级 | ✅（部分） | Dioxus 0.6.3 → 0.7.10 已完成（见 7.5）；comrak / gloo-net 仍未升级 |

## 7.3 新增/变更的构建约定

- 生产构建：`trunk build --release --no-default-features`（一条命令，自动预渲染）。
- 新增文章后如需更新字体子集：`cargo run -p font-subset`（Rust，不依赖 Python）。
- 站外仍需要旧 `posts.json`：构建前设置 `WEBSITE_LEGACY_POSTS_JSON=1`。
- 本地若曾用旧产物构建过，先 `trunk clean`，否则 Trunk 可能复用旧的 `dist/` 快照。

## 7.4 第二轮补充（2026-10-18）：去 Python 化与剩余项

按「不引入 Python、能用 Rust 就用 Rust」的要求，补齐了剩余项：

| 事项 | 实现 |
|---|---|
| 字体子集化 | 新增 `tools/font-subset`：`allsorts` 裁剪字形 + `woofwoof` 编码 woff2；删除 `scripts/subset_fonts.py` |
| 小程序 HTML 转换 | 新增 `tools/mini-html`：`lol_html` 流式解析 + CSS 选择器改写，替换全部正则拼接 |
| markov 体积 | 构建期剪枝：每个 key 最多 5 个后继，存在高频转移时丢弃只出现一次的噪声（4,628 → 4,323 KB） |
| 首页 SEO | prerender 注入 `<noscript>` 文章列表与 `ItemList` JSON-LD |
| dev server 循环重建 | Trunk 配置 `[watch] ignore = ["dist"]`，避免 post_build hook 写入 dist 触发无限重建 |

字体产物（Rust 工具输出，与之前 pyftsubset 的结果同量级）：

| 字体 | 原始 | 子集 woff2 |
|---|---:|---:|
| MiSans | 6,347 KB | 258 KB |
| JetBrains Mono | 2,151 KB | 13 KB |
| SmileySans | 1,956 KB | 327 KB |

> 说明：CFF/OTF 字体的子集化试过 `fontcull`、`oxifont-subset`，它们只重排字形编号、CFF 表本身并未缩小（输出仍有 4–5 MB）；最终选用 `allsorts`，它能把 MiSans 的 SFNT 从 6.3 MB 裁到 296 KB。
>
> `tools/` 是独立 workspace 成员，只在需要时手动运行，不参与站点 wasm 构建；CI 测试使用 `--exclude font-subset` 避免额外编译。

## 7.5 Dioxus 0.7 迁移（2026-10-18）

依赖从 `dioxus 0.6.3` 升级到 `0.7.10`，路由仍由 `router` feature 提供。

| 变化 | 处理方式 |
|---|---|
| `dioxus_router::prelude` 不再存在 | 删除全部该导入，统一使用 `dioxus::prelude::*`（0.7 的 prelude 已包含 `Link` / `Outlet` / `use_route` / `use_navigator` / `Routable`） |
| Hook 必须在组件顶层无条件调用 | 原 `blog_post` 在 `Option::map` 闭包里调用 `use_signal` 实现章节折叠，抽成独立组件 [SeriesNav](../src/pages/blog_post/series_nav.rs) |
| 渲染期写信号 | `PreviewTags` 原先在 render 中比较 slug 并 `set` 信号；改为调用处加 `key: "{post.slug}"`，让组件按文章重建 |
| `IntoDynNode` 不再接受 `&&str` | 迭代渲染统一解引用：`{ *label }`、`{ *tag }`、`{ *last_date }` |
| 版本与产物 | wasm 1,216 KB → **1,146 KB**（gzip 480 KB → **463 KB**） |

验证：

- `cargo fmt --check`、`cargo clippy`（wasm + host）、`cargo test --workspace --exclude font-subset` 全部通过
- `trunk build --release --no-default-features` 成功，93 篇文章页 + 2 个系列页 + 404 正常生成
- 用 headless Chrome 实际访问 `/`、`/post/20201222/`、`/search`、`/tags`、`/series/rust-llm`、`/circle-generator`、`/about`、404，SPA 均正常挂载，控制台无错误（`/tags` 只有一条外部 favicon 404）
