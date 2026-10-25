pub mod app;
pub mod components;
pub mod models;
pub mod nav;
pub mod pages;
pub mod routes;
pub mod site_config;
pub mod ssr;
pub mod utils;

pub use app::App;
pub use models::BlogPost;
pub use routes::Route;

// 引入构建脚本生成的文章列表
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
