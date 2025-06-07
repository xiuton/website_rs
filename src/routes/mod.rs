use dioxus::prelude::*;
use dioxus_router::prelude::Routable;

pub mod about;
pub mod dev;
pub mod home;
pub mod blog_post;
pub mod tags;

pub use about::About;
pub use dev::Dev;
pub use home::Home;
pub use blog_post::BlogPostView;
pub use tags::Tags;

#[derive(Routable, Clone)]
pub enum Route {
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
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub fn AppRouter() -> Element {
    rsx! {
        Router::<Route> {}
    }
}

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            h1 { "404 - Not Found" }
            p { "The requested page was not found: /{route:?}" }
        }
    }
} 