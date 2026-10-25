use crate::components::icons::RssIcon;
use crate::routes::Route;
use dioxus::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

// 子菜单状态放在全局：document 点击监听器可能比 Navbar 组件活得更久
// （例如从 TestLayout 跳到 Layout 时），不能捕获组件作用域的 Signal。
static SUB_MENU_OPEN: GlobalSignal<bool> = Signal::global(|| false);
static SUB_MENU_LEFT: GlobalSignal<f64> = Signal::global(|| 0.0);

/// 桌面端鼠标拖拽导航条滚动（触摸端保留原生滚动）。
const NAVBAR_DRAG_JS: &str = include_str!("../utils/navbar_drag.js");

type ClickListener = (web_sys::Document, Closure<dyn FnMut(web_sys::Event)>);
type ScrollListener = (web_sys::Window, Closure<dyn FnMut()>);
type KeyListener = (web_sys::Document, Closure<dyn FnMut(web_sys::KeyboardEvent)>);

fn close_submenu() {
    *SUB_MENU_OPEN.write() = false;
}

/// 「开发」分区包含开发首页及其二级菜单页面。
fn is_dev_section(route: &Route) -> bool {
    match route {
        Route::Dev | Route::CircleGenerator => true,
        #[cfg(feature = "dev-pages")]
        Route::Playground | Route::Test => true,
        _ => false,
    }
}

/// 打开/关闭「开发」二级菜单；桌面端按触发按钮位置定位，移动端由 CSS 改成底部面板。
fn toggle_submenu() {
    let will_open = !SUB_MENU_OPEN();
    if will_open {
        if let Some(window) = web_sys::window() {
            let Some(document) = window.document() else {
                return;
            };
            let item = document.query_selector(".nav-item-with-sub").ok().flatten();
            let wrap = document.query_selector(".navbar-sticky-wrap").ok().flatten();
            if let (Some(item), Some(wrap)) = (item, wrap) {
                let item_rect = item.get_bounding_client_rect();
                let wrap_rect = wrap.get_bounding_client_rect();
                let is_mobile = window.inner_width().ok().and_then(|width| width.as_f64()).unwrap_or(1024.0) <= 768.0;
                let panel_width = if is_mobile { 220.0 } else { 190.0 };
                let item_left = item_rect.left() - wrap_rect.left();
                let item_right = item_rect.right() - wrap_rect.left();
                // 右侧放不下时改为右对齐触发项，避免贴边或溢出
                let left = if item_left + panel_width <= wrap_rect.width() - 8.0 {
                    item_left.max(8.0)
                } else {
                    (item_right - panel_width).max(8.0)
                };
                *SUB_MENU_LEFT.write() = left;
            }
        }
    }
    *SUB_MENU_OPEN.write() = will_open;
}

