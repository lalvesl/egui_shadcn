//! Liquid fill gauge — circle with a sinusoidal waterline whose height is
//! controlled by `value` ∈ `[0, 1]`. Multiple `waves` paint slightly offset
//! phases for a layered look.

use crate::coord::polar::PolarLayout;
use crate::interaction::tooltip::TooltipDatum;
use crate::option::LiquidFillSeries;
use crate::render::ChartPainter;
use crate::render::text::title_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};
use std::f32::consts::TAU;

pub fn render(
    p: &ChartPainter,
    s: &LiquidFillSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let polar = PolarLayout::fit(rect, 0.85);
    let center = polar.center;
    let r = polar.outer_radius;
    let base = theme.series_color(palette_offset);

    // Background disc.
    let bg = Color32::from_rgba_unmultiplied(base.r(), base.g(), base.b(), 24);
    p.circle_filled(center, r, bg);

    let value = s.value.clamp(0.0, 1.0) as f32;
    let waterline_y = center.y + r - r * 2.0 * value;
    let amp = s.wave_amplitude;

    let samples = 96;
    let waves = s.waves.max(1);

    for w in 0..waves {
        let alpha = (180 - (w as i32 * 50)).clamp(60, 220) as u8;
        let phase = w as f32 * 1.4;
        let freq = 2.0 + w as f32 * 0.6;

        let mut top: Vec<Pos2> = Vec::with_capacity(samples + 1);
        for i in 0..=samples {
            let t = i as f32 / samples as f32;
            let x = center.x - r + 2.0 * r * t;
            // Only fill within the disc (clipped to the chord at each y).
            let y = waterline_y + (t * TAU * freq + phase).sin() * amp;
            top.push(Pos2::new(x, y));
        }

        // Bottom edge follows disc circumference from right to left at base y.
        let mut shape: Vec<Pos2> = Vec::with_capacity(top.len() + 2);
        shape.extend(top.iter().copied());
        shape.push(Pos2::new(center.x + r, center.y + r));
        shape.push(Pos2::new(center.x - r, center.y + r));

        let fill = Color32::from_rgba_unmultiplied(
            base.r(),
            base.g(),
            base.b(),
            alpha,
        );
        // Clip to disc by drawing through a temporary mask polygon — for
        // Phase 2 we accept slight overflow which is hidden by the outer
        // stroke below.
        for i in 0..(top.len() - 1) {
            let poly = vec![
                top[i],
                top[i + 1],
                Pos2::new(top[i + 1].x, center.y + r),
                Pos2::new(top[i].x, center.y + r),
            ];
            p.poly(poly, fill, Stroke::NONE);
        }
    }

    // Disc outline mask (covers overflow with the chart background).
    let segs = 96;
    let mut ring_outer: Vec<Pos2> = Vec::with_capacity(segs + 1);
    let mut ring_inner: Vec<Pos2> = Vec::with_capacity(segs + 1);
    for i in 0..=segs {
        let a = TAU * i as f32 / segs as f32;
        ring_outer.push(center + egui::vec2(a.cos(), a.sin()) * (r + 24.0));
        ring_inner.push(center + egui::vec2(a.cos(), a.sin()) * r);
    }
    // Draw the outside ring in background color to mask overflow.
    for i in 0..segs {
        let quad = vec![
            ring_inner[i],
            ring_outer[i],
            ring_outer[i + 1],
            ring_inner[i + 1],
        ];
        p.poly(quad, theme.background, Stroke::NONE);
    }
    p.circle_stroke(center, r, Stroke::new(2.0, base));

    // Center label.
    let text =
        format!("{:.0}{}", value * 100.0, s.unit.as_deref().unwrap_or(""));
    p.text(
        center,
        Align2::CENTER_CENTER,
        text,
        title_font(),
        theme.text,
    );
    if !s.name.is_empty() {
        p.text(
            center + egui::vec2(0.0, r * 0.55),
            Align2::CENTER_CENTER,
            s.name.clone(),
            crate::render::text::label_font(),
            theme.text_dim,
        );
    }

    // Hover: pointer inside the disc fires a tooltip.
    if let Some(h) = hover_pos
        && (h - center).length() <= r
    {
        return Some(TooltipDatum {
            series_index: series_idx,
            series_name: s.name.clone(),
            data_index: 0,
            value: s.value,
            color: base,
            screen_pos: Some(center),
        });
    }
    None
}
