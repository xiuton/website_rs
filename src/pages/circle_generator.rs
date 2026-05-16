use dioxus::prelude::*;
use wasm_bindgen::prelude::Closure;
use wasm_bindgen::JsCast;
use wasm_bindgen::JsValue;
use crate::utils::circle_generator::{generate_circles, Circle, GenerationConfig};
use crate::utils::title;

fn hsl_color(index: usize, total: usize) -> String {
    let hue = (index as f64 / total.max(1) as f64) * 360.0;
    format!("hsl({}, 65%, 60%)", hue as u32)
}

fn render_canvas(circles: &[Circle], config_width: f64, config_height: f64, highlight: Option<usize>, prev_highlight: Option<usize>, mask_hole: Option<(f64, f64, f64)>) {
    let window = web_sys::window().expect("Failed to get window");
    let document = window.document().expect("Failed to get document");

    let canvas = document
        .get_element_by_id("circle-canvas")
        .expect("Failed to get canvas element")
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .expect("Failed to cast to HtmlCanvasElement");

    let dpr = window.device_pixel_ratio();

    let display_width = canvas.client_width() as f64;
    let aspect_ratio = config_height / config_width;
    let display_height = display_width * aspect_ratio;

    canvas.set_width((display_width * dpr) as u32);
    canvas.set_height((display_height * dpr) as u32);

    let ctx = canvas
        .get_context("2d")
        .expect("Failed to get 2d context")
        .expect("Context is null")
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .expect("Failed to cast to CanvasRenderingContext2d");

    ctx.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0)
        .expect("Failed to set transform");
    ctx.clear_rect(0.0, 0.0, display_width, display_height);

    ctx.set_fill_style_str("#f8f9fa");
    ctx.fill_rect(0.0, 0.0, display_width, display_height);

    let scale_x = display_width / config_width;
    let scale_y = display_height / config_height;

    for (i, circle) in circles.iter().enumerate() {
        let color = hsl_color(i, circles.len());

        let cx = circle.x * scale_x;
        let cy = circle.y * scale_y;
        let cr = circle.radius * scale_x;

        ctx.begin_path();
        ctx.arc(cx, cy, cr, 0.0, std::f64::consts::PI * 2.0)
            .expect("Failed to create arc");
        ctx.set_fill_style_str(&color);
        ctx.set_global_alpha(0.5);
        ctx.fill();
        ctx.set_global_alpha(1.0);
        ctx.set_stroke_style_str(&color);
        ctx.set_line_width(2.0);
        ctx.stroke();

        ctx.set_fill_style_str("#333");
        ctx.set_font("11px sans-serif");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(&format!("{}", i + 1), cx, cy);
    }

    if let (Some(prev), Some(cur)) = (prev_highlight, highlight) {
        if prev != cur {
            if let (Some(pc), Some(cc)) = (circles.get(prev), circles.get(cur)) {
                let px = pc.x * scale_x;
                let py = pc.y * scale_y;
                let cx2 = cc.x * scale_x;
                let cy2 = cc.y * scale_y;

                ctx.save();
                ctx.set_stroke_style_str("#ffd700");
                ctx.set_line_width(2.5);
                ctx.set_global_alpha(0.5);
                ctx.set_shadow_blur(12.0);
                ctx.set_shadow_color("#ffd700");

                let dash = js_sys::Array::new();
                dash.push(&wasm_bindgen::JsValue::from_f64(6.0));
                dash.push(&wasm_bindgen::JsValue::from_f64(4.0));
                ctx.set_line_dash(&dash).expect("Failed to set line dash");

                ctx.begin_path();
                ctx.move_to(px, py);
                ctx.line_to(cx2, cy2);
                ctx.stroke();

                ctx.set_shadow_blur(0.0);
                ctx.set_global_alpha(1.0);
                ctx.set_line_dash(&js_sys::Array::new()).expect("Failed to clear line dash");
                ctx.restore();
            }
        }
    }

    if let Some(idx) = highlight {
        if let Some(circle) = circles.get(idx) {
            let cx = circle.x * scale_x;
            let cy = circle.y * scale_y;
            let cr = circle.radius * scale_x + 4.0;

            let (hole_cx, hole_cy, hole_cr) = mask_hole.unwrap_or((cx, cy, cr));

            ctx.save();
            ctx.begin_path();
            ctx.rect(0.0, 0.0, display_width, display_height);
            let _ = ctx.arc_with_anticlockwise(hole_cx, hole_cy, hole_cr, 0.0, std::f64::consts::PI * 2.0, true);
            ctx.clip();

            ctx.set_fill_style_str("rgba(0,0,0,0.35)");
            ctx.fill_rect(0.0, 0.0, display_width, display_height);
            ctx.restore();

            ctx.begin_path();
            ctx.arc(cx, cy, cr, 0.0, std::f64::consts::PI * 2.0)
                .expect("Failed to create arc");
            ctx.set_fill_style_str("#ffd700");
            ctx.set_global_alpha(0.9);
            ctx.fill();
            ctx.set_global_alpha(1.0);
            ctx.set_stroke_style_str("#ff8c00");
            ctx.set_line_width(4.0);
            ctx.stroke();

            ctx.set_fill_style_str("#000");
            ctx.set_font("bold 14px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(&format!("{}", idx + 1), cx, cy);
        }
    }
}

