//! Lines on the geo plane. Each line is drawn as a quadratic-bezier arc whose
//! midpoint is lifted perpendicular to the chord by `arc_height`.

use crate::coord::atlas::world_continents;
use crate::coord::geo::{GeoBbox, GeoLayout};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::LinesGeoSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &LinesGeoSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.lines.is_empty() {
        return None;
    }
    let bbox = if let Some((minl, maxl, mina, maxa)) = s.bbox {
        GeoBbox { min_lon: minl, max_lon: maxl, min_lat: mina, max_lat: maxa }
    } else {
        GeoBbox::world()
    };
    let layout = GeoLayout::fit(rect, bbox);

    // Faint atlas background so lines have geographic context.
    let atlas_color = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        40,
    );
    for (_, ring) in world_continents() {
        if ring.len() < 3 {
            continue;
        }
        let pts: Vec<Pos2> = ring.iter().map(|(lo, la)| layout.project(*lo, *la)).collect();
        let centroid = pts.iter().fold(Pos2::ZERO, |a, b| Pos2::new(a.x + b.x, a.y + b.y));
        let centroid = Pos2::new(centroid.x / pts.len() as f32, centroid.y / pts.len() as f32);
        for i in 0..pts.len() {
            let a = pts[i];
            let b = pts[(i + 1) % pts.len()];
            p.poly(vec![centroid, a, b], atlas_color, Stroke::NONE);
        }
    }

    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for l in &s.lines {
        vmin = vmin.min(l.value);
        vmax = vmax.max(l.value);
    }
    let vrange = (vmax - vmin).max(1e-12);
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    let mut tip: Option<TooltipDatum> = None;
    let mut best_dist = f32::INFINITY;

    for (li, line) in s.lines.iter().enumerate() {
        if line.points.len() < 2 {
            continue;
        }
        let t = ((line.value - vmin) / vrange).clamp(0.0, 1.0);
        let mapped = palette[((t * last as f64).round() as usize).min(last)];
        let stroke_color = if palette.is_empty() {
            theme.series_color(palette_offset + li)
        } else {
            mapped
        };

        let pts: Vec<Pos2> = line
            .points
            .iter()
            .map(|(lo, la)| layout.project(*lo, *la))
            .collect();

        // Treat each adjacent pair as an arc.
        for pair in pts.windows(2) {
            let a = pair[0];
            let b = pair[1];
            let mid = Pos2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5);
            let v = b - a;
            let perp = egui::vec2(-v.y, v.x).normalized();
            let height = line.arc_height * v.length();
            let ctrl = mid + perp * height;
            draw_quadratic(p, a, ctrl, b, 24, Stroke::new(s.line_width, stroke_color));

            if let Some(h) = hover_pos {
                let d = (h - mid).length();
                if d < best_dist && d <= 12.0 {
                    best_dist = d;
                    tip = Some(TooltipDatum {
                        series_index: series_idx,
                        series_name: line.name.clone(),
                        data_index: li,
                        value: line.value,
                        color: stroke_color,
                        screen_pos: Some(mid),
                    });
                }
            }
        }

        if s.show_endpoints {
            for pt in &pts {
                p.circle_filled(*pt, 3.0, stroke_color);
            }
        }
    }

    tip
}

fn draw_quadratic(p: &ChartPainter, a: Pos2, c: Pos2, b: Pos2, n: usize, stroke: Stroke) {
    let mut pts = Vec::with_capacity(n + 1);
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let mt = 1.0 - t;
        let x = mt * mt * a.x + 2.0 * mt * t * c.x + t * t * b.x;
        let y = mt * mt * a.y + 2.0 * mt * t * c.y + t * t * b.y;
        pts.push(Pos2::new(x, y));
    }
    p.path(pts, stroke);
}
