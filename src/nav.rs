//! 导航链接的单一来源。
//!
//! 实际数据定义在 `crates/site-model`，Dioxus 导航栏与预渲染工具都从那里读取，
//! 这里只做一层 re-export，保持现有 `crate::nav::*` 调用不变。

pub use site_model::PUBLIC_NAV_LINKS;
