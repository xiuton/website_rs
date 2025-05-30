use dioxus::prelude::*;
use dioxus_router::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;

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
    let posts = use_signal(|| Vec::new());

    // 加载最新的3篇博客文章
    use_effect(move || {
        let mut posts = posts.clone();
        spawn_local(async move {
            let loaded_posts = BLOG_POSTS.iter()
                .take(3)
                .map(|post| RuntimeBlogPost {
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
        div { class: "main-container",
            // 个人简介卡片
            div { class: "profile-card",
                div { class: "profile-content",
                    p { "目前主要使用Rust、Golang、JavaScript/TypeScript等技术栈。" }
                }
                div { class: "profile-stats",
                    div { class: "stat-item",
                        span { class: "stat-value", "5+" }
                        span { class: "stat-label", "年开发经验" }
                    }
                    div { class: "stat-item",
                        span { class: "stat-value", "20+" }
                        span { class: "stat-label", "开源项目" }
                    }
                    div { class: "stat-item",
                        span { class: "stat-value", "100+" }
                        span { class: "stat-label", "技术文章" }
                        }
                    }
                        }

            // 最新博客文章
            div { class: "latest-posts",
                h2 { "最新文章" }
                div { class: "posts-grid",
                    if posts.is_empty() {
                        div { class: "loading", "加载中..." }
                    } else {
                        {posts.iter().map(|post| {
                            let post = post.clone();
                            rsx! {
                                div { class: "post-card",
                                    Link { to: Route::BlogPostView { slug: post.slug.clone() },
                                        h3 { class: "post-title", {post.title.clone()} }
                                        div { class: "post-meta",
                                            span { class: "post-date", {post.date.clone()} }
                                        }
                                        div { class: "post-tags",
                                            {post.tags.iter().map(|tag| rsx! {
                                                span { class: "post-tag", {tag.clone()} }
                                            })}
                                        }
                                    }
                                }
                            }
                        })}
                    }
                }
            }

            // 技术栈展示
            div { class: "tech-stack",
                h2 { "技术栈" }
                div { class: "tech-grid",
                    div { class: "tech-category",
                        h3 { "前端" }
                        div { class: "tech-items",
                            span { class: "tech-item", "React" }
                            span { class: "tech-item", "Vue" }
                            span { class: "tech-item", "TypeScript" }
                            span { class: "tech-item", "Next.js" }
                        }
                    }
                    div { class: "tech-category",
                        h3 { "后端" }
                        div { class: "tech-items",
                            span { class: "tech-item", "Rust" }
                            span { class: "tech-item", "Golang" }
                            span { class: "tech-item", "Node.js" }
                            span { class: "tech-item", "PostgreSQL" }
                    }
                        }
                    div { class: "tech-category",
                        h3 { "工具" }
                        div { class: "tech-items",
                            span { class: "tech-item", "Git" }
                            span { class: "tech-item", "Docker" }
                            span { class: "tech-item", "Linux" }
                            span { class: "tech-item", "VSCode" }
                        }
                    }
                }
            }

            // 项目展示
            div { class: "featured-projects",
                h2 { "精选项目" }
                div { class: "projects-grid",
                    div { class: "project-card",
                        h3 { "个人博客" }
                        p { "使用Rust + Dioxus构建的现代化博客系统" }
                        div { class: "project-tags",
                            span { class: "project-tag", "Rust" }
                            span { class: "project-tag", "Dioxus" }
                            span { class: "project-tag", "Web" }
                        }
                        a { 
                            href: "https://github.com/xiuton/website_rs",
                            target: "_blank",
                            class: "project-link",
                            "查看源码 →"
                        }
                    }
                    div { class: "project-card",
                        h3 { "在线工具集" }
                        p { "集成多种实用工具的Web应用" }
                        div { class: "project-tags",
                            span { class: "project-tag", "Vue" }
                            span { class: "project-tag", "TypeScript" }
                            span { class: "project-tag", "Vite" }
                        }
                        a { 
                            href: "#",
                            class: "project-link",
                            "查看源码 →"
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
            let mut current_bg_index = current_bg_index.clone();
            let mut bg_timer_handle = bg_timer_handle.clone();
            
            // 初始化背景图片列表，添加默认图片
            let mut initial_images = Vec::new();
            initial_images.push(default_bg_image.to_string());
            background_images.set(initial_images);
            
            // If we already have loaded images (more than just the default one), start the carousel
            if background_images().len() > 1 {
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
                *bg_timer_handle.write() = Some(handle);
                closure.forget();
                return;
            }

            // If no images are loaded (except default), load new ones
            spawn_local(async move {
                let mut loaded_count = 0;
                while loaded_count < 5 {
                    let resp = gloo_net::http::Request::get("https://yun.ganto.cn/bgimg").send().await;
                    if let Ok(response) = resp {
                        if let Ok(filename) = response.text().await {
                            let url = format!("https://files.ganto.cn/files/{}", filename.trim());
                            let mut background_images = background_images.clone();
                            
                            // Try loading the image with retry
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
                            
                            // Try loading twice
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
                                            *bg_timer_handle.write() = Some(handle);
                                            closure.forget();
                                        }
                                    }
                                    break;
                                }
                            }
                            
                            // If both attempts failed, continue the loop to try a new image
                            if !success {
                                continue;
                            }
                        }
                    }
                }
                
                // 不再需要在这里处理默认图片的移除，因为已经在加载第一张图片时处理了
                if loaded_count > 0 {
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
                    *bg_timer_handle.write() = Some(handle);
                    closure.forget();
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
                            d: "M21 12V7a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v5m18 0v5a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-5m18 0H3"
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
                        Link { to: Route::Blog {}, class: "back-button",
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
                                    d: "M19 12H5M12 19l-7-7 7-7"
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
                                        d: "M12 19V5M5 12l7-7 7 7"
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