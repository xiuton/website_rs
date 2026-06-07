use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use crate::routes::Route;
use crate::utils::storage;
use crate::components::icons::{SunIcon, MoonIcon};

#[component]
pub fn Navbar(is_dark: Signal<bool>) -> Element {
    let nav_items: &[(&str, &str)] = &[
        ("/", "首页"),
        ("/about", "关于"),
        ("/tags", "书签"),
        ("/search", "搜索"),
        ("/dev", "开发"),
        #[cfg(feature = "dev-pages")]
        ("/playground", "操场"),
        #[cfg(feature = "dev-pages")]
        ("/test", "测试"),
    ];

    let onclick = move |e: Event<MouseData>| {
        let Some(window) = web_sys::window() else {
            web_sys::console::error_1(&"Navbar: window not available".into());
            return;
        };
        let Some(document) = window.document() else {
            web_sys::console::error_1(&"Navbar: document not available".into());
            return;
        };
        let Some(html) = document.document_element() else {
            web_sys::console::error_1(&"Navbar: document element not available".into());
            return;
        };
        let coords = e.client_coordinates();
        let x = coords.x;
        let y = coords.y;
        let width = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
        let height = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
        let end_radius = ((x.max(width - x)).powi(2) + (y.max(height - y)).powi(2)).sqrt();
        let _ = html.set_attribute("style", &format!("--x: {}px; --y: {}px; --r: {}px", x, y, end_radius));
        let supports_transition = js_sys::eval("Boolean(document.startViewTransition)")
            .ok()
            .and_then(|v| v.as_bool())
            .unwrap_or(false);
        if supports_transition {
            let _ = js_sys::eval("document.startViewTransition(() => { document.documentElement.classList.toggle('dark'); })");
        } else {
            let class = html.class_name();
            if class.contains("dark") {
                let _ = html.set_attribute("class", "");
            } else {
                let _ = html.set_attribute("class", "dark");
            }
        }
        is_dark.set(!is_dark());
        storage::set_theme(is_dark());
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
        let window = web_sys::window();
        let document = window.as_ref().and_then(|w| w.document());

        let (Some(window), Some(document)) = (window, document) else { return };

        let trigger_y = std::rc::Rc::new(std::cell::Cell::new(0.0));
        let is_stuck = std::rc::Rc::new(std::cell::Cell::new(false));
        let raf_id = std::rc::Rc::new(std::cell::Cell::new(0));

        // requestAnimationFrame 更新吸顶位置，仅在被 stuck 时执行
        let update_sticky_position = std::rc::Rc::new({
            let raf_id = raf_id.clone();
            let document = document.clone();
            Closure::<dyn FnMut()>::new(move || {
                raf_id.set(0);
                if let (Some(app), Some(nl)) = (
                    document.query_selector(".app").ok().flatten(),
                    document.query_selector(".navbar-links").ok().flatten(),
                ) {
                    let r = app.get_bounding_client_rect();
                    let _ = nl.set_attribute(
                        "style",
                        &format!("position:fixed;top:0.5rem;left:{}px;width:{}px;", r.left(), r.width()),
                    );
                }
            })
        });

        let scroll_handler = {
            let trigger_y = trigger_y.clone();
            let is_stuck = is_stuck.clone();
            let raf_id = raf_id.clone();
            let document = document.clone();
            let window = window.clone();
            let updater = update_sticky_position.clone();

            Closure::<dyn FnMut()>::new(move || {
                let nav_links = match document.query_selector(".navbar-links").ok().flatten() {
                    Some(el) => el,
                    None => return,
                };

                if trigger_y.get() == 0.0 {
                    let rect = nav_links.get_bounding_client_rect();
                    trigger_y.set(rect.top() + window.scroll_y().unwrap_or(0.0));
                }

                let scroll_y = window.scroll_y().unwrap_or(0.0);
                let should_stick = scroll_y >= trigger_y.get();

                if should_stick {
                    if !is_stuck.get() {
                        is_stuck.set(true);
                    }
                    let _ = nav_links.set_attribute("data-stuck", "true");

                    // 用 rAF 节流位置更新
                    if raf_id.get() == 0 {
                        if let Ok(id) = window.request_animation_frame(
                            (&*updater).as_ref().unchecked_ref(),
                        ) {
                            raf_id.set(id);
                        }
                    }
                } else {
                    if is_stuck.get() {
                        if raf_id.get() != 0 {
                            let _ = window.cancel_animation_frame(raf_id.get());
                            raf_id.set(0);
                        }
                        let _ = nav_links.remove_attribute("style");
                        let _ = nav_links.remove_attribute("data-stuck");
                        is_stuck.set(false);
                    }
                }
            })
        };

        let _ = window.add_event_listener_with_callback(
            "scroll",
            scroll_handler.as_ref().unchecked_ref(),
        );

        scroll_handler.forget();
    });

    rsx! {
        nav { class: "navbar-content", aria_label: "主导航",
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
                        aria_label: if is_dark() { "切换到浅色模式" } else { "切换到深色模式" },
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