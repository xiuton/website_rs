use dioxus::prelude::*;
use dioxus_router::prelude::{Routable};

use crate::components::Layout;
use crate::components::TestLayout;
use crate::pages::{
    About, Dev, Home, BlogPostView, Tags, NotFound, CircleGenerator, KnowledgeGraphView, Search, AiSummaryView, Rss,
};
#[cfg(feature = "dev-pages")]
use crate::pages::{Playground, Test};

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
    #[route("/search")]
    Search,
    #[route("/rss")]
    Rss,
    #[route("/knowledge-graph")]
    KnowledgeGraphView,
    #[route("/ai-summary")]
    AiSummaryView,
    #[route("/post/:slug")]
    BlogPostView { slug: String },
    #[cfg(feature = "dev-pages")]
    #[route("/playground")]
    Playground,
    #[route("/:..route")]
    NotFound { route: Vec<String> },
    #[end_layout]

    #[layout(TestLayout)]
    #[route("/circle-generator")]
    CircleGenerator,
    #[cfg(feature = "dev-pages")]
    #[route("/test")]
    Test,
}