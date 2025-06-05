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

安装Rust

安装Trunk包 0.22.x
```sh
$ cargo install trunk --force
```

安装CMake

安装dioxus-cli

安装wasm-opt
先创建目录 `C:\Users\{UserName}\.cache\trunk\bin`
在该目录下执行命令
```sh
$ Invoke-WebRequest -Uri "https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-windows.tar.gz" -OutFile "binaryen-version_123-x86_64-windows.tar.gz"
```
解压文件
```sh
$ tar binaryen-version_123-x86_64-windows.tar.gz
```

将解压出的`binaryen-version_123\bin\wasm-opt.exe`文件移动到 `C:\Users\{UserName}\.cache\trunk\bin`目录


或者 直接以脚本的形式安装并配置 wasm-opt
```sh
$ powershell -ExecutionPolicy Bypass -File setup-wasm-opt.ps1
```
setup-wasm-opt.ps1
```ps1
# Create target directory
$targetDir = "$env:USERPROFILE\.cache\trunk\bin"
New-Item -ItemType Directory -Force -Path $targetDir

# Download wasm-opt
$url = "https://github.com/WebAssembly/binaryen/releases/download/version_123/binaryen-version_123-x86_64-windows.tar.gz"
$outputFile = "$targetDir\binaryen-version_123-x86_64-windows.tar.gz"

Write-Host "Downloading wasm-opt..."
Invoke-WebRequest -Uri $url -OutFile $outputFile

# Extract files
Write-Host "Extracting files..."
tar -xf $outputFile -C $targetDir

# Move wasm-opt.exe to correct location
$sourceFile = "$targetDir\binaryen-version_123\bin\wasm-opt.exe"
$destFile = "$targetDir\wasm-opt.exe"
Move-Item -Path $sourceFile -Destination $destFile -Force

# Clean up temporary files
Write-Host "Cleaning up temporary files..."
Remove-Item -Path "$targetDir\binaryen-version_123" -Recurse -Force
Remove-Item -Path $outputFile -Force

Write-Host "wasm-opt setup completed!" 
```



安装sass
```sh
npm install -g sass
```

清空缓存并打包运行
```bash
$ cargo clean ; trunk clean ; trunk build ; trunk serve
```

指定端口
```bash
$ trunk serve --port 8081
```

新建博客文章
```bash
$ cargo run --bin cp my-post
```

新建博客帮助
```bash
$ cargo run --bin cp
```


// 浅色主题：
// github.min.css - GitHub 风格
// atom-one-light.min.css - Atom 编辑器浅色主题
// vs.min.css - Visual Studio 风格
// solarized-light.min.css - Solarized 浅色主题
// xcode.min.css - Xcode 风格
// stackoverflow-light.min.css - Stack Overflow 浅色主题
// default.min.css - 默认浅色主题

// 深色主题：
// atom-one-dark.min.css - Atom 编辑器深色主题
// vs2015.min.css - Visual Studio 2015 风格
// monokai.min.css - Monokai 风格
// dracula.min.css - Dracula 主题
// solarized-dark.min.css - Solarized 深色主题
// night-owl.min.css - Night Owl 主题
// tokyo-night-dark.min.css - Tokyo Night 深色主题
// github-dark.min.css - GitHub 深色主题
// stackoverflow-dark.min.css - Stack Overflow 深色主题

// 其他特色主题：
// gradient-dark.min.css - 渐变深色主题
// gradient-light.min.css - 渐变浅色主题
// rainbow.min.css - 彩虹主题
// brown-paper.min.css - 牛皮纸风格
// docco.min.css - Docco 风格
// far.min.css - Far 主题
// kimbie.dark.min.css - Kimbie 深色主题
// kimbie.light.min.css - Kimbie 浅色主题
