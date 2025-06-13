use dioxus::prelude::*;
use dioxus_router::prelude::{Routable, Outlet};

pub mod about;
pub mod dev;
pub mod home;
pub mod blog_post;
pub mod tags;
pub mod not_found;
pub mod playground;

pub use about::About;
pub use dev::Dev;
pub use home::Home;
pub use blog_post::BlogPostView;
pub use tags::Tags;
pub use not_found::NotFound;
pub use playground::Playground;
use crate::components::{Navbar, Footer};

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
    #[route("/playground")]
    Playground,
    #[route("/post/:slug")]
    BlogPostView { slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

pub fn use_dark_mode() -> Signal<bool> {
    use_signal(|| {
        if let Some(window) = web_sys::window() {
            if let Some(document) = window.document() {
                if let Some(html) = document.document_element() {
                    if let Some(storage) = window.local_storage().ok().flatten() {
                        if let Ok(Some(theme)) = storage.get_item("theme") {
                            if theme == "dark" {
                                html.set_attribute("class", "dark").unwrap();
                                return true;
                            } else {
                                html.remove_attribute("class").unwrap();
                                return false;
                            }
            }
                    }
        }
    }
        }
        false
    })
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