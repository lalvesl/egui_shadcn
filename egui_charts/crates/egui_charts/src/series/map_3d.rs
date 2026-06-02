//! Extruded map — each region becomes a prism whose height encodes the
//! value. Side faces shaded darker; top face uses the sequential ramp.

use crate::coord::atlas::world_continents;
use crate::coord::three_d::{ThreeDLayout, Vec3};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::Map3DSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke};
use std::cmp::Ordering;

pub fn render(
    p: &ChartPainter,
    s: &Map3DSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    _palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let layout = ThreeDLayout::fit(rect);

    // Use the built-in atlas if user supplied no regions.
    #[allow(clippy::type_complexity)]
    let regions: Vec<(String, Vec<(f64, f64)>, f64)> = if s.regions.is_empty() {
        world_continents()
            .into_iter()
            .enumerate()
            .map(|(i, (name, pts))| (name.to_string(), pts, (i as f64 * 13.0 % 60.0) + 20.0))
            .collect()
    } else {
        s.regions
            .iter()
            .map(|r| {
                let v = r.value.unwrap_or(0.0);
                let ring = r.paths.first().cloned().unwrap_or_default();
                (r.name.clone(), ring, v)
            })
            .collect()
    };

    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, _, v) in &regions {
        vmin = vmin.min(*v);
        vmax = vmax.max(*v);
    }
    let vrange = (vmax - vmin).max(1e-12);
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    // Map (lon, lat) → world unit cube ([-1, 1]² on XZ ground).
    let to_world = |lon: f64, lat: f64, y: f32| -> Vec3 {
        let wx = (lon as f32 / 180.0).clamp(-1.0, 1.0);
        let wz = (-lat as f32 / 90.0).clamp(-1.0, 1.0);
        Vec3::new(wx, y, wz)
    };

    let mut faces: Vec<(f32, Vec<Pos2>, Color32, Stroke, usize)> = Vec::new();

    // Floor.
    let p00 = layout.project(Vec3::new(-1.0, 0.0, -1.0));
    let p10 = layout.project(Vec3::new(1.0, 0.0, -1.0));
    let p11 = layout.project(Vec3::new(1.0, 0.0, 1.0));
    let p01 = layout.project(Vec3::new(-1.0, 0.0, 1.0));
    faces.push((
        (p00.1 + p10.1 + p11.1 + p01.1) * 0.25 + 1000.0,
        vec![p00.0, p10.0, p11.0, p01.0],
        Color32::from_rgba_unmultiplied(
            theme.surface.r(),
            theme.surface.g(),
            theme.surface.b(),
            180,
        ),
        Stroke::new(0.6, theme.axis_line),
        usize::MAX,
    ));

    for (region_idx, (_, ring, value)) in regions.iter().enumerate() {
        if ring.len() < 3 {
            continue;
        }
        let t = ((value - vmin) / vrange).clamp(0.0, 1.0);
        let top_color = if palette.is_empty() {
            Color32::from_gray(180)
        } else {
            palette[((t * last as f64).round() as usize).min(last)]
        };
        let side_color = Color32::from_rgb(
            ((top_color.r() as f32) * 0.65) as u8,
            ((top_color.g() as f32) * 0.65) as u8,
            ((top_color.b() as f32) * 0.65) as u8,
        );

        let h = (t as f32) * s.height_scale.max(0.01);

        // Project base & top rings.
        let base: Vec<(Pos2, f32)> = ring
            .iter()
            .map(|(lo, la)| layout.project(to_world(*lo, *la, 0.0)))
            .collect();
        let top: Vec<(Pos2, f32)> = ring
            .iter()
            .map(|(lo, la)| layout.project(to_world(*lo, *la, h)))
            .collect();

        // Side faces (one quad per pair of adjacent ring vertices).
        let n = ring.len();
        for i in 0..n {
            let j = (i + 1) % n;
            let poly = vec![base[i].0, base[j].0, top[j].0, top[i].0];
            let depth = (base[i].1 + base[j].1 + top[i].1 + top[j].1) * 0.25;
            faces.push((
                depth,
                poly,
                side_color,
                Stroke::new(0.5, theme.background),
                region_idx,
            ));
        }
        // Top face — fan-triangulate around centroid.
        let centroid_top = top.iter().fold(Pos2::ZERO, |a, b| a + b.0.to_vec2());
        let centroid_top = Pos2::new(centroid_top.x / n as f32, centroid_top.y / n as f32);
        let centroid_depth = top.iter().map(|t| t.1).sum::<f32>() / n as f32;
        for i in 0..n {
            let a = top[i].0;
            let b = top[(i + 1) % n].0;
            let depth = (top[i].1 + top[(i + 1) % n].1 + centroid_depth) / 3.0;
            faces.push((
                depth - 0.001,
                vec![centroid_top, a, b],
                top_color,
                Stroke::NONE,
                region_idx,
            ));
        }
    }

    faces.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap_or(Ordering::Equal));

    let mut hovered_region: Option<usize> = None;
    for (_, poly, fill, stroke, idx) in &faces {
        p.poly(poly.clone(), *fill, *stroke);
        if let Some(h) = hover_pos
            && *idx != usize::MAX && polygon_contains(h, poly)
        {
            hovered_region = Some(*idx);
        }
    }

    if let Some(i) = hovered_region {
        let (name, _, v) = &regions[i];
        return Some(TooltipDatum {
            series_index: series_idx,
            series_name: name.clone(),
            data_index: i,
            value: *v,
            color: palette
                .get(((((v - vmin) / vrange) as f32 * last as f32).round() as usize).min(last))
                .copied()
                .unwrap_or(theme.text),
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
