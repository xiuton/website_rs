use dioxus::prelude::*;
use wasm_bindgen::JsCast;
use crate::utils::circle_generator::{generate_circles, Circle, GenerationConfig};
use crate::utils::title;

fn hsl_color(index: usize, total: usize) -> String {
    let hue = (index as f64 / total.max(1) as f64) * 360.0;
    format!("hsl({}, 65%, 60%)", hue as u32)
}

fn render_canvas(circles: &[Circle], config_width: f64, config_height: f64) {
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

    let mut generate = move || {
        let cfg = GenerationConfig {
            width: config_width(),
            height: config_height(),
            gap: gap(),
            min_radius: min_radius(),
            max_radius: max_radius(),
            max_retries: None,
        };
        circles.set(generate_circles(&cfg));
    };

    {
        let mut generate = generate.clone();
        use_effect(move || {
            generate();
            ()
        });
    }

    use_effect(move || {
        let c = circles();
        let w = config_width();
        let h = config_height();
        render_canvas(&c, w, h);
        ()
    });

    rsx! {
        div { class: "circle-generator-page",
            h1 { "圆形生成器" }

            div { class: "cg-canvas-section",
                div { class: "cg-card cg-canvas-card",
                    canvas {
                        id: "circle-canvas",
                        style: "width: 100%;"
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
                        div { class: "cg-config-item cg-config-action",
                            button {
                                class: "cg-btn",
                                onclick: move |_| generate(),
                                "重新生成"
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
        }
    }
}