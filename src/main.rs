use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen_futures::spawn_local;

fn main() {
    // 初始化 panic hook
    console_error_panic_hook::set_once();
    
    // 启动应用
    launch(app);
}

#[component]
fn Navbar(is_dark: Signal<bool>) -> Element {
    const NAV_ITEMS: &[(&str, &str)] = &[
        ("/", "首页"),
        ("/about", "关于"),
        ("/tags", "书签"),
        ("/dev", "开发"),
        ("/blog", "博客"),
    ];

    let onclick = move |e: Event<MouseData>| {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            let html = document.document_element().unwrap();
            let coords = e.client_coordinates();
            let x = coords.x;
            let y = coords.y;
            let width = window.inner_width().unwrap().as_f64().unwrap();
            let height = window.inner_height().unwrap().as_f64().unwrap();
            let end_radius = ((x.max(width - x)).powi(2) + (y.max(height - y)).powi(2)).sqrt();
            html.set_attribute("style", &format!("--x: {}px; --y: {}px; --r: {}px", x, y, end_radius)).unwrap();
            let supports_transition = js_sys::eval("Boolean(document.startViewTransition)").unwrap().as_bool().unwrap_or(false);
            if supports_transition {
                let _ = js_sys::eval("document.startViewTransition(() => { document.documentElement.classList.toggle('dark'); })");
            } else {
                let class = html.class_name();
                if class.contains("dark") {
                    html.set_attribute("class", "").unwrap();
                } else {
                    html.set_attribute("class", "dark").unwrap();
                }
            }
        is_dark.set(!is_dark());
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if is_dark() { "dark" } else { "light" });
            }
        }
    };

    let window = web_sys::window().unwrap();
    let location = window.location();
    let current_path = location.pathname().unwrap_or_else(|_| "/".to_string());
    
    let is_active = |href: &str| {
        match href {
            "/" => current_path == "/",
            _ => current_path.starts_with(href)
        }
    };

    rsx! {
        div { class: "navbar-content",
            div { class: "navbar-ui",
                div { class: "navbar-title-wrapper",
                    h1 { class: "navbar-title", "干徒" }
                    div { class: "navbar-glow" }
                }
                div { class: "navbar-subtitle", "这很酷" }
                div { class: "navbar-links",
                    {NAV_ITEMS.iter().map(|(href, label)| {
                        let active = is_active(href);
                        rsx! {
                            a {
                                href: *href,
                                class: if active { "nav-link nav-active" } else { "nav-link" },
                                { label }
                            }
                        }
                    })}
                    button {
                        class: "theme-toggle",
                        onclick: onclick,
                        match is_dark() {
                            true => rsx! {
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 1024 1024",
                                    path {
                                        fill: "currentColor",
                                        d: "M512 704a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512m0-704a32 32 0 0 1 32 32v64a32 32 0 0 1-64 0V96a32 32 0 0 1 32-32m0 768a32 32 0 0 1 32 32v64a32 32 0 1 1-64 0v-64a32 32 0 0 1 32-32M195.2 195.2a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 1 1-45.248 45.248L195.2 240.448a32 32 0 0 1 0-45.248zm543.104 543.104a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 0 1-45.248 45.248l-45.248-45.248a32 32 0 0 1 0-45.248M64 512a32 32 0 0 1 32-32h64a32 32 0 0 1 0 64H96a32 32 0 0 1-32-32m768 0a32 32 0 0 1 32-32h64a32 32 0 1 1 0 64h-64a32 32 0 0 1-32-32M195.2 828.8a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248L240.448 828.8a32 32 0 0 1-45.248 0zm543.104-543.104a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248l-45.248 45.248a32 32 0 0 1-45.248 0"
                                    }
                                }
                            },
                            false => rsx! {
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 1024 1024",
                                    path {
                                        fill: "currentColor",
                                        d: "M512 704a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512m0-704a32 32 0 0 1 32 32v64a32 32 0 0 1-64 0V96a32 32 0 0 1 32-32m0 768a32 32 0 0 1 32 32v64a32 32 0 1 1-64 0v-64a32 32 0 0 1 32-32M195.2 195.2a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 1 1-45.248 45.248L195.2 240.448a32 32 0 0 1 0-45.248zm543.104 543.104a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 0 1-45.248 45.248l-45.248-45.248a32 32 0 0 1 0-45.248M64 512a32 32 0 0 1 32-32h64a32 32 0 0 1 0 64H96a32 32 0 0 1-32-32m768 0a32 32 0 0 1 32-32h64a32 32 0 1 1 0 64h-64a32 32 0 0 1-32-32M195.2 828.8a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248L240.448 828.8a32 32 0 0 1-45.248 0zm543.104-543.104a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248l-45.248 45.248a32 32 0 0 1-45.248 0"
                                    }
                                }
                            },
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn Footer() -> Element {
    rsx! {
        div { class: "footer-content",
            span { "2019-2025 " }
            span { style: "color: rgb(161, 98, 7);", "©" }
            span { " 干徒 / Ganto" }
        }
    }
}

#[component]
fn app() -> Element {
    let is_dark = use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = storage.get_item("theme") {
                            if theme == "dark" {
                                html.set_attribute("class", "dark").unwrap();
                                return true;
                            } else {
                                html.remove_attribute("class").unwrap();
                                return false;
                            }
                        }
                    }
                    let prefers_dark = js_sys::eval(
                        "window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches"
                    ).ok().and_then(|v| v.as_bool()).unwrap_or(false);
                    if prefers_dark {
                        html.set_attribute("class", "dark").unwrap();
                        return true;
                    }
                }
            }
        }
        false
    });

    rsx! {
        div { class: "app",
            Navbar { is_dark: is_dark }
            div { class: "main-content",
                Router::<Route> {}
            }
            Footer {}
        }
    }
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home,
    #[route("/about")]
    About,
    #[route("/tags")]
    Tags,
    #[route("/dev")]
    Dev,
    #[route("/blog")]
    Blog,
    #[route("/blog/:slug")]
    BlogPostView { slug: String },
}

