//! 圆形生成器页面。

mod config;
mod modal;
mod render;

use config::{load_config, save_all_config, PageConfig};
use modal::{ResizeModal, ResultModal};
use render::{render_canvas, RenderConfig};

use crate::components::PageHead;
use crate::utils::circle_generator::{generate_circles, Circle, GenerationConfig};
use dioxus::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;

type AnimRc = std::rc::Rc<std::cell::RefCell<Option<wasm_bindgen::prelude::Closure<dyn FnMut()>>>>;

#[component]
pub fn CircleGenerator() -> Element {
    let cfg = load_config();
    let mut circles = use_signal(Vec::<Circle>::new);
    let mut config_width = use_signal(|| cfg.config_width);
    let mut config_height = use_signal(|| cfg.config_height);
    let mut gap = use_signal(|| cfg.gap);
    let mut min_radius = use_signal(|| cfg.min_radius);
    let mut max_radius = use_signal(|| cfg.max_radius);
    let mut generating = use_signal(|| false);
    let mut selecting = use_signal(|| false);
    let mut highlight_index = use_signal(|| None::<usize>);
    let mut selected_circle = use_signal(|| None::<(usize, Circle)>);
    let mut show_modal = use_signal(|| false);

    let mut timer_handle = use_signal(|| None::<i32>);
    let mut picker_speed = use_signal(|| cfg.picker_speed);
    let mut anim_duration = use_signal(|| cfg.anim_duration);
    let mut dwell_time = use_signal(|| cfg.dwell_time);
    let mut last_picked_idx = use_signal(|| None::<usize>);
    let mut anim_frame_handle = use_signal(|| None::<i32>);
    let mut fullscreen_mode = use_signal(|| false);
    let mut pre_fs_width = use_signal(|| cfg.config_width);
    let mut pre_fs_height = use_signal(|| cfg.config_height);
    let mut panels_hidden = use_signal(|| false);
    let mut resize_pending = use_signal(|| false);
    let mut fs_trigger = use_signal(|| false);

    let mut toggle_fullscreen = move || {
        let entering = !fullscreen_mode();
        if entering {
            pre_fs_width.set(config_width());
            pre_fs_height.set(config_height());
            let window = match web_sys::window() {
                Some(w) => w,
                None => return,
            };
            let ww = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
            let wh = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
            config_width.set(ww);
            config_height.set(wh);
            fullscreen_mode.set(true);
        } else {
            config_width.set(pre_fs_width());
            config_height.set(pre_fs_height());
            fullscreen_mode.set(false);
        }
        fs_trigger.set(!fs_trigger());
    };

    use_effect(move || {
        fs_trigger();
        if !circles.peek().is_empty() {
            generating.set(true);
            let new_w = *config_width.peek();
            let new_h = *config_height.peek();
            let g = *gap.peek();
            let min_r = *min_radius.peek();
            let max_r = *max_radius.peek();
            let hl = *highlight_index.peek();
            let fs = *fullscreen_mode.peek();

            let Some(window) = web_sys::window() else {
                return;
            };
            let window2 = window.clone();
            let closure = Closure::once(move || {
                let closure2 = Closure::once(move || {
                    let cfg = GenerationConfig {
                        width: new_w,
                        height: new_h,
                        gap: g,
                        min_radius: min_r,
                        max_radius: max_r,
                        max_retries: None,
                    };
                    let new_circles = generate_circles(&cfg);
                    render_canvas(&RenderConfig {
                        circles: &new_circles,
                        config_width: new_w,
                        config_height: new_h,
                        fullscreen: fs,
                        highlight: hl,
                        prev_highlight: None,
                        mask_hole: None,
                    });
                    circles.set(new_circles);
                    generating.set(false);
                });
                window2
                    .set_timeout_with_callback_and_timeout_and_arguments_0(closure2.as_ref().unchecked_ref(), 0)
                    .ok();
                closure2.forget();
            });
            window.request_animation_frame(closure.as_ref().unchecked_ref()).ok();
            closure.forget();
        }
    });

    use_effect(move || {
        let fs = fullscreen_mode();
        if fs {
            resize_pending.set(false);
            let Some(window) = web_sys::window() else {
                return;
            };
            let window_for_listener = window.clone();
            let resize_handle = std::rc::Rc::new(std::cell::Cell::new(None::<i32>));
            let resize_handle_clone = resize_handle.clone();
            let closure = Closure::wrap(Box::new(move || {
                if let Some(handle) = resize_handle_clone.get() {
                    window.clear_timeout_with_handle(handle);
                }
                let handle = window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        Closure::once_into_js(move || {
                            resize_pending.set(true);
                        })
                        .as_ref()
                        .unchecked_ref(),
                        400,
                    )
                    .ok();
                resize_handle_clone.set(handle);
            }) as Box<dyn FnMut()>);
            window_for_listener
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())
                .ok();
            closure.forget();
        }
    });

    let mut generate = move || {
        generating.set(true);
        selecting.set(false);
        highlight_index.set(None);
        selected_circle.set(None);
        show_modal.set(false);
        if let Some(handle) = timer_handle() {
            let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
            timer_handle.set(None);
        }
        let w = config_width();
        let h = config_height();
        let g = gap();
        let min_r = min_radius();
        let max_r = max_radius();

        let Some(window) = web_sys::window() else {
            return;
        };
        let window2 = window.clone();
        let closure = Closure::once(move || {
            let closure2 = Closure::once(move || {
                let cfg = GenerationConfig {
                    width: w,
                    height: h,
                    gap: g,
                    min_radius: min_r,
                    max_radius: max_r,
                    max_retries: None,
                };
                let new_circles = generate_circles(&cfg);
                render_canvas(&RenderConfig {
                    circles: &new_circles,
                    config_width: w,
                    config_height: h,
                    fullscreen: false,
                    highlight: None,
                    prev_highlight: None,
                    mask_hole: None,
                });
                circles.set(new_circles);
                generating.set(false);
            });
            window2
                .set_timeout_with_callback_and_timeout_and_arguments_0(closure2.as_ref().unchecked_ref(), 0)
                .ok();
            closure2.forget();
        });
        window.request_animation_frame(closure.as_ref().unchecked_ref()).ok();
        closure.forget();
    };

    let mut toggle_picker = move || {
        if selecting() {
            if let Some(handle) = timer_handle() {
                let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                timer_handle.set(None);
            }
            if let Some(handle) = anim_frame_handle() {
                let _ = web_sys::window().map(|w| w.cancel_animation_frame(handle));
                anim_frame_handle.set(None);
            }
            selecting.set(false);
            let circles_snapshot = circles();
            let final_idx = last_picked_idx().or(highlight_index());
            if let Some(i) = final_idx {
                highlight_index.set(Some(i));
                let w = config_width();
                let h = config_height();
                render_canvas(&RenderConfig {
                    circles: &circles_snapshot,
                    config_width: w,
                    config_height: h,
                    fullscreen: fullscreen_mode(),
                    highlight: Some(i),
                    prev_highlight: None,
                    mask_hole: None,
                });
                if let Some(circle) = circles_snapshot.get(i) {
                    selected_circle.set(Some((i, circle.clone())));
                    show_modal.set(true);
                }
            }
        } else {
            let circles_snapshot = circles();
            if circles_snapshot.is_empty() {
                return;
            }
            selecting.set(true);
            selected_circle.set(None);
            show_modal.set(false);

            let Some(window) = web_sys::window() else {
                return;
            };
            let window_for_interval = window.clone();
            let prev_idx = std::cell::Cell::new(highlight_index());
            let interval_closure = Closure::wrap(Box::new(move || {
                let circles_snapshot = circles();
                if circles_snapshot.is_empty() {
                    return;
                }
                let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                let prev = prev_idx.get();
                prev_idx.set(Some(idx));
                last_picked_idx.set(Some(idx));

                if prev.is_none() {
                    highlight_index.set(Some(idx));
                }

                if let Some(handle) = anim_frame_handle() {
                    let _ = window_for_interval.cancel_animation_frame(handle);
                }

                let from_pos = prev.and_then(|p| circles_snapshot.get(p).map(|c| (c.x, c.y, c.radius)));
                let to_pos = circles_snapshot.get(idx).map(|c| (c.x, c.y, c.radius));

                let w = config_width();
                let h = config_height();
                let document = web_sys::window().and_then(|w| w.document());
                let canvas_el = document.and_then(|d| d.get_element_by_id("circle-canvas"));
                let client_w = canvas_el
                    .and_then(|c| {
                        c.dyn_into::<web_sys::HtmlCanvasElement>()
                            .ok()
                            .map(|el| el.client_width() as f64)
                    })
                    .unwrap_or(800.0);
                let scale = client_w / w;

                if let (Some(from), Some(to)) = (from_pos, to_pos) {
                    let from_sx = from.0 * scale;
                    let from_sy = from.1 * scale;
                    let from_sr = from.2 * scale + 4.0;
                    let to_sx = to.0 * scale;
                    let to_sy = to.1 * scale;
                    let to_sr = to.2 * scale + 4.0;

                    let start_time = js_sys::Date::now();
                    let duration = anim_duration();
                    let window2 = window_for_interval.clone();
                    let circles_clone = circles_snapshot.clone();
                    let target_idx = idx;
                    let prev_idx_val = prev;

                    let anim_rc: AnimRc = std::rc::Rc::new(std::cell::RefCell::new(None));
                    let anim_rc_clone = anim_rc.clone();

                    let frame_closure = Closure::wrap(Box::new(move || {
                        let now = js_sys::Date::now();
                        let elapsed = now - start_time;
                        let t = (elapsed / duration).min(1.0);
                        let eased = 1.0 - (1.0 - t) * (1.0 - t);

                        let mx = from_sx + (to_sx - from_sx) * eased;
                        let my = from_sy + (to_sy - from_sy) * eased;
                        let mr = from_sr + (to_sr - from_sr) * eased;

                        render_canvas(&RenderConfig {
                            circles: &circles_clone,
                            config_width: w,
                            config_height: h,
                            fullscreen: fullscreen_mode(),
                            highlight: None,
                            prev_highlight: None,
                            mask_hole: Some((mx, my, mr)),
                        });

                        if t < 1.0 {
                            let rc = anim_rc_clone.clone();
                            let next = Closure::once(move || {
                                let c = rc.borrow_mut().take();
                                if let Some(c) = c {
                                    let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                    let _ = func.call0(&JsValue::null());
                                    *rc.borrow_mut() = Some(c);
                                }
                            });
                            let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).ok();
                            anim_frame_handle.set(h);
                            next.forget();
                        } else {
                            highlight_index.set(Some(target_idx));
                            render_canvas(&RenderConfig {
                                circles: &circles_clone,
                                config_width: w,
                                config_height: h,
                                fullscreen: fullscreen_mode(),
                                highlight: Some(target_idx),
                                prev_highlight: prev_idx_val,
                                mask_hole: None,
                            });
                            anim_frame_handle.set(None);
                        }
                    }) as Box<dyn FnMut()>);

                    *anim_rc.borrow_mut() = Some(frame_closure);

                    {
                        let borrowed = anim_rc.borrow();
                        if let Some(c) = borrowed.as_ref() {
                            let func: &js_sys::Function = c.as_ref().unchecked_ref();
                            let handle = window_for_interval.request_animation_frame(func).ok();
                            anim_frame_handle.set(handle);
                        }
                    }
                } else {
                    highlight_index.set(Some(idx));
                    render_canvas(&RenderConfig {
                        circles: &circles_snapshot,
                        config_width: w,
                        config_height: h,
                        fullscreen: fullscreen_mode(),
                        highlight: Some(idx),
                        prev_highlight: prev,
                        mask_hole: None,
                    });
                }
            }) as Box<dyn FnMut()>);
            let handle = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    interval_closure.as_ref().unchecked_ref(),
                    (anim_duration() + dwell_time() + picker_speed()) as i32,
                )
                .ok(); // set_interval
            interval_closure.forget();
            timer_handle.set(handle);
        }
    };

    {
        let w = config_width();
        let h = config_height();
        let g = gap();
        let min_r = min_radius();
        let max_r = max_radius();
        use_effect(move || {
            let cfg = GenerationConfig {
                width: w,
                height: h,
                gap: g,
                min_radius: min_r,
                max_radius: max_r,
                max_retries: None,
            };
            let new_circles = generate_circles(&cfg);
            render_canvas(&RenderConfig {
                circles: &new_circles,
                config_width: w,
                config_height: h,
                fullscreen: false,
                highlight: None,
                prev_highlight: None,
                mask_hole: None,
            });
            circles.set(new_circles);
        });
    }

    rsx! {
        PageHead {
            title: "圆形生成器".to_string(),
            description: "圆形排布生成与挑选实验。".to_string(),
            canonical: "/circle-generator".to_string(),
        }
            div {
                id: "circle-generator-page",
                class: "circle-generator-page",
                class: if fullscreen_mode() { "cg-fullscreen-active" },

                button {
                    class: "cg-fullscreen-btn",
                    onclick: move |_| toggle_fullscreen(),
                    if fullscreen_mode() { "\u{2716}" } else { "\u{26F6}" }
                }

                div { class: "cg-canvas-section",
                    div { class: "cg-card cg-canvas-card",
                        div { class: "cg-canvas-wrap",
                            canvas {
                                id: "circle-canvas",
                                style: "width: 100%;"
                            }
                            if generating() {
                                div { class: "cg-canvas-loading",
                                    div { class: "cg-spinner" }
                                    span { "生成中..." }
                                }
                            }
                        }
                    }
                }

                div { class: if fullscreen_mode() { "cg-overlay-content" } else { "" }, class: if panels_hidden() { "cg-panels-hidden" },
                    if fullscreen_mode() {
                        button {
                            class: "cg-toggle-panels-btn",
                            onclick: move |_| {
                                panels_hidden.set(!panels_hidden());
                            },
                            if panels_hidden() { "\u{25A0}" } else { "\u{25A1}" }
                        }
                    }
                    h1 { "圆形生成器" }

                    div { class: "cg-config-section",
                        div { class: "cg-card",
                        div { class: "cg-card-header",
                            span { "重新生成配置" }
                        }
                        div { class: "cg-config-grid",
                            div { class: "cg-config-item",
                                label { r#for: "cg-width", "宽度" }
                                input {
                                    id: "cg-width",
                                    r#type: "number",
                                    min: "200",
                                    step: "50",
                                    value: "{config_width()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            config_width.set(v.max(200.0));
                                            save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                        }
                                    }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-height", "高度" }
                                input {
                                    id: "cg-height",
                                    r#type: "number",
                                    min: "200",
                                    step: "50",
                                    value: "{config_height()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            config_height.set(v.max(200.0));
                                            save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                        }
                                    }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-gap", "间隙" }
                                input {
                                    id: "cg-gap",
                                    r#type: "number",
                                    min: "0",
                                    step: "2",
                                    value: "{gap()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            gap.set(v.max(0.0));
                                            save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                        }
                                    }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-min-radius", "最小半径" }
                                input {
                                    id: "cg-min-radius",
                                    r#type: "number",
                                    min: "5",
                                    step: "5",
                                    value: "{min_radius()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            min_radius.set(v.max(5.0));
                                            save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                        }
                                    }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-max-radius", "最大半径" }
                                input {
                                    id: "cg-max-radius",
                                    r#type: "number",
                                    min: "5",
                                    step: "5",
                                    value: "{max_radius()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let min = min_radius();
                                            max_radius.set(v.max(min + 1.0));
                                            save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                        }
                                    }
                                }
                            }
                        }
                        div { class: "cg-card-actions",
                            button {
                                class: "cg-btn",
                                disabled: generating(),
                                onclick: move |_| generate(),
                                if generating() { "生成中..." } else { "重新生成" }
                            }
                        }
                    }

                    div { class: "cg-card",
                        div { class: "cg-card-header",
                            span { "随机选取配置" }
                        }
                        div { class: "cg-config-grid",
                            div { class: "cg-config-item",
                                label { r#for: "cg-picker-speed", "选取速度" }
                                div { class: "cg-speed-wrap",
                                    input {
                                        id: "cg-picker-speed",
                                        r#type: "range",
                                        min: "20",
                                        max: "1000",
                                        step: "10",
                                        value: "{picker_speed()}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f64>() {
                                                picker_speed.set(v);
                                                if anim_duration() > v {
                                                    anim_duration.set(v.max(10.0));
                                                }
                                                save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                                if selecting() {
                                                    if let Some(handle) = timer_handle() {
                                                        let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                                                        timer_handle.set(None);
                                                    }
                                                    if let Some(handle) = anim_frame_handle() {
                                                        let _ = web_sys::window().map(|w| w.cancel_animation_frame(handle));
                                                        anim_frame_handle.set(None);
                                                    }
                                                    let circles_snapshot = circles();
                                                    let w = config_width();
                                                    let h = config_height();
                                                    render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: last_picked_idx(), prev_highlight: None, mask_hole: None });
                                                    let Some(window) = web_sys::window() else { return; };
                                                    let window_for_interval = window.clone();
                                                    let prev_idx = std::cell::Cell::new(last_picked_idx());
                                                    let interval_closure = Closure::wrap(Box::new(move || {
                                                        let circles_snapshot = circles();
                                                        if circles_snapshot.is_empty() {
                                                            return;
                                                        }
                                                        let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                                                        let prev = prev_idx.get();
                                                        prev_idx.set(Some(idx));
                                                        last_picked_idx.set(Some(idx));

                                                        if prev.is_none() {
                                                            highlight_index.set(Some(idx));
                                                        }

                                                        if let Some(handle) = anim_frame_handle() {
                                                            let _ = window_for_interval.cancel_animation_frame(handle);
                                                        }

                                                        let from_pos = prev.and_then(|p| circles_snapshot.get(p).map(|c| (c.x, c.y, c.radius)));
                                                        let to_pos = circles_snapshot.get(idx).map(|c| (c.x, c.y, c.radius));

                                                        let w = config_width();
                                                        let h = config_height();
                                                        let document = web_sys::window().and_then(|w| w.document());
                                                        let canvas_el = document.and_then(|d| d.get_element_by_id("circle-canvas"));
                                                        let client_w = canvas_el.and_then(|c| {
                                                            c.dyn_into::<web_sys::HtmlCanvasElement>().ok().map(|el| el.client_width() as f64)
                                                        }).unwrap_or(800.0);
                                                        let scale = client_w / w;

                                                        if let (Some(from), Some(to)) = (from_pos, to_pos) {
                                                            let from_sx = from.0 * scale;
                                                            let from_sy = from.1 * scale;
                                                            let from_sr = from.2 * scale + 4.0;
                                                            let to_sx = to.0 * scale;
                                                            let to_sy = to.1 * scale;
                                                            let to_sr = to.2 * scale + 4.0;

                                                            let start_time = js_sys::Date::now();
                                                            let duration = anim_duration();
                                                            let window2 = window_for_interval.clone();
                                                            let circles_clone = circles_snapshot.clone();
                                                            let target_idx = idx;
                                                            let prev_idx_val = prev;

                                                            let anim_rc: AnimRc = std::rc::Rc::new(std::cell::RefCell::new(None));
                                                            let anim_rc_clone = anim_rc.clone();

                                                            let frame_closure = Closure::wrap(Box::new(move || {
                                                                let now = js_sys::Date::now();
                                                                let elapsed = now - start_time;
                                                                let t = (elapsed / duration).min(1.0);
                                                                let eased = 1.0 - (1.0 - t) * (1.0 - t);

                                                                let mx = from_sx + (to_sx - from_sx) * eased;
                                                                let my = from_sy + (to_sy - from_sy) * eased;
                                                                let mr = from_sr + (to_sr - from_sr) * eased;

                                                                render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: None, prev_highlight: None, mask_hole: Some((mx, my, mr)) });

                                                                if t < 1.0 {
                                                                    let rc = anim_rc_clone.clone();
                                                                    let next = Closure::once(move || {
                                                                        let c = rc.borrow_mut().take();
                                                                        if let Some(c) = c {
                                                                            let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                            let _ = func.call0(&JsValue::null());
                                                                            *rc.borrow_mut() = Some(c);
                                                                        }
                                                                    });
                                                                    let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).ok();
                                                                    anim_frame_handle.set(h);
                                                                    next.forget();
                                                                } else {
                                                                    highlight_index.set(Some(target_idx));
                                                                    render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(target_idx), prev_highlight: prev_idx_val, mask_hole: None });
                                                                    anim_frame_handle.set(None);
                                                                }
                                                            }) as Box<dyn FnMut()>);

                                                            *anim_rc.borrow_mut() = Some(frame_closure);

                                                            {
                                                                let borrowed = anim_rc.borrow();
                                                                if let Some(c) = borrowed.as_ref() {
                                                                    let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                    let handle = window_for_interval.request_animation_frame(func).ok();
                                                                    anim_frame_handle.set(handle);
                                                                }
                                                            }
                                                        } else {
                                                            highlight_index.set(Some(idx));
                                                            render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(idx), prev_highlight: prev, mask_hole: None });
                                                        }
                                                    }) as Box<dyn FnMut()>);
                                                    let handle = window
                                                        .set_interval_with_callback_and_timeout_and_arguments_0(
                                                            interval_closure.as_ref().unchecked_ref(),
                                                            (anim_duration() + dwell_time() + picker_speed()) as i32,
                                                        )
                                                        .ok(); // set_interval
                                                    interval_closure.forget();
                                                    timer_handle.set(handle);
                                                }
                                            }
                                        }
                                    }
                                    span { class: "cg-speed-label", "{picker_speed() as u32}ms" }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-dwell-time", "停留时间" }
                                div { class: "cg-speed-wrap",
                                    input {
                                        id: "cg-dwell-time",
                                        r#type: "range",
                                        min: "10",
                                        max: "1000",
                                        step: "10",
                                        value: "{dwell_time()}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f64>() {
                                                dwell_time.set(v);
                                                save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                                if selecting() {
                                                    if let Some(handle) = timer_handle() {
                                                        let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                                                        timer_handle.set(None);
                                                    }
                                                    if let Some(handle) = anim_frame_handle() {
                                                        let _ = web_sys::window().map(|w| w.cancel_animation_frame(handle));
                                                        anim_frame_handle.set(None);
                                                    }
                                                    let circles_snapshot = circles();
                                                    let w = config_width();
                                                    let h = config_height();
                                                    render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: last_picked_idx(), prev_highlight: None, mask_hole: None });
                                                    let Some(window) = web_sys::window() else { return; };
                                                    let window_for_interval = window.clone();
                                                    let prev_idx = std::cell::Cell::new(last_picked_idx());
                                                    let interval_closure = Closure::wrap(Box::new(move || {
                                                        let circles_snapshot = circles();
                                                        if circles_snapshot.is_empty() {
                                                            return;
                                                        }
                                                        let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                                                        let prev = prev_idx.get();
                                                        prev_idx.set(Some(idx));
                                                        last_picked_idx.set(Some(idx));

                                                        if prev.is_none() {
                                                            highlight_index.set(Some(idx));
                                                        }

                                                        if let Some(handle) = anim_frame_handle() {
                                                            let _ = window_for_interval.cancel_animation_frame(handle);
                                                        }

                                                        let from_pos = prev.and_then(|p| circles_snapshot.get(p).map(|c| (c.x, c.y, c.radius)));
                                                        let to_pos = circles_snapshot.get(idx).map(|c| (c.x, c.y, c.radius));

                                                        let w = config_width();
                                                        let h = config_height();
                                                        let document = web_sys::window().and_then(|w| w.document());
                                                        let canvas_el = document.and_then(|d| d.get_element_by_id("circle-canvas"));
                                                        let client_w = canvas_el.and_then(|c| {
                                                            c.dyn_into::<web_sys::HtmlCanvasElement>().ok().map(|el| el.client_width() as f64)
                                                        }).unwrap_or(800.0);
                                                        let scale = client_w / w;

                                                        if let (Some(from), Some(to)) = (from_pos, to_pos) {
                                                            let from_sx = from.0 * scale;
                                                            let from_sy = from.1 * scale;
                                                            let from_sr = from.2 * scale + 4.0;
                                                            let to_sx = to.0 * scale;
                                                            let to_sy = to.1 * scale;
                                                            let to_sr = to.2 * scale + 4.0;

                                                            let start_time = js_sys::Date::now();
                                                            let duration = anim_duration();
                                                            let window2 = window_for_interval.clone();
                                                            let circles_clone = circles_snapshot.clone();
                                                            let target_idx = idx;
                                                            let prev_idx_val = prev;

                                                            let anim_rc: AnimRc = std::rc::Rc::new(std::cell::RefCell::new(None));
                                                            let anim_rc_clone = anim_rc.clone();

                                                            let frame_closure = Closure::wrap(Box::new(move || {
                                                                let now = js_sys::Date::now();
                                                                let elapsed = now - start_time;
                                                                let t = (elapsed / duration).min(1.0);
                                                                let eased = 1.0 - (1.0 - t) * (1.0 - t);

                                                                let mx = from_sx + (to_sx - from_sx) * eased;
                                                                let my = from_sy + (to_sy - from_sy) * eased;
                                                                let mr = from_sr + (to_sr - from_sr) * eased;

                                                                render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: None, prev_highlight: None, mask_hole: Some((mx, my, mr)) });

                                                                if t < 1.0 {
                                                                    let rc = anim_rc_clone.clone();
                                                                    let next = Closure::once(move || {
                                                                        let c = rc.borrow_mut().take();
                                                                        if let Some(c) = c {
                                                                            let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                            let _ = func.call0(&JsValue::null());
                                                                            *rc.borrow_mut() = Some(c);
                                                                        }
                                                                    });
                                                                    let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).ok();
                                                                    anim_frame_handle.set(h);
                                                                    next.forget();
                                                                } else {
                                                                    highlight_index.set(Some(target_idx));
                                                                    render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(target_idx), prev_highlight: prev_idx_val, mask_hole: None });
                                                                    anim_frame_handle.set(None);
                                                                }
                                                            }) as Box<dyn FnMut()>);

                                                            *anim_rc.borrow_mut() = Some(frame_closure);

                                                            {
                                                                let borrowed = anim_rc.borrow();
                                                                if let Some(c) = borrowed.as_ref() {
                                                                    let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                    let handle = window_for_interval.request_animation_frame(func).ok();
                                                                    anim_frame_handle.set(handle);
                                                                }
                                                            }
                                                        } else {
                                                            highlight_index.set(Some(idx));
                                                            render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(idx), prev_highlight: prev, mask_hole: None });
                                                        }
                                                    }) as Box<dyn FnMut()>);
                                                    let handle = window
                                                        .set_interval_with_callback_and_timeout_and_arguments_0(
                                                            interval_closure.as_ref().unchecked_ref(),
                                                            (anim_duration() + dwell_time() + picker_speed()) as i32,
                                                        )
                                                        .ok(); // set_interval
                                                    interval_closure.forget();
                                                    timer_handle.set(handle);
                                                }
                                            }
                                        }
                                    }
                                    span { class: "cg-speed-label", "{dwell_time() as u32}ms" }
                                }
                            }
                            div { class: "cg-config-item",
                                label { r#for: "cg-anim-duration", "动画时长" }
                                div { class: "cg-speed-wrap",
                                    input {
                                        id: "cg-anim-duration",
                                        r#type: "range",
                                        min: "10",
                                        max: "1000",
                                        step: "10",
                                        value: "{anim_duration()}",
                                        oninput: move |e| {
                                            if let Ok(v) = e.value().parse::<f64>() {
                                                let clamped = v.clamp(10.0, 1000.0);
                                                anim_duration.set(clamped);
                                                if clamped > picker_speed() {
                                                    picker_speed.set(clamped);
                                                }
                                                save_all_config(&PageConfig { config_width: config_width(), config_height: config_height(), gap: gap(), min_radius: min_radius(), max_radius: max_radius(), picker_speed: picker_speed(), anim_duration: anim_duration(), dwell_time: dwell_time() });
                                                if selecting() {
                                                    if let Some(handle) = timer_handle() {
                                                        let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                                                        timer_handle.set(None);
                                                    }
                                                    if let Some(handle) = anim_frame_handle() {
                                                        let _ = web_sys::window().map(|w| w.cancel_animation_frame(handle));
                                                        anim_frame_handle.set(None);
                                                    }
                                                    let circles_snapshot = circles();
                                                    let w = config_width();
                                                    let h = config_height();
                                                    render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: last_picked_idx(), prev_highlight: None, mask_hole: None });
                                                    let Some(window) = web_sys::window() else { return; };
                                                    let window_for_interval = window.clone();
                                                    let prev_idx = std::cell::Cell::new(last_picked_idx());
                                                    let interval_closure = Closure::wrap(Box::new(move || {
                                                        let circles_snapshot = circles();
                                                        if circles_snapshot.is_empty() {
                                                            return;
                                                        }
                                                        let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                                                        let prev = prev_idx.get();
                                                        prev_idx.set(Some(idx));
                                                        last_picked_idx.set(Some(idx));

                                                        if prev.is_none() {
                                                            highlight_index.set(Some(idx));
                                                        }

                                                        if let Some(handle) = anim_frame_handle() {
                                                            let _ = window_for_interval.cancel_animation_frame(handle);
                                                        }

                                                        let from_pos = prev.and_then(|p| circles_snapshot.get(p).map(|c| (c.x, c.y, c.radius)));
                                                        let to_pos = circles_snapshot.get(idx).map(|c| (c.x, c.y, c.radius));

                                                        let w = config_width();
                                                        let h = config_height();
                                                        let document = web_sys::window().and_then(|w| w.document());
                                                        let canvas_el = document.and_then(|d| d.get_element_by_id("circle-canvas"));
                                                        let client_w = canvas_el.and_then(|c| {
                                                            c.dyn_into::<web_sys::HtmlCanvasElement>().ok().map(|el| el.client_width() as f64)
                                                        }).unwrap_or(800.0);
                                                        let scale = client_w / w;

                                                        if let (Some(from), Some(to)) = (from_pos, to_pos) {
                                                            let from_sx = from.0 * scale;
                                                            let from_sy = from.1 * scale;
                                                            let from_sr = from.2 * scale + 4.0;
                                                            let to_sx = to.0 * scale;
                                                            let to_sy = to.1 * scale;
                                                            let to_sr = to.2 * scale + 4.0;

                                                            let start_time = js_sys::Date::now();
                                                            let duration = anim_duration();
                                                            let window2 = window_for_interval.clone();
                                                            let circles_clone = circles_snapshot.clone();
                                                            let target_idx = idx;
                                                            let prev_idx_val = prev;

                                                            let anim_rc: AnimRc = std::rc::Rc::new(std::cell::RefCell::new(None));
                                                            let anim_rc_clone = anim_rc.clone();

                                                            let frame_closure = Closure::wrap(Box::new(move || {
                                                                let now = js_sys::Date::now();
                                                                let elapsed = now - start_time;
                                                                let t = (elapsed / duration).min(1.0);
                                                                let eased = 1.0 - (1.0 - t) * (1.0 - t);

                                                                let mx = from_sx + (to_sx - from_sx) * eased;
                                                                let my = from_sy + (to_sy - from_sy) * eased;
                                                                let mr = from_sr + (to_sr - from_sr) * eased;

                                                                render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: None, prev_highlight: None, mask_hole: Some((mx, my, mr)) });

                                                                if t < 1.0 {
                                                                    let rc = anim_rc_clone.clone();
                                                                    let next = Closure::once(move || {
                                                                        let c = rc.borrow_mut().take();
                                                                        if let Some(c) = c {
                                                                            let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                            let _ = func.call0(&JsValue::null());
                                                                            *rc.borrow_mut() = Some(c);
                                                                        }
                                                                    });
                                                                    let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).ok();
                                                                    anim_frame_handle.set(h);
                                                                    next.forget();
                                                                } else {
                                                                    highlight_index.set(Some(target_idx));
                                                                    render_canvas(&RenderConfig { circles: &circles_clone, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(target_idx), prev_highlight: prev_idx_val, mask_hole: None });
                                                                    anim_frame_handle.set(None);
                                                                }
                                                            }) as Box<dyn FnMut()>);

                                                            *anim_rc.borrow_mut() = Some(frame_closure);

                                                            {
                                                                let borrowed = anim_rc.borrow();
                                                                if let Some(c) = borrowed.as_ref() {
                                                                    let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                    let handle = window_for_interval.request_animation_frame(func).ok();
                                                                    anim_frame_handle.set(handle);
                                                                }
                                                            }
                                                        } else {
                                                            highlight_index.set(Some(idx));
                                                            render_canvas(&RenderConfig { circles: &circles_snapshot, config_width: w, config_height: h, fullscreen: fullscreen_mode(), highlight: Some(idx), prev_highlight: prev, mask_hole: None });
                                                        }
                                                    }) as Box<dyn FnMut()>);
                                                    let handle = window
                                                        .set_interval_with_callback_and_timeout_and_arguments_0(
                                                            interval_closure.as_ref().unchecked_ref(),
                                                            (anim_duration() + dwell_time() + picker_speed()) as i32,
                                                        )
                                                        .ok(); // set_interval
                                                    interval_closure.forget();
                                                    timer_handle.set(handle);
                                                }
                                            }
                                        }
                                    }
                                    span { class: "cg-speed-label", "{anim_duration() as u32}ms" }
                                }
                            }
                            div { class: "cg-config-item",
                                label { "总周期" }
                                span { class: "cg-speed-label cg-cycle-label", "{((anim_duration() + dwell_time() + picker_speed()) as u32)}ms" }
                            }
                        }
                        div { class: "cg-card-actions",
                            button {
                                class: "cg-btn cg-btn-picker",
                                disabled: generating() || circles().is_empty(),
                                onclick: move |_| toggle_picker(),
                                if selecting() { "停止选取" } else { "随机选取" }
                            }
                        }
                    }
                }

                if !fullscreen_mode() {
                    div { class: "cg-data-section",
                    div { class: "cg-card",
                        div { class: "cg-data-header",
                            span { "圆形数据" }
                            span { class: "cg-count",
                                "共 " span { "{circles().len()}" } " 个圆"
                            }
                        }
                        div { class: "cg-table-wrap",
                            table {
                                thead {
                                    tr {
                                        th { "#" }
                                        th { "X" }
                                        th { "Y" }
                                        th { "半径" }
                                    }
                                }
                                tbody {
                                    {circles().iter().enumerate().map(|(i, c)| {
                                        rsx! {
                                            tr {
                                                td { "{i + 1}" }
                                                td { "{c.x:.1}" }
                                                td { "{c.y:.1}" }
                                                td { "{c.radius:.1}" }
                                            }
                                        }
                                    })}
                                }
                            }
                        }
                    }
                }
                }

                if show_modal() {
                    ResultModal {
                        selected: selected_circle(),
                        on_close: move |_| show_modal.set(false),
                    }
                }

                if resize_pending() {
                    ResizeModal {
                        on_close: move |_| resize_pending.set(false),
                        on_regenerate: move |_| {
                            resize_pending.set(false);
                            let Some(window) = web_sys::window() else { return; };
                            let ww = window.inner_width().ok().and_then(|w| w.as_f64()).unwrap_or(0.0);
                            let wh = window.inner_height().ok().and_then(|h| h.as_f64()).unwrap_or(0.0);
                            config_width.set(ww);
                            config_height.set(wh);
                            generate();
                        },
                    }
                }
            }
        }
    }
}
