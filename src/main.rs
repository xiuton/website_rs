use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use toml;

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
                                    fill: "none",
                                    view_box: "0 0 24 24",
                                    stroke_width: "1.5",
                                    stroke: "currentColor",
                                    class: "size-6", 
                                    path {
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        d: "M21.752 15.002A9.72 9.72 0 0 1 18 15.75c-5.385 0-9.75-4.365-9.75-9.75 0-1.33.266-2.597.748-3.752A9.753 9.753 0 0 0 3 11.25C3 16.635 7.365 21 12.75 21a9.753 9.753 0 0 0 9.002-5.998Z"
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
    #[route("/post/:slug")]
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
    let posts = use_signal(Vec::new);
    let current_post = use_signal(|| None::<RuntimeBlogPost>);
    let mut is_wide_mode = use_signal(|| false);

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

    // 初始化代码高亮
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // 添加 highlight.js CSS
        let link = document.create_element("link").unwrap();
        link.set_attribute("rel", "stylesheet").unwrap();
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
                        { name: "markdown", file: "markdown.min.js" },
                        { name: "html", file: "xml.min.js" }
                    ];
                    
                    // 创建一个 Promise 来跟踪所有语言模块的加载
                    const loadPromises = languages.map(lang => {
                        return new Promise((resolve) => {
                        const script = document.createElement('script');
                        script.src = `https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/${lang.file}`;
                        script.async = true;
                            script.onload = () => resolve();
                        document.head.appendChild(script);
                    });
                    });

                    // 等待所有语言模块加载完成后再应用高亮
                    Promise.all(loadPromises).then(() => {
                        // 确保代码块有正确的语言类名
                        document.querySelectorAll('pre code').forEach((block) => {
                            const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
                            if (languageClass) {
                                const language = languageClass.replace('language-', '');
                                block.parentElement.setAttribute('data-lang', language);
                            }
                        });
                        // 应用高亮
                    hljs.highlightAll();
                    });
                } else {
                    setTimeout(loadLanguages, 100);
                }
            }

            // 初始加载语言模块
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
                        // 确保代码块有正确的语言类名
                        document.querySelectorAll('pre code').forEach((block) => {
                            const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
                            if (languageClass) {
                                const language = languageClass.replace('language-', '');
                                block.parentElement.setAttribute('data-lang', language);
                            }
                        });
                        // 应用高亮
                        hljs.highlightAll();
                    } else {
                        setTimeout(applyHighlight, 100);
                    }
                }

                // 初始应用高亮
                applyHighlight();
            "#));
            let _ = document.body().unwrap().append_child(&init_script);
        }
    });

    // 在页面加载时，读取 localStorage 恢复宽屏状态
    use_effect(move || {
        if current_post().is_some() {
            if let Some(window) = web_sys::window() {
                if let Some(storage) = window.local_storage().ok().flatten() {
                    if let Ok(Some(wide_mode)) = storage.get_item("blog_wide_mode") {
                        is_wide_mode.set(wide_mode == "true");
                    }
                }
            }
        }
        ()
    });

    rsx! {
        div { class: "blog-container",
            div { class: "blog-list",
                if posts().is_empty() {
                    div { class: "loading", "加载中..." }
                } else {
                    div { class: "blog-posts",
                        {posts().iter().map(|post| {
                            let post = post.clone();
                            rsx! {
                                div { class: "blog-preview",
                                    Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                        h2 { class: "preview-title", {post.title.clone()} }
                                        div { class: "preview-meta",
                                            span { class: "preview-date", {post.date.clone()} }
                                            span { class: "preview-author", {post.author.clone()} }
                                        }
                                        div { class: "preview-tags",
                                            {post.tags.iter().map(|tag| {
                                                rsx! {
                                                    span { class: "preview-tag", 
                                                        svg {
                                                            xmlns: "http://www.w3.org/2000/svg",
                                                            fill: "none",
                                                            view_box: "0 0 24 24",
                                                            stroke_width: "1.5",
                                                            stroke: "currentColor",
                                                            class: "size-6",
                                                            path {
                                                                stroke_linecap: "round",
                                                                stroke_linejoin: "round",
                                                                d: "M5.25 8.25h15m-16.5 7.5h15m-1.8-13.5-3.9 19.5m-2.1-19.5-3.9 19.5"
                                                            }
                                                        }
                                                        {tag.clone()} 
                                                    }
                                                }
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

#[component]
fn About() -> Element {
    rsx! {
        div { class: "about-container",
            // 个人介绍部分
            section { class: "about-section intro-section",
                h2 { "👋 你好" }
            div { class: "tech-stack",
                    span { "🦀 Rust" }
                    span { "🐹 Go" }
                    span { "☕ Java" }
                    span { "💛 JavaScript" }
                    span { "💙 TypeScript" }
                    span { "⚛️ React" }
                    span { "💚 Vue" }
                    span { "🐢 Node.js" }
                    span { "🦕 Deno" }
                }
            }

            // 技能部分
            section { class: "about-section skills-section",
                h2 { "💪 技能特长" }
                div { class: "skills-grid",
                    div { class: "skill-card",
                        h3 { "前端开发" }
                        p { "现代前端框架、响应式设计、性能优化" }
                        div { class: "skill-tags",
                            span { "TypeScript" }
                            span { "React" }
                            span { "Vue" }
                            span { "Webpack" }
                        }
                    }
                    div { class: "skill-card",
                        h3 { "后端开发" }
                        p { "服务端开发、API设计、数据库优化" }
                        div { class: "skill-tags",
                            span { "Rust" }
                            span { "Go" }
                            span { "Node.js" }
                            span { "MySQL" }
                        }
                    }
                    div { class: "skill-card",
                        h3 { "DevOps" }
                        p { "自动化部署、容器化、CI/CD" }
                        div { class: "skill-tags",
                            span { "Docker" }
                            span { "Kubernetes" }
                            span { "GitHub Actions" }
                            span { "Jenkins" }
                        }
                    }
                }
            }

            // 联系方式
            section { class: "about-section contact-section",
                h2 { "📫 联系我" }
                div { class: "contact-links",
                    a { 
                        href: "mailto:i@ganto.me",
                        class: "contact-link email-link",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M20 4H4c-1.1 0-1.99.9-1.99 2L2 18c0 1.1.9 2 2 2h16c1.1 0 2-.9 2-2V6c0-1.1-.9-2-2-2zm0 4l-8 5-8-5V6l8 5 8-5v2z"
                            }
                        }
                        span { "i@ganto.me" }
                    }
                    a { 
                        href: "https://github.com/gantoho",
                            target: "_blank",
                        class: "contact-link github-link",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                            }
                        }
                        span { "GitHub" }
                    }
                    a { 
                        href: "https://cnblogs.com/ganto",
                            target: "_blank",
                        class: "contact-link blog-link",
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M12.75 19.5v-.75a7.5 7.5 0 0 0-7.5-7.5H4.5m0-6.75h.75c7.87 0 14.25 6.38 14.25 14.25v.75M6 18.75a.75.75 0 1 1-1.5 0 .75.75 0 0 1 1.5 0Z"
                            }
                        }
                        span { "博客园" }
                    }
                }
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize, PartialEq)]
struct Bookmark {
    title: String,
    url: String,
    description: String,
    icon: String,
}

struct BookmarkManager {
    bookmarks: Vec<Bookmark>,
}

impl BookmarkManager {
    fn new() -> Self {
        let config = include_str!("../bookmarks.toml");
        let bookmarks: toml::Value = toml::from_str(config).unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        
        let bookmarks = bookmarks["bookmark"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|item| {
                if let (Some(title), Some(url), Some(description), Some(icon)) = (
                    item.get("title").and_then(|v| v.as_str()),
                    item.get("url").and_then(|v| v.as_str()),
                    item.get("description").and_then(|v| v.as_str()),
                    item.get("icon").and_then(|v| v.as_str()),
                ) {
                    Some(Bookmark {
                        title: title.to_string(),
                        url: url.to_string(),
                        description: description.to_string(),
                        icon: icon.to_string(),
                    })
                } else {
                    None
                }
            })
            .collect();

        Self { bookmarks }
    }

    fn load_from_storage() -> Self {
        Self::new()
    }
}

#[component]
fn Tags() -> Element {
    let bookmark_manager = use_signal(BookmarkManager::load_from_storage);
    let mut search_text = use_signal(String::new);
    let mut search_query = use_signal(String::new);  // 实际的搜索关键词

    // 处理搜索
    let handle_search = move |_| {
        search_query.set(search_text());
    };

    // 处理回车键
    let handle_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            search_query.set(search_text());
        }
    };

    let filtered_bookmarks = use_memo(move || {
        let search = search_query().to_lowercase();
        let manager = bookmark_manager.read();
        
        manager.bookmarks.iter()
            .map(|b| {
                let matches = if !search.is_empty() {
                    b.title.to_lowercase().contains(&search) ||
                    b.description.to_lowercase().contains(&search)
                } else {
                    false
                };
                (b.clone(), matches)
            })
            .collect::<Vec<_>>()
    });

    rsx! {
        div { class: "bookmarks-container",
            // 搜索栏
            div { class: "search-bar",
                div { class: "search-input-wrapper",
                    input {
                        class: "search-input",
                        placeholder: "搜索书签...",
                        value: "{search_text}",
                        oninput: move |evt| search_text.set(evt.value().clone()),
                        onkeydown: handle_keydown
                    }
                    button {
                        class: "search-button",
                        onclick: handle_search,
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            fill: "none",
                            view_box: "0 0 24 24",
                            stroke_width: "2",
                            stroke: "currentColor",
                            path {
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                d: "m21 21-5.197-5.197m0 0A7.5 7.5 0 1 0 5.196 5.196a7.5 7.5 0 0 0 10.607 10.607Z"
                            }
                        }
                    }
                }
            }

            // 书签列表
            div { class: "bookmarks-list",
                {filtered_bookmarks.read().iter().map(|(bookmark, is_match)| {
                    rsx! {
                        div { 
                            class: if *is_match { "bookmark-item highlight" } else { "bookmark-item" },
                            a {
                                href: "{bookmark.url}",
                                target: "_blank",
                                class: "bookmark-link",
                                div { class: "bookmark-icon",
                                    {get_bookmark_icon(&bookmark.icon)}
                                }
                                div { class: "bookmark-info",
                                    h3 { class: "bookmark-title", "{bookmark.title}" }
                                    p { class: "bookmark-description", "{bookmark.description}" }
                                }
                            }
                        }
                    }
                })}
            }
        }
    }
}

fn get_bookmark_icon(icon_name: &str) -> Element {
    match icon_name {
        "github" => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.839 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                }
            }
        },
        // 如果是完整的URL（以 http:// 或 https:// 开头），则使用图片
        url if url.starts_with("http://") || url.starts_with("https://") => rsx! {
            img {
                src: "{url}",
                alt: "bookmark icon",
                class: "bookmark-icon-img",
                style: "object-fit: contain;"
            }
        },
        _ => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "none",
                stroke: "currentColor",
                stroke_width: "2",
                stroke_linecap: "round",
                stroke_linejoin: "round",
                path {
                    d: "M13.19 8.688a4.5 4.5 0 0 1 1.242 7.244l-4.5 4.5a4.5 4.5 0 0 1-6.364-6.364l1.757-1.757m13.35-.622 1.757-1.757a4.5 4.5 0 0 0-6.364-6.364l-4.5 4.5a4.5 4.5 0 0 0 1.242 7.244"
                }
            }
        }
    }
}

#[component]
fn Dev() -> Element {
    let img_url = use_signal(|| None::<String>);
    let mut is_background_mode = use_signal(|| false);
    let background_images = use_signal(Vec::new);
    let current_bg_index = use_signal(|| 0);
    let bg_timer_handle = use_signal(|| None::<i32>);
    let mut show_exit_button = use_signal(|| false);
    let mut hide_btn_timer = use_signal(|| None::<i32>);
    let mut hide_cursor = use_signal(|| false);
    let default_bg_image = "https://files.ganto.cn/files/default-bg.jpg";  // 添加默认背景图片URL

    // 小图片区域：点击换一张
    let fetch_random_img = {
        let img_url = img_url.clone();
        move |_evt: Event<MouseData>| {
            let mut img_url = img_url.clone();
            spawn_local(async move {
                let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                if let Ok(response) = resp {
                    if let Ok(filename) = response.text().await {
                        let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                        img_url.set(Some(url));
                    }
                }
            });
        }
    };

    // 进入背景墙时预加载多张图片
    let mut enter_background_mode = {
        let mut is_background_mode = is_background_mode.clone();
        let background_images = background_images.clone();
        let current_bg_index = current_bg_index.clone();
        let bg_timer_handle = bg_timer_handle.clone();
        move || {
            is_background_mode.set(true);
            let mut background_images = background_images.clone();
            let current_bg_index = current_bg_index.clone();
            let mut bg_timer_handle = bg_timer_handle.clone();
            
            // 初始化背景图片列表，添加默认图片
            let mut initial_images = Vec::new();
            initial_images.push(default_bg_image.to_string());
            background_images.set(initial_images);

            // 创建轮播定时器的函数
            let create_carousel_timer = move || {
                let background_images = background_images.clone();
                let mut current_bg_index = current_bg_index.clone();
                let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                    let len = background_images().len();
                    if len > 0 {
                        current_bg_index.set((current_bg_index() + 1) % len);
                    }
                }) as Box<dyn FnMut()>);
                let handle = web_sys::window().unwrap().set_interval_with_callback_and_timeout_and_arguments_0(
                    closure.as_ref().unchecked_ref(),
                    15000
                ).unwrap();
                closure.forget();
                handle
            };
            
            // 如果已经有加载的图片（不只是默认图片），直接启动轮播
            if background_images().len() > 1 {
                let handle = create_carousel_timer();
                *bg_timer_handle.write() = Some(handle);
                return;
            }

            // 如果没有加载的图片（除默认外），加载新图片
            spawn_local(async move {
                let mut loaded_count = 0;
                while loaded_count < 5 {
                    let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                    if let Ok(response) = resp {
                        if let Ok(filename) = response.text().await {
                            let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                            let mut background_images = background_images.clone();
                            
                            // 尝试加载图片
                            let load_image = |url: String| -> std::pin::Pin<Box<dyn std::future::Future<Output = bool>>> {
                                Box::pin(async move {
                            let img = web_sys::HtmlImageElement::new().unwrap();
                                    let (tx, rx) = futures::channel::oneshot::channel();
                                    
                                    let tx_success = std::sync::Arc::new(std::sync::Mutex::new(Some(tx)));
                                    let tx_error = tx_success.clone();
                                    
                                    let success_callback = wasm_bindgen::closure::Closure::wrap(
                                        Box::new(move || {
                                            if let Some(tx) = tx_success.lock().unwrap().take() {
                                                let _ = tx.send(true);
                                            }
                                        }) as Box<dyn FnMut()>
                                    );
                                    
                                    let error_callback = wasm_bindgen::closure::Closure::wrap(
                                        Box::new(move || {
                                            if let Some(tx) = tx_error.lock().unwrap().take() {
                                                let _ = tx.send(false);
                                            }
                                        }) as Box<dyn FnMut()>
                                    );
                                    
                                    img.set_onload(Some(success_callback.as_ref().unchecked_ref()));
                                    img.set_onerror(Some(error_callback.as_ref().unchecked_ref()));
                                    img.set_src(&url);
                                    
                                    success_callback.forget();
                                    error_callback.forget();
                                    
                                    rx.await.unwrap_or(false)
                                })
                            };
                            
                            // 尝试加载两次
                            let mut success = false;
                            for _ in 0..2 {
                                if load_image(url.clone()).await {
                                    success = true;
                                let mut imgs = background_images();
                                    if !imgs.contains(&url) {
                                        // 如果这是第一张成功加载的图片（除了默认图片），移除默认图片
                                        if imgs.len() == 1 && imgs[0] == default_bg_image {
                                            imgs.clear();
                                        }
                                        imgs.push(url.clone());
                                background_images.set(imgs);
                                        loaded_count += 1;

                                        // 如果这是第一张成功加载的图片，启动轮播
                                        if loaded_count == 1 {
                                            // 确保没有其他定时器在运行
                                            if let Some(old_handle) = *bg_timer_handle.read() {
                                                web_sys::window().unwrap().clear_interval_with_handle(old_handle);
                                            }
                                            let handle = create_carousel_timer();
                                            *bg_timer_handle.write() = Some(handle);
                                        }
                                    }
                                    break;
                                }
                            }
                            
                            if !success {
                                continue;
                            }
                        }
                    }
                }
            });
        }
    };

    // 退出背景墙时停止轮播
    let mut exit_background_mode = {
        let mut bg_timer_handle = bg_timer_handle.clone();
        move || {
            is_background_mode.set(false);
            let handle_opt = bg_timer_handle.read().clone();
            if let Some(handle) = handle_opt {
                web_sys::window().unwrap().clear_interval_with_handle(handle);
                *bg_timer_handle.write() = None;
            }
            // 不再清空图片列表
        }
    };

    // 首次加载小图片
    use_effect(move || {
        if img_url().is_none() {
            let mut img_url = img_url.clone();
            spawn_local(async move {
                let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                if let Ok(response) = resp {
                    if let Ok(filename) = response.text().await {
                        let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                        img_url.set(Some(url));
                    }
                }
            });
        }
        ()
    });

    // 预加载背景墙图片
    use_effect(move || {
        for url in background_images().iter() {
            let img = web_sys::HtmlImageElement::new().unwrap();
            img.set_src(url);
        }
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
                div {
                    class: "dev-btns",
                button {
                    class: "img-switch-btn",
                    style: "backdrop-filter: blur(8px); background: rgba(255, 255, 255, 0.2); border: 1px solid rgba(255, 255, 255, 0.3); padding: 8px; border-radius: 8px; transition: all 0.3s ease;",
                    onclick: fetch_random_img,
                    svg {
                        xmlns: "http://www.w3.org/2000/svg",
                        view_box: "0 0 24 24",
                        width: "24",
                        height: "24",
                        fill: "none",
                        stroke: "currentColor",
                        stroke_width: "2",
                        stroke_linecap: "round",
                        stroke_linejoin: "round",
                        path {
                            d: "M23 4v6h-6M1 20v-6h6M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15"
                        }
                    }
                    }
                    button {
                        class: "background-mode-btn",
                    style: "backdrop-filter: blur(8px); background: rgba(255, 255, 255, 0.2); border: 1px solid rgba(255, 255, 255, 0.3); padding: 8px; border-radius: 8px; margin-left: 8px; transition: all 0.3s ease;",
                        onclick: move |_| {
                            enter_background_mode();
                        },
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            width: "24",
                            height: "24",
                            fill: "none",
                            stroke: "currentColor",
                            stroke_width: "2",
                            stroke_linecap: "round",
                            stroke_linejoin: "round",
                            path {
                            d: "m15.75 10.5 4.72-4.72a.75.75 0 0 1 1.28.53v11.38a.75.75 0 0 1-1.28.53l-4.72-4.72M4.5 18.75h9a2.25 2.25 0 0 0 2.25-2.25v-9a2.25 2.25 0 0 0-2.25-2.25h-9A2.25 2.25 0 0 0 2.25 7.5v9a2.25 2.25 0 0 0 2.25 2.25Z"
                            }
                        }
                    }
                }
            }
            // 最上层的背景墙元素
            if is_background_mode() {
                div { 
                    class: if hide_cursor() { "background-wall hide-cursor" } else { "background-wall" },
                    onmousemove: move |_| {
                        show_exit_button.set(true);
                        hide_cursor.set(false);
                        // 清除旧的定时器
                        if let Some(handle) = hide_btn_timer() {
                            web_sys::window().unwrap().clear_timeout_with_handle(handle);
                        }
                        // 新建定时器
                        let mut show_exit_button = show_exit_button.clone();
                        let mut hide_btn_timer = hide_btn_timer.clone();
                        let mut hide_cursor = hide_cursor.clone();
                        let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                            show_exit_button.set(false);
                            hide_cursor.set(true);
                            hide_btn_timer.set(None);
                        }) as Box<dyn FnMut()>);
                        let handle = web_sys::window().unwrap()
                            .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 3000)
                            .unwrap();
                        hide_btn_timer.set(Some(handle));
                        closure.forget();
                    },
                    {background_images().iter().enumerate().map(|(index, url)| {
                        rsx! {
                        img {
                                key: "{url}",
                            src: url.clone(),
                                class: format_args!("background-wall-img {}", if index == current_bg_index() { "active" } else { "" })
                        }
                    }
                    })}
                    div { 
                        class: "exit-button-container",
                        style: "position: fixed; top: 20px; right: 20px; z-index: 1000;",
                        onmouseenter: move |_| {
                            show_exit_button.set(true);
                            if let Some(handle) = hide_btn_timer() {
                                web_sys::window().unwrap().clear_timeout_with_handle(handle);
                                hide_btn_timer.set(None);
                            }
                        },
                        onmouseleave: move |_| {
                            let mut show_exit_button = show_exit_button.clone();
                            let mut hide_btn_timer = hide_btn_timer.clone();
                            let closure = wasm_bindgen::closure::Closure::wrap(Box::new(move || {
                                show_exit_button.set(false);
                                hide_btn_timer.set(None);
                            }) as Box<dyn FnMut()>);
                            let handle = web_sys::window().unwrap()
                                .set_timeout_with_callback_and_timeout_and_arguments_0(closure.as_ref().unchecked_ref(), 1000)
                                .unwrap();
                            hide_btn_timer.set(Some(handle));
                            closure.forget();
                        },
                        button {
                            class: "exit-background-btn",
                            onclick: move |_| {
                                exit_background_mode();
                            },
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path {
                                    d: "M6 6l12 12M6 18L18 6"
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

#[component]
fn BlogPostView(slug: String) -> Element {
    let posts = use_signal(Vec::new);
    let current_post = use_signal(|| None::<RuntimeBlogPost>);
    let mut is_wide_mode = use_signal(|| false);

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
                        { name: "markdown", file: "markdown.min.js" },
                        { name: "html", file: "xml.min.js" }
                    ];
                    
                    // 创建一个 Promise 来跟踪所有语言模块的加载
                    const loadPromises = languages.map(lang => {
                        return new Promise((resolve) => {
                        const script = document.createElement('script');
                        script.src = `https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/languages/${lang.file}`;
                        script.async = true;
                            script.onload = () => resolve();
                        document.head.appendChild(script);
                    });
                    });

                    // 等待所有语言模块加载完成后再应用高亮
                    Promise.all(loadPromises).then(() => {
                        // 确保代码块有正确的语言类名
                        document.querySelectorAll('pre code').forEach((block) => {
                            const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
                            if (languageClass) {
                                const language = languageClass.replace('language-', '');
                                block.parentElement.setAttribute('data-lang', language);
                            }
                        });
                        // 应用高亮
                    hljs.highlightAll();
                    });
                } else {
                    setTimeout(loadLanguages, 100);
                }
            }

            // 初始加载语言模块
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
                        // 确保代码块有正确的语言类名
                        document.querySelectorAll('pre code').forEach((block) => {
                            const languageClass = block.className.split(' ').find(cls => cls.startsWith('language-'));
                            if (languageClass) {
                                const language = languageClass.replace('language-', '');
                                block.parentElement.setAttribute('data-lang', language);
                            }
                        });
                        // 应用高亮
                        hljs.highlightAll();
                    } else {
                        setTimeout(applyHighlight, 100);
                    }
                }

                // 初始应用高亮
                applyHighlight();
            "#));
            let _ = document.body().unwrap().append_child(&init_script);
        }
    });

    // 在页面加载时，读取 localStorage 恢复宽屏状态
    use_effect(move || {
        if current_post().is_some() {
            if let Some(window) = web_sys::window() {
                if let Some(storage) = window.local_storage().ok().flatten() {
                    if let Ok(Some(wide_mode)) = storage.get_item("blog_wide_mode") {
                        is_wide_mode.set(wide_mode == "true");
                    }
                }
            }
        }
        ()
    });

    rsx! {
        div { class: "blog-container",
            if let Some(post) = current_post() {
                div { 
                    class: if is_wide_mode() { "blog-post wide-mode" } else { "blog-post" },
                    div { class: "blog-nav",
                        Link { to: Route::Home {}, class: "back-button",
                            svg {
                                xmlns: "http://www.w3.org/2000/svg",
                                view_box: "0 0 24 24",
                                width: "24",
                                height: "24",
                                fill: "none",
                                stroke: "currentColor",
                                stroke_width: "2",
                                stroke_linecap: "round",
                                stroke_linejoin: "round",
                                path {
                                    d: "M15.75 19.5 8.25 12l7.5-7.5"
                                }
                            }
                        }
                        div { class: "function-buttons",
                            button { 
                                class: "function-button",
                                onclick: move |_| {
                                    let window = web_sys::window().unwrap();
                                    let _ = window.scroll_to_with_x_and_y(0.0, 0.0);
                                },
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 24 24",
                                    width: "24",
                                    height: "24",
                                    fill: "none",
                                    stroke: "currentColor",
                                    stroke_width: "2",
                                    stroke_linecap: "round",
                                    stroke_linejoin: "round",
                                    path {
                                        d: "m4.5 15.75 7.5-7.5 7.5 7.5"
                                    }
                                }
                            }
                            button { 
                                class: "function-button",
                                onclick: move |_| {
                                    let new_mode = !is_wide_mode();
                                    is_wide_mode.set(new_mode);
                                    // 存储宽屏状态
                                    if let Some(window) = web_sys::window() {
                                        if let Some(storage) = window.local_storage().ok().flatten() {
                                            let _ = storage.set_item("blog_wide_mode", if new_mode { "true" } else { "false" });
                                        }
                                    }
                                },
                                {
                                    if is_wide_mode() {
                                        rsx! {
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 24 24",
                                                width: "24",
                                                height: "24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                path {
                                                    d: "M8 3h8m-8 18h8M4 12h16M3 9l3 3-3 3m18-6l-3 3 3 3"
                                                }
                                            }
                                        }
                                    } else {
                                        rsx! {
                                            svg {
                                                xmlns: "http://www.w3.org/2000/svg",
                                                view_box: "0 0 24 24",
                                                width: "24",
                                                height: "24",
                                                fill: "none",
                                                stroke: "currentColor",
                                                stroke_width: "2",
                                                stroke_linecap: "round",
                                                stroke_linejoin: "round",
                                                path {
                                                    d: "M8 3h8m-8 18h8M4 12h16M4 12l3-3m-3 3l3 3m13-3l-3-3m3 3l-3 3"
                                                }
                                            }
                                        }
                                    }
                                }
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
                                span { class: "blog-tag",
                                    svg {
                                        xmlns: "http://www.w3.org/2000/svg",
                                        fill: "none",
                                        view_box: "0 0 24 24",
                                        stroke_width: "1.5",
                                        stroke: "currentColor",
                                        class: "size-6",
                                        path {
                                            stroke_linecap: "round",
                                            stroke_linejoin: "round",
                                            d: "M5.25 8.25h15m-16.5 7.5h15m-1.8-13.5-3.9 19.5m-2.1-19.5-3.9 19.5"
                                        }
                                    }
                                    {tag.clone()}
                                }
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
                            // 确保代码块的语言标识被正确保留
                            let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
                            let html = html.replace("<pre><code class=\"", "<pre><code class=\"language-");
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
                                                span { class: "preview-tag", 
                                                    svg {
                                                        xmlns: "http://www.w3.org/2000/svg",
                                                        fill: "none",
                                                        view_box: "0 0 24 24",
                                                        stroke_width: "1.5",
                                                        stroke: "currentColor",
                                                        class: "size-6",
                                                        path {
                                                            stroke_linecap: "round",
                                                            stroke_linejoin: "round",
                                                            d: "M5.25 8.25h15m-16.5 7.5h15m-1.8-13.5-3.9 19.5m-2.1-19.5-3.9 19.5"
                                                        }
                                                    }
                                                    {tag.clone()} 
                                                }
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