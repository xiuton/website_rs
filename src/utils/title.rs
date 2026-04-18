use web_sys::window;

#[allow(unused)]
pub fn set_page_title(title: &str) {
    if let Some(window) = window() {
        if let Some(document) = window.document() {
            let _ = document.set_title(title);
        }
    }
} 