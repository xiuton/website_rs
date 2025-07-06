use dioxus::prelude::*;
use dioxus_router::prelude::{Routable};

use crate::components::{ Layout, TestLayout };
use crate::pages::{
    About, Dev, Home, BlogPostView, Tags, NotFound, Playground, Test
};

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

    #[layout[TestLayout]]
    #[route("/test")]
    Test,
}