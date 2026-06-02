//! Polar bar — each item gets an equal angular slot, radial length encodes
//! the value.

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::PolarBarSeries;
use crate::render::ChartPainter;
use crate::render::shapes::annular_sector;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Pos2, Rect, Stroke};
use std::f32::consts::{PI, TAU};

fn chart_to_screen(a: f32) -> f32 {
    a - PI * 0.5
}

pub fn render(
    p: &ChartPainter,
    s: &PolarBarSeries,
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
    let outer = polar.outer_radius;
    let inner = outer * s.inner_ratio;

    let max_v = s
        .data
        .iter()
        .map(|d| d.value.max(0.0))
        .fold(0.0_f64, f64::max)
        .max(1e-12);

    let n = s.data.len();
    let slot = TAU / n as f32;
    let pad = s.pad_angle.to_radians().min(slot * 0.45);
    let font = label_font();

    let mut tip: Option<TooltipDatum> = None;

    for (i, datum) in s.data.iter().enumerate() {
        let v = datum.value.max(0.0);
        let mid = slot * i as f32;
        let start_chart = mid - slot * 0.5 + pad * 0.5;
        let end_chart = mid + slot * 0.5 - pad * 0.5;
        let bar_outer = inner + (outer - inner) * ((v / max_v) as f32).clamp(0.0, 1.0);
        let a0 = chart_to_screen(start_chart);
        let a1 = chart_to_screen(end_chart);

        let color = theme.series_color(palette_offset + i);
        let hovered = hover_pos
            .map(|h| {
                let v = h - polar.center;
                let d2 = v.length_sq();
                if d2 < inner * inner || d2 > bar_outer * bar_outer {
                    return false;
                }
                let mut ang = v.y.atan2(v.x) + PI * 0.5;
                while ang < 0.0 {
                    ang += TAU;
                }
                while ang >= TAU {
                    ang -= TAU;
                }
                let mut s_n = start_chart;
                let mut e_n = end_chart;
                while s_n < 0.0 {
                    s_n += TAU;
                    e_n += TAU;
                }
                ang >= s_n && ang <= e_n
            })
            .unwrap_or(false);

        annular_sector(
            p,
            polar.center,
            inner,
            bar_outer,
            a0,
            a1,
            48,
            color,
            Stroke::new(1.0, theme.background),
        );

        if hovered {
            annular_sector(
                p,
                polar.center,
                bar_outer + 1.0,
                bar_outer + 3.0,
                a0,
                a1,
                32,
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

        // Label on outside.
        let dir = polar.dir(mid);
        let label_pos = polar.center + dir * (outer + 14.0);
        let anchor = if dir.x.abs() > 0.5 {
            if dir.x > 0.0 { Align2::LEFT_CENTER } else { Align2::RIGHT_CENTER }
        } else if dir.y < 0.0 {
            Align2::CENTER_BOTTOM
        } else {
            Align2::CENTER_TOP
        };
        p.text(label_pos, anchor, datum.name.clone(), font.clone(), theme.text_dim);
    }

    tip
}
