//! 知识图谱探索器 — 交互式力导向图。

mod canvas_renderer;
mod force_graph;

use canvas_renderer::CanvasRenderer;
use dioxus::prelude::*;
use force_graph::{community_color, ForceGraph};
use std::cell::RefCell;
use std::collections::{BTreeSet, HashSet};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;

use crate::components::PageHead;
use crate::routes::Route;
use crate::utils::knowledge_graph;

// ============================================================================
// 页面组件
// ============================================================================

/// 将 ForceGraph 包在 Rc<RefCell<>> 中以便跨闭包共享
type SharedGraph = Rc<RefCell<Option<ForceGraph>>>;

#[component]
pub fn KnowledgeGraphView() -> Element {
    // use_resource 统一 loading / error 状态，数据层仍走 typed cache。
    let kg_resource = use_resource(|| async { knowledge_graph::load_graph().await });
    let mut graph_rc = use_signal(SharedGraph::default);
    let mut communities = use_signal(Vec::new);
    let canvas_id = "kg-canvas";
    let nav = use_navigator();

    let kg_loading = kg_resource.read().is_none();
    let kg_error = matches!(&*kg_resource.read(), Some(None));

    // 初始化 ForceGraph
    use_effect(move || {
        let state = kg_resource.read();
        let Some(Some(kg)) = &*state else {
            return;
        };
        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();

        // 读取 ?highlight= 查询参数
        let highlighted_slugs: HashSet<String> = window
            .location()
            .search()
            .ok()
            .and_then(|s| web_sys::UrlSearchParams::new_with_str(&s).ok())
            .and_then(|params| params.get("highlight"))
            .map(|s| {
                s.split(',')
                    .map(|slug| slug.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect()
            })
            .unwrap_or_default();

        if let Some(canvas_el) = document.get_element_by_id(canvas_id) {
            let canvas: web_sys::HtmlCanvasElement = canvas_el.dyn_into().unwrap();
            let w = canvas.client_width() as f64;
            let h = canvas.client_height() as f64;
            let g = ForceGraph::new(kg.as_ref(), w, h, highlighted_slugs);

            // 提取社区列表
            let mut set = BTreeSet::new();
            for node in &g.nodes {
                set.insert(node.community.clone());
            }
            communities.set(set.into_iter().collect());

            graph_rc.set(Rc::new(RefCell::new(Some(g))));
        }
    });

    // 力模拟 + 渲染循环 + 事件绑定（只初始化一次）
    let _graph_for_animation = graph_rc;
    use_effect(move || {
        let g_rc = _graph_for_animation.read().clone();
        // 等待数据就绪
        if g_rc.borrow().is_none() {
            return;
        }

        let window = web_sys::window().unwrap();
        let document = window.document().unwrap();
        let Some(canvas_el) = document.get_element_by_id(canvas_id) else {
            return;
        };
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
                    .request_animation_frame(anim_closure2.borrow().as_ref().unwrap().as_ref().unchecked_ref())
                    .unwrap();
            }));
        }

        web_sys::window()
            .unwrap()
            .request_animation_frame(anim_closure.borrow().as_ref().unwrap().as_ref().unchecked_ref())
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
    let graph_for_click = graph_rc;
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
    let graph_for_tooltip = graph_rc;
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
        PageHead {
            title: "知识图谱".to_string(),
            description: "文章关联关系与社区发现的可视化探索。".to_string(),
            canonical: "/knowledge-graph".to_string(),
        }
        div { class: "kg-container",
            div { class: "kg-header",
                h1 { "知识图谱探索器" }
                p { class: "kg-subtitle",
                    "拖拽节点移动 | 滚轮缩放 | 点击节点跳转文章 | 节点越大 = 关联越广"
                }
            }

            if kg_loading {
                div { class: "kg-loading",
                    div { class: "kg-spinner" }
                    p { "正在加载知识图谱..." }
                }
            } else if kg_error {
                div { class: "kg-error",
                    p { "无法加载知识图谱数据，请稍后重试。" }
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
