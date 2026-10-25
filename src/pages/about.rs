use crate::components::icons::{EmailIcon, GitHubIcon};
use crate::components::PageHead;
use crate::site_config::{HIDDEN_HARDWARE, HIDDEN_HARDWARE_GROUPS, SHOW_HARDWARE_PRICES, SHOW_HARDWARE_SECTION};
use dioxus::prelude::*;

struct HardwareItem {
    key: &'static str,
    label: &'static str,
    detail: &'static str,
    price: &'static str,
}

impl HardwareItem {
    const fn new(key: &'static str, label: &'static str, detail: &'static str, price: &'static str) -> Self {
        Self { key, label, detail, price }
    }
}

fn hardware_group(group_key: &'static str, title: &'static str, items: &[HardwareItem]) -> Element {
    if HIDDEN_HARDWARE_GROUPS.contains(&group_key) {
        return rsx! {};
    }

    let visible: Vec<_> = items.iter().filter(|item| !HIDDEN_HARDWARE.contains(&item.key)).collect();

    if visible.is_empty() {
        return rsx! {};
    }

    rsx! {
        div { class: "about-hardware-group",
            h3 { {title} }
            ul {
                class: if SHOW_HARDWARE_PRICES {
                    "about-hardware-list"
                } else {
                    "about-hardware-list no-price"
                },
                {visible.into_iter().map(|item| rsx! {
                    li {
                        span { class: "hardware-name", {item.label} }
                        span { class: "hardware-detail", {item.detail} }
                        if SHOW_HARDWARE_PRICES {
                            span { class: "hardware-price", {item.price} }
                        }
                    }
                })}
            }
        }
    }
}

