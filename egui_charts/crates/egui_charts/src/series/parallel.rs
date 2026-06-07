//! Parallel coordinates — one vertical axis per dimension, each line crosses
//! all axes at the y for that dimension's value.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::ParallelSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &ParallelSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.axes.len() < 2 || s.lines.is_empty() {
        return None;
    }
    let inner = rect.shrink2(egui::vec2(32.0, 24.0));
    let n_axes = s.axes.len();
    let axis_x: Vec<f32> = (0..n_axes)
        .map(|i| {
            inner.min.x + (inner.width()) * (i as f32 / (n_axes - 1) as f32)
        })
        .collect();

    let top = inner.min.y + 8.0;
    let bot = inner.max.y - 18.0;
    let font = label_font();
    let axis_stroke = Stroke::new(1.0, theme.axis_line);

    // Draw axes + min/max labels.
    for (i, axis) in s.axes.iter().enumerate() {
        let x = axis_x[i];
        p.line(Pos2::new(x, top), Pos2::new(x, bot), axis_stroke);
        p.text(
            Pos2::new(x, bot + 4.0),
            Align2::CENTER_TOP,
            axis.name.clone(),
            font.clone(),
            theme.text_dim,
        );
        p.text(
            Pos2::new(x, top - 4.0),
            Align2::CENTER_BOTTOM,
            format_v(axis.max),
            font.clone(),
            theme.text_dim,
        );
        p.text(
            Pos2::new(x, bot - 4.0),
            Align2::CENTER_BOTTOM,
            format_v(axis.min),
            font.clone(),
            theme.text_dim,
        );
    }

    let mut tip: Option<TooltipDatum> = None;
    let mut best_dist = f32::INFINITY;

    for (li, line) in s.lines.iter().enumerate() {
        let color = theme.series_color(palette_offset + li);
        let mut pts: Vec<Pos2> = Vec::with_capacity(n_axes);
        for (ai, val) in line.values.iter().enumerate() {
            if ai >= n_axes {
                break;
            }
            let axis = &s.axes[ai];
            let range = (axis.max - axis.min).max(1e-12);
            let t = ((val - axis.min) / range).clamp(0.0, 1.0) as f32;
            let y = bot - t * (bot - top);
            pts.push(Pos2::new(axis_x[ai], y));
        }
        if pts.len() < 2 {
            continue;
        }
        let alpha: u8 = if hover_pos.is_some() { 200 } else { 230 };
        let stroke_color = Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            alpha,
        );
        p.path(pts.clone(), Stroke::new(1.4, stroke_color));

        if let Some(h) = hover_pos {
            for w in pts.windows(2) {
                let d = dist_seg(h, w[0], w[1]);
                if d < best_dist && d <= 6.0 {
                    best_dist = d;
                    tip = Some(TooltipDatum {
                        series_index: series_idx + li,
                        series_name: line.name.clone(),
                        data_index: li,
                        value: line.values.first().copied().unwrap_or(0.0),
                        color,
                        screen_pos: Some(w[0]),
                    });
                }
            }
        }
    }

    tip
}

fn format_v(v: f64) -> String {
    if v.fract().abs() < 1e-6 {
        format!("{:.0}", v)
    } else {
        format!("{:.1}", v)
    }
}

fn dist_seg(p: Pos2, a: Pos2, b: Pos2) -> f32 {
    let ab = b - a;
    let len2 = ab.length_sq();
    if len2 < 1e-6 {
        return (p - a).length();
    }
    let t = ((p - a).dot(ab) / len2).clamp(0.0, 1.0);
    let proj = a + ab * t;
    (p - proj).length()
}
