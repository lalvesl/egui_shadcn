//! 3D surface plot. Triangulates the height grid, shades each triangle by
//! its height in the sequential ramp, and draws back-to-front. `wireframe`
//! mode draws edges instead of fills.

use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Surface3DSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Surface3DSeries,
    _series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    _palette_offset: usize,
    _hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let rows = s.heights.len();
    if rows == 0 {
        return None;
    }
    let cols = s.heights[0].len();
    if cols < 2 || rows < 2 {
        return None;
    }
    let layout = ThreeDLayout::fit(rect);

    let mut hmin = f64::INFINITY;
    let mut hmax = f64::NEG_INFINITY;
    for row in &s.heights {
        for &v in row {
            if v.is_finite() {
                hmin = hmin.min(v);
                hmax = hmax.max(v);
            }
        }
    }
    let hrange = (hmax - hmin).max(1e-12);

    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    // Build vertex grid in world coords.
    let mut grid: Vec<Vec<(Pos2, f32, f64)>> = Vec::with_capacity(rows);
    for (zi, row) in s.heights.iter().enumerate() {
        let mut out = Vec::with_capacity(cols);
        for (xi, v) in row.iter().enumerate() {
            let wx = (xi as f32 / (cols - 1) as f32) * 2.0 - 1.0;
            let wz = (zi as f32 / (rows - 1) as f32) * 2.0 - 1.0;
            let wy = (((*v - hmin) / hrange) * 1.5 - 0.0) as f32;
            let (pt, depth) = layout.project(Vec3::new(wx, wy, wz));
            out.push((pt, depth, *v));
        }
        grid.push(out);
    }

    let mut tris: Vec<(f32, [Pos2; 3], Color32, Stroke)> = Vec::new();
    for z in 0..(rows - 1) {
        for x in 0..(cols - 1) {
            let a = grid[z][x];
            let b = grid[z][x + 1];
            let c = grid[z + 1][x + 1];
            let d = grid[z + 1][x];
            let avg_h = (a.2 + b.2 + c.2 + d.2) * 0.25;
            let t = ((avg_h - hmin) / hrange).clamp(0.0, 1.0);
            let color = if palette.is_empty() {
                Color32::from_gray(200)
            } else {
                palette[((t * last as f64).round() as usize).min(last)]
            };
            let depth1 = (a.1 + b.1 + c.1) / 3.0;
            let depth2 = (a.1 + c.1 + d.1) / 3.0;
            let stroke = if s.wireframe {
                Stroke::new(1.0, color)
            } else {
                Stroke::new(0.4, theme.background)
            };
            if s.wireframe {
                tris.push((depth1, [a.0, b.0, c.0], Color32::TRANSPARENT, stroke));
                tris.push((depth2, [a.0, c.0, d.0], Color32::TRANSPARENT, stroke));
            } else {
                tris.push((depth1, [a.0, b.0, c.0], color, stroke));
                tris.push((depth2, [a.0, c.0, d.0], color, stroke));
            }
        }
    }
    tris.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    for (_, tri, fill, stroke) in tris {
        if s.wireframe {
            p.line(tri[0], tri[1], stroke);
            p.line(tri[1], tri[2], stroke);
            p.line(tri[2], tri[0], stroke);
        } else {
            p.poly(vec![tri[0], tri[1], tri[2]], fill, stroke);
        }
    }

    None
}
