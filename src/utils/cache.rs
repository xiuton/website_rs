//! 会话级静态数据缓存 + 构建版本号。
//!
//! 站点有多个页面会消费同一批 `static/*.json`（搜索索引、知识图谱、LDA 主题等）。
//! 以前每进入一次页面都会重新下载并解析一遍，翻几篇文章就重复下载几百 KB。
//! 这里按 URL 缓存响应文本：同一会话内只发一次请求。
//!
//! 同时给所有静态数据 URL 带上 `BUILD_ID`，文章更新后浏览器不会继续使用旧索引
//! （静态托管对 `/static/*.json` 没有文件名指纹）。

use serde::de::DeserializeOwned;
use std::any::Any;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

thread_local! {
    static TEXT_CACHE: RefCell<HashMap<String, String>> = RefCell::new(HashMap::new());
    static TYPED_CACHE: RefCell<HashMap<String, Rc<dyn Any>>> = RefCell::new(HashMap::new());
}

/// 给静态资源 URL 追加构建指纹（已带查询串时用 `&` 连接）。
pub fn versioned(path: &str) -> String {
    let sep = if path.contains('?') { '&' } else { '?' };
    format!("{path}{sep}v={}", crate::BUILD_ID)
}

/// 带缓存的 GET 文本请求。返回 `None` 表示请求失败或非 2xx。
pub async fn fetch_text(path: &str) -> Option<String> {
    let url = versioned(path);

    if let Some(cached) = TEXT_CACHE.with(|cache| cache.borrow().get(&url).cloned()) {
        return Some(cached);
    }

    let resp = gloo_net::http::Request::get(&url).send().await.ok()?;
    if !resp.ok() {
        return None;
    }
    let text = resp.text().await.ok()?;

    TEXT_CACHE.with(|cache| {
        cache.borrow_mut().insert(url, text.clone());
    });
    Some(text)
}

/// 带缓存的 GET + 反序列化。同一会话内只解析一次。
///
/// 返回 `Rc<T>` 而不是 `T`，避免每次进页面都深拷贝大索引；
/// 调用方通过 `Deref` 仍可按 `&T` 使用。
pub async fn fetch_typed<T: DeserializeOwned + 'static>(path: &str) -> Option<Rc<T>> {
    let url = versioned(path);

    if let Some(cached) =
        TYPED_CACHE.with(|cache| cache.borrow().get(&url).cloned().and_then(|value| value.downcast::<T>().ok()))
    {
        return Some(cached);
    }

    let text = fetch_text(path).await?;
    let value = serde_json::from_str::<T>(&text).ok()?;
    let value = Rc::new(value);

    TYPED_CACHE.with(|cache| {
        cache.borrow_mut().insert(url, value.clone());
    });
    Some(value)
}
