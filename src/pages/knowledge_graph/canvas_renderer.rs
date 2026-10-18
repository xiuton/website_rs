//! Canvas 2D 渲染器。

use super::force_graph::{community_color, text_color_for_bg, ForceGraph};
use wasm_bindgen::JsCast;

// ============================================================================
// Canvas 渲染器
// ============================================================================

pub(super) struct CanvasRenderer {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl CanvasRenderer {
    pub(super) fn new(canvas_id: &str) -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let canvas = document.get_element_by_id(canvas_id)?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().ok()?;
        let ctx: web_sys::CanvasRenderingContext2d = canvas.get_context("2d").ok()??.dyn_into().ok()?;
        Some(Self { canvas, ctx })
    }

    pub(super) fn resize(&self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    pub(super) fn render(&self, graph: &ForceGraph) {
        let w = self.canvas.width() as f64;
        let h = self.canvas.height() as f64;
        let ctx = &self.ctx;

        ctx.clear_rect(0.0, 0.0, w, h);

        ctx.save();
        let _ = ctx.translate(graph.offset_x, graph.offset_y);
        let _ = ctx.scale(graph.scale, graph.scale);

        // 绘制边
        ctx.set_stroke_style_str("#4b5563");
        ctx.set_line_width(0.5 / graph.scale);
        ctx.begin_path();
        for edge in &graph.edges {
            let src = &graph.nodes[edge.source_idx];
            let tgt = &graph.nodes[edge.target_idx];
            ctx.move_to(src.x, src.y);
            ctx.line_to(tgt.x, tgt.y);
        }
        ctx.stroke();

        // 绘制节点
        for (i, node) in graph.nodes.iter().enumerate() {
            let color = community_color(&node.community);
            let is_hovered = graph.hovered == Some(i);
            let is_dragging = graph.dragging == Some(i);
            let r = if is_hovered || is_dragging { node.radius * 1.15 } else { node.radius };

            if is_hovered || is_dragging {
                ctx.set_fill_style_str(color);
                ctx.set_global_alpha(0.2);
                ctx.begin_path();
                let _ = ctx.arc(node.x, node.y, r + 6.0, 0.0, std::f64::consts::TAU);
                ctx.fill();
                ctx.set_global_alpha(1.0);
            }

            // 搜索结果高亮：绘制金色光环
            let is_highlighted = graph.highlighted_slugs.contains(&node.slug);
            if is_highlighted {
                ctx.set_stroke_style_str("#f59e0b");
                ctx.set_line_width(2.5 / graph.scale);
                ctx.set_global_alpha(0.8);
                ctx.begin_path();
                let _ = ctx.arc(node.x, node.y, r + 4.0, 0.0, std::f64::consts::TAU);
                ctx.stroke();
                // 外圈更大光晕
                ctx.set_stroke_style_str("#fbbf24");
                ctx.set_line_width(1.5 / graph.scale);
                ctx.set_global_alpha(0.35);
                ctx.begin_path();
                let _ = ctx.arc(node.x, node.y, r + 8.0, 0.0, std::f64::consts::TAU);
                ctx.stroke();
                ctx.set_global_alpha(1.0);
            }

            ctx.set_fill_style_str(color);
            ctx.begin_path();
            let _ = ctx.arc(node.x, node.y, r, 0.0, std::f64::consts::TAU);
            ctx.fill();

            ctx.set_stroke_style_str("#1f2937");
            ctx.set_line_width(1.5 / graph.scale);
            ctx.stroke();

            ctx.set_fill_style_str(text_color_for_bg(color));
            let font_size = (r * 0.28).max(3.0);
            ctx.set_font(&format!("{:.1}px 'MiSans', sans-serif", font_size));
            ctx.set_text_align("center");
            ctx.set_text_baseline("middle");
            ctx.set_global_alpha(0.9);
            // 悬停或拖拽时显示完整标题，否则截断
            let display_title = if is_hovered || is_dragging {
                node.title.clone()
            } else if node.title.chars().count() > 6 {
                format!("{}…", &node.title.chars().take(5).collect::<String>())
            } else {
                node.title.clone()
            };
            let _ = ctx.fill_text(&display_title, node.x, node.y);
            ctx.set_global_alpha(1.0);
        }

        ctx.restore();
    }
}
