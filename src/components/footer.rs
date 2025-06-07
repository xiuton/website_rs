use dioxus::prelude::*;

#[component]
pub fn Footer() -> Element {
    rsx! {
        div { class: "footer-content",
            span { "2019-2025 " }
            span { style: "color: rgb(161, 98, 7);", "©" }
            span { " 干徒 / Ganto" }
        }
    }
} 