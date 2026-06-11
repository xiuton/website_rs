//! 知识图谱探索器 — 交互式力导向图
//! 展示所有文章的关联关系，支持拖拽、缩放、点击跳转

use dioxus::prelude::*;
use dioxus_router::prelude::use_navigator;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::rc::Rc;

use crate::routes::Route;
use crate::utils::title;
use crate::utils::knowledge_graph::{self, KnowledgeGraph};
use crate::BLOG_POSTS;

// ============================================================================
// 力导向图数据结构
// ============================================================================

const NODE_RADIUS_MIN: f64 = 10.0;
const NODE_RADIUS_MAX: f64 = 30.0;
const REPULSION_STRENGTH: f64 = 800.0;
const ATTRACTION_STRENGTH: f64 = 0.01;
const DAMPING: f64 = 0.85;
const MAX_VELOCITY: f64 = 5.0;

/// 社区颜色映射
fn community_color(community: &str) -> &'static str {
    match community {
        s if s.contains("Rust") => "#f74c00",
        s if s.contains("Dioxus") => "#7c3aed",
        s if s.contains("Windows") => "#0078d4",
        s if s.contains("Golang") || s.contains("Gin") => "#00add8",
        s if s.contains("前端") => "#f7df1e",
        s if s.contains("TypeScript") => "#3178c6",
        s if s.contains("Vue") => "#42b883",
        s if s.contains("Web Component") => "#e67910",
        s if s.contains("AI") || s.contains("Artificial") => "#10b981",
        s if s.contains("技术") => "#6366f1",
        s if s.contains("React") => "#61dafb",
        _ => "#9ca3af",
    }
}

/// 根据背景色计算对比度合适的文字颜色（深色背景用浅色字，浅色背景用深色字）
fn text_color_for_bg(hex: &str) -> &'static str {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    // 相对亮度公式 (sRGB)
    let luminance = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    if luminance > 180.0 { "#1f2937" } else { "#e5e7eb" }
}

#[derive(Clone)]
struct GraphNode {
    slug: String,
    title: String,
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    radius: f64,
    community: String,
    pagerank: f64,
}

#[derive(Clone)]
struct GraphEdge {
    source_idx: usize,
    target_idx: usize,
    weight: f64,
}

struct ForceGraph {
    nodes: Vec<GraphNode>,
    edges: Vec<GraphEdge>,
    #[allow(dead_code)]
    slug_to_idx: HashMap<String, usize>,
    width: f64,
    height: f64,
    offset_x: f64,
    offset_y: f64,
    scale: f64,
    dragging: Option<usize>,
    hovered: Option<usize>,
    mouse_x: f64,
    mouse_y: f64,
    pan_start: Option<(f64, f64)>,
    offset_start: Option<(f64, f64)>,
    /// 从搜索结果来的高亮节点 slug 集合
    highlighted_slugs: HashSet<String>,
    /// 拖拽起始屏幕坐标（用于区分点击与拖拽）
    drag_start: Option<(f64, f64)>,
    /// 是否发生了有效拖拽（鼠标移动超过阈值）
    was_dragged: bool,
}

