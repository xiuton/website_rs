mod footer;
mod navbar;

pub use footer::Footer;
pub use navbar::Navbar;

use dioxus::prelude::*;

#[component]
pub fn Layout(children: Element) -> Element {
    rsx! {
        div { class: "layout",
            Navbar { is_dark: use_signal(|| false) }
            div { class: "content",
                { children }
            }
            Footer {}
        }
    }
} 