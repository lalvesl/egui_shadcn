//! Tree (node-link diagram). Lays the root on one edge and arranges leaves
//! evenly along the perpendicular. Edges are drawn as right-angle elbows.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::{TreeNode, TreeOrientation, TreeSeries};
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &TreeSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let leaves = count_leaves(&s.root).max(1);
    let depth = max_depth(&s.root).max(1);

    // Allocate space per depth and per leaf.
    let pad_x = 32.0;
    let pad_y = 24.0;
    let inner = rect.shrink2(egui::vec2(pad_x, pad_y));

    let (level_axis_size, leaf_axis_size, level_dir, leaf_dir) =
        match s.orientation {
            TreeOrientation::LeftRight => (
                inner.width(),
                inner.height(),
                egui::vec2(1.0, 0.0),
                egui::vec2(0.0, 1.0),
            ),
            TreeOrientation::RightLeft => (
                inner.width(),
                inner.height(),
                egui::vec2(-1.0, 0.0),
                egui::vec2(0.0, 1.0),
            ),
            TreeOrientation::TopBottom => (
                inner.height(),
                inner.width(),
                egui::vec2(0.0, 1.0),
                egui::vec2(1.0, 0.0),
            ),
            TreeOrientation::BottomTop => (
                inner.height(),
                inner.width(),
                egui::vec2(0.0, -1.0),
                egui::vec2(1.0, 0.0),
            ),
            TreeOrientation::Radial => {
                return draw_radial(
                    p,
                    s,
                    series_idx,
                    rect,
                    theme,
                    palette_offset,
                    hover_pos,
                );
            }
        };

    let dx = level_axis_size / (depth as f32 + 0.0).max(1.0);
    let leaf_step = leaf_axis_size / (leaves as f32 + 1.0).max(1.0);
    let origin = match s.orientation {
        TreeOrientation::LeftRight => Pos2::new(inner.min.x, inner.min.y),
        TreeOrientation::RightLeft => Pos2::new(inner.max.x, inner.min.y),
        TreeOrientation::TopBottom => Pos2::new(inner.min.x, inner.min.y),
        TreeOrientation::BottomTop => Pos2::new(inner.min.x, inner.max.y),
        TreeOrientation::Radial => unreachable!(),
    };

    let mut nodes: Vec<LaidOutNode> = Vec::new();
    let mut leaf_counter = 0_usize;
    layout(
        &s.root,
        None,
        0,
        origin,
        level_dir,
        leaf_dir,
        dx,
        leaf_step,
        &mut leaf_counter,
        &mut nodes,
    );

    draw_nodes(p, &nodes, s, theme, palette_offset, hover_pos, series_idx)
}

struct LaidOutNode {
    pos: Pos2,
    name: String,
    parent: Option<usize>,
    leaf: bool,
}

#[allow(clippy::too_many_arguments)]
fn layout(
    node: &TreeNode,
    parent_idx: Option<usize>,
    depth: usize,
    origin: Pos2,
    level_dir: egui::Vec2,
    leaf_dir: egui::Vec2,
    dx: f32,
    leaf_step: f32,
    leaf_counter: &mut usize,
    out: &mut Vec<LaidOutNode>,
) -> usize {
    let pos_level_offset = level_dir * dx * depth as f32;
    let self_idx = out.len();
    out.push(LaidOutNode {
        pos: origin + pos_level_offset, // leaf offset filled in later
        name: node.name.clone(),
        parent: parent_idx,
        leaf: node.children.is_empty(),
    });

    if node.children.is_empty() {
        *leaf_counter += 1;
        let leaf_pos = origin
            + pos_level_offset
            + leaf_dir * leaf_step * *leaf_counter as f32;
        out[self_idx].pos = leaf_pos;
        self_idx
    } else {
        let mut child_y_sum = 0.0_f32;
        let mut count = 0_usize;
        for child in &node.children {
            let ci = layout(
                child,
                Some(self_idx),
                depth + 1,
                origin,
                level_dir,
                leaf_dir,
                dx,
                leaf_step,
                leaf_counter,
                out,
            );
            // Sum leaf-axis component of child position.
            let along = (out[ci].pos - origin).dot(leaf_dir);
            child_y_sum += along;
            count += 1;
        }
        let avg = child_y_sum / count as f32;
        out[self_idx].pos = origin + pos_level_offset + leaf_dir * avg;
        self_idx
    }
}

