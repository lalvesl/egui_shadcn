//! Candlestick (OHLC) series. Each datum draws a high/low wick and a
//! body spanning open→close. Up days (close ≥ open) use the bullish color,
//! down days the bearish color.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::CandlestickSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &CandlestickSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    let up_color = s.up_color.unwrap_or(Color32::from_rgb(0x16, 0xa3, 0x4a));
    let down_color = s.down_color.unwrap_or(Color32::from_rgb(0xdc, 0x26, 0x26));

    let slot_w = layout.plot_rect.width() / s.data.len().max(1) as f32;
    let body_w = (slot_w * s.body_ratio).max(2.0);
    let hovered_idx = hover.map(|h| h.x.round() as i64);
    let mut tip: Option<TooltipDatum> = None;

    for (i, c) in s.data.iter().enumerate() {
        let center_x = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: 0.0,
            })
            .x;
        let high_y = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: c.high,
            })
            .y;
        let low_y = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: c.low,
            })
            .y;
        let open_y = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: c.open,
            })
            .y;
        let close_y = layout
            .to_screen(DataPoint {
                x: i as f64,
                y: c.close,
            })
            .y;

        let up = c.close >= c.open;
        let body_color = if up { up_color } else { down_color };

        p.line(
            Pos2::new(center_x, high_y),
            Pos2::new(center_x, low_y),
            Stroke::new(1.0, body_color),
        );

        let rect = Rect::from_min_max(
            Pos2::new(center_x - body_w * 0.5, open_y.min(close_y)),
            Pos2::new(center_x + body_w * 0.5, open_y.max(close_y)),
        );
        if rect.height() < 1.0 {
            // Doji — degenerate body. Draw a thin line.
            p.line(
                Pos2::new(rect.min.x, rect.center().y),
                Pos2::new(rect.max.x, rect.center().y),
                Stroke::new(1.5, body_color),
            );
        } else {
            p.rect_filled(rect, body_color);
        }

        let is_hovered = hovered_idx == Some(i as i64);
        if is_hovered {
            p.painter.rect_stroke(
                rect,
                0.0,
                Stroke::new(1.5, theme.text),
                egui::StrokeKind::Inside,
            );
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: format!("{} (O/H/L/C)", s.name),
                data_index: i,
                value: c.close,
                color: body_color,
                screen_pos: Some(Pos2::new(center_x, rect.min.y)),
            });
        }
    }

    let _ = color;
    tip
}
