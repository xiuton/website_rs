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

/// 切换主题：更新 `<html class="dark">`、按钮状态和 localStorage。
///
/// 支持 View Transition 的浏览器会用点击位置做圆形扩散动画。
pub fn toggle_theme(is_dark: &mut Signal<bool>, event: &Event<MouseData>) {
    let Some(window) = web_sys::window() else {
        return;
    };
    let Some(document) = window.document() else {
        return;
    };
    let Some(html) = document.document_element() else {
        return;
    };

    let coords = event.client_coordinates();
    let x = coords.x;
    let y = coords.y;
    let width = window.inner_width().ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
    let height = window.inner_height().ok().and_then(|value| value.as_f64()).unwrap_or(0.0);
    let end_radius = ((x.max(width - x)).powi(2) + (y.max(height - y)).powi(2)).sqrt();
    let _ = html.set_attribute("style", &format!("--x: {}px; --y: {}px; --r: {}px", x, y, end_radius));

    let supports_transition = js_sys::eval("Boolean(document.startViewTransition)")
        .ok()
        .and_then(|value| value.as_bool())
        .unwrap_or(false);

    if supports_transition {
        let _ =
            js_sys::eval("document.startViewTransition(() => { document.documentElement.classList.toggle('dark'); })");
    } else {
        let class = html.class_name();
        if class.contains("dark") {
            let _ = html.set_attribute("class", "");
        } else {
            let _ = html.set_attribute("class", "dark");
        }
    }

    is_dark.set(!is_dark());
    #[cfg(target_arch = "wasm32")]
    storage::set_theme(is_dark());
}
