use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Outlet};

pub mod about;
pub mod dev;
pub mod home;
pub mod blog_post;
pub mod tags;
pub mod not_found;

pub use about::About;
pub use dev::Dev;
pub use home::Home;
pub use blog_post::BlogPostView;
pub use tags::Tags;
pub use not_found::NotFound;
use crate::components::{Navbar, Footer};
use crate::app::use_dark_mode;

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
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
fn Layout() -> Element {
    let is_dark = use_dark_mode();

    rsx! {
        div { class: "app",
            Navbar { is_dark: is_dark }
            div { class: "main-content",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
}