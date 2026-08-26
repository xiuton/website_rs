use dioxus::prelude::*;
use dioxus_router::prelude::Outlet;
use crate::routes::Route;
use super::{Navbar, Footer};
use crate::utils::{dark_mode, title};

#[component]
pub fn Layout() -> Element {
    let is_dark = dark_mode::use_dark_mode();
    let route = use_route::<Route>();
    let is_wide = matches!(route, Route::KnowledgeGraphView);
    let app_class = if is_wide { "app app-standalone" } else { "app" };

    // 全局同步 canonical / og:url 为当前路径（页面级 set_seo 会覆盖此值）
    use_effect(move || {
        let _ = route;
        let path = web_sys::window()
            .and_then(|w| w.location().pathname().ok())
            .unwrap_or_default();
        if !path.is_empty() {
            title::set_page_canonical(&path);
        }
    });

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