#[cfg(target_arch = "wasm32")]
use crate::utils::storage;
use dioxus::prelude::*;

fn initial_dark() -> bool {
    // SSR（宿主端）没有 localStorage / DOM，直接返回浅色，避免 web_sys panic
    #[cfg(not(target_arch = "wasm32"))]
    {
        false
    }

    #[cfg(target_arch = "wasm32")]
    {
        let Some(theme) = storage::get_theme() else {
            return false;
        };
        let is_dark = theme == "dark";
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if is_dark {
                        let _ = html.set_attribute("class", "dark");
                    } else {
                        let _ = html.remove_attribute("class");
                    }
                }
            }
        }
        is_dark
    }
}

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(initial_dark)
}
