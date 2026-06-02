//! Pictorial bar — bar height is rendered as a vertical stack of `symbol`s
//! rather than a solid rectangle. Each glyph represents `unit_value`.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::PictorialBarSeries;
use crate::render::ChartPainter;
use crate::render::shapes::symbol;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2};

pub fn render(
    p: &ChartPainter,
    s: &PictorialBarSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    _theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }
    let unit = s.unit_value.max(1e-9);
    let baseline_y = layout
        .to_screen(DataPoint { x: 0.0, y: 0.0 })
        .y
        .clamp(layout.plot_rect.min.y, layout.plot_rect.max.y);

    let hovered_idx = hover.map(|h| h.x.round() as i64);
    let mut tip: Option<TooltipDatum> = None;

    for (i, &v) in s.data.iter().enumerate() {
        if !v.is_finite() || v <= 0.0 {
            continue;
        }
        let center_x = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: 0.0,
            })
            .x;
        let top_y = layout.to_screen(DataPoint { x: i as f64, y: v }).y;
        let height = (baseline_y - top_y).abs().max(1.0);
        let glyph_count = ((v / unit).ceil() as usize).max(1);
        let pitch = height / glyph_count as f32;

        for g in 0..glyph_count {
            let cy = baseline_y - (g as f32 + 0.5) * pitch;
            symbol(
                p,
                s.symbol,
                Pos2::new(center_x, cy),
                s.symbol_size.min(pitch * 0.9),
                color,
            );
        }

        if hovered_idx == Some(i as i64) {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: s.name.clone(),
                data_index: i,
                value: v,
                color,
                screen_pos: Some(Pos2::new(center_x, top_y)),
            });
        }
    }

    let _ = Color32::TRANSPARENT;
    tip
}
