use dioxus::prelude::*;

use crate::components::icons::RssIcon;

#[component]
pub fn Rss() -> Element {
    rsx! {
        div { class: "rss-page",
            h1 { class: "rss-title",
                RssIcon {}
                " 订阅"
            }

            p { class: "rss-intro",
                "通过 RSS / Atom 订阅干徒的博客，随时获取最新文章更新。"
            }

            div { class: "rss-feeds",
                div { class: "rss-card",
                    h2 { "RSS 2.0" }
                    p { class: "rss-card-desc",
                        "兼容绝大多数 RSS 阅读器（Feedly、Inoreader、Folo 等）。"
                    }
                    a {
                        class: "rss-link-btn",
                        href: "/static/feed.xml",
                        target: "_blank",
                        "订阅"
                    }
                    div { class: "rss-url",
                        code { "https://ganto.me/static/feed.xml" }
                    }
                }

                div { class: "rss-card",
                    h2 { "Atom 1.0" }
                    p { class: "rss-card-desc",
                        "更现代的订阅格式，支持更多元数据，适合 NetNewsWire、Reeder 等客户端。"
                    }
                    a {
                        class: "rss-link-btn",
                        href: "/static/atom.xml",
                        target: "_blank",
                        "订阅"
                    }
                    div { class: "rss-url",
                        code { "https://ganto.me/static/atom.xml" }
                    }
                }
            }
        }
    }
}
