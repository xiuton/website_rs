use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::routes::Route;
use crate::components::icons::{SunIcon, MoonIcon};

#[component]
pub fn Navbar(is_dark: Signal<bool>) -> Element {
    let nav_items: &[(&str, &str)] = &[
        ("/", "首页"),
        ("/about", "关于"),
        ("/tags", "书签"),
        ("/dev", "开发"),
        #[cfg(feature = "dev-pages")]
        ("/playground", "操场"),
        #[cfg(feature = "dev-pages")]
        ("/test", "测试"),
    ];

    let onclick = move |e: Event<MouseData>| {
        let window = web_sys::window().expect("Failed to get window");
        let document = window.document().expect("Failed to get document");
        let html = document.document_element().expect("Failed to get document element");
        let coords = e.client_coordinates();
        let x = coords.x;
        let y = coords.y;
        let width = window.inner_width().expect("Failed to get inner width").as_f64().expect("Failed to convert width to f64");
        let height = window.inner_height().expect("Failed to get inner height").as_f64().expect("Failed to convert height to f64");
        let end_radius = ((x.max(width - x)).powi(2) + (y.max(height - y)).powi(2)).sqrt();
        html.set_attribute("style", &format!("--x: {}px; --y: {}px; --r: {}px", x, y, end_radius)).expect("Failed to set style attribute");
        let supports_transition = js_sys::eval("Boolean(document.startViewTransition)").expect("Failed to eval startViewTransition").as_bool().unwrap_or(false);
        if supports_transition {
            let _ = js_sys::eval("document.startViewTransition(() => { document.documentElement.classList.toggle('dark'); })");
        } else {
            let class = html.class_name();
            if class.contains("dark") {
                html.set_attribute("class", "").expect("Failed to remove dark class");
            } else {
                html.set_attribute("class", "dark").expect("Failed to set dark class");
            }
        }
        is_dark.set(!is_dark());
        if let Some(window) = web_sys::window() {
            if let Some(storage) = window.local_storage().ok().flatten() {
                let _ = storage.set_item("theme", if is_dark() { "dark" } else { "light" });
            }
        }
    };

    let route = use_route::<Route>();
    
    let is_active = move |href: &str| {
        match href {
            "/" => matches!(route, Route::Home | Route::BlogPostView { slug: _ }),
            "/about" => matches!(route, Route::About),
            "/tags" => matches!(route, Route::Tags),
            "/dev" => matches!(route, Route::Dev),
            #[cfg(feature = "dev-pages")]
            "/playground" => matches!(route, Route::Playground),
            #[cfg(feature = "dev-pages")]
            "/test" => matches!(route, Route::Test),
            _ => false
        }
    };

    use_effect(move || {
        let window = web_sys::window().expect("Failed to get window");
        let document = window.document().expect("Failed to get document");

        let is_stuck = std::rc::Rc::new(std::cell::Cell::new(false));
        let trigger_y = std::rc::Rc::new(std::cell::Cell::new(0.0));
        let interval_id = std::rc::Rc::new(std::cell::Cell::new(0));

        let document_for_interval = document.clone();
        let window_for_interval = window.clone();

        let closure = {
            let document = document.clone();
            let window = window.clone();
            let is_stuck = is_stuck.clone();
            let trigger_y = trigger_y.clone();
            let interval_id = interval_id.clone();
            let document_for_interval = document_for_interval.clone();
            let window_for_interval = window_for_interval.clone();
            Closure::<dyn FnMut()>::new(move || {
                if let Some(nav_links) = document.query_selector(".navbar-links").ok().flatten() {
                    if trigger_y.get() == 0.0 {
                        let rect = nav_links.get_bounding_client_rect();
                        let scroll_y = window.scroll_y().unwrap_or(0.0);
                        trigger_y.set(rect.top() + scroll_y);
                    }

                    let current_scroll_y = window.scroll_y().unwrap_or(0.0);
                    if current_scroll_y >= trigger_y.get() {
                        if !is_stuck.get() {
                            is_stuck.set(true);
                        }
                        if interval_id.get() == 0 {
                            let doc = document_for_interval.clone();
                            let win = window_for_interval.clone();
                            let stuck = is_stuck.clone();
                            let iid = interval_id.clone();
                            let cb = Closure::<dyn FnMut()>::new(move || {
                                if stuck.get() {
                                    if let Some(nl) = doc.query_selector(".navbar-links").ok().flatten() {
                                        if let Some(app) = doc.query_selector(".app").ok().flatten() {
                                            let r = app.get_bounding_client_rect();
                                            let _ = nl.set_attribute(
                                                "style",
                                                &format!("position:fixed;top:0.5rem;left:{}px;width:{}px;", r.left(), r.width()),
                                            );
                                        }
                                    }
                                }
                            });
                            let id = win.set_interval_with_callback_and_timeout_and_arguments_0(
                                cb.as_ref().unchecked_ref(),
                                100,
                            ).unwrap_or(0);
                            iid.set(id);
                            cb.forget();
                        }
                        if let Some(app) = document.query_selector(".app").ok().flatten() {
                            let app_rect = app.get_bounding_client_rect();
                            let _ = nav_links.set_attribute(
                                "style",
                                &format!("position:fixed;top:0.5rem;left:{}px;width:{}px;", app_rect.left(), app_rect.width()),
                            );
                        }
                        let _ = nav_links.set_attribute("data-stuck", "true");
                    } else {
                        if is_stuck.get() {
                            let id = interval_id.get();
                            if id != 0 {
                                window_for_interval.clear_interval_with_handle(id);
                                interval_id.set(0);
                            }
                            let _ = nav_links.remove_attribute("style");
                            let _ = nav_links.remove_attribute("data-stuck");
                            is_stuck.set(false);
                        }
                    }
                }
            })
        };

        let _ = window.add_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());

        closure.forget();
    });

    rsx! {
        div { class: "navbar-content",
            div { class: "navbar-ui",
                div { class: "navbar-title-wrapper",
                    h1 { class: "navbar-title", "干徒" }
                    div { class: "navbar-glow" }
                }
                div { class: "navbar-subtitle", "这很酷" }
                div { class: "navbar-links",
                    {nav_items.iter().map(|(href, label)| {
                        let active = is_active(href);
                        if *href == "/dev" {
                            rsx! {
                                div { class: "nav-item-with-sub",
                                    Link {
                                        to: *href,
                                        class: if active { "nav-link nav-active" } else { "nav-link" },
                                        { label }
                                    }
                                    div { class: "nav-submenu",
                                        Link {
                                            to: Route::CircleGenerator,
                                            class: "nav-sub-link",
                                            "圆形生成器"
                                        }
                                    }
                                }
                            }
                        } else {
                            rsx! {
                                Link {
                                    to: *href,
                                    class: if active { "nav-link nav-active" } else { "nav-link" },
                                    { label }
                                }
                            }
                        }
                    })}
                    button {
                        class: "theme-toggle",
                        onclick: onclick,
                        match is_dark() {
                            true => rsx! { SunIcon {} },
                            false => rsx! { MoonIcon {} }
                        }
                    }
                }
            }
        }
    }
}