impl ForceGraph {
    fn new(kg: &KnowledgeGraph, width: f64, height: f64, highlighted_slugs: HashSet<String>) -> Self {
        let cx = width / 2.0;
        let cy = height / 2.0;
        let mut nodes = Vec::new();
        let mut slug_to_idx = HashMap::new();

        let title_map: HashMap<&str, &str> = BLOG_POSTS
            .iter()
            .map(|p| (p.slug, p.title))
            .collect();

        for (slug, article) in &kg.articles {
            let idx = nodes.len();
            slug_to_idx.insert(slug.clone(), idx);

            let pagerank: f64 = article.pagerank.parse().unwrap_or(0.01);
            let radius = NODE_RADIUS_MIN + (NODE_RADIUS_MAX - NODE_RADIUS_MIN) * pagerank.min(0.2) * 5.0;

            let angle = (idx as f64 / kg.articles.len() as f64) * std::f64::consts::TAU;
            let spread = (width.min(height) / 2.0) * 0.6;
            let x = cx + angle.cos() * spread;
            let y = cy + angle.sin() * spread;

            let title = title_map.get(slug.as_str()).copied().unwrap_or(slug).to_string();

            nodes.push(GraphNode {
                slug: slug.clone(),
                title,
                x,
                y,
                vx: 0.0,
                vy: 0.0,
                radius,
                community: article.community.clone(),
                pagerank,
            });
        }

        let mut edge_pairs = HashMap::new();
        for (slug, article) in &kg.articles {
            if let Some(&source_idx) = slug_to_idx.get(slug) {
                for rel in &article.related.articles {
                    if let Some(&target_idx) = slug_to_idx.get(&rel.slug) {
                        let (a, b) = if source_idx < target_idx {
                            (source_idx, target_idx)
                        } else {
                            (target_idx, source_idx)
                        };
                        let score: f64 = rel.score.parse().unwrap_or(0.01);
                        let key = (a, b);
                        edge_pairs
                            .entry(key)
                            .and_modify(|e: &mut f64| *e = e.max(score))
                            .or_insert(score);
                    }
                }
            }
        }

        let edges: Vec<GraphEdge> = edge_pairs
            .into_iter()
            .map(|((source_idx, target_idx), weight)| GraphEdge {
                source_idx,
                target_idx,
                weight,
            })
            .collect();

        Self {
            nodes,
            edges,
            slug_to_idx,
            width,
            height,
            offset_x: 0.0,
            offset_y: 0.0,
            scale: 1.0,
            dragging: None,
            hovered: None,
            mouse_x: 0.0,
            mouse_y: 0.0,
            pan_start: None,
            offset_start: None,
            highlighted_slugs,
            drag_start: None,
            was_dragged: false,
        }
    }

    fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    fn step(&mut self) {
        let n = self.nodes.len();
        if n == 0 {
            return;
        }

        for i in 0..n {
            for j in (i + 1)..n {
                let dx = self.nodes[j].x - self.nodes[i].x;
                let dy = self.nodes[j].y - self.nodes[i].y;
                let dist_sq = dx * dx + dy * dy;
                let min_dist = self.nodes[i].radius + self.nodes[j].radius + 20.0;
                let dist_sq = dist_sq.max(min_dist * min_dist * 0.01);
                let dist = dist_sq.sqrt();
                let force = REPULSION_STRENGTH / dist_sq;
                let fx = (dx / dist) * force;
                let fy = (dy / dist) * force;
                self.nodes[i].vx -= fx;
                self.nodes[i].vy -= fy;
                self.nodes[j].vx += fx;
                self.nodes[j].vy += fy;
            }
        }

        for edge in &self.edges {
            let i = edge.source_idx;
            let j = edge.target_idx;
            let dx = self.nodes[j].x - self.nodes[i].x;
            let dy = self.nodes[j].y - self.nodes[i].y;
            let dist = (dx * dx + dy * dy).sqrt().max(1.0);
            let force = ATTRACTION_STRENGTH * dist * edge.weight;
            let fx = (dx / dist) * force;
            let fy = (dy / dist) * force;
            self.nodes[i].vx += fx;
            self.nodes[i].vy += fy;
            self.nodes[j].vx -= fx;
            self.nodes[j].vy -= fy;
        }

        let cx = self.width / 2.0;
        let cy = self.height / 2.0;
        for node in &mut self.nodes {
            let dx = cx - node.x;
            let dy = cy - node.y;
            let dist = (dx * dx + dy * dy).sqrt().max(1.0);
            let force = dist * 0.0001;
            node.vx += dx / dist * force;
            node.vy += dy / dist * force;

            node.vx = node.vx.clamp(-MAX_VELOCITY, MAX_VELOCITY) * DAMPING;
            node.vy = node.vy.clamp(-MAX_VELOCITY, MAX_VELOCITY) * DAMPING;
            node.x += node.vx;
            node.y += node.vy;
        }
    }

    fn screen_to_canvas(&self, sx: f64, sy: f64) -> (f64, f64) {
        let x = (sx - self.offset_x) / self.scale;
        let y = (sy - self.offset_y) / self.scale;
        (x, y)
    }

    fn hit_test(&self, sx: f64, sy: f64) -> Option<usize> {
        let (cx, cy) = self.screen_to_canvas(sx, sy);
        for (i, node) in self.nodes.iter().enumerate() {
            let dx = cx - node.x;
            let dy = cy - node.y;
            if dx * dx + dy * dy <= (node.radius / self.scale + 8.0).powi(2) {
                return Some(i);
            }
        }
        None
    }
}

