//! Sunburst — recursive annular sectors. Each ring is one tree depth; child
//! sectors share their parent's angular range partitioned by `value`.

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::{SunburstNode, SunburstSeries};
use crate::render::ChartPainter;
use crate::render::shapes::annular_sector;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, FontId, Pos2, Rect, Stroke};
use std::f32::consts::{PI, TAU};

fn chart_to_screen(a: f32) -> f32 {
    a - PI * 0.5
}

fn max_depth(node: &SunburstNode) -> usize {
    if node.children.is_empty() {
        1
    } else {
        1 + node.children.iter().map(max_depth).max().unwrap_or(0)
    }
}

pub fn render(
    p: &ChartPainter,
    s: &SunburstSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.roots.is_empty() {
        return None;
    }

    let polar = PolarLayout::fit(rect, s.outer_ratio);
    let max_outer = polar.outer_radius;
    let min_inner = max_outer * s.inner_ratio;

    let depth = s.roots.iter().map(max_depth).max().unwrap_or(1).max(1);
    let ring_w = (max_outer - min_inner) / depth as f32;

    let total: f64 = s.roots.iter().map(|n| n.subtree_value()).sum::<f64>().max(1e-12);

    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;

    let mut acc = 0.0_f32;
    for (root_i, root) in s.roots.iter().enumerate() {
        let sweep = TAU * (root.subtree_value() / total) as f32;
        draw_node(
            p,
            root,
            0,
            acc,
            acc + sweep,
            min_inner,
            ring_w,
            polar.center,
            theme,
            palette_offset + root_i * 7,
            series_idx,
            hover_pos,
            &font,
            &mut tip,
        );
        acc += sweep;
    }

    tip
}

#[allow(clippy::too_many_arguments)]
fn draw_node(
    p: &ChartPainter,
    node: &SunburstNode,
    level: usize,
    start_chart: f32,
    end_chart: f32,
    base_inner: f32,
    ring_w: f32,
    center: Pos2,
    theme: &ChartTheme,
    palette_offset: usize,
    series_idx: usize,
    hover_pos: Option<Pos2>,
    font: &FontId,
    tip: &mut Option<TooltipDatum>,
) {
    let inner = base_inner + ring_w * level as f32;
    let outer = inner + ring_w;
    let a0 = chart_to_screen(start_chart);
    let a1 = chart_to_screen(end_chart);
    let color = theme.series_color(palette_offset + level * 3);

    let hovered = hover_pos
        .map(|h| {
            let v = h - center;
            let d2 = v.length_sq();
            if d2 < inner * inner || d2 > outer * outer {
                return false;
            }
            let mut ang = v.y.atan2(v.x) + PI * 0.5;
            while ang < 0.0 {
                ang += TAU;
            }
            while ang >= TAU {
                ang -= TAU;
            }
            let mut s_n = start_chart;
            let mut e_n = end_chart;
            while s_n < 0.0 {
                s_n += TAU;
                e_n += TAU;
            }
            ang >= s_n && ang <= e_n
        })
        .unwrap_or(false);

    let fill = if hovered {
        color
    } else if hover_pos.is_some() {
        egui::Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 220)
    } else {
        color
    };

    annular_sector(
        p,
        center,
        inner,
        outer,
        a0,
        a1,
        64,
        fill,
        Stroke::new(1.0, theme.background),
    );

    if hovered {
        *tip = Some(TooltipDatum {
            series_index: series_idx,
            series_name: node.name.clone(),
            data_index: level,
            value: node.subtree_value(),
            color,
            screen_pos: None,
        });
    }

    // Label inside the sector if there's room.
    let sweep = end_chart - start_chart;
    let label_radius = (inner + outer) * 0.5;
    if sweep > 0.18 && ring_w > 14.0 {
        let mid_chart = (start_chart + end_chart) * 0.5;
        let mid_screen = chart_to_screen(mid_chart);
        let pos = center
            + egui::vec2(mid_screen.cos(), mid_screen.sin()) * label_radius;
        p.text(pos, Align2::CENTER_CENTER, node.name.clone(), font.clone(), theme.background);
    }

    // Recurse into children, partitioning the sweep proportionally to value.
    if !node.children.is_empty() {
        let total: f64 = node.children.iter().map(|c| c.subtree_value()).sum::<f64>().max(1e-12);
        let mut acc = start_chart;
        for (ci, child) in node.children.iter().enumerate() {
            let cs = sweep * (child.subtree_value() / total) as f32;
            draw_node(
                p,
                child,
                level + 1,
                acc,
                acc + cs,
                base_inner,
                ring_w,
                center,
                theme,
                palette_offset + ci * 2,
                series_idx,
                hover_pos,
                font,
                tip,
            );
            acc += cs;
        }
    }
}
