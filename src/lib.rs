pub mod app;
pub mod components;
pub mod routes;
pub mod models;
pub mod utils;

pub use app::App;
pub use routes::Route;
pub use models::{BlogPost, RuntimeBlogPost};

// 引入构建脚本生成的文章列表
include!(concat!(env!("OUT_DIR"), "/blog_posts.rs"));
