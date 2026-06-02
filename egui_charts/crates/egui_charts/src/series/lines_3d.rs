//! Arbitrary 3D line segments, value-colored from the sequential ramp.

use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Lines3DSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Pos2, Rect, Stroke};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Lines3DSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.segments.is_empty() {
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
    let mut vmin = f64::INFINITY;
    let mut vmax = f64::NEG_INFINITY;
    for (a, b, v) in &s.segments {
        for (x, y, z) in [a, b] {
            xmin = xmin.min(*x);
            xmax = xmax.max(*x);
            ymin = ymin.min(*y);
            ymax = ymax.max(*y);
            zmin = zmin.min(*z);
            zmax = zmax.max(*z);
        }
        vmin = vmin.min(*v);
        vmax = vmax.max(*v);
    }
    let norm = |v: f64, lo: f64, hi: f64| -> f32 {
        let r = (hi - lo).max(1e-12);
        (((v - lo) / r) * 2.0 - 1.0) as f32
    };
    let vrange = (vmax - vmin).max(1e-12);
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    let mut segs: Vec<(f32, Pos2, Pos2, egui::Color32, usize)> = Vec::new();
    for (i, (a, b, v)) in s.segments.iter().enumerate() {
        let (pa, da) = layout.project(Vec3::new(
            norm(a.0, xmin, xmax),
            norm(a.1, ymin, ymax),
            norm(a.2, zmin, zmax),
        ));
        let (pb, db) = layout.project(Vec3::new(
            norm(b.0, xmin, xmax),
            norm(b.1, ymin, ymax),
            norm(b.2, zmin, zmax),
        ));
        let t = ((v - vmin) / vrange).clamp(0.0, 1.0);
        let color = if palette.is_empty() {
            theme.series_color(palette_offset + i)
        } else {
            palette[((t * last as f64).round() as usize).min(last)]
        };
        segs.push(((da + db) * 0.5, pa, pb, color, i));
    }
    segs.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    let mut tip: Option<TooltipDatum> = None;
    let mut best = f32::INFINITY;

    for (_, a, b, color, i) in &segs {
        p.line(*a, *b, Stroke::new(s.line_width, *color));
        if let Some(h) = hover_pos {
            let d = dist_seg(h, *a, *b);
            if d < best && d <= 6.0 {
                best = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.name.clone(),
                    data_index: *i,
                    value: s.segments[*i].2,
                    color: *color,
                    screen_pos: Some(Pos2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)),
                });
            }
        }
    }
    tip
}

fn dist_seg(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let len2 = ab.length_sq();
    if len2 < 1e-6 {
        return (p - a).length();
    }
    let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
    let proj = a + ab * t;
    (p - proj).length()
}
