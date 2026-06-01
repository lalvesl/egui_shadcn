//! Scatter / bubble series.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::ScatterSeries;
use crate::render::ChartPainter;
use crate::render::shapes::{symbol, symbol_outline};
use crate::theme::ChartTheme;
use egui::{Color32, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &ScatterSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    let mut nearest: Option<(usize, f32, egui::Pos2, f64, f64, f32)> = None;
    let hover_screen = hover.map(|d| layout.to_screen(d));

    for (i, &(x, y, sz)) in s.data.iter().enumerate() {
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        let pt = layout.to_screen(DataPoint { x, y });
        let size = sz.unwrap_or(s.symbol_size);
        symbol(p, s.symbol, pt, size, color);

        if let Some(h) = hover_screen {
            let d = (pt - h).length();
            if d <= size {
                let best = nearest.as_ref().map(|n| n.1).unwrap_or(f32::INFINITY);
                if d < best {
                    nearest = Some((i, d, pt, x, y, size));
                }
            }
        }
    }

    let _ = theme;
    if let Some((i, _, pt, _, _, size)) = nearest {
        symbol_outline(
            p,
            s.symbol,
            pt,
            size + 4.0,
            Stroke::new(1.5, color),
            Color32::TRANSPARENT,
        );
        return Some(TooltipDatum {
            series_index: series_idx,
            series_name: s.name.clone(),
            data_index: i,
            value: s.data[i].1,
            color,
            screen_pos: Some(pt),
        });
    }
    None
}
