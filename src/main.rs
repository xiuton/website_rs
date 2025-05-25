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
}

#[derive(Clone, PartialEq)]
struct BlogPost {
    title: &'static str,
    date: &'static str,
    author: &'static str,
    tags: &'static [&'static str],
    content: &'static str,
}

// 运行时使用的BlogPost结构体
#[derive(Clone, PartialEq)]
struct RuntimeBlogPost {
    title: String,
    date: String,
    author: String,
    tags: Vec<String>,
    content: String,
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
    let mut current_post = use_signal(|| None::<RuntimeBlogPost>);

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
            }).collect();
            posts.set(loaded_posts);
        });
    });

    rsx! {
        div { class: "blog-container",
            if let Some(post) = current_post() {
                div { class: "blog-post",
                    div { class: "blog-nav",
                        button {
                            class: "back-button",
                            onclick: move |_| current_post.set(None),
                            "←"
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
                    div { class: "blog-content",
                        dangerous_inner_html: markdown::to_html(&post.content)
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
                                    onclick: move |_| {
                                        current_post.set(Some(post.clone()));
                                    },
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
                        })}
                    }
                }
            }
        }
    }
}