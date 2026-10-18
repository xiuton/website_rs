//! 导航链接的单一来源。
//!
//! 实际数据定义在 `crates/site-model`，Dioxus 导航栏与预渲染工具都从那里读取，
//! 这里只做一层 re-export，保持现有 `crate::nav::*` 调用不变。

pub use site_model::PUBLIC_NAV_LINKS;

/// 仅在 `dev-pages` feature 下存在的导航链接（本地开发时才编译进产物）。
#[cfg(feature = "dev-pages")]
pub use site_model::DEV_ONLY_NAV_LINKS;
