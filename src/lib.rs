// 使用 wee_alloc 替代默认分配器，减小 WASM 体积
#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

pub mod app;
pub mod components;
pub mod routes;
pub mod models;
pub mod utils;
pub mod pages;

pub use app::App;
pub use routes::Route;
pub use models::BlogPost;

// 引入构建脚本生成的文章列表
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs")); 
