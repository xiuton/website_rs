/// 全局 localStorage 配置管理
/// 所有持久化存储的 key 统一在此定义，通过类型化 getter/setter 访问。

// ── Key Constants ──

const KEY_THEME: &str = "theme";
const KEY_BLOG_WIDE_MODE: &str = "blog_wide_mode";
const KEY_BLOG_PAGE_SIZE: &str = "blog_page_size";
const KEY_CIRCLE_GENERATOR: &str = "circle_generator_config";

// ── Helpers ──

fn storage() -> Option<web_sys::Storage> {
    web_sys::window()?.local_storage().ok().flatten()
}

fn get_str(key: &str) -> Option<String> {
    storage()?.get_item(key).ok().flatten()
}

fn set_str(key: &str, value: &str) {
    if let Some(s) = storage() {
        let _ = s.set_item(key, value);
    }
}

// ── Typed API ──

/// 深色模式主题
pub fn get_theme() -> Option<String> {
    get_str(KEY_THEME)
}

pub fn set_theme(is_dark: bool) {
    set_str(KEY_THEME, if is_dark { "dark" } else { "light" });
}

/// 博客宽屏模式
pub fn get_blog_wide_mode() -> bool {
    get_str(KEY_BLOG_WIDE_MODE)
        .map(|v| v == "true")
        .unwrap_or(false)
}

pub fn set_blog_wide_mode(wide: bool) {
    set_str(KEY_BLOG_WIDE_MODE, if wide { "true" } else { "false" });
}

/// 博客每页显示条数
pub fn get_blog_page_size() -> usize {
    get_str(KEY_BLOG_PAGE_SIZE)
        .and_then(|v| v.parse().ok())
        .unwrap_or(10)
}

pub fn set_blog_page_size(size: usize) {
    set_str(KEY_BLOG_PAGE_SIZE, &size.to_string());
}

/// 圆形生成器配置（透传 JSON 字符串，由调用方序列化/反序列化）
pub fn get_circle_generator_config() -> Option<String> {
    get_str(KEY_CIRCLE_GENERATOR)
}

pub fn set_circle_generator_config(json: &str) {
    set_str(KEY_CIRCLE_GENERATOR, json);
}
