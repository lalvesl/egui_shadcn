//! Funnel / pyramid series. Stacks horizontal trapezoids in the content rect,
//! widest at the top by default (or bottom when `inverted`).

use crate::interaction::tooltip::TooltipDatum;
use crate::option::FunnelSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &FunnelSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }
    let max_val = s
        .data
        .iter()
        .map(|d| d.value.max(0.0))
        .fold(0.0_f64, f64::max)
        .max(1e-12);

    let n = s.data.len();
    let inner = rect.shrink(8.0);
    let band_h =
        (inner.height() - s.gap * (n.saturating_sub(1)) as f32) / n as f32;
    if band_h <= 0.0 {
        return None;
    }
    let cx = inner.center().x;
    let font = label_font();

    let mut tip: Option<TooltipDatum> = None;

    // Map each slot (top→bottom row) to a data index. `inverted` flips so
    // the smallest value sits on top.
    let data_at_slot =
        |slot: usize| -> usize { if s.inverted { n - 1 - slot } else { slot } };

    for slot in 0..n {
        let data_idx = data_at_slot(slot);
        let datum = &s.data[data_idx];
        let v = datum.value.max(0.0);

        // Bottom edge width tapers towards the next slot's value (or the
        // current value × 0.5 for the final row).
        let next_v = if slot + 1 < n {
            s.data[data_at_slot(slot + 1)].value.max(0.0)
        } else {
            v * 0.5
        };

        let width_top = inner.width() * ((v / max_val) as f32).clamp(0.05, 1.0);
        let width_bot =
            inner.width() * ((next_v / max_val) as f32).clamp(0.02, 1.0);

        let y_top = inner.min.y + (band_h + s.gap) * slot as f32;
        let y_bot = y_top + band_h;

        let color = theme.series_color(palette_offset + data_idx);

        let pts = vec![
            Pos2::new(cx - width_top * 0.5, y_top),
            Pos2::new(cx + width_top * 0.5, y_top),
            Pos2::new(cx + width_bot * 0.5, y_bot),
            Pos2::new(cx - width_bot * 0.5, y_bot),
        ];

        let hovered = hover_pos
            .map(|h| point_in_trapezoid(h, &pts))
            .unwrap_or(false);

        let fill = if hovered {
            color
        } else if hover_pos.is_some() {
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                220,
            )
        } else {
            color
        };

        p.poly(pts, fill, Stroke::new(1.0, theme.background));

        let mid = Pos2::new(cx, (y_top + y_bot) * 0.5);
        p.text(
            mid,
            Align2::CENTER_CENTER,
            format!("{}  {}", datum.name, format_value(datum.value)),
            font.clone(),
            theme.background,
        );

        if hovered {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: datum.name.clone(),
                data_index: data_idx,
                value: datum.value,
                color,
                screen_pos: Some(mid),
            });
        }
    }

    tip
}

pub fn dispatch(
    p: &ChartPainter,
    s: &FunnelSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    render(p, s, series_idx, rect, theme, 0, hover_pos)
}

fn point_in_trapezoid(p: Pos2, pts: &[Pos2]) -> bool {
    if pts.len() < 3 {
        return false;
    }
    let mut sign = 0.0_f32;
    for i in 0..pts.len() {
        let a = pts[i];
        let b = pts[(i + 1) % pts.len()];
        let cross = (b.x - a.x) * (p.y - a.y) - (b.y - a.y) * (p.x - a.x);
        if i == 0 {
            sign = cross.signum();
        } else if cross.signum() != sign && cross != 0.0 {
            return false;
        }
    }
    true
}

fn format_value(v: f64) -> String {
    if v.fract().abs() < 1e-6 || v.abs() >= 100.0 {
        format!("{:.0}", v)
    } else {
        format!("{:.1}", v)
    }
}