#[derive(Clone, PartialEq)]
struct BlogPost {
    title: &'static str,
    date: &'static str,
    author: &'static str,
    tags: &'static [&'static str],
    content: &'static str,
    slug: &'static str,
}

// 运行时使用的BlogPost结构体
#[derive(Clone, PartialEq)]
struct RuntimeBlogPost {
    title: String,
    date: String,
    author: String,
    tags: Vec<String>,
    content: String,
    slug: String,
}

// 引入构建脚本生成的文章列表
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));

#[component]
fn Home() -> Element {
    let str_l_br = "{";
    let str_r_br = "}";
    rsx! {
        div { class: "main-container",
                    div { class: "code-cards",
                        div { class: "code-card",
                            div { class: "code-card-title", "TypeScript" }
                    pre { class: "code-block ts",
                        code {
                            span { class: "kw", "const" },
                            " str",
                            ": ",
                            span { class: "kw", "string" },
                            span { class: "ty", " = " },
                            span { class: "str", "\"Hello, TypeScript!\"" },
                            ";",
                            br {},
                            span { class: "fn", "console" },
                            ".",
                            span { class: "ty", "log" },
                            "(str);",
                        }
                    }
                        }
                        div { class: "code-card",
                            div { class: "code-card-title", "Golang" }
                    pre { class: "code-block go",
                        code {
                            span { class: "kw", "package" },
                            " main",
                            br {},
                            br {},
                            span { class: "kw", "import" },
                            " ",
                            span { class: "str", "\"fmt\"" },
                            br {},
                            br {},
                            span { class: "kw", "func" },
                            span { class: "fn", " main" },
                            "() ",
                            {str_l_br},
                            br {},
                            "    ",
                            span { class: "kw", "var" },
                            " str ",
                            span { class: "ty", "string" },
                            " = ",
                            span { class: "str", "\"Hello, Golang!\"" },
                            ";",
                            br {},
                            span { class: "fn", "    fmt" },
                            ".",
                            span { class: "fn", "Println" },
                            "(str)",
                            br {},
                            {str_r_br},
                        }
                    }
                        }
                        div { class: "code-card",
                            div { class: "code-card-title", "Rust" }
                    pre { class: "code-block rust",
                        code {
                            span { class: "kw", "fn" },
                            " ",
                            span { class: "fn", "main" },
                            "() ",
                            {str_l_br},
                            br {},
                            "    ",
                            span { class: "kw", "let" },
                            " str: String = String::",
                            span { class: "fn", "from" },
                            "(",
                            span { class: "str", "\"Hello, Rust!\"" },
                            ");",
                            br {},
                            span { class: "ty", "    println!" },
                            "(",
                            span { class: "str", "\"" },
                            span { class: "ty", {str_l_br}, {str_r_br} },
                            span { class: "str", "\"" },
                            ", str);",
                            br {},
                            {str_r_br},
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn About() -> Element {
    rsx! {
        div { class: "about-container",
            div { "你好！我是一名开发爱好者" },
            div { "常用技术栈：Rust、Golang、JavaScript、TypeScript、React、Vue" },
            div { class: "call-container",
                p { "你可以在此处找到我：" },
                p { class: "call-content",
                    a { href: "https://github.com/gantoho", "GitHub@gantoho" },
                    a { href: "https://gitee.com/ganto", "Gitee@ganto" },
                    a { href: "https://space.bilibili.com/1112912961", "哔哩哔哩@干徒" },
                    a { href: "https://cnblogs.com/ganto", "博客园@干徒" },
                    a { href: "mailto:i@ganto.me?cc=i@ganto.icu&amp;body=你好，干徒！我有一些想法要与你分享：", "i#ganto.me（#替换成@）" },
                }
            },
            div { "本站使用Rust语言开发，由Dioxus框架构建WEB服务，采用SCSS对样式进行管理，博客内容使用Markdown格式编写并由Dioxus渲染。" },
            div { "本站部署在Netlify。" },
        }
    }
}

#[component]
fn Tags() -> Element {
    let tags = vec![
        ("JS", "#ffe033", "#222"),
        ("Vue", "#4dbd8b", "#222"),
        ("React", "#5edfff", "#222"),
        ("Angular", "#f44336", "#fff"),
        ("Svelte", "#ff6f3d", "#fff"),
        ("Solid", "#6c93be", "#fff"),
        ("TS", "#3b82f6", "#fff"),
        ("Vite", "#8b8cf7", "#fff"),
        ("Nuxt", "#19e28c", "#222"),
        ("Next", "#1e90ff", "#fff"),
        ("Remix", "#4fc3f7", "#fff"),
        ("Go", "#6ad6f7", "#fff"),
        ("Rust", "#fff", "#222"),
        ("Java", "#4b7ca8", "#fff"),
    ];
    rsx! {
        div { class: "tags-container",
            div { class: "tags-grid",
                {tags.iter().map(|(name, bg, color)| {
                    let style_str = format!("background:{};color:{};", bg, color);
                    rsx! {
                        div {
                            class: "tag-block",
                            style: Box::leak(style_str.into_boxed_str()) as &str,
                            b { {name} }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
fn Dev() -> Element {
    let img_url = use_signal(|| None::<String>);

    // 图片加载逻辑，供初始化和按钮复用
    let load_img = {
        let img_url = img_url.clone();
        move || {
            let mut img_url = img_url.clone();
            spawn_local(async move {
                let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg")
                    .send()
                    .await;

                if let Ok(response) = resp {
                    if let Ok(filename) = response.text().await {
                        let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                        img_url.set(Some(url));
                    }
                }
            });
        }
    };

    // 按钮点击事件
    let fetch_random_img = {
        let load_img = load_img.clone();
        move |_evt: Event<MouseData>| {
            load_img();
        }
    };

    // 首次加载自动获取
    use_effect(move || {
        load_img();
        ()
    });

    rsx! {
        div { class: "dev-container",
            div { class: "identification",
                div { class: "content",
                    span { class: "hole"}
                    div { class: "header" }
                    div { class: "default",
                        div { class: "before", "Ganto" }
                        div { class: "middle", "." }
                        div { class: "after", "Me" }
                    }
                    div { class: "foot" }
                }
            }
            div { class: "dev-img-wrapper",
                if let Some(url) = img_url() {
                    img {
                        src: url,
                        class: "dev-img"
                    }
                }
                button {
                    class: "img-switch-btn",
                    onclick: fetch_random_img,
                    "Regain"
                }
            }
        }
    }
}

#[component]
fn Blog() -> Element {
    let posts = use_signal(Vec::new);

    // 加载博客文章列表
    use_effect(move || {
        let mut posts = posts.clone();
        spawn_local(async move {
            // 使用构建脚本生成的文章列表
            let loaded_posts = BLOG_POSTS.iter().map(|post| RuntimeBlogPost {
                title: post.title.to_string(),
                date: post.date.to_string(),
                author: post.author.to_string(),
                tags: post.tags.iter().map(|&s| s.to_string()).collect(),
                content: post.content.to_string(),
                slug: post.slug.to_string(),
            }).collect();
            posts.set(loaded_posts);
        });
    });

    rsx! {
        div { class: "blog-container",
            div { class: "blog-list",
                if posts().is_empty() {
                    div { class: "loading", "加载中..." }
                } else {
                    {posts().iter().map(|post| {
                        let post = post.clone();
                        rsx! {
                            div {
                                class: "blog-preview",
                                Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                    h2 { class: "preview-title", {post.title.clone()} }
                                    div { class: "preview-meta",
                                        span { class: "preview-date", {post.date.clone()} }
                                        span { class: "preview-author", {post.author.clone()} }
                                    }
                                    div { class: "preview-tags",
                                        {post.tags.iter().map(|tag| rsx! {
                                            span { class: "preview-tag", {tag.clone()} }
                                        })}
                                    }
                                }
                            }
                        }
                    })}
                }
            }
        }
    }
}

#[component]
fn BlogPostView(slug: String) -> Element {
    let posts = use_signal(Vec::new);
    let current_post = use_signal(|| None::<RuntimeBlogPost>);

    // 加载博客文章列表
    use_effect(move || {
        let mut posts = posts.clone();
        spawn_local(async move {
            // 使用构建脚本生成的文章列表
            let loaded_posts = BLOG_POSTS.iter().map(|post| RuntimeBlogPost {
                title: post.title.to_string(),
                date: post.date.to_string(),
                author: post.author.to_string(),
                tags: post.tags.iter().map(|&s| s.to_string()).collect(),
                content: post.content.to_string(),
                slug: post.slug.to_string(),
            }).collect();
            posts.set(loaded_posts);
        });
    });

    // 根据标题查找文章
    use_effect(move || {
        let slug = slug.clone();
        let mut current_post = current_post.clone();
        let posts = posts();
        
        if let Some(post) = posts.iter().find(|p| p.slug == slug) {
            current_post.set(Some(post.clone()));
        }
    });

    // 初始化代码高亮
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // 添加 highlight.js CSS
        let link = document.create_element("link").unwrap();
        link.set_attribute("rel", "stylesheet").unwrap();

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
        link.set_attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/stackoverflow-dark.min.css").unwrap();
        document.head().unwrap().append_child(&link).unwrap();
        
        // 添加 highlight.js 脚本
        let script = document.create_element("script").unwrap();
        script.set_attribute("src", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/highlight.min.js").unwrap();
        script.set_attribute("async", "false").unwrap();
        document.head().unwrap().append_child(&script).unwrap();
        
        // 等待主脚本加载完成后再加载语言模块
        let init_script = document.create_element("script").unwrap();
        let _ = init_script.set_text_content(Some(r#"
            function loadLanguages() {
                if (typeof hljs !== 'undefined') {
                    const languages = [
                        { name: "rust", file: "rust.min.js" },
                        { name: "javascript", file: "javascript.min.js" },
                        { name: "typescript", file: "typescript.min.js" },
                        { name: "python", file: "python.min.js" },
                        { name: "go", file: "go.min.js" },
                        { name: "java", file: "java.min.js" },
                        { name: "cpp", file: "cpp.min.js" },
                        { name: "csharp", file: "csharp.min.js" },
                        { name: "php", file: "php.min.js" },
                        { name: "ruby", file: "ruby.min.js" },
                        { name: "swift", file: "swift.min.js" },
                        { name: "kotlin", file: "kotlin.min.js" },
                        { name: "scala", file: "scala.min.js" },
                        { name: "bash", file: "bash.min.js" },
                        { name: "shell", file: "shell.min.js" },
                        { name: "sql", file: "sql.min.js" },
                        { name: "xml", file: "xml.min.js" },
                        { name: "yaml", file: "yaml.min.js" },
                        { name: "json", file: "json.min.js" },
                        { name: "markdown", file: "markdown.min.js" }
                    ];
                    languages.forEach(lang => {
                        const script = document.createElement('script');
                        script.src = `https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/${lang.file}`;
                        script.async = true;
                        document.head.appendChild(script);
                    });
                    hljs.highlightAll();
                } else {
                    setTimeout(loadLanguages, 100);
                }
            }
            loadLanguages();
        "#));
        let _ = document.body().unwrap().append_child(&init_script);
    });

    // 监听文章内容变化，重新应用代码高亮
    use_effect(move || {
        let current_post = current_post();
        if current_post.is_some() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();
            
            // 等待 DOM 更新后应用高亮
            let init_script = document.create_element("script").unwrap();
            let _ = init_script.set_text_content(Some(r#"
                function applyHighlight() {
                    if (typeof hljs !== 'undefined') {
                        // 为每个代码块添加语言标识
                        document.querySelectorAll('pre code').forEach((block) => {
                            // 从代码块的类名中获取语言
                            const language = block.className.split(' ').find(cls => cls.startsWith('language-'))?.replace('language-', '');
                            if (language) {
                                block.parentElement.setAttribute('data-lang', language);
                            } else {
                                // 如果没有指定语言，尝试从代码内容推断
                                const content = block.textContent || '';
                                if (content.includes('function') || content.includes('const') || content.includes('let')) {
                                    block.parentElement.setAttribute('data-lang', 'javascript');
                                } else if (content.includes('fn') || content.includes('let mut')) {
                                    block.parentElement.setAttribute('data-lang', 'rust');
                                } else if (content.includes('package') || content.includes('import')) {
                                    block.parentElement.setAttribute('data-lang', 'go');
                                } else if (content.includes('class') || content.includes('public')) {
                                    block.parentElement.setAttribute('data-lang', 'java');
                                } else if (content.includes('def') || content.includes('import')) {
                                    block.parentElement.setAttribute('data-lang', 'python');
                                }
                            }
                        });
                        hljs.highlightAll();
                    } else {
                        setTimeout(applyHighlight, 100);
                    }
                }

                // 使用 MutationObserver 监听 DOM 变化
                const observer = new MutationObserver((mutations) => {
                    mutations.forEach((mutation) => {
                        if (mutation.type === 'childList' || mutation.type === 'subtree') {
                            applyHighlight();
                        }
                    });
                });

                // 开始观察整个文档
                observer.observe(document.body, {
                    childList: true,
                    subtree: true
                });

                // 初始应用高亮
                applyHighlight();
            "#));
            let _ = document.body().unwrap().append_child(&init_script);
        }
    });

    rsx! {
        div { class: "blog-container",
            if let Some(post) = current_post() {
                div { class: "blog-post",
                    div { class: "blog-nav",
                        Link { to: Route::Blog {}, class: "back-button", "←" }
                        div { class: "function-buttons",
                            button { 
                                class: "function-button",
                                onclick: move |_| {
                                    let window = web_sys::window().unwrap();
                                    let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                                },
                                "↑"
                            }
                            button { 
                                class: "function-button",
                                onclick: move |_| {
                                    let document = web_sys::window().unwrap().document().unwrap();
                                    let blog_post = document.query_selector(".blog-post").unwrap().unwrap();
                                    let class_name = blog_post.get_attribute("class").unwrap_or_default();
                                    if class_name.contains("wide-mode") {
                                        blog_post.set_attribute("class", class_name.replace("wide-mode", "").trim()).unwrap();
                                    } else {
                                        blog_post.set_attribute("class", format!("{} wide-mode", class_name).trim()).unwrap();
                                    }
                                },
                                "↔"
                            }
                        }
                    }
                    div { class: "blog-title-wrapper",
                        div { class: "blog-title",
                            h2 { {post.title.clone()} }
                        }
                        div { class: "blog-meta",
                            span { class: "blog-date", {post.date.clone()} }
                            span { class: "blog-author", {post.author.clone()} }
                        }
                        div { class: "blog-tags",
                            {post.tags.iter().map(|tag| rsx! {
                                span { class: "blog-tag", {tag.clone()} }
                            })}
                        }
                    }
                    div { 
                        class: "blog-content",
                        dangerous_inner_html: {
                            let options = comrak::ComrakOptions {
                                extension: comrak::ComrakExtensionOptions {
                                    table: true,
                                    strikethrough: true,
                                    autolink: true,
                                    tagfilter: false,
                                    tasklist: true,
                                    superscript: true,
                                    header_ids: Some("".to_string()),
                                    footnotes: true,
                                    description_lists: true,
                                    front_matter_delimiter: None,
                                },
                                parse: comrak::ComrakParseOptions {
                                    smart: true,
                                    default_info_string: None,
                                    relaxed_tasklist_matching: false,
                                },
                                render: comrak::ComrakRenderOptions {
                                    hardbreaks: true,
                                    github_pre_lang: true,
                                    width: 0,
                                    list_style: comrak::ListStyleType::Dash,
                                    unsafe_: true,
                                    escape: false,
                                },
                            };
                            let html = comrak::markdown_to_html(&post.content, &options);
                            web_sys::console::log_1(&format!("Markdown content: {}", &post.content).into());
                            web_sys::console::log_1(&format!("Rendered HTML: {}", &html).into());
                            html
                        }
                    }
                }
            } else {
                div { class: "blog-list",
                    if posts().is_empty() {
                        div { class: "loading", "加载中..." }
                    } else {
                        {posts().iter().map(|post| {
                            let post = post.clone();
                            rsx! {
                                div {
                                    class: "blog-preview",
                                    Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                        h2 { class: "preview-title", {post.title.clone()} }
                                        div { class: "preview-meta",
                                            span { class: "preview-date", {post.date.clone()} }
                                            span { class: "preview-author", {post.author.clone()} }
                                        }
                                        div { class: "preview-tags",
                                            {post.tags.iter().map(|tag| rsx! {
                                                span { class: "preview-tag", {tag.clone()} }
                                            })}
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }
        }
    }
}