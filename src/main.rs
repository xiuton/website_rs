use dioxus::prelude::*;
use dioxus_router::prelude::*;
use web_sys::window;

fn main() {
    // 初始化 panic hook
    console_error_panic_hook::set_once();
    
    // 启动应用
    dioxus_web::launch(App);
}

#[component]
fn Navbar<'a>(cx: Scope<'a>, is_dark: &'a UseState<bool>) -> Element<'a> {
    const NAV_ITEMS: &[(&str, &str)] = &[
        ("/", "首页"),
        ("/blog", "博客"),
        ("/tags", "书签"),
        ("/dev", "开发"),
    ];

    let onclick = {
        let is_dark = is_dark.clone();
        move |e: Event<MouseData>| {
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
            is_dark.set(!*is_dark.get());
            if let Some(local_storage) = window.local_storage().ok().flatten() {
                let _ = local_storage.set_item("theme", if !is_dark.get() { "dark" } else { "light" });
            }
        }
    };

    let route = use_route::<Route>(&cx);
    let window = web_sys::window().unwrap();
    let location = window.location();
    let current_path = location.pathname().unwrap_or_else(|_| "/".to_string());
    
    // 调试输出当前路径
    web_sys::console::log_1(&format!("current_path: {}", current_path).into());
    
    let is_active = |href: &str| {
        // 调试输出每个href和比较结果
        web_sys::console::log_1(&format!("comparing: current_path={} href={}", current_path, href).into());
        match href {
            "/" => current_path == "/",
            _ => current_path.starts_with(href)
        }
    };

    cx.render(rsx! {
        div { class: "navbar-container",
            div { class: "navbar-ui",
                div { class: "navbar-title-wrapper",
                    h1 { class: "navbar-title", "干徒" }
                    div { class: "navbar-glow" }
                }
                div { class: "navbar-subtitle", "这很酷" }
                div { class: "navbar-links",
                    NAV_ITEMS.iter().map(|(href, label)| {
                        let active = is_active(href);
                        // 调试输出每个链接的激活状态
                        web_sys::console::log_1(&format!("link: {} active: {}", href, active).into());
                        rsx! {
                            a {
                                href: *href,
                                class: if active { "nav-link nav-active" } else { "nav-link" },
                                label
                            }
                        }
                    })
                    button {
                        class: "theme-toggle",
                        onclick: onclick,
                        if *is_dark.get() {
                            rsx! {
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 1024 1024",
                                    path {
                                        fill: "currentColor",
                                        d: "M512 704a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512m0-704a32 32 0 0 1 32 32v64a32 32 0 0 1-64 0V96a32 32 0 0 1 32-32m0 768a32 32 0 0 1 32 32v64a32 32 0 1 1-64 0v-64a32 32 0 0 1 32-32M195.2 195.2a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 1 1-45.248 45.248L195.2 240.448a32 32 0 0 1 0-45.248zm543.104 543.104a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 0 1-45.248 45.248l-45.248-45.248a32 32 0 0 1 0-45.248M64 512a32 32 0 0 1 32-32h64a32 32 0 0 1 0 64H96a32 32 0 0 1-32-32m768 0a32 32 0 0 1 32-32h64a32 32 0 1 1 0 64h-64a32 32 0 0 1-32-32M195.2 828.8a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248L240.448 828.8a32 32 0 0 1-45.248 0zm543.104-543.104a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248l-45.248 45.248a32 32 0 0 1-45.248 0"
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                svg {
                                    xmlns: "http://www.w3.org/2000/svg",
                                    view_box: "0 0 1024 1024",
                                    path {
                                        fill: "currentColor",
                                        d: "M512 704a192 192 0 1 0 0-384 192 192 0 0 0 0 384m0 64a256 256 0 1 1 0-512 256 256 0 0 1 0 512m0-704a32 32 0 0 1 32 32v64a32 32 0 0 1-64 0V96a32 32 0 0 1 32-32m0 768a32 32 0 0 1 32 32v64a32 32 0 1 1-64 0v-64a32 32 0 0 1 32-32M195.2 195.2a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 1 1-45.248 45.248L195.2 240.448a32 32 0 0 1 0-45.248zm543.104 543.104a32 32 0 0 1 45.248 0l45.248 45.248a32 32 0 0 1-45.248 45.248l-45.248-45.248a32 32 0 0 1 0-45.248M64 512a32 32 0 0 1 32-32h64a32 32 0 0 1 0 64H96a32 32 0 0 1-32-32m768 0a32 32 0 0 1 32-32h64a32 32 0 1 1 0 64h-64a32 32 0 0 1-32-32M195.2 828.8a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248L240.448 828.8a32 32 0 0 1-45.248 0zm543.104-543.104a32 32 0 0 1 0-45.248l45.248-45.248a32 32 0 0 1 45.248 45.248l-45.248 45.248a32 32 0 0 1-45.248 0"
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    })
}

#[component]
fn Footer(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "footer",
            span { "2019-2025 " }
            span { style: "color: rgb(161, 98, 7);", "©" }
            span { " 干徒 / Ganto" }
        }
    })
}

fn App(cx: Scope) -> Element {
    let is_dark = use_state(&cx, || {
        if let Some(window) = window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
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
    cx.render(rsx! {
        div { class: "app",
            Navbar { is_dark: &is_dark }
            div { class: "main-content",
                Router::<Route> {}
            }
            Footer {}
        }
    })
}

#[derive(Routable, Clone)]
enum Route {
    #[route("/")]
    Home,
    #[route("/blog")]
    Blog,
    #[route("/tags")]
    Tags,
    #[route("/dev")]
    Dev,
}

fn Home(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "main-bg",
            div { class: "center-container",
                div { class: "main-center",
                    div { class: "code-cards",
                        div { class: "code-card",
                            div { class: "code-card-title", "TypeScript" }
                            pre { class: "code-block ts", { "const str: string = \"Hello TS\";\nconsole.log(str);" } }
                        }
                        div { class: "code-card",
                            div { class: "code-card-title", "Golang" }
                            pre { class: "code-block go", { "package main\n\nimport \"fmt\"\n\nfunc main() {\n    var str string = \"Hello Golang\"\n    fmt.Println(str)\n}" } }
                        }
                        div { class: "code-card",
                            div { class: "code-card-title", "Rust" }
                            pre { class: "code-block rust", { "fn main() {\n    let str = \"Hello Rust\";\n    print!(\"{}\", str);\n}" } }
                        }
                    }
                }
            }
        }
    })
}

fn Blog(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "main-bg",
            div { class: "center-container",
                div { class: "main-center blog-center",
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
            }
        }
    })
}

fn Tags(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "main-bg",
            div { class: "center-container",
                div { class: "main-center tags-center",
                    div { class: "tags-list",
                        div { class: "tag-group",
                            h2 { "开发工具" }
                            ul {
                                li { a { href: "https://github.com/", target: "_blank", "GitHub" } }
                                li { a { href: "https://gitee.com/", target: "_blank", "Gitee" } }
                                li { a { href: "https://code.visualstudio.com/", target: "_blank", "VS Code" } }
                                li { a { href: "https://rust-lang.org/", target: "_blank", "Rust 官网" } }
                                li { a { href: "https://crates.io/", target: "_blank", "Crates.io" } }
                            }
                        }
                        div { class: "tag-group",
                            h2 { "学习资源" }
                            ul {
                                li { a { href: "https://zh.javascript.info/", target: "_blank", "JavaScript 教程" } }
                                li { a { href: "https://doc.rust-lang.org/book/", target: "_blank", "Rust Book" } }
                                li { a { href: "https://react.dev/", target: "_blank", "React 官方文档" } }
                                li { a { href: "https://dioxuslabs.com/", target: "_blank", "Dioxus 文档" } }
                            }
                        }
                    }
                }
            }
        }
    })
}

fn Dev(cx: Scope) -> Element {
    cx.render(rsx! {
        div { class: "main-bg",
            div { class: "center-container",
                div { class: "main-center dev-center",
                    p { "这里是开发相关内容与工具集合。" }
                    ul {
                        li { a { href: "https://github.com/ganto", target: "_blank", "GitHub 主页" } }
                        li { a { href: "https://ganto.me/blog", target: "_blank", "个人博客" } }
                        li { a { href: "https://ganto.me/projects", target: "_blank", "项目展示" } }
                        li { a { href: "https://ganto.me/contact", target: "_blank", "联系我" } }
                    }
                    div { class: "dev-note",
                        h2 { "开发笔记" }
                        ul {
                            li { "Rust + Dioxus 构建现代 Web 应用" }
                            li { "WASM 技术栈实践与优化" }
                            li { "持续集成与自动化部署" }
                        }
                    }
                }
            }
        }
    })
}
