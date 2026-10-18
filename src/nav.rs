//! 导航链接的单一来源。
//!
//! Dioxus 导航栏（`components/navbar.rs`）和构建期的静态预渲染页（`bin/prerender.rs`）
//! 以前各自维护一份链接列表，改导航要改两处、并且容易让预渲染页链到不存在的页面。
//! 这里集中定义，两边都从这里取。

/// 生产环境存在的导航链接（顺序即展示顺序）。
pub const PUBLIC_NAV_LINKS: &[(&str, &str)] = &[
    ("/", "首页"),
    ("/about", "关于"),
    ("/tags", "书签"),
    ("/search", "搜索"),
    ("/knowledge-graph", "图谱"),
    ("/ai-summary", "AI摘要"),
    ("/dev", "开发"),
];

/// 仅在 `dev-pages` feature 下存在的导航链接（本地开发时才编译进产物）。
#[cfg(feature = "dev-pages")]
pub const DEV_ONLY_NAV_LINKS: &[(&str, &str)] = &[("/playground", "操场"), ("/test", "测试")];
