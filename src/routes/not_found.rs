use dioxus::prelude::*;

#[component]
pub fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        div {
            class: "not-found",
            h1 { "404 - Not Found" }
            p { "The requested page was not found: /{route:?}" }
        }
    }
} 