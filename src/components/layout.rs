use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::routes::Route;
use super::{Navbar, Footer};
use crate::utils::dark_mode;

#[component]
pub fn Layout() -> Element {
    let is_dark = dark_mode::use_dark_mode();
    let route = use_route::<Route>();
    let is_wide = matches!(route, Route::KnowledgeGraphView);
    let app_class = if is_wide { "app app-standalone" } else { "app" };

    rsx! {
        div { class: "{app_class}",
            Navbar { is_dark: is_dark }
            main { class: "main-content", id: "main-content",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
} 