// ============================================================================
// Canvas 渲染器
// ============================================================================

struct CanvasRenderer {
    canvas: web_sys::HtmlCanvasElement,
    ctx: web_sys::CanvasRenderingContext2d,
}

impl CanvasRenderer {
    fn new(canvas_id: &str) -> Option<Self> {
        let window = web_sys::window()?;
        let document = window.document()?;
        let canvas = document.get_element_by_id(canvas_id)?;
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().ok()?;
        let ctx: web_sys::CanvasRenderingContext2d = canvas
            .get_context("2d")
            .ok()??
            .dyn_into()
            .ok()?;
        Some(Self { canvas, ctx })
    }

    fn resize(&self, width: u32, height: u32) {
        self.canvas.set_width(width);
        self.canvas.set_height(height);
    }

    fn render(&self, graph: &ForceGraph) {
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
            let r = if is_hovered || is_dragging {
                node.radius * 1.15
            } else {
                node.radius
            };

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

// ============================================================================
// 页面组件
// ============================================================================

/// 将 ForceGraph 包在 Rc<RefCell<>> 中以便跨闭包共享
type SharedGraph = Rc<RefCell<Option<ForceGraph>>>;

#[component]
pub fn KnowledgeGraphView() -> Element {
    title::set_page_title("知识图谱 - 干徒");

    let mut kg_data = use_signal(|| Option::<KnowledgeGraph>::None);
    let mut loading = use_signal(|| true);
    let mut error_msg = use_signal(|| Option::<String>::None);
    let mut graph_rc = use_signal(|| SharedGraph::default());
    let mut communities = use_signal(Vec::new);
    let canvas_id = "kg-canvas";
    let nav = use_navigator();

    // 加载知识图谱数据
    use_effect(move || {
        spawn(async move {
            match knowledge_graph::load_graph().await {
                Some(kg) => {
                    kg_data.set(Some(kg));
                    loading.set(false);
                }
                None => {
                    error_msg.set(Some("无法加载知识图谱数据，请稍后重试。".into()));
                    loading.set(false);
                }
            }
        });
    });

    // 初始化 ForceGraph
    use_effect(move || {
        if let Some(ref kg) = *kg_data.read() {
            let window = web_sys::window().unwrap();
            let document = window.document().unwrap();

            // 读取 ?highlight= 查询参数
            let highlighted_slugs: HashSet<String> = window
                .location()
                .search()
                .ok()
                .and_then(|s| {
                    web_sys::UrlSearchParams::new_with_str(&s).ok()
                })
                .and_then(|params| params.get("highlight"))
                .map(|s| s.split(',').map(|slug| slug.trim().to_string()).filter(|s| !s.is_empty()).collect())
                .unwrap_or_default();

            if let Some(canvas_el) = document.get_element_by_id(canvas_id) {
                let canvas: web_sys::HtmlCanvasElement = canvas_el.dyn_into().unwrap();
                let w = canvas.client_width() as f64;
                let h = canvas.client_height() as f64;
                let g = ForceGraph::new(kg, w, h, highlighted_slugs);

                // 提取社区列表
                let mut set = BTreeSet::new();
                for node in &g.nodes {
                    set.insert(node.community.clone());
                }
                communities.set(set.into_iter().collect());

                graph_rc.set(Rc::new(RefCell::new(Some(g))));
            }
        }
    });

    // 力模拟 + 渲染循环 + 事件绑定（只初始化一次）
    let _graph_for_animation = graph_rc.clone();
    use_effect(move || {
        let g_rc = _graph_for_animation.read().clone();
        // 等待数据就绪
        if g_rc.borrow().is_none() {
            return;
        }

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let Some(canvas_el) = document.get_element_by_id(canvas_id) else { return };
        let canvas: web_sys::HtmlCanvasElement = canvas_el.dyn_into().unwrap();

        // 初始化 renderer
        let renderer = CanvasRenderer::new(canvas_id).unwrap();
        let w = canvas.client_width() as u32;
        let h = canvas.client_height() as u32;
        renderer.resize(w, h);

        let renderer_rc = Rc::new(RefCell::new(renderer));

        // 动画循环
        let anim_g = Rc::clone(&g_rc);
        let anim_r = Rc::clone(&renderer_rc);
        let anim_closure = Rc::new(RefCell::new(None::<Closure<dyn FnMut()>>));

        {
            let anim_closure2 = Rc::clone(&anim_closure);
            *anim_closure.borrow_mut() = Some(Closure::new(move || {
                if let Some(ref mut graph) = *anim_g.borrow_mut() {
                    if graph.dragging.is_none() {
                        graph.step();
                    }
                    anim_r.borrow().render(graph);
                }
                web_sys::window()
                    .unwrap()
                    .request_animation_frame(
                        anim_closure2.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
                    )
                    .unwrap();
            }));
        }

        web_sys::window()
            .unwrap()
            .request_animation_frame(
                anim_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref(),
            )
            .unwrap();

        // --- 事件绑定 ---
        let mousedown_g = Rc::clone(&g_rc);
        let mousedown_canvas = canvas.clone();
        let mousedown = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            e.prevent_default();
            if let Some(ref mut graph) = *mousedown_g.borrow_mut() {
                let rect = mousedown_canvas.get_bounding_client_rect();
                let sx = e.client_x() as f64 - rect.left();
                let sy = e.client_y() as f64 - rect.top();
                if let Some(idx) = graph.hit_test(sx, sy) {
                    graph.dragging = Some(idx);
                    graph.drag_start = Some((sx, sy));
                    graph.was_dragged = false;
                } else {
                    graph.pan_start = Some((e.client_x() as f64, e.client_y() as f64));
                    graph.offset_start = Some((graph.offset_x, graph.offset_y));
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousedown", mousedown.as_ref().unchecked_ref())
            .unwrap();

        let mousemove_g = Rc::clone(&g_rc);
        let mousemove_canvas = canvas.clone();
        let mousemove = Closure::wrap(Box::new(move |e: web_sys::MouseEvent| {
            if let Some(ref mut graph) = *mousemove_g.borrow_mut() {
                let rect = mousemove_canvas.get_bounding_client_rect();
                let sx = e.client_x() as f64 - rect.left();
                let sy = e.client_y() as f64 - rect.top();
                graph.mouse_x = sx;
                graph.mouse_y = sy;

                if let Some(idx) = graph.dragging {
                    // 检测拖拽位移（超过 5px 才算有效拖拽，避免松手后跳转文章）
                    let dx = sx - graph.drag_start.map_or(sx, |(sx, _)| sx);
                    let dy = sy - graph.drag_start.map_or(sy, |(_, sy)| sy);
                    if dx * dx + dy * dy > 25.0 {
                        graph.was_dragged = true;
                    }
                    let (cx, cy) = graph.screen_to_canvas(sx, sy);
                    graph.nodes[idx].x = cx;
                    graph.nodes[idx].y = cy;
                    graph.nodes[idx].vx = 0.0;
                    graph.nodes[idx].vy = 0.0;
                } else if let (Some((px, py)), Some((ox, oy))) = (graph.pan_start, graph.offset_start) {
                    graph.offset_x = ox + (e.client_x() as f64 - px);
                    graph.offset_y = oy + (e.client_y() as f64 - py);
                } else {
                    graph.hovered = graph.hit_test(sx, sy);
                }
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mousemove", mousemove.as_ref().unchecked_ref())
            .unwrap();

        let mouseup_g = Rc::clone(&g_rc);
        let mouseup = Closure::wrap(Box::new(move |_e: web_sys::MouseEvent| {
            if let Some(ref mut graph) = *mouseup_g.borrow_mut() {
                graph.dragging = None;
                graph.pan_start = None;
                graph.offset_start = None;
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
            .unwrap();
        window
            .add_event_listener_with_callback("mouseup", mouseup.as_ref().unchecked_ref())
            .unwrap();

        let wheel_g = Rc::clone(&g_rc);
        let wheel_canvas = canvas.clone();
        let wheel = Closure::wrap(Box::new(move |e: web_sys::WheelEvent| {
            e.prevent_default();
            if let Some(ref mut graph) = *wheel_g.borrow_mut() {
                let rect = wheel_canvas.get_bounding_client_rect();
                let mx = e.client_x() as f64 - rect.left();
                let my = e.client_y() as f64 - rect.top();
                let zoom_speed = 0.001;
                let new_scale = (graph.scale * (1.0 - e.delta_y() * zoom_speed)).clamp(0.2, 3.0);
                let ratio = new_scale / graph.scale;
                graph.offset_x = mx - (mx - graph.offset_x) * ratio;
                graph.offset_y = my - (my - graph.offset_y) * ratio;
                graph.scale = new_scale;
            }
        }) as Box<dyn FnMut(_)>);
        canvas
            .add_event_listener_with_callback("wheel", wheel.as_ref().unchecked_ref())
            .unwrap();

        let resize_g = Rc::clone(&g_rc);
        let resize_renderer = Rc::clone(&renderer_rc);
        let resize_canvas = canvas.clone();
        let resize = Closure::wrap(Box::new(move || {
            let w = resize_canvas.client_width() as u32;
            let h = resize_canvas.client_height() as u32;
            resize_renderer.borrow().resize(w, h);
            if let Some(ref mut graph) = *resize_g.borrow_mut() {
                graph.resize(w as f64, h as f64);
            }
        }) as Box<dyn FnMut()>);
        window
            .add_event_listener_with_callback("resize", resize.as_ref().unchecked_ref())
            .unwrap();

        mousedown.forget();
        mousemove.forget();
        mouseup.forget();
        wheel.forget();
        resize.forget();
    });

    // 点击导航
    let graph_for_click = graph_rc.clone();
    let nav_for_click = nav;
    let handle_node_click = move |_: Event<MouseData>| {
        if let Some(ref g) = *graph_for_click.read().borrow() {
            if g.pan_start.is_some() || g.was_dragged {
                return;
            }
            if let Some(idx) = g.hovered {
                let slug = g.nodes[idx].slug.clone();
                nav_for_click.push(Route::BlogPostView { slug });
            }
        }
    };

    // Tooltip 信息
    let graph_for_tooltip = graph_rc.clone();
    let tooltip_info = use_memo(move || {
        if let Some(ref g) = *graph_for_tooltip.read().borrow() {
            if let Some(idx) = g.hovered {
                let node = &g.nodes[idx];
                return Some((
                    node.title.clone(),
                    node.community.clone(),
                    format!("{:.1}%", node.pagerank * 100.0),
                    g.mouse_x,
                    g.mouse_y,
                ));
            }
        }
        None
    });

    let communities_for_rsx = communities;

    rsx! {
        div { class: "kg-container",
            div { class: "kg-header",
                h1 { "知识图谱探索器" }
                p { class: "kg-subtitle",
                    "拖拽节点移动 | 滚轮缩放 | 点击节点跳转文章 | 节点越大 = 关联越广"
                }
            }

            if loading() {
                div { class: "kg-loading",
                    div { class: "kg-spinner" }
                    p { "正在加载知识图谱..." }
                }
            } else if let Some(ref msg) = *error_msg.read() {
                div { class: "kg-error",
                    p { "{msg}" }
                }
            } else {
                div { class: "kg-canvas-wrap",
                    canvas {
                        id: canvas_id,
                        class: "kg-canvas",
                        onclick: handle_node_click,
                    }

                    if let Some((title, community, pr, tx, ty)) = tooltip_info() {
                        div {
                            class: "kg-tooltip",
                            style: "left: {tx + 12.0}px; top: {ty - 10.0}px",
                            div { class: "kg-tooltip-title", "{title}" }
                            div { class: "kg-tooltip-meta",
                                span {
                                    class: "kg-tooltip-community",
                                    style: "color: {community_color(&community)}",
                                    "{community}"
                                }
                                span { class: "kg-tooltip-pr", "权重: {pr}" }
                            }
                            div { class: "kg-tooltip-hint", "点击查看文章 →" }
                        }
                    }
                }

                div { class: "kg-legend",
                    p { class: "kg-legend-title", "社区图例" }
                    div { class: "kg-legend-items",
                        {
                            communities_for_rsx.read().iter().map(|c| {
                                let color = community_color(c);
                                rsx! {
                                    span {
                                        key: "{c}",
                                        class: "kg-legend-item",
                                        span {
                                            class: "kg-legend-dot",
                                            style: "background-color: {color}",
                                        }
                                        "{c}"
                                    }
                                }
                            })
                        }
                    }
                }
            }
        }
    }
}