#[component]
pub fn Navbar() -> Element {
    // 顶部导航只放公共链接；操场/测试等开发页收进「开发」二级菜单
    let nav_items = crate::nav::PUBLIC_NAV_LINKS;

    let route = use_route::<Route>();

    // 监听器在组件卸载时显式移除，避免跨布局导航后继续停留在 document/window 上。
    let click_listener: Rc<RefCell<Option<ClickListener>>> = use_hook(|| Rc::new(RefCell::new(None)));
    let scroll_listener: Rc<RefCell<Option<ScrollListener>>> = use_hook(|| Rc::new(RefCell::new(None)));
    let key_listener: Rc<RefCell<Option<KeyListener>>> = use_hook(|| Rc::new(RefCell::new(None)));
    {
        let click_listener = click_listener.clone();
        let scroll_listener = scroll_listener.clone();
        let key_listener = key_listener.clone();
        use_drop(move || {
            if let Some((document, closure)) = click_listener.borrow_mut().take() {
                let _ = document.remove_event_listener_with_callback("click", closure.as_ref().unchecked_ref());
            }
            if let Some((window, closure)) = scroll_listener.borrow_mut().take() {
                let _ = window.remove_event_listener_with_callback("scroll", closure.as_ref().unchecked_ref());
            }
            if let Some((document, closure)) = key_listener.borrow_mut().take() {
                let _ = document.remove_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref());
            }
        });
    }

    // 点击空白处关闭子菜单（只安装一次）
    use_effect(move || {
        if click_listener.borrow().is_some() {
            return;
        }
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let closure = Closure::<dyn FnMut(_)>::new(move |e: web_sys::Event| {
            if !SUB_MENU_OPEN() {
                return;
            }
            let target = e.target().unwrap();
            let el = target.dyn_ref::<web_sys::Element>();
            if let Some(el) = el {
                if el.closest(".nav-sub-trigger").ok().flatten().is_none()
                    && el.closest(".nav-submenu-popup").ok().flatten().is_none()
                {
                    close_submenu();
                }
            }
        });
        document
            .add_event_listener_with_callback("click", closure.as_ref().unchecked_ref())
            .unwrap();
        *click_listener.borrow_mut() = Some((document.clone(), closure));

        let key_closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::KeyboardEvent| {
            if event.key() == "Escape" {
                close_submenu();
            }
        });
        document
            .add_event_listener_with_callback("keydown", key_closure.as_ref().unchecked_ref())
            .unwrap();
        *key_listener.borrow_mut() = Some((document, key_closure));
    });

    let is_active = move |href: &str| match href {
        "/" => matches!(route, Route::Home | Route::BlogPostView { slug: _ }),
        "/about" => matches!(route, Route::About),
        "/tags" => matches!(route, Route::Tags),
        "/search" => matches!(route, Route::Search),
        "/knowledge-graph" => matches!(route, Route::KnowledgeGraphView),
        "/ai-summary" => matches!(route, Route::AiSummaryView),
        "/dev" => is_dev_section(&route),
        "/rss" => matches!(route, Route::Rss),
        _ => false,
    };

    use_effect(move || {
        let window = web_sys::window().expect("Failed to get window");
        let document = window.document().expect("Failed to get document");

        // 绑定鼠标拖拽（脚本内部会等待 .navbar-links 出现，且幂等）
        let _ = js_sys::eval(NAVBAR_DRAG_JS);

        if scroll_listener.borrow().is_some() {
            return;
        }

        let window_for_closure = window.clone();
        let was_stuck = std::cell::Cell::new(false);
        let scroll_closure = Closure::<dyn FnMut()>::new(move || {
            if SUB_MENU_OPEN() {
                close_submenu();
            }
            if let Some(wrap) = document.query_selector(".navbar-sticky-wrap").ok().flatten() {
                let rect = wrap.get_bounding_client_rect();
                let scroll_top = window_for_closure.scroll_y().unwrap_or(0.0);
                let is_stuck = scroll_top > 10.0 && rect.top() <= 10.0;

                if is_stuck && !was_stuck.get() {
                    // Entering sticky: freeze width at content size, then let CSS animate to 100%
                    if let Some(links) = document.query_selector(".navbar-links").ok().flatten() {
                        let w = links.client_width();
                        let _ = links.set_attribute("style", &format!("width:{}px", w));
                    }
                    let _ = wrap.set_attribute("data-stuck", "true");
                    // Next tick: remove inline style → [data-stuck] width:100% takes over, transition kicks in
                    let doc = document.clone();
                    let timeout = Closure::once(move || {
                        if let Some(l) = doc.query_selector(".navbar-links").ok().flatten() {
                            let _ = l.remove_attribute("style");
                        }
                    });
                    let _ = window_for_closure
                        .set_timeout_with_callback_and_timeout_and_arguments_0(timeout.as_ref().unchecked_ref(), 0);
                    timeout.forget();
                    was_stuck.set(true);
                } else if !is_stuck && was_stuck.get() {
                    let _ = wrap.remove_attribute("data-stuck");
                    if let Some(links) = document.query_selector(".navbar-links").ok().flatten() {
                        let _ = links.remove_attribute("style");
                    }
                    was_stuck.set(false);
                }
            }
        });

        let _ = window.add_event_listener_with_callback("scroll", scroll_closure.as_ref().unchecked_ref());
        *scroll_listener.borrow_mut() = Some((window, scroll_closure));
    });

    rsx! {
        div { class: "navbar-content",
            div { class: "navbar-ui",
                div { class: "navbar-title-wrapper",
                    h1 { class: "navbar-title", "干徒" }
                    div { class: "navbar-glow" }
                }
                div { class: "navbar-subtitle", "这很酷" }
            }
        }
        div { class: "navbar-sticky-wrap",
            div { class: "navbar-links",
                {nav_items.iter().map(|(href, label)| {
                    let active = is_active(href);
                    if *href == "/dev" {
                        rsx! {
                            div { class: "nav-item-with-sub",
                                Link {
                                    to: *href,
                                    class: if active { "nav-link nav-active" } else { "nav-link" },
                                    { *label }
                                }
                                button {
                                    class: if active {
                                        "nav-sub-trigger nav-active"
                                    } else {
                                        "nav-sub-trigger"
                                    },
                                    r#type: "button",
                                    aria_label: "打开开发菜单",
                                    aria_haspopup: "menu",
                                    aria_expanded: if SUB_MENU_OPEN() { "true" } else { "false" },
                                    aria_controls: "nav-submenu",
                                    onclick: move |e: Event<MouseData>| {
                                        e.stop_propagation();
                                        toggle_submenu();
                                    },
                                    span { class: "nav-sub-trigger-label", { *label } }
                                    svg {
                                        width: "10",
                                        height: "10",
                                        view_box: "0 0 10 10",
                                        fill: "none",
                                        stroke: "currentColor",
                                        stroke_width: "2",
                                        stroke_linecap: "round",
                                        stroke_linejoin: "round",
                                        path { d: "M2 3.5 L5 6.5 L8 3.5" }
                                    }
                                }
                            }
                        }
                    } else {
                        rsx! {
                            Link {
                                to: *href,
                                class: if active { "nav-link nav-active" } else { "nav-link" },
                                { *label }
                            }
                        }
                    }
                })}
                Link {
                    class: if is_active("/rss") {
                        "nav-icon-btn rss-toggle active"
                    } else {
                        "nav-icon-btn rss-toggle"
                    },
                    title: "RSS 订阅",
                    to: Route::Rss,
                    RssIcon {}
                }
            }
            if SUB_MENU_OPEN() {
                NavSubmenu { left: SUB_MENU_LEFT() }
            }
        }
    }
}

