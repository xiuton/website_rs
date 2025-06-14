mod models;
mod routes;
mod app;
mod components;
pub mod utils;

use web_rs::BLOG_POSTS;
use dioxus::launch;

fn main() {
    // 初始化 panic hook
    console_error_panic_hook::set_once();
    
    // 启动应用
    launch(web_rs::App);
}