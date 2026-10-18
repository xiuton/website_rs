//! 力导向图数据结构与布局算法。

use crate::utils::knowledge_graph::KnowledgeGraph;
use crate::BLOG_POSTS;
use std::collections::{HashMap, HashSet};

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
pub(super) fn community_color(community: &str) -> &'static str {
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
pub(super) fn text_color_for_bg(hex: &str) -> &'static str {
    let hex = hex.trim_start_matches('#');
    let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
    // 相对亮度公式 (sRGB)
    let luminance = 0.299 * r as f64 + 0.587 * g as f64 + 0.114 * b as f64;
    if luminance > 180.0 {
        "#1f2937"
    } else {
        "#e5e7eb"
    }
}

#[derive(Clone)]
pub(super) struct GraphNode {
    pub(super) slug: String,
    pub(super) title: String,
    pub(super) x: f64,
    pub(super) y: f64,
    pub(super) vx: f64,
    pub(super) vy: f64,
    pub(super) radius: f64,
    pub(super) community: String,
    pub(super) pagerank: f64,
}

#[derive(Clone)]
pub(super) struct GraphEdge {
    pub(super) source_idx: usize,
    pub(super) target_idx: usize,
    pub(super) weight: f64,
}

pub(super) struct ForceGraph {
    pub(super) nodes: Vec<GraphNode>,
    pub(super) edges: Vec<GraphEdge>,
    #[allow(dead_code)]
    pub(super) slug_to_idx: HashMap<String, usize>,
    pub(super) width: f64,
    pub(super) height: f64,
    pub(super) offset_x: f64,
    pub(super) offset_y: f64,
    pub(super) scale: f64,
    pub(super) dragging: Option<usize>,
    pub(super) hovered: Option<usize>,
    pub(super) mouse_x: f64,
    pub(super) mouse_y: f64,
    pub(super) pan_start: Option<(f64, f64)>,
    pub(super) offset_start: Option<(f64, f64)>,
    /// 从搜索结果来的高亮节点 slug 集合
    pub(super) highlighted_slugs: HashSet<String>,
    /// 拖拽起始屏幕坐标（用于区分点击与拖拽）
    pub(super) drag_start: Option<(f64, f64)>,
    /// 是否发生了有效拖拽（鼠标移动超过阈值）
    pub(super) was_dragged: bool,
}

impl ForceGraph {
    pub(super) fn new(kg: &KnowledgeGraph, width: f64, height: f64, highlighted_slugs: HashSet<String>) -> Self {
        let cx = width / 2.0;
        let cy = height / 2.0;
        let mut nodes = Vec::new();
        let mut slug_to_idx = HashMap::new();

        let title_map: HashMap<&str, &str> = BLOG_POSTS.iter().map(|p| (p.slug, p.title)).collect();

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
            .map(|((source_idx, target_idx), weight)| GraphEdge { source_idx, target_idx, weight })
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

    pub(super) fn resize(&mut self, width: f64, height: f64) {
        self.width = width;
        self.height = height;
    }

    pub(super) fn step(&mut self) {
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

    pub(super) fn screen_to_canvas(&self, sx: f64, sy: f64) -> (f64, f64) {
        let x = (sx - self.offset_x) / self.scale;
        let y = (sy - self.offset_y) / self.scale;
        (x, y)
    }

    pub(super) fn hit_test(&self, sx: f64, sy: f64) -> Option<usize> {
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
