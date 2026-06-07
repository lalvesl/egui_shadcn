//! Gauge series — speedometer-style arc with progress fill, pointer, and
//! center value readout.
//!
//! Angles in `GaugeSeries` follow the math convention (0 = right, CCW
//! positive), matching ECharts: a default speedometer uses
//! `start = 225°`, `end = -45°`.

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::GaugeSeries;
use crate::render::ChartPainter;
use crate::render::shapes::annular_sector;
use crate::render::text::{label_font, title_font};
use crate::theme::ChartTheme;
use egui::{Align2, Pos2, Rect, Stroke};
use std::f32::consts::PI;

pub fn render(
    p: &ChartPainter,
    s: &GaugeSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let polar = PolarLayout::fit(rect, 0.78);
    let outer = polar.outer_radius;
    let thickness = (outer * s.thickness).max(2.0);
    let inner = (outer - thickness).max(0.0);
    let center = polar.center;

    // ECharts angle convention → screen-math angle. They already match
    // (0 = right, +X axis), but ECharts uses CCW-positive *degrees* while
    // `annular_sector` expects CCW-positive screen radians — which on screen
    // (y-down) means the arc paints CW. So we negate to draw the visual arc
    // CW from start_deg to end_deg.
    let start_rad = -s.start_angle_deg.to_radians();
    let end_rad = -s.end_angle_deg.to_radians();

    let color = theme.series_color(palette_offset);

    // Background track.
    let track_color = theme.split_area[1];
    annular_sector(
        p,
        center,
        inner,
        outer,
        start_rad,
        end_rad,
        96,
        track_color,
        Stroke::NONE,
    );

    // Progress fill.
    let range = (s.max - s.min).max(1e-12);
    let t = ((s.value - s.min) / range).clamp(0.0, 1.0) as f32;
    let prog_end = start_rad + (end_rad - start_rad) * t;
    if t > 0.0 {
        annular_sector(
            p,
            center,
            inner,
            outer,
            start_rad,
            prog_end,
            96,
            color,
            Stroke::NONE,
        );
    }

    // Pointer needle.
    let needle_angle = prog_end;
    let needle_len = inner * 0.92;
    let needle_tip = center
        + egui::vec2(needle_angle.cos(), needle_angle.sin()) * needle_len;
    let perp = needle_angle + PI * 0.5;
    let base_half = (outer * 0.04).max(2.0);
    let base_a = center + egui::vec2(perp.cos(), perp.sin()) * base_half;
    let base_b = center - egui::vec2(perp.cos(), perp.sin()) * base_half;
    p.poly(vec![needle_tip, base_a, base_b], color, Stroke::NONE);
    p.circle_filled(center, base_half * 1.4, theme.background);
    p.circle_stroke(center, base_half * 1.4, Stroke::new(1.5, color));

    // Value text + name.
    let value_label = format!(
        "{}{}",
        format_value(s.value),
        s.unit.as_deref().unwrap_or("")
    );
    p.text(
        center + egui::vec2(0.0, outer * 0.32),
        Align2::CENTER_CENTER,
        value_label,
        title_font(),
        theme.text,
    );
    if !s.name.is_empty() {
        p.text(
            center + egui::vec2(0.0, outer * 0.50),
            Align2::CENTER_CENTER,
            s.name.clone(),
            label_font(),
            theme.text_dim,
        );
    }

    // Tick marks at each "nice" step.
    let ticks = nice_ticks(s.min, s.max, 6);
    let small_font = label_font();
    for tv in &ticks {
        let tt = ((*tv - s.min) / range).clamp(0.0, 1.0) as f32;
        let ta = start_rad + (end_rad - start_rad) * tt;
        let p1 = center + egui::vec2(ta.cos(), ta.sin()) * outer;
        let p2 = center + egui::vec2(ta.cos(), ta.sin()) * (outer + 5.0);
        p.line(p1, p2, Stroke::new(1.0, theme.axis_line));
        let label_pos =
            center + egui::vec2(ta.cos(), ta.sin()) * (outer + 14.0);
        p.text(
            label_pos,
            Align2::CENTER_CENTER,
            format_value(*tv),
            small_font.clone(),
            theme.text_dim,
        );
    }

    // Hover detection: anywhere inside the ring fires a tooltip.
    if let Some(h) = hover_pos {
        let v = h - center;
        let d = v.length();
        if d >= inner && d <= outer {
            return Some(TooltipDatum {
                series_index: series_idx,
                series_name: s.name.clone(),
                data_index: 0,
                value: s.value,
                color,
                screen_pos: Some(needle_tip),
            });
        }
    }

    None
}

pub fn dispatch(
    p: &ChartPainter,
    s: &GaugeSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    render(p, s, series_idx, rect, theme, 0, hover_pos)
}

fn format_value(v: f64) -> String {
    if v.fract().abs() < 1e-6 {
        format!("{:.0}", v)
    } else {
        format!("{:.1}", v)
    }
}

fn nice_ticks(min: f64, max: f64, target: usize) -> Vec<f64> {
    if max <= min {
        return vec![min];
    }
    let raw = (max - min) / target.max(1) as f64;
    let exp = raw.abs().log10().floor();
    let f = raw / 10f64.powf(exp);
    let nf = if f < 1.5 {
        1.0
    } else if f < 3.0 {
        2.0
    } else if f < 7.0 {
        5.0
    } else {
        10.0
    };
    let step = nf * 10f64.powf(exp);
    let mut t = (min / step).ceil() * step;
    let mut out = Vec::new();
    while t <= max + step * 1e-6 {
        out.push(t);
        t += step;
    }
    if out.is_empty() {
        out.push(min);
        out.push(max);
    }
    out
}
