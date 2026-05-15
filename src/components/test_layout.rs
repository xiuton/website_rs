use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::routes::Route;

#[component]
pub fn TestLayout() -> Element {
    rsx! {
        div { class: "app app-standalone",
            div {
                style: "display: flex; justify-content: center; padding: 2rem 0;",
                Link {
                    to: "/",
                    "返回首页"
                }
            }
            Outlet::<Route> {}
        }
    }
} 