// 这里可以添加一些工具函数
// 暂时为空，因为我们还没有任何工具函数需要放在这里 

use web_sys::{window, Document, Element};
use pulldown_cmark::{Parser, html::push_html, Options};

pub fn get_window() -> Option<web_sys::Window> {
    window()
}

pub fn get_document() -> Option<Document> {
    get_window()?.document()
}

pub fn get_html_element() -> Option<Element> {
    get_document()?.document_element()
}

pub fn set_theme(is_dark: bool) -> Result<(), String> {
    let html = get_html_element().ok_or("Failed to get HTML element")?;
    
    if is_dark {
        html.set_attribute("class", "dark")
            .map_err(|_| "Failed to set dark theme")?;
    } else {
        html.remove_attribute("class")
            .map_err(|_| "Failed to remove dark theme")?;
    }
    
    if let Some(storage) = get_window()
        .and_then(|w| w.local_storage().ok())
        .flatten() 
    {
        storage.set_item("theme", if is_dark { "dark" } else { "light" })
            .map_err(|_| "Failed to save theme preference")?;
    }
    
    Ok(())
}

pub fn get_theme_preference() -> bool {
    if let Some(window) = get_window() {
        if let Some(storage) = window.local_storage().ok().flatten() {
            if let Ok(Some(theme)) = storage.get_item("theme") {
                return theme == "dark";
            }
        }
        
        // 检查系统主题偏好
        if let Ok(prefers_dark) = js_sys::eval(
            "window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches"
        ) {
            if let Some(is_dark) = prefers_dark.as_bool() {
                return is_dark;
            }
        }
    }
    false
}

pub fn markdown_to_html(markdown: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TABLES);
    
    let parser = Parser::new_ext(markdown, options);
    let mut html_output = String::new();
    push_html(&mut html_output, parser);
    html_output
} 