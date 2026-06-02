//! Choropleth map series. Regions are filled by `value` against the sequential
//! ramp; regions without a value are drawn in the muted neutral color.

use crate::coord::atlas::world_continents;
use crate::coord::geo::{GeoBbox, GeoLayout};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::MapSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &MapSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    _palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let regions: Vec<(String, Vec<(f64, f64)>, Option<f64>)> = if s.regions.is_empty() {
        // No regions supplied → draw the world atlas as a neutral background.
        world_continents()
            .into_iter()
            .map(|(name, pts)| (name.to_string(), pts, None))
            .collect()
    } else {
        s.regions
            .iter()
            .flat_map(|r| {
                r.paths
                    .iter()
                    .map(move |path| (r.name.clone(), path.clone(), r.value))
            })
            .collect()
    };

    if regions.is_empty() {
        return None;
    }

    let bbox = if let Some((minl, maxl, mina, maxa)) = s.bbox {
        GeoBbox { min_lon: minl, max_lon: maxl, min_lat: mina, max_lat: maxa }
    } else {
        GeoBbox::world()
    };
    let layout = GeoLayout::fit(rect, bbox);

    // Auto-fit value range when not provided.
    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, _, v) in &regions {
        if let Some(vv) = v {
            vmin = vmin.min(*vv);
            vmax = vmax.max(*vv);
        }
    }
    let vmin = s.min.unwrap_or(if vmin.is_finite() { vmin } else { 0.0 });
    let vmax = s.max.unwrap_or(if vmax.is_finite() { vmax } else { 1.0 }).max(vmin + 1e-12);
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);
    let neutral = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        60,
    );
    let stroke = Stroke::new(1.0, theme.background);
    let font = label_font();

    let mut tip: Option<TooltipDatum> = None;

    for (name, ring, value) in &regions {
        if ring.len() < 3 {
            continue;
        }
        let screen_ring: Vec<Pos2> = ring
            .iter()
            .map(|(lon, lat)| layout.project(*lon, *lat))
            .collect();

        let color = if let Some(v) = value {
            let t = ((v - vmin) / (vmax - vmin)).clamp(0.0, 1.0);
            palette[((t * last as f64).round() as usize).min(last)]
        } else {
            neutral
        };

        // Fan-triangulate around centroid for safety (regions may be concave).
        let centroid = polygon_centroid(&screen_ring);
        for i in 0..screen_ring.len() {
            let a = screen_ring[i];
            let b = screen_ring[(i + 1) % screen_ring.len()];
            p.poly(vec![centroid, a, b], color, Stroke::NONE);
        }
        // Outline along the ring.
        let mut outline = screen_ring.clone();
        outline.push(outline[0]);
        p.path(outline, stroke);

        let hovered = hover_pos
            .map(|h| point_in_poly(h, &screen_ring))
            .unwrap_or(false);
        if hovered {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: name.clone(),
                data_index: 0,
                value: value.unwrap_or(0.0),
                color,
                screen_pos: Some(centroid),
            });
            // Region label on hover.
            p.text(centroid, Align2::CENTER_CENTER, name.clone(), font.clone(), theme.text);
        }
    }

    tip
}

fn polygon_centroid(pts: &[Pos2]) -> Pos2 {
    let mut sx = 0.0;
    let mut sy = 0.0;
    for p in pts {
        sx += p.x;
        sy += p.y;
    }
    let n = pts.len().max(1) as f32;
    Pos2::new(sx / n, sy / n)
}

fn point_in_poly(p: Pos2, pts: &[Pos2]) -> bool {
    let mut inside = false;
    let n = pts.len();
    if n < 3 {
        return false;
    }
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
