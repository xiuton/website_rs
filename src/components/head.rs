//! 页面级 head 声明。
//!
//! 页面组件只声明“标题/描述/canonical”，预渲染工具通过 SSR 收集后统一生成
//! `<title>`、description、Open Graph 等标签，避免预渲染代码再维护一份页面元数据。
//!
//! 当前 wasm 端仍由页面已有的 `title::set_seo` 逻辑更新 head；这个组件在 wasm 下
//! 只做占位，SSR 端负责把声明写入线程局部收集器。

use dioxus::prelude::*;
use std::cell::RefCell;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HeadData {
    pub title: String,
    pub description: String,
    pub canonical: String,
}

thread_local! {
    static SSR_HEAD: RefCell<Option<HeadData>> = const { RefCell::new(None) };
}

/// 每次 SSR 渲染前清空，避免上一页的 head 泄漏。
pub fn reset_ssr_head() {
    SSR_HEAD.with(|cell| *cell.borrow_mut() = None);
}

/// 取出本次 SSR 渲染声明的 head。
pub fn take_ssr_head() -> Option<HeadData> {
    SSR_HEAD.with(|cell| cell.borrow_mut().take())
}

#[component]
pub fn PageHead(title: String, #[props(default)] description: String, canonical: String) -> Element {
    #[cfg(not(target_arch = "wasm32"))]
    {
        SSR_HEAD.with(|cell| {
            *cell.borrow_mut() = Some(HeadData { title, description, canonical });
        });
    }

    #[cfg(target_arch = "wasm32")]
    {
        let title = title.clone();
        let description = description.clone();
        let canonical = canonical.clone();
        use_effect(move || {
            crate::utils::title::set_seo(&title, &description, &canonical);
        });
    }

    rsx! {}
}
