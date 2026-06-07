//! Effect scatter — scatter with concentric halos around each point.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::EffectScatterSeries;
use crate::render::ChartPainter;
use crate::render::shapes::{symbol, symbol_outline};
use crate::theme::ChartTheme;
use egui::{Color32, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &EffectScatterSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    _theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }
    let hover_screen = hover.map(|d| layout.to_screen(d));
    let mut nearest: Option<(usize, f32, egui::Pos2, f32, f64)> = None;

    for (i, &(x, y, sz)) in s.data.iter().enumerate() {
        if !x.is_finite() || !y.is_finite() {
            continue;
        }
        let pt = layout.to_screen(DataPoint { x, y });
        let size = sz.unwrap_or(s.symbol_size);

        // Halos behind the marker, fading by ring index.
        for r in 1..=s.ripple_count {
            let radius = size * (0.6 + r as f32 * 0.55);
            let alpha = (90 / (r as u32 + 1)).min(80) as u8;
            let halo = Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                alpha,
            );
            p.circle_filled(pt, radius, halo);
        }

        symbol(p, s.symbol, pt, size, color);

        if let Some(h) = hover_screen {
            let d = (pt - h).length();
            if d <= size * 1.4 {
                let best =
                    nearest.as_ref().map(|n| n.1).unwrap_or(f32::INFINITY);
                if d < best {
                    nearest = Some((i, d, pt, size, y));
                }
            }
        }
    }

    if let Some((i, _, pt, size, value)) = nearest {
        symbol_outline(
            p,
            s.symbol,
            pt,
            size + 6.0,
            Stroke::new(1.5, color),
            Color32::TRANSPARENT,
        );
        return Some(TooltipDatum {
            series_index: series_idx,
            series_name: s.name.clone(),
            data_index: i,
            value,
            color,
            screen_pos: Some(pt),
        });
    }
    None
}
