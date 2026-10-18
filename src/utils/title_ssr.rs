//! SSR（非 wasm）下的 head 占位实现。
//!
//! 页面组件会在渲染期调用 `title::*`；wasm 下这些函数直接操作 DOM，
//! 在宿主端 SSR 时必须变成 no-op，否则会触发 wasm-bindgen 的 imported static panic。
//! 真正的 SSR head 输出后续由 `dioxus-document` / prerender 负责。

pub fn set_page_title(_title: &str) {}

pub fn set_page_canonical(_path: &str) {}

pub fn set_seo(_title: &str, _description: &str, _url: &str) {}

pub fn set_article_jsonld(_title: &str, _description: &str, _url: &str, _date: &str, _author: &str, _tags: &[&str]) {}

pub fn reset_seo_default() {}
