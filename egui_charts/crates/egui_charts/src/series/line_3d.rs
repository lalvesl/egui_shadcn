//! 3D polyline. Each segment is drawn back-to-front so the line correctly
//! occludes itself when looping or self-intersecting.

use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Line3DSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Pos2, Rect, Stroke};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Line3DSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.data.len() < 2 {
        return None;
    }
    let layout = ThreeDLayout::fit(rect);

    let (mut xmin, mut xmax, mut ymin, mut ymax, mut zmin, mut zmax) = (
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
        f64::INFINITY,
        f64::NEG_INFINITY,
    );
    for &(x, y, z) in &s.data {
        xmin = xmin.min(x);
        xmax = xmax.max(x);
        ymin = ymin.min(y);
        ymax = ymax.max(y);
        zmin = zmin.min(z);
        zmax = zmax.max(z);
    }
    let norm = |v: f64, lo: f64, hi: f64| -> f32 {
        let r = (hi - lo).max(1e-12);
        (((v - lo) / r) * 2.0 - 1.0) as f32
    };

    let color = theme.series_color(palette_offset);
    let projected: Vec<(Pos2, f32)> = s
        .data
        .iter()
        .map(|&(x, y, z)| {
            layout.project(Vec3::new(
                norm(x, xmin, xmax),
                norm(y, ymin, ymax),
                norm(z, zmin, zmax),
            ))
        })
        .collect();

    let mut segs: Vec<(f32, [Pos2; 2])> = Vec::with_capacity(projected.len());
    for w in projected.windows(2) {
        let depth = (w[0].1 + w[1].1) * 0.5;
        segs.push((depth, [w[0].0, w[1].0]));
    }
    segs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    let stroke = Stroke::new(s.line_width, color);
    for (_, [a, b]) in &segs {
        p.line(*a, *b, stroke);
    }

    let mut tip: Option<TooltipDatum> = None;
    if let Some(h) = hover_pos {
        let mut best = f32::INFINITY;
        for (i, (pt, _)) in projected.iter().enumerate() {
            let d = (h - *pt).length();
            if d < best && d <= 8.0 {
                best = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.name.clone(),
                    data_index: i,
                    value: s.data[i].1,
                    color,
                    screen_pos: Some(*pt),
                });
            }
        }
    }
    tip
}
