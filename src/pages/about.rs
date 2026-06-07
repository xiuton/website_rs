use dioxus::prelude::*;
use crate::utils::title;
use crate::components::icons::{GitHubIcon, EmailIcon, BlogIcon};

#[component]
pub fn About() -> Element {
    title::set_page_title("关于 - 干徒");

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
                        aria_label: "发送邮件",
                        EmailIcon {}
                        span { "i@ganto.me" }
                    }
                    a { 
                        href: "https://github.com/gantoho",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link github-link",
                        aria_label: "GitHub 主页",
                        GitHubIcon {}
                        span { "GitHub" }
                    }
                    a { 
                        href: "https://cnblogs.com/ganto",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        class: "contact-link blog-link",
                        aria_label: "博客园主页",
                        BlogIcon {}
                        span { "博客园" }
                    }
                }
            }
        }
    }
} 