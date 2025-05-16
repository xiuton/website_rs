use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::window;

fn main() {
    // 初始化 panic hook
    console_error_panic_hook::set_once();
    
    // 启动应用
    dioxus_web::launch(App);
}

fn App(cx: Scope) -> Element {
    let is_dark = use_state(&cx, || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    // 优先从 localStorage 读取主题设置
                    if let Some(local_storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = local_storage.get_item("theme") {
                            if theme == "dark" {
                                html.set_attribute("class", "dark").unwrap();
                                return true;
                            } else {
                                html.remove_attribute("class").unwrap();
                                return false;
                            }
                        }
                    }
                    // 如果 localStorage 中没有设置，则用 JS 检查系统主题
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

    let toggle_theme = move |e: Event<MouseData>| {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let html = document.document_element().unwrap();
        let x = e.client_x as f64;
        let y = e.client_y as f64;
        // 计算最大圆半径
        let width = window.inner_width().unwrap().as_f64().unwrap();
        let height = window.inner_height().unwrap().as_f64().unwrap();
        let end_radius = ((x.max(width - x)).powi(2) + (y.max(height - y)).powi(2)).sqrt();
        html.set_attribute("style", &format!("--x: {}px; --y: {}px; --r: {}px", x, y, end_radius)).unwrap();
    
        // 判断是否支持 startViewTransition
        let supports_transition = js_sys::eval("Boolean(document.startViewTransition)").unwrap().as_bool().unwrap_or(false);
        if supports_transition {
            let _ = js_sys::eval("document.startViewTransition(() => { document.documentElement.classList.toggle('dark'); })");
        } else {
            // 直接切换 class
            let class = html.class_name();
            if class.contains("dark") {
                html.set_attribute("class", "").unwrap();
            } else {
                html.set_attribute("class", "dark").unwrap();
            }
        }
    
        is_dark.set(!is_dark.get());
    
        // 保存主题设置到 localStorage
        if let Some(local_storage) = window.local_storage().ok().flatten() {
            let _ = local_storage.set_item("theme", if !is_dark.get() { "dark" } else { "light" });
        }
    };

    cx.render(rsx! {
        div { class: "app",
            nav { class: "navbar",
                div { class: "nav-content",
                    div { class: "nav-left",
                        a { href: "/", class: "nav-logo", "Ganto" }
                    }
                    div { class: "nav-right",
                        a { href: "/", class: "nav-link", "首页" }
                        a { href: "/about", class: "nav-link", "关于" }
                        a { href: "/projects", class: "nav-link", "项目" }
                        a { href: "/blog", class: "nav-link", "博客" }
                        a { href: "/contact", class: "nav-link", "联系" }
                        button { 
                            class: "theme-toggle",
                            onclick: toggle_theme,
                            if *is_dark.get() {
                                "🌞"
                            } else {
                                "🌙"
                            }
                        }
                    }
                }
            }
            div { class: "main-content",
                Router::<Route> {}
            }
        }
    })
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home,
    #[route("/about")]
    About,
    #[route("/projects")]
    Projects,
    #[route("/blog")]
    Blog,
    #[route("/contact")]
    Contact,
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container home",
            div { class: "hero",
                h1 { class: "title", "你好，我是 Ganto" }
                p { class: "subtitle", "一名热爱技术的全栈开发者，专注于 Web 开发和系统架构设计。" }
                div { class: "social-links",
                    a { href: "https://github.com/ganto", target: "_blank", "GitHub" }
                    a { href: "https://twitter.com/ganto", target: "_blank", "Twitter" }
                }
            }
            div { class: "featured-projects",
                h2 { "精选项目" }
                div { class: "project-grid",
                    div { class: "project-card",
                        h3 { "Dioxus Web" }
                        p { "使用 Rust 和 Dioxus 构建的现代化 Web 应用框架" }
                    }
                    div { class: "project-card",
                        h3 { "Rust 学习笔记" }
                        p { "记录 Rust 语言学习过程中的心得体会和最佳实践" }
                    }
                }
            }
        }
    })
}

fn About(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container about",
            h1 { "关于我" }
            div { class: "about-content",
                p { "我是一名全栈开发者，专注于 Web 开发和系统架构设计。热爱开源，喜欢探索新技术。" }
                p { "目前主要使用 Rust、TypeScript、React 等技术栈进行开发。同时也关注系统设计、性能优化和用户体验。" }
                div { class: "skills",
                    h2 { "技术栈" }
                    div { class: "skill-tags",
                        span { "Rust" }
                        span { "TypeScript" }
                        span { "React" }
                        span { "Node.js" }
                        span { "WebAssembly" }
                        span { "系统设计" }
                    }
                }
            }
        }
    })
}

fn Projects(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container projects",
            h1 { "项目" }
            div { class: "project-list",
                div { class: "project-item",
                    h2 { "Dioxus Web" }
                    p { "一个使用 Rust 和 Dioxus 构建的现代化 Web 应用框架，支持服务端渲染和客户端渲染。" }
                    div { class: "project-tags",
                        span { "Rust" }
                        span { "WebAssembly" }
                        span { "Web" }
                    }
                }
                div { class: "project-item",
                    h2 { "Rust 学习笔记" }
                    p { "记录 Rust 语言学习过程中的心得体会和最佳实践，包括内存安全、并发编程等内容。" }
                    div { class: "project-tags",
                        span { "Rust" }
                        span { "学习笔记" }
                    }
                }
            }
        }
    })
}

fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container blog",
            h1 { "博客" }
            div { class: "post-list",
                div { class: "post-item",
                    h2 { "Rust 中的内存安全" }
                    p { "深入探讨 Rust 的所有权系统和内存安全机制，以及如何在实际项目中应用这些概念。" }
                    div { class: "post-meta",
                        span { "2024-05-16" }
                        span { "Rust" }
                    }
                }
                div { class: "post-item",
                    h2 { "WebAssembly 入门指南" }
                    p { "介绍 WebAssembly 的基础知识，以及如何使用 Rust 开发 WebAssembly 应用。" }
                    div { class: "post-meta",
                        span { "2024-05-15" }
                        span { "WebAssembly" }
                    }
                }
            }
        }
    })
}

fn Contact(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "container contact",
            h1 { "联系我" }
            div { class: "contact-content",
                p { "如果你有任何问题或合作意向，欢迎通过以下方式联系我：" }
                div { class: "contact-methods",
                    a { href: "mailto:ganto@example.com", "Email" }
                    a { href: "https://github.com/ganto", target: "_blank", "GitHub" }
                    a { href: "https://twitter.com/ganto", target: "_blank", "Twitter" }
                }
            }
        }
    })
}
