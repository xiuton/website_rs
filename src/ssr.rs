//! SSR 期间把文章正文注入组件树的桥。
//!
//! 预渲染工具在 render 前调用 `set_content`，`BlogPostView` 在宿主端
//! 直接读取，避免走 `web_sys`/模板提取路径。wasm 端不会设置，始终返回 `None`。

use std::cell::RefCell;

thread_local! {
    static CONTENT: RefCell<Option<(String, String)>> = const { RefCell::new(None) };
}

/// 设置当前待渲染文章的 slug 与 HTML。
pub fn set_content(slug: impl Into<String>, html: impl Into<String>) {
    CONTENT.with(|cell| {
        *cell.borrow_mut() = Some((slug.into(), html.into()));
    });
}

/// 取出匹配 slug 的正文；取出后清空，避免泄漏到下一次渲染。
pub fn take_content(slug: &str) -> Option<String> {
    CONTENT.with(|cell| {
        let mut guard = cell.borrow_mut();
        match guard.as_ref() {
            Some((stored_slug, html)) if stored_slug == slug => {
                let html = html.clone();
                *guard = None;
                Some(html)
            }
            _ => None,
        }
    })
}
