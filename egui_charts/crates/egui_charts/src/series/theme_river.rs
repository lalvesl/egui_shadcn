//! Theme river / stream graph — stacked bands centered around y = 0, smoothed
//! with Catmull-Rom interpolation across slots.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::ThemeRiverSeries;
use crate::render::ChartPainter;
use crate::render::shapes::smooth_polyline;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &ThemeRiverSeries,
    series_idx: usize,
    _color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.bands.is_empty() {
        return None;
    }
    let n_slots = s.bands.iter().map(|b| b.data.len()).max().unwrap_or(0);
    if n_slots == 0 {
        return None;
    }

    // Per-slot totals + per-band running offsets (centered stack).
    let mut tops = vec![Vec::<f64>::with_capacity(n_slots); s.bands.len()];
    let mut bots = vec![Vec::<f64>::with_capacity(n_slots); s.bands.len()];
    for slot in 0..n_slots {
        let total: f64 = s
            .bands
            .iter()
            .map(|b| b.data.get(slot).copied().unwrap_or(0.0).max(0.0))
            .sum();
        let mut acc = -total / 2.0;
        for (bi, band) in s.bands.iter().enumerate() {
            let v = band.data.get(slot).copied().unwrap_or(0.0).max(0.0);
            bots[bi].push(acc);
            acc += v;
            tops[bi].push(acc);
        }
    }

    let hover_screen = hover.map(|d| layout.to_screen(d));
    let mut tip: Option<TooltipDatum> = None;
    let mut best_dist = f32::INFINITY;

    for (bi, band) in s.bands.iter().enumerate() {
        let band_color = theme.series_color(series_idx + bi);

        let top_pts: Vec<Pos2> = (0..n_slots)
            .map(|i| {
                layout.to_screen(DataPoint {
                    x: i as f64,
                    y: tops[bi][i],
                })
            })
            .collect();
        let bot_pts: Vec<Pos2> = (0..n_slots)
            .map(|i| {
                layout.to_screen(DataPoint {
                    x: i as f64,
                    y: bots[bi][i],
                })
            })
            .collect();

        let top_smooth = smooth_polyline(&top_pts, 12);
        let bot_smooth = smooth_polyline(&bot_pts, 12);

        // Build closed band polygon (top forward + bottom reversed).
        let mut shape: Vec<Pos2> = Vec::with_capacity(top_smooth.len() + bot_smooth.len());
        shape.extend(top_smooth.iter().copied());
        shape.extend(bot_smooth.iter().rev().copied());

        let fill =
            Color32::from_rgba_unmultiplied(band_color.r(), band_color.g(), band_color.b(), 200);
        // Decompose into trapezoids — band may be concave around y = 0.
        let trapezoid_count = top_smooth.len().min(bot_smooth.len()).saturating_sub(1);
        for i in 0..trapezoid_count {
            let poly = vec![
                top_smooth[i],
                top_smooth[i + 1],
                bot_smooth[i + 1],
                bot_smooth[i],
            ];
            p.poly(poly, fill, Stroke::NONE);
        }
        p.path(top_smooth.clone(), Stroke::new(1.0, band_color));

        if let Some(h) = hover_screen {
            for i in 0..n_slots {
                let mid_x = top_pts[i].x;
                let band_top = top_pts[i].y.min(bot_pts[i].y);
                let band_bot = top_pts[i].y.max(bot_pts[i].y);
                if (h.x - mid_x).abs() < 12.0 && h.y >= band_top && h.y <= band_bot {
                    let d = (h.x - mid_x).abs();
                    if d < best_dist {
                        best_dist = d;
                        let v = band.data.get(i).copied().unwrap_or(0.0);
                        tip = Some(TooltipDatum {
                            series_index: series_idx + bi,
                            series_name: band.name.clone(),
                            data_index: i,
                            value: v,
                            color: band_color,
                            screen_pos: Some(top_pts[i]),
                        });
                    }
                }
            }
        }
    }

    tip
}