fn draw_nodes(
    p: &ChartPainter,
    nodes: &[LaidOutNode],
    s: &TreeSeries,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
    series_idx: usize,
) -> Option<TooltipDatum> {
    let font = label_font();
    let edge_stroke = Stroke::new(1.0, theme.text_dim);
    let horizontal = matches!(
        s.orientation,
        TreeOrientation::LeftRight | TreeOrientation::RightLeft
    );

    // Edges first so dots overlay them.
    for (i, n) in nodes.iter().enumerate() {
        if let Some(parent) = n.parent {
            let a = nodes[parent].pos;
            let b = n.pos;
            if horizontal {
                let mid_x = (a.x + b.x) * 0.5;
                p.line(a, Pos2::new(mid_x, a.y), edge_stroke);
                p.line(
                    Pos2::new(mid_x, a.y),
                    Pos2::new(mid_x, b.y),
                    edge_stroke,
                );
                p.line(Pos2::new(mid_x, b.y), b, edge_stroke);
            } else {
                let mid_y = (a.y + b.y) * 0.5;
                p.line(a, Pos2::new(a.x, mid_y), edge_stroke);
                p.line(
                    Pos2::new(a.x, mid_y),
                    Pos2::new(b.x, mid_y),
                    edge_stroke,
                );
                p.line(Pos2::new(b.x, mid_y), b, edge_stroke);
            }
        }
        let _ = i;
    }

    let mut tip: Option<TooltipDatum> = None;
    for (i, n) in nodes.iter().enumerate() {
        let color =
            theme.series_color(palette_offset + (if n.leaf { 1 } else { 0 }));
        p.circle_filled(n.pos, 5.0, color);
        p.circle_stroke(n.pos, 5.0, Stroke::new(1.0, theme.background));

        let offset = if horizontal {
            egui::vec2(8.0, 0.0)
        } else {
            egui::vec2(0.0, -10.0)
        };
        let anchor = if horizontal {
            Align2::LEFT_CENTER
        } else {
            Align2::CENTER_BOTTOM
        };
        p.text(
            n.pos + offset,
            anchor,
            n.name.clone(),
            font.clone(),
            theme.text,
        );

        if let Some(h) = hover_pos
            && (h - n.pos).length_sq() <= 64.0
        {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: n.name.clone(),
                data_index: i,
                value: 0.0,
                color,
                screen_pos: Some(n.pos),
            });
        }
    }

    tip
}

fn count_leaves(node: &TreeNode) -> usize {
    if node.children.is_empty() {
        1
    } else {
        node.children.iter().map(count_leaves).sum()
    }
}

fn max_depth(node: &TreeNode) -> usize {
    if node.children.is_empty() {
        1
    } else {
        1 + node.children.iter().map(max_depth).max().unwrap_or(0)
    }
}

fn draw_radial(
    p: &ChartPainter,
    s: &TreeSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    use std::f32::consts::TAU;

    let leaves = count_leaves(&s.root).max(1);
    let depth = max_depth(&s.root).max(1);
    let center = rect.center();
    let max_r = rect.size().min_elem() * 0.42;
    let ring_w = max_r / depth as f32;

    let mut nodes: Vec<LaidOutNode> = Vec::new();
    let mut leaf_counter = 0_usize;
    #[allow(clippy::too_many_arguments)]
    fn layout_polar(
        node: &TreeNode,
        depth: usize,
        center: Pos2,
        ring_w: f32,
        leaves: usize,
        leaf_counter: &mut usize,
        parent_idx: Option<usize>,
        out: &mut Vec<LaidOutNode>,
    ) -> (usize, f32) {
        let self_idx = out.len();
        out.push(LaidOutNode {
            pos: center,
            name: node.name.clone(),
            parent: parent_idx,
            leaf: node.children.is_empty(),
        });
        let angle = if node.children.is_empty() {
            *leaf_counter += 1;
            TAU * (*leaf_counter as f32 - 0.5) / leaves as f32
        } else {
            let mut sum = 0.0;
            let mut cnt = 0;
            for child in &node.children {
                let (_, ca) = layout_polar(
                    child,
                    depth + 1,
                    center,
                    ring_w,
                    leaves,
                    leaf_counter,
                    Some(self_idx),
                    out,
                );
                sum += ca;
                cnt += 1;
            }
            sum / cnt as f32
        };
        let r = ring_w * depth as f32;
        let pos = center + egui::vec2(angle.sin(), -angle.cos()) * r;
        out[self_idx].pos = pos;
        (self_idx, angle)
    }
    layout_polar(
        &s.root,
        0,
        center,
        ring_w,
        leaves,
        &mut leaf_counter,
        None,
        &mut nodes,
    );

    let edge_stroke = Stroke::new(1.0, theme.text_dim);
    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;

    for n in &nodes {
        if let Some(parent) = n.parent {
            p.line(nodes[parent].pos, n.pos, edge_stroke);
        }
    }
    for (i, n) in nodes.iter().enumerate() {
        let color =
            theme.series_color(palette_offset + (if n.leaf { 1 } else { 0 }));
        p.circle_filled(n.pos, 5.0, color);
        p.text(
            n.pos + egui::vec2(8.0, -8.0),
            Align2::LEFT_CENTER,
            n.name.clone(),
            font.clone(),
            theme.text,
        );
        if let Some(h) = hover_pos
            && (h - n.pos).length_sq() <= 64.0
        {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: n.name.clone(),
                data_index: i,
                value: 0.0,
                color,
                screen_pos: Some(n.pos),
            });
        }
    }

    tip
}
