use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::routes::Route;
use super::{Navbar, Footer};
use crate::utils::dark_mode;

#[component]
pub fn Layout() -> Element {
    let is_dark = dark_mode::use_dark_mode();

    rsx! {
        a { href: "#main-content", class: "skip-to-content", "跳转到主要内容" }
        div { class: "app",
            Navbar { is_dark: is_dark }
            main { class: "main-content", id: "main-content",
                Outlet::<Route> {}
            }
            Footer {}
        }
    }
} 