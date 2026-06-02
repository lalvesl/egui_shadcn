//! Radar / spider chart series.
//!
//! Each `RadarIndicator` becomes a spoke from the center; each `RadarDataset`
//! is a polygon overlaid on those spokes, optionally filled translucently.

use crate::coord::polar::RadarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::RadarSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, vec2};

pub fn render(
    p: &ChartPainter,
    s: &RadarSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.indicators.is_empty() {
        return None;
    }

    let indicators: Vec<(String, f64)> = s
        .indicators
        .iter()
        .map(|ind| (ind.name.clone(), ind.max.max(1e-12)))
        .collect();
    let layout = RadarLayout::fit(rect, &indicators, 0.72);
    let center = layout.polar.center;
    let outer = layout.polar.outer_radius;
    let n = layout.spokes.len();

    let font = label_font();

    // Grid rings.
    let ring_stroke = Stroke::new(1.0, theme.grid_line);
    for r in 1..=s.rings {
        let t = r as f32 / s.rings as f32;
        let radius = outer * t;
        if s.polygon_grid {
            let mut pts = Vec::with_capacity(n);
            for spoke in &layout.spokes {
                pts.push(layout.polar.point(spoke.angle_rad, radius));
            }
            // Closed polyline.
            pts.push(pts[0]);
            p.path(pts, ring_stroke);
        } else {
            // Approximate circle as 64-gon stroke.
            let mut pts = Vec::with_capacity(65);
            for i in 0..=64 {
                let a = std::f32::consts::TAU * i as f32 / 64.0;
                pts.push(center + vec2(a.sin(), -a.cos()) * radius);
            }
            p.path(pts, ring_stroke);
        }
    }

    // Spokes + labels.
    let spoke_stroke = Stroke::new(1.0, theme.axis_line);
    for spoke in &layout.spokes {
        let tip = layout.polar.point(spoke.angle_rad, outer);
        p.line(center, tip, spoke_stroke);
        let dir = layout.polar.dir(spoke.angle_rad);
        let label_anchor = center + dir * (outer + 14.0);
        let anchor = if dir.x.abs() > 0.5 {
            if dir.x > 0.0 { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
        } else if dir.y < 0.0 {
            Align2::CENTER_BOTTOM
        } else {
            Align2::CENTER_TOP
        };
        p.text(label_anchor, anchor, spoke.label.clone(), font.clone(), theme.text_dim);
    }

    // Hover hit: nearest dataset vertex within 12px.
    let mut tip: Option<TooltipDatum> = None;
    let mut best_dist2 = f32::INFINITY;

    // Datasets.
    for (ds_idx, ds) in s.datasets.iter().enumerate() {
        if ds.values.is_empty() {
            continue;
        }
        let color = theme.series_color(palette_offset + ds_idx);
        let mut polygon: Vec<Pos2> = Vec::with_capacity(n);
        for (i, spoke) in layout.spokes.iter().enumerate() {
            let v = ds.values.get(i).copied().unwrap_or(0.0).max(0.0);
            let norm = (v / spoke.max).clamp(0.0, 1.0) as f32;
            polygon.push(layout.polar.point(spoke.angle_rad, outer * norm));
        }

        // Fill (translucent).
        if polygon.len() >= 3 {
            let fill = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), ds.fill_alpha);
            // convex_polygon expects convex shape; radar polygon may be
            // non-convex, so fan-triangulate from center instead.
            for i in 0..polygon.len() {
                let a = polygon[i];
                let b = polygon[(i + 1) % polygon.len()];
                p.poly(vec![center, a, b], fill, Stroke::NONE);
            }
        }

        // Outline.
        let mut closed = polygon.clone();
        if let Some(first) = polygon.first().copied() {
            closed.push(first);
        }
        p.path(closed, Stroke::new(ds.line_width, color));

        // Vertex markers + hover hit-test.
        for (i, pt) in polygon.iter().enumerate() {
            p.circle_filled(*pt, 3.0, color);
            if let Some(h) = hover_pos {
                let d2 = (h - *pt).length_sq();
                if d2 < 144.0 && d2 < best_dist2 {
                    best_dist2 = d2;
                    let v = ds.values.get(i).copied().unwrap_or(0.0);
                    tip = Some(TooltipDatum {
                        series_index: series_idx,
                        series_name: format!(
                            "{} · {}",
                            ds.name,
                            s.indicators[i].name
                        ),
                        data_index: i,
                        value: v,
                        color,
                        screen_pos: Some(*pt),
                    });
                }
            }
        }
    }

    tip
}

pub fn dispatch(
    p: &ChartPainter,
    s: &RadarSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    render(p, s, series_idx, rect, theme, 0, hover_pos)
}
