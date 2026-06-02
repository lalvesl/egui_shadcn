//! Calendar heatmap — 7-row × 53-column grid; cell color encodes value.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::CalendarHeatmapSeries;
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, StrokeKind, vec2};

pub fn render(
    p: &ChartPainter,
    s: &CalendarHeatmapSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    _palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }
    let (mut vmin, mut vmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for &(_, v) in &s.data {
        if v.is_finite() {
            vmin = vmin.min(v);
            vmax = vmax.max(v);
        }
    }
    let vmin = s.min.unwrap_or(vmin);
    let vmax = s.max.unwrap_or(vmax).max(vmin + 1e-12);

    let inner = rect.shrink(8.0);
    let weeks = 53.0;
    let label_h: f32 = 18.0;
    let avail_w = inner.width() - 24.0;
    let avail_h = inner.height() - label_h;
    let cell = (avail_w / weeks).min(avail_h / 7.0).max(4.0);
    let grid_w = cell * weeks;
    let grid_h = cell * 7.0;
    let origin = Pos2::new(
        inner.min.x + 24.0 + (avail_w - grid_w) * 0.5,
        inner.min.y + (avail_h - grid_h) * 0.5 + label_h,
    );

    let palette = &theme.sequential;
    let last = palette.len().saturating_sub(1);
    let font = label_font();

    // Year label.
    p.text(
        Pos2::new(rect.min.x + 8.0, inner.min.y),
        Align2::LEFT_TOP,
        format!("{}", s.year),
        font.clone(),
        theme.text,
    );

    // Day-of-week labels.
    let dow = ["S", "M", "T", "W", "T", "F", "S"];
    for (i, label) in dow.iter().enumerate() {
        p.text(
            Pos2::new(origin.x - 6.0, origin.y + (i as f32 + 0.5) * cell),
            Align2::RIGHT_CENTER,
            (*label).to_string(),
            font.clone(),
            theme.text_dim,
        );
    }

    // Background grid (empty cells).
    let empty = Color32::from_rgba_unmultiplied(
        theme.text_dim.r(),
        theme.text_dim.g(),
        theme.text_dim.b(),
        30,
    );
    for col in 0..53 {
        for row in 0..7 {
            let r = Rect::from_min_size(
                Pos2::new(origin.x + col as f32 * cell, origin.y + row as f32 * cell),
                vec2(cell - 1.0, cell - 1.0),
            );
            p.rect_filled(r, empty);
        }
    }

    let mut tip: Option<TooltipDatum> = None;

    for &(doy, v) in &s.data {
        if !v.is_finite() {
            continue;
        }
        let offset = (doy as i32 - 1) + s.start_weekday as i32;
        if offset < 0 {
            continue;
        }
        let row = (offset % 7) as usize;
        let col = (offset / 7) as usize;
        if col >= 53 {
            continue;
        }
        let t = ((v - vmin) / (vmax - vmin)).clamp(0.0, 1.0);
        let color = palette[((t * last as f64).round() as usize).min(last)];
        let cell_rect = Rect::from_min_size(
            Pos2::new(origin.x + col as f32 * cell, origin.y + row as f32 * cell),
            vec2(cell - 1.0, cell - 1.0),
        );
        p.rect_filled(cell_rect, color);

        let hovered = hover_pos.map(|h| cell_rect.contains(h)).unwrap_or(false);
        if hovered {
            p.painter
                .rect_stroke(cell_rect, 0.0, Stroke::new(1.5, theme.text), StrokeKind::Inside);
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: format!("{} · day {}", s.name, doy),
                data_index: doy as usize,
                value: v,
                color,
                screen_pos: Some(cell_rect.center()),
            });
        }
    }

    tip
}
