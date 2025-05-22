# Ganto's Website

这是一个使用 Dioxus 和 Rust 构建的个人网站项目。

## 功能特点

- 🌓 支持亮色/暗色主题切换
- 📱 响应式设计，适配移动端
- ⚡ 基于 Rust 和 WebAssembly 的高性能实现
- 🎨 现代化的 UI 设计
- 🔄 平滑的主题切换动画

## 环境要求

- Rust 1.70.0 或更高版本
- Node.js 16.0.0 或更高版本
- Trunk (Rust WebAssembly 构建工具)

## 安装步骤

1. 安装 Rust：
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. 安装 Trunk：
```bash
cargo install trunk
```

3. 克隆项目：
```bash
git clone https://github.com/xiuton/website_rs.git
cd website_rs
```

## 开发指南

1. 启动开发服务器：
```bash
trunk serve
```
这将启动一个开发服务器，通常在 http://localhost:8080 访问。

2. 构建项目：
```bash
trunk build
```
构建后的文件将生成在 `dist` 目录中。

## 项目结构

```
.
├── src/
│   ├── main.rs          # 主程序入口
│   └── styles.css       # 样式文件
├── index.html           # HTML 模板
├── Cargo.toml           # Rust 依赖配置
└── README.md           # 项目说明文档
```

## 部署说明

### 静态文件部署

1. 构建项目：
```bash
trunk build
```

2. 将 `dist` 目录下的所有文件部署到你的 Web 服务器。

### 使用 Docker 部署

1. 构建 Docker 镜像：
```bash
docker build -t ganto-website .
```

2. 运行容器：
```bash
docker run -p 8080:80 ganto-website
```

## 主题定制

项目使用 CSS 变量实现主题切换，可以在 `src/styles.css` 中修改以下变量来自定义主题：

```css
:root {
  --bg-color: #f7f7f7;
  --text-color: #222;
  --card-bg: #fff;
  /* 更多变量... */
}

.dark {
  --bg-color: #18181c;
  --text-color: #f7f7f7;
  --card-bg: #1a1a20;
  /* 更多变量... */
}
```

## 贡献指南

1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 创建 Pull Request

## 许可证

本项目采用 MIT 许可证 - 查看 [LICENSE](LICENSE) 文件了解详情

sass
```sh
npm install -g sass
```

清空缓存并打包运行
```bash
$ cargo clean ; trunk clean ; trunk build ; trunk serve
```

新建博客文章
```bash
$ cargo run --bin cp my-post
```

新建博客帮助
```bash
$ cargo run --bin cp
```

