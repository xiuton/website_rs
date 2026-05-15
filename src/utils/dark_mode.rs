use dioxus::prelude::*;

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = storage.get_item("theme") {
                            if theme == "dark" {
                                html.set_attribute("class", "dark").expect("Failed to set dark class on html element");
                                return true;
                            } else {
                                html.remove_attribute("class").expect("Failed to remove class from html element");
                                return false;
                            }
                        }
                    }
                }
            }
        }
        false
    })
}