fn submenu_link(route: Route, title: &'static str, description: &'static str, extra_class: &'static str) -> Element {
    rsx! {
        Link {
            to: route,
            class: "nav-sub-link {extra_class}",
            role: "menuitem",
            onclick: move |_| close_submenu(),
            span { class: "nav-sub-link-title", {title} }
            span { class: "nav-sub-link-desc", {description} }
        }
    }
}

#[component]
fn NavSubmenu(left: f64) -> Element {
    // 桌面端一级「开发」可点击，移动端一级项是菜单按钮，因此仅移动端保留「开发首页」
    let mut items = vec![submenu_link(Route::Dev, "开发首页", "开发工具与实验页面", "nav-sub-link-dev-home")];

    #[cfg(feature = "dev-pages")]
    {
        items.push(submenu_link(Route::Playground, "操场", "交互实验与示例", ""));
        items.push(submenu_link(Route::Test, "测试", "组件与页面测试", ""));
    }

    items.push(submenu_link(Route::CircleGenerator, "圆形生成器", "参数化生成圆形图案", ""));

    rsx! {
        div {
            id: "nav-submenu",
            class: "nav-submenu-popup",
            role: "menu",
            aria_label: "开发工具",
            style: format!("--submenu-left:{left}px"),
            div { class: "nav-submenu-list",
                {items.into_iter()}
            }
        }
    }
}
