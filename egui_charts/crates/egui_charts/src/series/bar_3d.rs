//! 3D bar chart — each datum is an extruded box on the XZ ground plane;
//! height encodes the value. Faces are sorted back-to-front via painter's
//! algorithm on the camera-space depth of each face centroid.

use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Bar3DSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Bar3DSeries,
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

    let xn = s.data.iter().map(|(x, _, _)| *x).max().unwrap_or(0) + 1;
    let zn = s.data.iter().map(|(_, z, _)| *z).max().unwrap_or(0) + 1;
    let max_h = s
        .data
        .iter()
        .map(|(_, _, h)| h.max(0.0))
        .fold(0.0_f64, f64::max)
        .max(1e-12);

    let wx = |i: usize| (i as f32 / xn.max(1) as f32) * 2.0 - 1.0;
    let wz = |i: usize| (i as f32 / zn.max(1) as f32) * 2.0 - 1.0;
    let wh = |h: f64| ((h / max_h) as f32) * 1.5;
    let bw = s.bar_size / xn.max(1) as f32;
    let bd = s.bar_size / zn.max(1) as f32;

    let mut faces: Vec<(f32, Vec<Pos2>, Color32, Stroke, usize)> = Vec::new();

    // Floor grid for context.
    let grid_color = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        70,
    );
    let p00 = layout.project(Vec3::new(-1.0, 0.0, -1.0));
    let p10 = layout.project(Vec3::new(1.0, 0.0, -1.0));
    let p11 = layout.project(Vec3::new(1.0, 0.0, 1.0));
    let p01 = layout.project(Vec3::new(-1.0, 0.0, 1.0));
    faces.push((
        (p00.1 + p10.1 + p11.1 + p01.1) * 0.25 + 1000.0, // push behind everything
        vec![p00.0, p10.0, p11.0, p01.0],
        Color32::from_rgba_unmultiplied(
            theme.surface.r(),
            theme.surface.g(),
            theme.surface.b(),
            180,
        ),
        Stroke::new(0.8, grid_color),
        usize::MAX,
    ));

    for (data_idx, &(xi, zi, h)) in s.data.iter().enumerate() {
        if h <= 0.0 {
            continue;
        }
        let x0 = wx(xi) - bw;
        let x1 = wx(xi) + bw;
        let z0 = wz(zi) - bd;
        let z1 = wz(zi) + bd;
        let y0 = 0.0;
        let y1 = wh(h);
        let color = theme.series_color(palette_offset + (xi + zi * 5));

        let c = [
            Vec3::new(x0, y0, z0),
            Vec3::new(x1, y0, z0),
            Vec3::new(x1, y0, z1),
            Vec3::new(x0, y0, z1),
            Vec3::new(x0, y1, z0),
            Vec3::new(x1, y1, z0),
            Vec3::new(x1, y1, z1),
            Vec3::new(x0, y1, z1),
        ];
        let pr: [(Pos2, f32); 8] = std::array::from_fn(|i| layout.project(c[i]));

        let shaded = |shade: f32| {
            let s_clamped = shade.clamp(0.2, 1.0);
            Color32::from_rgb(
                ((color.r() as f32) * s_clamped).clamp(0.0, 255.0) as u8,
                ((color.g() as f32) * s_clamped).clamp(0.0, 255.0) as u8,
                ((color.b() as f32) * s_clamped).clamp(0.0, 255.0) as u8,
            )
        };

        let face_specs: [([usize; 4], f32); 6] = [
            ([0, 1, 2, 3], 0.55), // bottom (rarely visible)
            ([4, 5, 6, 7], 1.0),  // top
            ([0, 1, 5, 4], 0.78), // front (-Z)
            ([1, 2, 6, 5], 0.85), // right (+X)
            ([2, 3, 7, 6], 0.7),  // back (+Z)
            ([3, 0, 4, 7], 0.65), // left (-X)
        ];
        for (idxs, shade) in face_specs.iter() {
            let depth = idxs.iter().map(|i| pr[*i].1).sum::<f32>() * 0.25;
            let poly = idxs.iter().map(|i| pr[*i].0).collect::<Vec<_>>();
            faces.push((
                depth,
                poly,
                shaded(*shade),
                Stroke::new(0.6, theme.background),
                data_idx,
            ));
        }
    }

    faces.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    let mut hovered_data_idx: Option<usize> = None;
    for (_, poly, fill, stroke, idx) in &faces {
        p.poly(poly.clone(), *fill, *stroke);
        if let Some(h) = hover_pos
            && *idx != usize::MAX
            && polygon_contains(h, poly)
        {
            hovered_data_idx = Some(*idx);
        }
    }

    if let Some(i) = hovered_data_idx {
        let (xi, zi, v) = s.data[i];
        let color = theme.series_color(palette_offset + (xi + zi * 5));
        return Some(TooltipDatum {
            series_index: series_idx,
            series_name: format!("{} @ ({}, {})", s.name, xi, zi),
            data_index: i,
            value: v,
            color,
            screen_pos: None,
        });
    }
    None
}

fn polygon_contains(p: Pos2, pts: &[Pos2]) -> bool {
    if pts.len() < 3 {
        return false;
    }
    let mut inside = false;
    let n = pts.len();
    let mut j = n - 1;
    for i in 0..n {
        let pi = pts[i];
        let pj = pts[j];
        if (pi.y > p.y) != (pj.y > p.y) {
            let slope = (p.x - pi.x) * (pj.y - pi.y) - (pj.x - pi.x) * (p.y - pi.y);
            if (slope < 0.0) != (pj.y < pi.y) {
                inside = !inside;
            }
        }
        j = i;
    }
    inside
}
