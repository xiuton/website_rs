//! Canvas 绘制逻辑。

use super::config::hsl_color;
use crate::utils::circle_generator::Circle;
use wasm_bindgen::JsCast;

pub(super) struct RenderConfig<'a> {
    pub(super) circles: &'a [Circle],
    pub(super) config_width: f64,
    pub(super) config_height: f64,
    pub(super) fullscreen: bool,
    pub(super) highlight: Option<usize>,
    pub(super) prev_highlight: Option<usize>,
    pub(super) mask_hole: Option<(f64, f64, f64)>,
}

pub(super) fn render_canvas(cfg: &RenderConfig) {
    render_canvas_inner(cfg)
}

pub(super) fn render_canvas_inner(cfg: &RenderConfig) {
    let Some(window) = web_sys::window() else { return };
    let Some(document) = window.document() else { return };

    let Some(canvas) = document
        .get_element_by_id("circle-canvas")
        .and_then(|el| el.dyn_into::<web_sys::HtmlCanvasElement>().ok())
    else {
        return;
    };

    let dpr = window.device_pixel_ratio();

    let display_width = canvas.client_width() as f64;
    let (final_w, final_h) = if cfg.fullscreen {
        let dh = canvas.client_height() as f64;
        canvas.set_width((display_width * dpr) as u32);
        canvas.set_height((dh * dpr) as u32);
        (display_width, dh)
    } else {
        let dh = display_width * cfg.config_height / cfg.config_width;
        canvas.set_width((display_width * dpr) as u32);
        canvas.set_height((dh * dpr) as u32);
        (display_width, dh)
    };

    let Some(ctx) = canvas
        .get_context("2d")
        .ok()
        .flatten()
        .and_then(|ctx| ctx.dyn_into::<web_sys::CanvasRenderingContext2d>().ok())
    else {
        return;
    };

    let is_dark = document
        .document_element()
        .map(|el| el.matches(".dark").unwrap_or(false))
        .unwrap_or(false);

    let _ = ctx.set_transform(dpr, 0.0, 0.0, dpr, 0.0, 0.0);
    ctx.clear_rect(0.0, 0.0, final_w, final_h);

    let bg_color = if is_dark { "#1a1a1a" } else { "#f8f9fa" };
    ctx.set_fill_style_str(bg_color);
    ctx.fill_rect(0.0, 0.0, final_w, final_h);

    let scale_x = final_w / cfg.config_width;
    let scale_y = final_h / cfg.config_height;

    for (i, circle) in cfg.circles.iter().enumerate() {
        let color = hsl_color(i, cfg.circles.len(), is_dark);

        let cx = circle.x * scale_x;
        let cy = circle.y * scale_y;
        let cr = circle.radius * scale_x;

        ctx.begin_path();
        let _ = ctx.arc(cx, cy, cr, 0.0, std::f64::consts::PI * 2.0);
        ctx.set_fill_style_str(&color);
        ctx.set_global_alpha(if is_dark { 0.6 } else { 0.5 });
        ctx.fill();
        ctx.set_global_alpha(1.0);
        ctx.set_stroke_style_str(&color);
        ctx.set_line_width(2.0);
        ctx.stroke();

        let text_color = if is_dark { "#ccc" } else { "#333" };
        ctx.set_fill_style_str(text_color);
        ctx.set_font("11px sans-serif");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(&format!("{}", i + 1), cx, cy);
    }

    if let (Some(prev), Some(cur)) = (cfg.prev_highlight, cfg.highlight) {
        if prev != cur {
            if let (Some(pc), Some(cc)) = (cfg.circles.get(prev), cfg.circles.get(cur)) {
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
                let _ = ctx.set_line_dash(&dash);

                ctx.begin_path();
                ctx.move_to(px, py);
                ctx.line_to(cx2, cy2);
                ctx.stroke();

                ctx.set_shadow_blur(0.0);
                ctx.set_global_alpha(1.0);
                ctx.set_line_dash(&js_sys::Array::new()).ok();
                ctx.restore();
            }
        }
    }

    let mask_pos = cfg.mask_hole.or_else(|| {
        cfg.highlight.and_then(|idx| {
            cfg.circles
                .get(idx)
                .map(|c| (c.x * scale_x, c.y * scale_y, c.radius * scale_x + 4.0))
        })
    });

    if let Some((hole_cx, hole_cy, hole_cr)) = mask_pos {
        ctx.save();
        ctx.begin_path();
        ctx.rect(0.0, 0.0, final_w, final_h);
        let _ = ctx.arc_with_anticlockwise(hole_cx, hole_cy, hole_cr, 0.0, std::f64::consts::PI * 2.0, true);
        ctx.clip();

        ctx.set_fill_style_str(if is_dark { "rgba(0,0,0,0.55)" } else { "rgba(0,0,0,0.35)" });
        ctx.fill_rect(0.0, 0.0, final_w, final_h);
        ctx.restore();
    }

    if let Some(idx) = cfg.highlight {
        if let Some(circle) = cfg.circles.get(idx) {
            let cx = circle.x * scale_x;
            let cy = circle.y * scale_y;
            let cr = circle.radius * scale_x + 4.0;

            ctx.begin_path();
            let _ = ctx.arc(cx, cy, cr, 0.0, std::f64::consts::PI * 2.0);
            ctx.set_fill_style_str("#ffd700");
            ctx.set_global_alpha(0.9);
            ctx.fill();
            ctx.set_global_alpha(1.0);
            ctx.set_stroke_style_str("#ff8c00");
            ctx.set_line_width(4.0);
            ctx.stroke();

            ctx.set_fill_style_str(if is_dark { "#fff" } else { "#000" });
            ctx.set_font("bold 14px sans-serif");
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            let _ = ctx.fill_text(&format!("{}", idx + 1), cx, cy);
        }
    }
}
