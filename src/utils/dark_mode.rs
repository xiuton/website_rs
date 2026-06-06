use dioxus::prelude::*;

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = storage.get_item("theme") {
                            if theme == "dark" {
                                let _ = html.set_attribute("class", "dark");
                                return true;
                            } else {
                                let _ = html.remove_attribute("class");
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