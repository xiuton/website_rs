use dioxus::prelude::*;
use crate::utils::title;

#[component]
pub fn About() -> Element {
    // Set page title
    use_effect(move || {
        title::set_page_title("关于 - 干徒");
        ()
    });

    rsx! {
        div { class: "about-container",
            // 个人介绍部分
            section { class: "about-section intro-section",
                h2 { "👋 你好" }
                p { class: "intro-text",
                    "我是一名全栈开发者爱好者，热爱技术，喜欢探索新的编程语言和框架。"
                    "专注于构建高性能、可维护的编程应用。"
                }
                div { class: "tech-stack",
                    span { "🦀 Rust" }
                    span { "🧬 Dioxus" }
                    span { "🐍 Python" }
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

            // 本站介绍部分
            section { class: "about-section site-section",
                h2 { "🌐 关于本站" }
                div { class: "site-info",
                    p { class: "site-description",
                        "本站是基于现代Web技术栈构建的个人网站，采用前沿的WebAssembly技术，"
                        "实现了高性能的前端应用。"
                    }
                    div { class: "tech-highlight",
                        h3 { "🚀 核心技术栈" }
                        div { class: "tech-grid",
                            div { class: "tech-item",
                                div { class: "tech-icon", "🦀" }
                                div { class: "tech-content",
                                    h4 { "Rust" }
                                    p { "系统级编程语言，提供内存安全和并发性能" }
                                }
                            }
                            div { class: "tech-item",
                                div { class: "tech-icon", "🧬" }
                                div { class: "tech-content",
                                    h4 { "Dioxus" }
                                    p { "基于Rust的声明式UI框架，类似React但性能更优" }
                                }
                            }
                            div { class: "tech-item",
                                div { class: "tech-icon", "📦" }
                                div { class: "tech-content",
                                    h4 { "Trunk" }
                                    p { "Rust WebAssembly应用构建工具，简化开发流程" }
                                }
                            }
                            div { class: "tech-item",
                                div { class: "tech-icon", "⚡" }
                                div { class: "tech-content",
                                    h4 { "WebAssembly" }
                                    p { "高性能的Web技术，接近原生性能的执行速度" }
                                }
                            }
                        }
                    }
                    div { class: "features-list",
                        h3 { "✨ 主要特性" }
                        ul {
                            li { "⚡ 基于WebAssembly的高性能渲染" }
                            li { "🎨 支持明暗主题切换" }
                            li { "📱 完全响应式设计" }
                            li { "🔍 内置博客系统" }
                            li { "🏷️ 标签分类管理" }
                            li { "🎮 交互式组件演示" }
                            li { "🚀 快速加载和流畅动画" }
                        }
                    }
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
                                d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.237 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
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