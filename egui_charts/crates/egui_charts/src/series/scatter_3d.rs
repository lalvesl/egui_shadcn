//! 3D scatter. Points sorted by depth so closer markers occlude farther ones.

use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Scatter3DSeries;
use crate::render::ChartPainter;
use crate::render::shapes::symbol;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Scatter3DSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
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
    for &(x, y, z, _) in &s.data {
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
    let mut entries: Vec<(f32, Pos2, f32, usize)> = s
        .data
        .iter()
        .enumerate()
        .map(|(i, &(x, y, z, sz))| {
            let (pt, depth) = layout.project(Vec3::new(
                norm(x, xmin, xmax),
                norm(y, ymin, ymax),
                norm(z, zmin, zmax),
            ));
            let size = sz.unwrap_or(s.symbol_size);
            (depth, pt, size, i)
        })
        .collect();

    entries.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    for (depth, pt, size, _) in &entries {
        // Slight depth attenuation: further points fade and shrink a bit.
        let t = ((*depth - 2.0) / 4.0).clamp(0.0, 1.0);
        let attenuated_size = size * (1.0 - 0.25 * t);
        let alpha = (255.0 - 80.0 * t) as u8;
        let c = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha);
        symbol(p, s.symbol, *pt, attenuated_size, c);
    }

    let mut tip: Option<TooltipDatum> = None;
    if let Some(h) = hover_pos {
        let mut best = f32::INFINITY;
        for (_, pt, size, i) in &entries {
            let d = (h - *pt).length();
            if d < best && d <= *size {
                best = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.name.clone(),
                    data_index: *i,
                    value: s.data[*i].2,
                    color,
                    screen_pos: Some(*pt),
                });
            }
        }
    }
    tip
}
