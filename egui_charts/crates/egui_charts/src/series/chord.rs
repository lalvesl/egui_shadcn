//! Chord diagram — nodes arranged on a ring, relationships drawn as cubic
//! Bezier ribbons through the center. Each node's arc is sized in proportion
//! to its total flow (in + out).

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::ChordSeries;
use crate::render::ChartPainter;
use crate::render::shapes::annular_sector;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};
use std::f32::consts::{PI, TAU};

fn chart_to_screen(a: f32) -> f32 {
    a - PI * 0.5
}

pub fn render(
    p: &ChartPainter,
    s: &ChordSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let n = s.labels.len();
    if n == 0 || s.matrix.is_empty() {
        return None;
    }
    let polar = PolarLayout::fit(rect, 0.75);
    let outer = polar.outer_radius;
    let ring_w = (outer * s.ring_thickness).max(2.0);
    let inner = outer - ring_w;

    // Per-node totals (sum of out + in).
    let mut totals = vec![0.0_f64; n];
    for i in 0..n {
        for j in 0..n {
            let out = s.matrix.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            let inn = s.matrix.get(j).and_then(|r| r.get(i)).copied().unwrap_or(0.0);
            totals[i] += out;
            if i != j {
                totals[i] += inn;
            }
        }
    }
    let grand: f64 = totals.iter().sum::<f64>().max(1e-12);

    let pad = s.pad_angle.to_radians();
    let usable = TAU - pad * n as f32;

    // Node arc ranges (chart angles, 0 = up, +CW).
    let mut node_arcs = vec![(0.0_f32, 0.0_f32); n];
    let mut acc = 0.0_f32;
    for i in 0..n {
        let span = usable * (totals[i] / grand) as f32;
        node_arcs[i] = (acc + pad * 0.5, acc + pad * 0.5 + span);
        acc += span + pad;
    }

    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;

    // Sub-slot start positions per node (one cursor per node, refilled below).
    let mut node_cursor: Vec<f32> = node_arcs.iter().map(|(a, _)| *a).collect();

    // Pair sub-slot lookup table.
    // Each chord (i,j) consumes sub-arc on i = matrix[i][j], on j = matrix[j][i].
    let mut sub_arcs: Vec<((usize, usize), (f32, f32), (f32, f32))> = Vec::new();
    for i in 0..n {
        let (start_i, end_i) = node_arcs[i];
        let span_i = (end_i - start_i).max(1e-6);
        let total_i = totals[i].max(1e-12);
        for j in 0..n {
            let out = s.matrix.get(i).and_then(|r| r.get(j)).copied().unwrap_or(0.0);
            if out <= 0.0 {
                continue;
            }
            let frac = (out / total_i) as f32 * span_i;
            let i_arc = (node_cursor[i], node_cursor[i] + frac);
            node_cursor[i] += frac;

            // Sub-arc on j (incoming).
            let span_j = (node_arcs[j].1 - node_arcs[j].0).max(1e-6);
            let total_j = totals[j].max(1e-12);
            let frac_j = (out / total_j) as f32 * span_j;
            let j_arc = (node_cursor[j], node_cursor[j] + frac_j);
            node_cursor[j] += frac_j;

            sub_arcs.push(((i, j), i_arc, j_arc));
        }
    }

    // Draw ribbons (cubic bezier through center).
    for (idx, ((i, j), i_arc, j_arc)) in sub_arcs.iter().enumerate() {
        let color = theme.series_color(palette_offset + *i);
        let i_a = polar.point(i_arc.0, inner);
        let i_b = polar.point(i_arc.1, inner);
        let j_a = polar.point(j_arc.0, inner);
        let j_b = polar.point(j_arc.1, inner);

        let mut pts: Vec<Pos2> = Vec::with_capacity(64);
        let n_seg = 24;
        for k in 0..=n_seg {
            let t = k as f32 / n_seg as f32;
            pts.push(cubic(i_a, polar.center, polar.center, j_a, t));
        }
        for k in 0..=n_seg {
            let t = k as f32 / n_seg as f32;
            pts.push(cubic(j_b, polar.center, polar.center, i_b, t));
        }

        let fill = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 100);
        // Approximate ribbon by fanning from center.
        for q in 0..(n_seg) {
            let poly = vec![pts[q], pts[q + 1], pts[2 * (n_seg + 1) - 1 - q - 1], pts[2 * (n_seg + 1) - 1 - q]];
            p.poly(poly, fill, Stroke::NONE);
        }

        if let Some(h) = hover_pos {
            // Hover when cursor is within bounding box centered between arcs.
            let mid = Pos2::new(
                (i_a.x + j_a.x + i_b.x + j_b.x) * 0.25,
                (i_a.y + j_a.y + i_b.y + j_b.y) * 0.25,
            );
            if (h - mid).length() < 22.0 {
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: format!("{} → {}", s.labels[*i], s.labels[*j]),
                    data_index: idx,
                    value: s.matrix[*i][*j],
                    color,
                    screen_pos: Some(mid),
                });
            }
        }
    }

    // Outer label ring per node.
    for (i, (start_chart, end_chart)) in node_arcs.iter().enumerate() {
        let color = theme.series_color(palette_offset + i);
        let a0 = chart_to_screen(*start_chart);
        let a1 = chart_to_screen(*end_chart);
        annular_sector(
            p,
            polar.center,
            inner,
            outer,
            a0,
            a1,
            48,
            color,
            Stroke::new(0.6, theme.background),
        );

        let mid = (*start_chart + *end_chart) * 0.5;
        let dir = polar.dir(mid);
        let label_pos = polar.center + dir * (outer + 14.0);
        let anchor = if dir.x.abs() > 0.5 {
            if dir.x > 0.0 { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
        } else if dir.y < 0.0 {
            Align2::CENTER_BOTTOM
        } else {
            Align2::CENTER_TOP
        };
        p.text(label_pos, anchor, s.labels[i].clone(), font.clone(), theme.text);
    }

    tip
}

fn cubic(a: Pos2, b: Pos2, c: Pos2, d: Pos2, t: f32) -> Pos2 {
    let mt = 1.0 - t;
    let x = mt * mt * mt * a.x + 3.0 * mt * mt * t * b.x + 3.0 * mt * t * t * c.x + t * t * t * d.x;
    let y = mt * mt * mt * a.y + 3.0 * mt * mt * t * b.y + 3.0 * mt * t * t * c.y + t * t * t * d.y;
    Pos2::new(x, y)
}
