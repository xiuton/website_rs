use dioxus::prelude::*;
use crate::routes::Route;

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = storage.get_item("theme") {
                            if theme == "dark" {
                                html.set_attribute("class", "dark").unwrap();
                                return true;
                            } else {
                                html.remove_attribute("class").unwrap();
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

#[component]
pub fn App() -> Element {
    rsx! {
        Router::<Route> {}
    }
}