use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::routes::Route;
use super::{Navbar, Footer};
use crate::utils::dark_mode;

#[component]
pub fn Layout() -> Element {
    let is_dark = dark_mode::use_dark_mode();

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