//! Pie / doughnut / rose (nightingale) series.
//!
//! Renders annular sectors in a polar layout. Doughnut = positive
//! `inner_ratio`; rose = `radius` proportional to slice value rather than
//! sweep angle.

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::PieSeries;
use crate::render::ChartPainter;
use crate::render::shapes::annular_sector;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, vec2};
use std::f32::consts::{PI, TAU};

#[derive(Clone, Debug)]
pub struct PieHover {
    pub slice_index: usize,
    pub value: f64,
    pub center_screen: Pos2,
}

/// Convert chart-convention angle (0 = up, +CW) to the screen-math angle
/// expected by `annular_sector` (0 = +X axis, +CW on screen).
fn chart_to_screen(angle_chart: f32) -> f32 {
    angle_chart - PI * 0.5
}

pub fn render(
    p: &ChartPainter,
    s: &PieSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }
    let polar = PolarLayout::fit(rect, s.outer_ratio);
    let outer_r = polar.outer_radius;
    let inner_r = outer_r * s.inner_ratio;

    let total: f64 = s.data.iter().map(|d| d.value.max(0.0)).sum();
    if total <= 0.0 {
        return None;
    }
    let max_val: f64 = s
        .data
        .iter()
        .map(|d| d.value.max(0.0))
        .fold(f64::NEG_INFINITY, f64::max)
        .max(1e-12);

    let pad = s.pad_angle.to_radians();
    let mut start_chart = 0.0_f32;
    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;

    for (i, datum) in s.data.iter().enumerate() {
        let v = datum.value.max(0.0);
        if v == 0.0 {
            continue;
        }
        let sweep = if s.rose {
            // Rose: equal angular slices, radius scales with value.
            TAU / s.data.len() as f32
        } else {
            (v / total) as f32 * TAU
        };
        let end_chart = start_chart + sweep;
        let inner_pad = pad.min(sweep * 0.45);
        let a0 = chart_to_screen(start_chart + inner_pad * 0.5);
        let a1 = chart_to_screen(end_chart - inner_pad * 0.5);

        let slice_outer = if s.rose {
            outer_r * ((v / max_val) as f32).sqrt().clamp(0.15, 1.0)
        } else {
            outer_r
        };

        let color = theme.series_color(palette_offset + i);
        let hovered = hover_pos
            .map(|h| {
                let v = h - polar.center;
                let d2 = v.length_sq();
                if d2 < inner_r * inner_r || d2 > slice_outer * slice_outer {
                    return false;
                }
                let ang_screen = v.y.atan2(v.x);
                // Map screen-math back to chart (0=up, +CW): chart = screen + π/2.
                let mut ang = ang_screen + PI * 0.5;
                while ang < 0.0 {
                    ang += TAU;
                }
                while ang >= TAU {
                    ang -= TAU;
                }
                let mut s_norm = start_chart;
                let mut e_norm = end_chart;
                while s_norm < 0.0 {
                    s_norm += TAU;
                    e_norm += TAU;
                }
                ang >= s_norm && ang <= e_norm
            })
            .unwrap_or(false);

        let fill = if hovered {
            color
        } else if hover_pos.is_some() {
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 220)
        } else {
            color
        };

        let stroke = if hovered {
            Stroke::new(2.0, theme.background)
        } else {
            Stroke::new(1.0, theme.background)
        };

        annular_sector(p, polar.center, inner_r, slice_outer, a0, a1, 64, fill, stroke);

        // Hovered slice "pops" with an extra ring outline.
        if hovered {
            annular_sector(
                p,
                polar.center,
                slice_outer + 1.0,
                slice_outer + 3.0,
                a0,
                a1,
                48,
                color,
                Stroke::NONE,
            );
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: datum.name.clone(),
                data_index: i,
                value: datum.value,
                color,
                screen_pos: None,
            });
        }

        // Label outside slice, with connector.
        if s.show_labels {
            let mid_chart = (start_chart + end_chart) * 0.5;
            let dir = polar.dir(mid_chart);
            let connector_start = polar.center + dir * (slice_outer + 2.0);
            let connector_end = polar.center + dir * (slice_outer + 16.0);
            p.line(connector_start, connector_end, Stroke::new(1.0, theme.text_dim));
            let label_anchor = connector_end + dir * 2.0;
            let anchor = if dir.x.abs() > 0.5 {
                if dir.x > 0.0 { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
            } else if dir.y < 0.0 {
                Align2::CENTER_BOTTOM
            } else {
                Align2::CENTER_TOP
            };
            let label = format!("{}  {:.0}%", datum.name, v / total * 100.0);
            p.text(label_anchor, anchor, label, font.clone(), theme.text);
        }

        start_chart = end_chart;
    }

    // Center label for doughnuts.
    if s.inner_ratio > 0.25 {
        p.text(
            polar.center,
            Align2::CENTER_CENTER,
            s.name.clone(),
            font.clone(),
            theme.text_dim,
        );
    }

    let _ = vec2;
    tip
}

/// Convenience used by widget when a series is selected from a palette by
/// per-slice index (so the palette wraps around the wheel even within one pie).
pub fn dispatch(
    p: &ChartPainter,
    s: &PieSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    render(p, s, series_idx, rect, theme, 0, hover_pos)
}
