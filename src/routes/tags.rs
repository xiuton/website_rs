use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde::{Serialize, Deserialize};

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
        let config = include_str!("../../bookmarks.toml");
        let bookmarks: toml::Value = toml::from_str(config).unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));
        
        let bookmarks = bookmarks["bookmark"]
            .as_array()
            .unwrap_or(&Vec::new())
            .iter()
            .filter_map(|item| {
                if let (Some(title), Some(url), Some(description)) = (
                    item.get("title").and_then(|v| v.as_str()),
                    item.get("url").and_then(|v| v.as_str()),
                    item.get("description").and_then(|v| v.as_str()),
                ) {
                    // 如果 icon 字段不存在，使用空字符串
                    let icon = item.get("icon")
                        .and_then(|v| v.as_str())
                        .unwrap_or("")
                        .to_string();

                    Some(Bookmark {
                        title: title.to_string(),
                        url: url.to_string(),
                        description: description.to_string(),
                        icon,
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
pub fn Tags() -> Element {
    let bookmark_manager = use_signal(BookmarkManager::load_from_storage);
    let mut search_text = use_signal(String::new);
    let mut search_query = use_signal(String::new);  // 实际的搜索关键词

    // 处理搜索
    let handle_search = move |_| {
        search_query.set(search_text());
        // 在搜索后添加滚动到第一个匹配项的逻辑
        if !search_text().is_empty() {
            spawn_local(async {
                // 给DOM一点时间更新
                gloo_timers::callback::Timeout::new(100, move || {
                    if let Some(window) = web_sys::window() {
                        if let Some(document) = window.document() {
                            if let Some(element) = document.query_selector(".highlight").ok().flatten() {
                                element.scroll_into_view_with_bool(true);
                            }
                        }
                    }
                }).forget();
            });
        }
    };

    // 处理回车键
    let handle_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            search_query.set(search_text());
            // 在回车搜索后也添加滚动到第一个匹配项的逻辑
            if !search_text().is_empty() {
                spawn_local(async {
                    // 给DOM一点时间更新
                    gloo_timers::callback::Timeout::new(100, move || {
                        if let Some(window) = web_sys::window() {
                            if let Some(document) = window.document() {
                                if let Some(element) = document.query_selector(".highlight").ok().flatten() {
                                    element.scroll_into_view_with_bool(true);
                                }
                            }
                        }
                    }).forget();
                });
            }
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
    // 如果 icon_name 为空字符串或者没有设置 icon 字段（传入空字符串），显示默认图标
    if icon_name.is_empty() {
        return rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M17 3H7c-1.1 0-2 .9-2 2v16l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z"
                }
            }
        };
    }

    match icon_name {
        "github" => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M12 0c-6.626 0-12 5.373-12 12 0 5.302 3.438 9.8 8.207 11.387.599.111.793-.261.793-.577v-2.234c-3.338.726-4.033-1.416-4.033-1.416-.546-1.387-1.333-1.756-1.333-1.756-1.089-.745.083-.729.083-.729 1.205.084 1.839 1.237 1.237 1.237 1.07 1.834 2.807 1.304 3.492.997.107-.775.418-1.305.762-1.604-2.665-.305-5.467-1.334-5.467-5.931 0-1.311.469-2.381 1.236-3.221-.124-.303-.535-1.524.117-3.176 0 0 1.008-.322 3.301 1.23.957-.266 1.983-.399 3.003-.404 1.02.005 2.047.138 3.006.404 2.291-1.552 3.297-1.23 3.297-1.23.653 1.653.242 2.874.118 3.176.77.84 1.235 1.911 1.235 3.221 0 4.609-2.807 5.624-5.479 5.921.43.372.823 1.102.823 2.222v3.293c0 .319.192.694.801.576 4.765-1.589 8.199-6.086 8.199-11.386 0-6.627-5.373-12-12-12z"
                }
            }
        },
        // 如果是完整的URL（以 http:// 或 https:// 开头），则使用图片
        url if url.starts_with("http://") || url.starts_with("https://") => {
            let mut use_default_icon = use_signal(|| false);
            rsx! {
                {if *use_default_icon.read() {
                    rsx! {
                        svg {
                            xmlns: "http://www.w3.org/2000/svg",
                            view_box: "0 0 24 24",
                            fill: "currentColor",
                            path {
                                d: "M17 3H7c-1.1 0-2 .9-2 2v16l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z"
                            }
                        }
                    }
                } else {
                    rsx! {
                        img {
                            src: "{url}",
                            alt: "bookmark icon",
                            class: "bookmark-icon-img",
                            style: "object-fit: contain;",
                            onload: move |_| use_default_icon.set(false),
                            onerror: move |_| use_default_icon.set(true)
                        }
                    }
                }}
            }
        },
        _ => rsx! {
            svg {
                xmlns: "http://www.w3.org/2000/svg",
                view_box: "0 0 24 24",
                fill: "currentColor",
                path {
                    d: "M17 3H7c-1.1 0-2 .9-2 2v16l7-3 7 3V5c0-1.1-.9-2-2-2zm0 15l-5-2.18L7 18V5h10v13z"
                }
            }
        }
    }
} 