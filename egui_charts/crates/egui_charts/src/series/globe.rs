//! Globe — sphere with latitude/longitude grid + point markers. Orbit
//! camera-style projection: each `(lon, lat)` point lives on the unit sphere
//! and is rotated by `yaw`/`pitch` into camera space.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::GlobeSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};
use std::f32::consts::{PI, TAU};

struct SphereProj {
    center: Pos2,
    radius: f32,
    sin_yaw: f32,
    cos_yaw: f32,
    sin_pitch: f32,
    cos_pitch: f32,
}

impl SphereProj {
    fn new(rect: Rect, yaw: f32, pitch: f32) -> Self {
        Self {
            center: rect.center(),
            radius: rect.size().min_elem() * 0.42,
            sin_yaw: yaw.sin(),
            cos_yaw: yaw.cos(),
            sin_pitch: pitch.sin(),
            cos_pitch: pitch.cos(),
        }
    }

    fn project(&self, lon: f32, lat: f32) -> (Pos2, f32) {
        let cl = lat.cos();
        // Unit sphere vector (z out of screen before rotation).
        let x0 = cl * lon.sin();
        let y0 = lat.sin();
        let z0 = cl * lon.cos();
        // Yaw about Y, then pitch about X.
        let x1 = x0 * self.cos_yaw + z0 * self.sin_yaw;
        let z1 = -x0 * self.sin_yaw + z0 * self.cos_yaw;
        let y2 = y0 * self.cos_pitch - z1 * self.sin_pitch;
        let z2 = y0 * self.sin_pitch + z1 * self.cos_pitch;
        let pos = Pos2::new(
            self.center.x + x1 * self.radius,
            self.center.y - y2 * self.radius,
        );
        (pos, z2)
    }
}

pub fn render(
    p: &ChartPainter,
    s: &GlobeSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let proj = SphereProj::new(rect, s.yaw, s.pitch);

    // Sphere disc.
    let disc_color = Color32::from_rgba_unmultiplied(
        theme.surface.r(),
        theme.surface.g(),
        theme.surface.b(),
        220,
    );
    p.circle_filled(proj.center, proj.radius, disc_color);
    p.circle_stroke(
        proj.center,
        proj.radius,
        Stroke::new(1.0, theme.axis_line),
    );

    let grid_color = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        90,
    );

    // Meridians (every 30° lon).
    for lon_deg in (-180..=180).step_by(30) {
        let lon = (lon_deg as f32).to_radians();
        let mut pts: Vec<Pos2> = Vec::new();
        let mut prev_visible = false;
        for lat_deg in -90..=90 {
            let lat = (lat_deg as f32).to_radians();
            let (pt, depth) = proj.project(lon, lat);
            let visible = depth > -0.05;
            if visible {
                pts.push(pt);
                prev_visible = true;
            } else {
                if prev_visible && pts.len() >= 2 {
                    p.path(pts.clone(), Stroke::new(0.8, grid_color));
                }
                pts.clear();
                prev_visible = false;
            }
        }
        if pts.len() >= 2 {
            p.path(pts, Stroke::new(0.8, grid_color));
        }
    }
    // Parallels (every 30° lat).
    for lat_deg in (-60..=60).step_by(30) {
        let lat = (lat_deg as f32).to_radians();
        let mut pts: Vec<Pos2> = Vec::new();
        let mut prev_visible = false;
        for lon_deg in -180..=180 {
            let lon = (lon_deg as f32).to_radians();
            let (pt, depth) = proj.project(lon, lat);
            let visible = depth > -0.05;
            if visible {
                pts.push(pt);
                prev_visible = true;
            } else {
                if prev_visible && pts.len() >= 2 {
                    p.path(pts.clone(), Stroke::new(0.8, grid_color));
                }
                pts.clear();
                prev_visible = false;
            }
        }
        if pts.len() >= 2 {
            p.path(pts, Stroke::new(0.8, grid_color));
        }
    }

    // Continents as faint silhouettes on the visible hemisphere.
    let continent_fill = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        60,
    );
    for (_, ring) in crate::coord::atlas::world_continents() {
        if ring.len() < 3 {
            continue;
        }
        let mut polys: Vec<Vec<Pos2>> = vec![Vec::new()];
        for (lo, la) in &ring {
            let (pt, depth) = proj
                .project((*lo as f32).to_radians(), (*la as f32).to_radians());
            if depth > 0.0 {
                polys.last_mut().unwrap().push(pt);
            } else if !polys.last().unwrap().is_empty() {
                polys.push(Vec::new());
            }
        }
        for poly in polys {
            if poly.len() >= 3 {
                let centroid =
                    poly.iter().fold(Pos2::ZERO, |a, b| a + b.to_vec2());
                let centroid = Pos2::new(
                    centroid.x / poly.len() as f32,
                    centroid.y / poly.len() as f32,
                );
                for i in 0..poly.len() {
                    let a = poly[i];
                    let b = poly[(i + 1) % poly.len()];
                    p.poly(vec![centroid, a, b], continent_fill, Stroke::NONE);
                }
            }
        }
    }

    // Hemispherical shading — soft shadow on far side using a darker ring.
    p.circle_stroke(
        proj.center,
        proj.radius * 0.94,
        Stroke::new(
            proj.radius * 0.06,
            Color32::from_rgba_unmultiplied(0, 0, 0, 40),
        ),
    );

    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;
    let mut best = f32::INFINITY;

    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, _, v, _) in &s.points {
        vmin = vmin.min(*v);
        vmax = vmax.max(*v);
    }
    let vrange = (vmax - vmin).max(1e-12);

    for (i, (lon_d, lat_d, value, name)) in s.points.iter().enumerate() {
        let (pt, depth) = proj.project(
            (*lon_d as f32).to_radians(),
            (*lat_d as f32).to_radians(),
        );
        if depth < 0.0 {
            continue;
        }
        let t = ((value - vmin) / vrange).clamp(0.0, 1.0) as f32;
        let size = 4.0 + 6.0 * t;
        let color = theme.series_color(palette_offset + i);
        p.circle_filled(
            pt,
            size + 4.0,
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                50,
            ),
        );
        p.circle_filled(pt, size, color);
        p.text(
            pt + egui::vec2(0.0, -(size + 4.0)),
            Align2::CENTER_BOTTOM,
            name.clone(),
            font.clone(),
            theme.text,
        );
        if let Some(h) = hover_pos {
            let d = (h - pt).length();
            if d < best && d <= size + 6.0 {
                best = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: name.clone(),
                    data_index: i,
                    value: *value,
                    color,
                    screen_pos: Some(pt),
                });
            }
        }
    }

    let _ = PI;
    let _ = TAU;
    tip
}