#[component]
pub fn CircleGenerator() -> Element {
    use_effect(move || {
        title::set_page_title("圆形生成器 - 干徒");
        ()
    });

    let mut circles = use_signal(|| Vec::<Circle>::new());
    let mut config_width = use_signal(|| 800.0);
    let mut config_height = use_signal(|| 600.0);
    let mut gap = use_signal(|| 10.0);
    let mut min_radius = use_signal(|| 10.0);
    let mut max_radius = use_signal(|| 60.0);
    let mut generating = use_signal(|| false);
    let mut selecting = use_signal(|| false);
    let mut highlight_index = use_signal(|| None::<usize>);
    let mut selected_circle = use_signal(|| None::<(usize, Circle)>);
    let mut show_modal = use_signal(|| false);

    let mut timer_handle = use_signal(|| None::<i32>);
    let mut picker_speed = use_signal(|| 80.0);
    let mut anim_duration = use_signal(|| 60.0);

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

        let window = web_sys::window().expect("Failed to get window");
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
                render_canvas(&new_circles, w, h, None, None, None);
                circles.set(new_circles);
                generating.set(false);
            });
            window2
                .set_timeout_with_callback_and_timeout_and_arguments_0(
                    closure2.as_ref().unchecked_ref(),
                    0,
                )
                .expect("Failed to set timeout");
            closure2.forget();
        });
        window
            .request_animation_frame(closure.as_ref().unchecked_ref())
            .expect("Failed to request animation frame");
        closure.forget();
    };

    let mut toggle_picker = move || {
        if selecting() {
            if let Some(handle) = timer_handle() {
                let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                timer_handle.set(None);
            }
            selecting.set(false);
            let circles_snapshot = circles();
            let idx = highlight_index();
            if let Some(i) = idx {
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
            highlight_index.set(None);
            selected_circle.set(None);
            show_modal.set(false);

            let window = web_sys::window().expect("Failed to get window");
            let window_for_interval = window.clone();
            let prev_idx = std::cell::Cell::new(None::<usize>);
            let anim_handle = std::rc::Rc::new(std::cell::Cell::new(None::<i32>));
            let interval_closure = Closure::wrap(Box::new(move || {
                let circles_snapshot = circles();
                if circles_snapshot.is_empty() {
                    return;
                }
                let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                let prev = prev_idx.get();
                prev_idx.set(Some(idx));

                if prev.is_none() {
                    highlight_index.set(Some(idx));
                }

                if let Some(handle) = anim_handle.get() {
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
                    let anim_handle_for_frame = anim_handle.clone();
                    let target_idx = idx;
                    let prev_idx_val = prev;

                    let anim_rc: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
                    let anim_rc_clone = anim_rc.clone();

                    let frame_closure = Closure::wrap(Box::new(move || {
                        let now = js_sys::Date::now();
                        let elapsed = now - start_time;
                        let t = (elapsed / duration).min(1.0);
                        let eased = 1.0 - (1.0 - t) * (1.0 - t);

                        let mx = from_sx + (to_sx - from_sx) * eased;
                        let my = from_sy + (to_sy - from_sy) * eased;
                        let mr = from_sr + (to_sr - from_sr) * eased;

                        render_canvas(&circles_clone, w, h, prev_idx_val, None, Some((mx, my, mr)));

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
                            let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).expect("Failed to request animation frame");
                            anim_handle_for_frame.set(Some(h));
                            next.forget();
                        } else {
                            highlight_index.set(Some(target_idx));
                            render_canvas(&circles_clone, w, h, Some(target_idx), prev_idx_val, None);
                            anim_handle_for_frame.set(None);
                        }
                    }) as Box<dyn FnMut()>);

                    *anim_rc.borrow_mut() = Some(frame_closure);

                    {
                        let borrowed = anim_rc.borrow();
                        if let Some(c) = borrowed.as_ref() {
                            let func: &js_sys::Function = c.as_ref().unchecked_ref();
                            let handle = window_for_interval.request_animation_frame(func).expect("Failed to request animation frame");
                            anim_handle.set(Some(handle));
                        }
                    }
                } else {
                    highlight_index.set(Some(idx));
                    render_canvas(&circles_snapshot, w, h, Some(idx), prev, None);
                }
            }) as Box<dyn FnMut()>);
            let handle = window
                .set_interval_with_callback_and_timeout_and_arguments_0(
                    interval_closure.as_ref().unchecked_ref(),
                    picker_speed() as i32,
                )
                .expect("Failed to set interval");
            interval_closure.forget();
            timer_handle.set(Some(handle));
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
            render_canvas(&new_circles, w, h, None, None, None);
            circles.set(new_circles);
            ()
        });
    }

    rsx! {
        div { class: "circle-generator-page",
            h1 { "圆形生成器" }

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

            div { class: "cg-config-section",
                div { class: "cg-card",
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
                                    }
                                }
                            }
                        }
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
                                                anim_duration.set(v);
                                            }
                                            if selecting() {
                                                if let Some(handle) = timer_handle() {
                                                    let _ = web_sys::window().map(|w| w.clear_interval_with_handle(handle));
                                                    timer_handle.set(None);
                                                }
                                                let window = web_sys::window().expect("Failed to get window");
                                                let window_for_interval = window.clone();
                                                let prev_idx = std::cell::Cell::new(None::<usize>);
                                                let anim_handle = std::rc::Rc::new(std::cell::Cell::new(None::<i32>));
                                                let interval_closure = Closure::wrap(Box::new(move || {
                                                    let circles_snapshot = circles();
                                                    if circles_snapshot.is_empty() {
                                                        return;
                                                    }
                                                    let idx = (js_sys::Math::random() * circles_snapshot.len() as f64) as usize;
                                                    let prev = prev_idx.get();
                                                    prev_idx.set(Some(idx));

                                                    if prev.is_none() {
                                                        highlight_index.set(Some(idx));
                                                    }

                                                    if let Some(handle) = anim_handle.get() {
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
                                                        let anim_handle_for_frame = anim_handle.clone();
                                                        let target_idx = idx;
                                                        let prev_idx_val = prev;

                                                        let anim_rc: std::rc::Rc<std::cell::RefCell<Option<Closure<dyn FnMut()>>>> = std::rc::Rc::new(std::cell::RefCell::new(None));
                                                        let anim_rc_clone = anim_rc.clone();

                                                        let frame_closure = Closure::wrap(Box::new(move || {
                                                            let now = js_sys::Date::now();
                                                            let elapsed = now - start_time;
                                                            let t = (elapsed / duration).min(1.0);
                                                            let eased = 1.0 - (1.0 - t) * (1.0 - t);

                                                            let mx = from_sx + (to_sx - from_sx) * eased;
                                                            let my = from_sy + (to_sy - from_sy) * eased;
                                                            let mr = from_sr + (to_sr - from_sr) * eased;

                                                            render_canvas(&circles_clone, w, h, prev_idx_val, None, Some((mx, my, mr)));

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
                                                                let h = window2.request_animation_frame(next.as_ref().unchecked_ref()).expect("Failed to request animation frame");
                                                                anim_handle_for_frame.set(Some(h));
                                                                next.forget();
                                                            } else {
                                                                highlight_index.set(Some(target_idx));
                                                                render_canvas(&circles_clone, w, h, Some(target_idx), prev_idx_val, None);
                                                                anim_handle_for_frame.set(None);
                                                            }
                                                        }) as Box<dyn FnMut()>);

                                                        *anim_rc.borrow_mut() = Some(frame_closure);

                                                        {
                                                            let borrowed = anim_rc.borrow();
                                                            if let Some(c) = borrowed.as_ref() {
                                                                let func: &js_sys::Function = c.as_ref().unchecked_ref();
                                                                let handle = window_for_interval.request_animation_frame(func).expect("Failed to request animation frame");
                                                                anim_handle.set(Some(handle));
                                                            }
                                                        }
                                                    } else {
                                                        highlight_index.set(Some(idx));
                                                        render_canvas(&circles_snapshot, w, h, Some(idx), prev, None);
                                                    }
                                                }) as Box<dyn FnMut()>);
                                                let handle = window
                                                    .set_interval_with_callback_and_timeout_and_arguments_0(
                                                        interval_closure.as_ref().unchecked_ref(),
                                                        v as i32,
                                                    )
                                                    .expect("Failed to set interval");
                                                interval_closure.forget();
                                                timer_handle.set(Some(handle));
                                            }
                                        }
                                    }
                                }
                                span { class: "cg-speed-label", "{picker_speed() as u32}ms" }
                            }
                        }
                        div { class: "cg-config-item",
                            label { r#for: "cg-anim-duration", "动画时长" }
                            div { class: "cg-speed-wrap",
                                input {
                                    id: "cg-anim-duration",
                                    r#type: "range",
                                    min: "10",
                                    max: "{picker_speed()}",
                                    step: "10",
                                    value: "{anim_duration()}",
                                    oninput: move |e| {
                                        if let Ok(v) = e.value().parse::<f64>() {
                                            let max = picker_speed();
                                            let clamped = v.min(max).max(10.0);
                                            anim_duration.set(clamped);
                                        }
                                    }
                                }
                                span { class: "cg-speed-label", "{anim_duration() as u32}ms" }
                            }
                        }
                        div { class: "cg-config-item cg-config-action",
                            button {
                                class: "cg-btn",
                                disabled: generating(),
                                onclick: move |_| generate(),
                                if generating() { "生成中..." } else { "重新生成" }
                            }
                            button {
                                class: "cg-btn cg-btn-picker",
                                disabled: generating() || circles().is_empty(),
                                onclick: move |_| toggle_picker(),
                                if selecting() { "停止选取" } else { "随机选取" }
                            }
                        }
                    }
                }
            }

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

            if show_modal() {
                div { class: "cg-modal-overlay",
                    div { class: "cg-modal",
                        div { class: "cg-modal-header",
                            span { "选取结果" }
                            button {
                                class: "cg-modal-close",
                                onclick: move |_| show_modal.set(false),
                                "\u{2716}"
                            }
                        }
                        div { class: "cg-modal-body",
                            {selected_circle().map(|(idx, circle)| {
                                rsx! {
                                    div { class: "cg-result-info",
                                        div { class: "cg-result-row",
                                            span { class: "cg-result-label", "编号" }
                                            span { class: "cg-result-value", "{idx + 1}" }
                                        }
                                        div { class: "cg-result-row",
                                            span { class: "cg-result-label", "X 坐标" }
                                            span { class: "cg-result-value", "{circle.x:.1}" }
                                        }
                                        div { class: "cg-result-row",
                                            span { class: "cg-result-label", "Y 坐标" }
                                            span { class: "cg-result-value", "{circle.y:.1}" }
                                        }
                                        div { class: "cg-result-row",
                                            span { class: "cg-result-label", "半径" }
                                            span { class: "cg-result-value", "{circle.radius:.1}" }
                                        }
                                    }
                                }
                            })}
                        }
                        div { class: "cg-modal-footer",
                            button {
                                class: "cg-btn",
                                onclick: move |_| show_modal.set(false),
                                "关闭"
                            }
                        }
                    }
                }
            }
        }
    }
}