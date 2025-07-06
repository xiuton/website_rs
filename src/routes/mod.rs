use dioxus::prelude::*;
use dioxus_router::prelude::{Routable};

pub mod about;
pub mod dev;
pub mod home;
pub mod blog_post;
pub mod tags;
pub mod not_found;
pub mod playground;
pub mod test;

pub use about::About;
pub use dev::Dev;
pub use home::Home;
pub use blog_post::BlogPostView;
pub use tags::Tags;
pub use not_found::NotFound;
pub use playground::Playground;
pub use test::Test;
use crate::components::Layout;

#[derive(Routable, Clone)]
pub enum Route {
    #[layout(Layout)]
    #[route("/")]
    Home,
    #[route("/about")]
    About,
    #[route("/dev")]
    Dev,
    #[route("/tags")]
    Tags,
    #[route("/post/:slug")]
    BlogPostView { slug: String },
    #[route("/playground")]
    Playground,
    #[route("/:..route")]
    NotFound { route: Vec<String> },
    #[end_layout]

    #[route("/test")]
    Test,
}