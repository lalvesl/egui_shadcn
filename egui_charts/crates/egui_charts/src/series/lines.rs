//! `LinesCartesian` — arbitrary `(from, to)` line segments, color-mapped by
//! `value` via the sequential palette.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::LinesCartesianSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &LinesCartesianSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.segments.is_empty() {
        return None;
    }

    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for seg in &s.segments {
        vmin = vmin.min(seg.value);
        vmax = vmax.max(seg.value);
    }
    let vrange = (vmax - vmin).max(1e-12);
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    let hover_screen = hover.map(|d| layout.to_screen(d));
    let mut tip: Option<TooltipDatum> = None;
    let mut best_dist = f32::INFINITY;

    for (i, seg) in s.segments.iter().enumerate() {
        let a = layout.to_screen(DataPoint { x: seg.from.0, y: seg.from.1 });
        let b = layout.to_screen(DataPoint { x: seg.to.0, y: seg.to.1 });

        let t = ((seg.value - vmin) / vrange).clamp(0.0, 1.0);
        let mapped = palette[((t * last as f64).round() as usize).min(last)];
        let stroke_color = if palette.is_empty() { color } else { mapped };

        p.line(a, b, Stroke::new(s.line_width, stroke_color));
        if s.show_endpoints {
            p.circle_filled(a, 2.5, stroke_color);
            p.circle_filled(b, 2.5, stroke_color);
        }

        if let Some(h) = hover_screen {
            let d = distance_point_segment(h, a, b);
            if d < best_dist && d <= 6.0 {
                best_dist = d;
                tip = Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.name.clone(),
                    data_index: i,
                    value: seg.value,
                    color: stroke_color,
                    screen_pos: Some(midpoint(a, b)),
                });
            }
        }
    }

    let _ = Color32::TRANSPARENT;
    tip
}

fn distance_point_segment(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let len2 = ab.length_sq();
    if len2 < 1e-6 {
        return (p - a).length();
    }
    let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
    let proj = a + ab * t;
    (p - proj).length()
}

fn midpoint(a: Pos2, b: Pos2) -> Pos2 {
    Pos2::new((a.x + b.x) * 0.5, (a.y + b.y) * 0.5)
}
