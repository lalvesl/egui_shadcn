//! Heatmap — colored cells on a (category, category) grid.
//!
//! Cell color comes from the theme `sequential` ramp by normalizing `value`
//! to `[0,1]` against `min`/`max` (auto-fit when omitted).

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::HeatmapSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, StrokeKind, vec2};

pub fn render(
    p: &ChartPainter,
    s: &HeatmapSeries,
    series_idx: usize,
    _color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    let (mut data_min, mut data_max) = (f64::INFINITY, f64::NEG_INFINITY);
    for &(_, _, v) in &s.data {
        if v.is_finite() {
            data_min = data_min.min(v);
            data_max = data_max.max(v);
        }
    }
    let vmin = s.min.unwrap_or(data_min);
    let vmax = s.max.unwrap_or(data_max).max(vmin + 1e-12);

    let xn = s.data.iter().map(|(x, _, _)| *x).max().unwrap_or(0) + 1;
    let yn = s.data.iter().map(|(_, y, _)| *y).max().unwrap_or(0) + 1;
    let cell_w = layout.plot_rect.width() / xn as f32;
    let cell_h = layout.plot_rect.height() / yn as f32;

    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;
    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);

    let hover_xy = hover.map(|h| (h.x.round() as i64, h.y.round() as i64));

    for &(x, y, v) in &s.data {
        if !v.is_finite() {
            continue;
        }
        let t = ((v - vmin) / (vmax - vmin)).clamp(0.0, 1.0);
        let pidx = ((t * last as f64).round() as usize).min(last);
        let color = palette[pidx];

        let center = layout.to_screen(DataPoint {
            x: x as f64,
            y: y as f64,
        });
        let rect = Rect::from_center_size(center, vec2(cell_w, cell_h));
        let inner = rect.shrink(0.5);
        p.rect_filled(inner, color);

        let is_hovered = hover_xy == Some((x as i64, y as i64));
        if is_hovered {
            p.painter.rect_stroke(
                inner,
                0.0,
                Stroke::new(2.0, theme.text),
                StrokeKind::Inside,
            );
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: format!("{} @ ({}, {})", s.name, x, y),
                data_index: x + y * xn,
                value: v,
                color,
                screen_pos: Some(center),
            });
        }

        if s.show_values && cell_w > 22.0 && cell_h > 14.0 {
            let lum = luminance(color);
            let txt = if lum > 0.55 {
                Color32::BLACK
            } else {
                Color32::WHITE
            };
            p.text(
                center,
                Align2::CENTER_CENTER,
                format!("{:.1}", v),
                font.clone(),
                txt,
            );
        }
    }

    let _ = Pos2::ZERO;
    tip
}

fn luminance(c: Color32) -> f32 {
    let r = c.r() as f32 / 255.0;
    let g = c.g() as f32 / 255.0;
    let b = c.b() as f32 / 255.0;
    0.2126 * r + 0.7152 * g + 0.0722 * b
}
