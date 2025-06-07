use dioxus::prelude::*;
use crate::routes::Route;
use crate::models::RuntimeBlogPost;
use crate::BLOG_POSTS;

#[component]
pub fn BlogPostView(slug: String) -> Element {
    let mut is_wide_mode = use_signal(|| false);
    let post = use_memo(move || {
        BLOG_POSTS.iter()
            .find(|p| p.slug == slug)
            .map(|p| RuntimeBlogPost {
                title: p.title.to_string(),
                date: p.date.to_string(),
                author: p.author.to_string(),
                tags: p.tags.iter().map(|&s| s.to_string()).collect(),
                content: p.content.to_string(),
                slug: p.slug.to_string(),
            })
    });

    // 在页面加载时，读取 localStorage 恢复宽屏状态
    use_effect(move || {
        if post().is_some() {
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

    // 初始化代码高亮
    use_effect(move || {
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        
        // 添加 highlight.js CSS
        let link = document.create_element("link").unwrap();
        link.set_attribute("rel", "stylesheet").unwrap();
        link.set_attribute("href", "https://cdnjs.cloudflare.com/ajax/libs/highlight.js/11.9.0/styles/github-dark.min.css").unwrap();
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
        let post = post();
        if post.is_some() {
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

    rsx! {
        div { class: "blog-container",
            if let Some(post) = post() {
                div { 
                    class: if is_wide_mode() { "blog-post wide-mode" } else { "blog-post" },
                    div { class: "blog-nav",
                        Link { to: Route::Home, class: "back-button",
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
                            let mut options = comrak::ComrakOptions::default();
                            options.extension.table = true;
                            options.extension.strikethrough = true;
                            options.extension.autolink = true;
                            options.extension.tasklist = true;
                            options.extension.superscript = true;
                            options.extension.header_ids = Some("".to_string());
                            options.extension.footnotes = true;
                            options.extension.description_lists = true;
                            options.parse.smart = true;
                            options.render.hardbreaks = true;
                            options.render.github_pre_lang = true;
                            options.render.unsafe_ = true;
                            options.render.escape = false;
                            
                            let html = comrak::markdown_to_html(&post.content, &options);
                            // 确保代码块的语言标识被正确保留
                            let html = html.replace("<pre><code>", "<pre><code class=\"language-plaintext\">");
                            let html = html.replace("<pre><code class=\"", "<pre><code class=\"language-");
                            html
                        }
                    }
                }
            } else {
                div { class: "not-found",
                    h2 { "文章未找到" }
                    p { "抱歉，找不到请求的文章。" }
                }
            }
        }
    }
} 