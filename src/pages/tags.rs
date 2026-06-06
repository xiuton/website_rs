use dioxus::prelude::*;
use wasm_bindgen_futures::spawn_local;
use wasm_bindgen::JsCast;
use serde::{Serialize, Deserialize};
use std::sync::LazyLock;
use crate::utils::title;
use crate::components::icons::{BookmarkIcon, GitHubIcon, SearchIcon};

#[derive(Clone, Serialize, Deserialize, PartialEq, Debug)]
struct Bookmark {
    title: String,
    url: String,
    description: String,
    icon: String,
}

static BOOKMARKS: LazyLock<Vec<Bookmark>> = LazyLock::new(|| {
    let config = include_str!("../../data/bookmarks.toml");
    let bookmarks: toml::Value = toml::from_str(config).unwrap_or_else(|_| toml::Value::Table(toml::Table::new()));

    bookmarks["bookmark"]
        .as_array()
        .unwrap_or(&Vec::new())
        .iter()
        .filter_map(|item| {
            if let (Some(title), Some(url), Some(description)) = (
                item.get("title").and_then(|v| v.as_str()),
                item.get("url").and_then(|v| v.as_str()),
                item.get("description").and_then(|v| v.as_str()),
            ) {
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
        .collect()
});

fn scroll_to_first_highlight() {
    spawn_local(async {
        let closure = wasm_bindgen::closure::Closure::once_into_js(Box::new(move || {
            if let Some(window) = web_sys::window() {
                if let Some(document) = window.document() {
                    if let Some(element) = document.query_selector(".highlight").ok().flatten() {
                        element.scroll_into_view_with_bool(true);
                    }
                }
            }
        }) as Box<dyn FnOnce()>);
        let _handle = web_sys::window()
            .and_then(|w| w.set_timeout_with_callback_and_timeout_and_arguments_0(
                closure.unchecked_ref(),
                100
            ).ok());
    });
}

#[component]
pub fn Tags() -> Element {
    // Set page title
    use_effect(move || {
        title::set_page_title("书签 - 干徒");
    });

    let bookmark_manager = use_signal(|| BOOKMARKS.clone());
    let mut search_text = use_signal(String::new);
    let mut search_query = use_signal(String::new);  // 实际的搜索关键词

    // 处理搜索
    let handle_search = move |_| {
        search_query.set(search_text());
        if !search_text().is_empty() {
            scroll_to_first_highlight();
        }
    };

    // 处理回车键
    let handle_keydown = move |evt: Event<KeyboardData>| {
        if evt.key() == Key::Enter {
            search_query.set(search_text());
            if !search_text().is_empty() {
                scroll_to_first_highlight();
            }
        }
    };

    let filtered_bookmarks = use_memo(move || {
        let search = search_query().to_lowercase();
        let bookmarks = bookmark_manager.read();

        bookmarks.iter()
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
                        SearchIcon {}
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
        return rsx! { BookmarkIcon {} };
    }

    match icon_name {
        "github" => rsx! { GitHubIcon {} },
        // 如果是完整的URL（以 http:// 或 https:// 开头），则使用图片
        url if url.starts_with("http://") || url.starts_with("https://") => {
            let mut use_default_icon = use_signal(|| false);
            rsx! {
                {if *use_default_icon.read() {
                    rsx! { BookmarkIcon {} }
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
        _ => rsx! { BookmarkIcon {} }
    }
} 