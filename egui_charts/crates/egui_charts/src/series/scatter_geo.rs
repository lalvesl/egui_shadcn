//! Scatter points on a geo background. Marker size scales with `value`.

use crate::coord::atlas::world_continents;
use crate::coord::geo::{GeoBbox, GeoLayout};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::ScatterGeoSeries;
use crate::render::ChartPainter;
use crate::render::shapes::symbol;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &ScatterGeoSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.points.is_empty() {
        return None;
    }
    let bbox = if let Some((minl, maxl, mina, maxa)) = s.bbox {
        GeoBbox { min_lon: minl, max_lon: maxl, min_lat: mina, max_lat: maxa }
    } else {
        GeoBbox::world()
    };
    let layout = GeoLayout::fit(rect, bbox);

    // Background atlas.
    let atlas_color = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        50,
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
    for pt in &s.points {
        vmin = vmin.min(pt.value);
        vmax = vmax.max(pt.value);
    }
    let vrange = (vmax - vmin).max(1e-12);
    let mut tip: Option<TooltipDatum> = None;
    let mut nearest_d = f32::INFINITY;
    let font = label_font();

    for (i, pt) in s.points.iter().enumerate() {
        let t = ((pt.value - vmin) / vrange).clamp(0.0, 1.0) as f32;
        let size = s.symbol_size_min + (s.symbol_size_max - s.symbol_size_min) * t;
        let screen = layout.project(pt.lon, pt.lat);
        let color = theme.series_color(palette_offset + i);
        // Halo + marker.
        p.circle_filled(
            screen,
            size + 4.0,
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 60),
        );
        symbol(p, s.symbol, screen, size, color);
        p.text(
            screen + egui::vec2(0.0, -(size + 6.0)),
            Align2::CENTER_BOTTOM,
            pt.name.clone(),
            font.clone(),
            theme.text,
        );

        if let Some(h) = hover_pos {
            let d = (h - screen).length();
            if d < nearest_d && d <= size + 6.0 {
                nearest_d = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: pt.name.clone(),
                    data_index: i,
                    value: pt.value,
                    color,
                    screen_pos: Some(screen),
                });
            }
        }
    }

    tip
}