#[component]
pub fn About() -> Element {
    rsx! {
        PageHead {
            title: "关于".to_string(),
            description: "关于干徒：Rust、Web、AI 与工程实践。".to_string(),
            canonical: "/about".to_string(),
        }
        div { class: "about-page",
            header { class: "about-hero",
                div { class: "about-hero-head",
                    div { class: "about-avatar", aria_hidden: "true", "干" }
                    div { class: "about-identity",
                        p { class: "about-kicker", "ABOUT" }
                        h1 { "干徒" }
                        p { class: "about-role", "全栈开发者 · Rust / Web / AI" }
                    }
                }
                p { class: "about-bio",
                    "我是一名全栈开发者爱好者，热爱技术，喜欢探索新的编程语言和框架。"
                    "专注于构建高性能、可维护的编程应用。"
                }
                div { class: "about-actions",
                    a {
                        class: "about-action about-action-primary",
                        href: "mailto:i@ganto.me",
                        aria_label: "发送邮件",
                        EmailIcon {}
                        span { "i@ganto.me" }
                    }
                    a {
                        class: "about-action",
                        href: "https://github.com/gantoho",
                        target: "_blank",
                        rel: "noopener noreferrer",
                        aria_label: "GitHub 主页",
                        GitHubIcon {}
                        span { "GitHub" }
                    }
                }
                div { class: "about-stack", aria_label: "技术栈",
                    span { "Rust" }
                    span { "Dioxus" }
                    span { "Python" }
                    span { "Go" }
                    span { "Java" }
                    span { "JavaScript" }
                    span { "TypeScript" }
                    span { "React" }
                    span { "Vue" }
                    span { "Node.js" }
                    span { "Deno" }
                }
            }

            section { class: "about-block", id: "skills",
                header { class: "about-block-head",
                    span { class: "about-index", "01" }
                    div { class: "about-block-title",
                        h2 { "技能" }
                        p { "从界面到服务端，再到部署。" }
                    }
                }
                div { class: "about-skill-grid",
                    article { class: "about-skill-card",
                        h3 { "前端开发" }
                        p { "现代前端框架、响应式设计与性能优化。" }
                        div { class: "about-skill-tags",
                            span { "TypeScript" }
                            span { "React" }
                            span { "Vue" }
                            span { "Webpack" }
                        }
                    }
                    article { class: "about-skill-card",
                        h3 { "后端开发" }
                        p { "服务端开发、API 设计与数据库优化。" }
                        div { class: "about-skill-tags",
                            span { "Rust" }
                            span { "Go" }
                            span { "Node.js" }
                            span { "MySQL" }
                        }
                    }
                    article { class: "about-skill-card",
                        h3 { "DevOps" }
                        p { "自动化部署、容器化与持续交付。" }
                        div { class: "about-skill-tags",
                            span { "Docker" }
                            span { "Kubernetes" }
                            span { "GitHub Actions" }
                            span { "Jenkins" }
                        }
                    }
                }
            }

            section { class: "about-block", id: "site",
                header { class: "about-block-head",
                    span { class: "about-index", "02" }
                    div { class: "about-block-title",
                        h2 { "本站" }
                        p { "一个用 Rust 写成的个人站点。" }
                    }
                }
                div { class: "about-site-panel",
                    p { class: "about-site-desc",
                        "本站基于现代 Web 技术栈构建，使用 WebAssembly 运行前端逻辑，"
                        "在浏览器里获得接近原生的加载与交互体验。"
                    }
                    div { class: "about-site-tech",
                        div {
                            strong { "Rust" }
                            span { "系统级语言，内存安全" }
                        }
                        div {
                            strong { "Dioxus" }
                            span { "Rust 声明式 UI 框架" }
                        }
                        div {
                            strong { "Trunk" }
                            span { "WebAssembly 构建工具" }
                        }
                        div {
                            strong { "WebAssembly" }
                            span { "接近原生的运行性能" }
                        }
                    }
                    div { class: "about-features",
                        h3 { "包含" }
                        ul {
                            li { "WebAssembly 高性能渲染" }
                            li { "明暗主题切换" }
                            li { "响应式布局" }
                            li { "博客与系列文章" }
                            li { "标签与全文搜索" }
                            li { "交互式组件" }
                            li { "快速加载与流畅动画" }
                        }
                    }
                }
            }

            if SHOW_HARDWARE_SECTION {
                section { class: "about-block", id: "hardware",
                header { class: "about-block-head",
                    span { class: "about-index", "03" }
                    div { class: "about-block-title",
                        h2 { "硬件" }
                        p {
                            if SHOW_HARDWARE_PRICES {
                                "这台机器的实际配置与购入价格。"
                            } else {
                                "这台机器的实际配置。"
                            }
                        }
                    }
                }
                div { class: "about-hardware",
                    {hardware_group("core", "核心平台", &[
                        HardwareItem::new("cpu", "CPU", "i5-13600K", "¥2,098"),
                        HardwareItem::new("mainboard", "主板", "铭瑄 Z690 D4 终结者", "¥835"),
                        HardwareItem::new("gpu-rx460", "显卡", "RX460 讯景 刷 RX560 BIOS", "¥155"),
                        HardwareItem::new("gpu-gtx1060", "显卡", "七彩虹 GTX 1060 6G 双风扇", "¥645.70"),
                    ])}
                    {hardware_group("storage", "存储与内存", &[
                        HardwareItem::new("ssd-kp230-1", "固态", "金百达 KP230 512G M.2 NVMe * 2", "¥215.00 + ¥172.42"),
                        HardwareItem::new("ram-samsung", "内存条", "金百达 银爵 DDR4 3200MHz 8G * 4（三星 * 2 + 长鑫 * 2）", "¥258.93 + ¥249.00"),
                    ])}
                    {hardware_group("peripherals", "外设与机电", &[
                        HardwareItem::new("monitor", "显示器", "Redmi 1A 23.8", "¥469"),
                        HardwareItem::new("psu", "电源", "鑫谷战斧600 额定500W", "¥190"),
                        HardwareItem::new("cooler", "散热器", "利民 PA120-SE 青春版", "¥119.28"),
                        HardwareItem::new("case", "机箱", "爱国者 A15", "¥100"),
                    ])}
                    {hardware_group("fees", "其他费用", &[
                        HardwareItem::new("shipping", "运费", "", "¥12"),
                        HardwareItem::new("bios", "刷 BIOS", "", "¥30"),
                        HardwareItem::new("ethernet", "网线", "", "¥10"),
                    ])}
                    if SHOW_HARDWARE_PRICES {
                        p { class: "about-hardware-note",
                            "金百达固态与内存条合并顺丰运费 ¥12，列在其他费用中。"
                        }
                    }
                }
            }
            }
        }
    }
}
