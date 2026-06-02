//! Graph (node + link) diagrams. Phase 2 supports circular layout and a
//! deterministic spring-force simulation seeded from a circular layout.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::{GraphLayout, GraphSeries};
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, vec2};
use std::f32::consts::TAU;

pub fn render(
    p: &ChartPainter,
    s: &GraphSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.nodes.is_empty() {
        return None;
    }

    let n = s.nodes.len();
    let center = rect.center();
    let max_r = rect.size().min_elem() * 0.42;
    let inner = rect.shrink(40.0);

    // Initialize positions.
    let mut pos: Vec<Pos2> = (0..n)
        .map(|i| {
            let a = TAU * i as f32 / n as f32;
            center + vec2(a.sin(), -a.cos()) * max_r
        })
        .collect();

    if matches!(s.layout, GraphLayout::Force) {
        run_force(&mut pos, &s.nodes.len(), &s.links_idx_pairs(), inner, 80);
    }

    let mut tip: Option<TooltipDatum> = None;

    // Edges first.
    let max_link_val = s
        .links
        .iter()
        .map(|l| l.value)
        .fold(0.0_f64, f64::max)
        .max(1e-12);
    for link in &s.links {
        if link.source >= n || link.target >= n {
            continue;
        }
        let w = 0.6 + 2.5 * (link.value / max_link_val) as f32;
        let stroke_color = Color32::from_rgba_unmultiplied(
            theme.axis_line.r(),
            theme.axis_line.g(),
            theme.axis_line.b(),
            200,
        );
        p.line(
            pos[link.source],
            pos[link.target],
            Stroke::new(w, stroke_color),
        );
    }

    // Sizes.
    let max_v = s
        .nodes
        .iter()
        .map(|n| n.value)
        .fold(0.0_f64, f64::max)
        .max(1e-12);
    let min_v = s
        .nodes
        .iter()
        .map(|n| n.value)
        .fold(f64::INFINITY, f64::min);
    let font = label_font();

    for (i, node) in s.nodes.iter().enumerate() {
        let t = ((node.value - min_v) / (max_v - min_v).max(1e-12)).clamp(0.0, 1.0) as f32;
        let size = s.node_size_min + (s.node_size_max - s.node_size_min) * t;
        let color = theme.series_color(palette_offset + i);
        let hovered = hover_pos
            .map(|h| (h - pos[i]).length() <= size)
            .unwrap_or(false);
        let fill = if hovered {
            color
        } else {
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 230)
        };
        p.circle_filled(pos[i], size, fill);
        p.circle_stroke(pos[i], size, Stroke::new(1.0, theme.background));

        p.text(
            pos[i] + vec2(0.0, -size - 4.0),
            Align2::CENTER_BOTTOM,
            node.name.clone(),
            font.clone(),
            theme.text,
        );

        if hovered {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: node.name.clone(),
                data_index: i,
                value: node.value,
                color,
                screen_pos: Some(pos[i]),
            });
        }
    }

    tip
}

fn run_force(pos: &mut [Pos2], n: &usize, links: &[(usize, usize)], rect: Rect, iterations: usize) {
    let n = *n;
    if n == 0 {
        return;
    }
    let area = rect.width() * rect.height();
    let k = (area / n as f32).sqrt() * 0.6;
    let mut t = (rect.width().min(rect.height())) * 0.1;
    let cooling = 0.92_f32;

    for _ in 0..iterations {
        let mut disp = vec![egui::Vec2::ZERO; n];

        // Repulsion.
        for i in 0..n {
            for j in (i + 1)..n {
                let delta = pos[i] - pos[j];
                let d = delta.length().max(1e-3);
                let force = k * k / d;
                let dv = delta / d * force;
                disp[i] += dv;
                disp[j] -= dv;
            }
        }
        // Attraction along links.
        for &(a, b) in links {
            if a >= n || b >= n {
                continue;
            }
            let delta = pos[a] - pos[b];
            let d = delta.length().max(1e-3);
            let force = d * d / k;
            let dv = delta / d * force;
            disp[a] -= dv;
            disp[b] += dv;
        }
        // Integrate + clamp.
        for i in 0..n {
            let l = disp[i].length().max(1e-3);
            pos[i] += disp[i] / l * l.min(t);
            pos[i].x = pos[i].x.clamp(rect.min.x, rect.max.x);
            pos[i].y = pos[i].y.clamp(rect.min.y, rect.max.y);
        }
        t *= cooling;
    }
}

impl GraphSeries {
    /// Internal helper — flatten `links` to `(src, tgt)` pairs for the layout
    /// routines.
    pub(crate) fn links_idx_pairs(&self) -> Vec<(usize, usize)> {
        self.links.iter().map(|l| (l.source, l.target)).collect()
    }
}
