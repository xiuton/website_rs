use dioxus::prelude::*;
use crate::utils::storage;

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(|| {
        if let Some(theme) = storage::get_theme() {
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
            return is_dark;
        }
        false
    })
}