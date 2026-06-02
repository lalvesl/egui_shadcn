//! Bar series — vertical/horizontal, stacked.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::BarSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Rect, Stroke, StrokeKind};
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn render(
    p: &ChartPainter,
    s: &BarSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    _theme: &ChartTheme,
    bar_stack_pos: &mut HashMap<(String, usize), f64>,
    bar_stack_neg: &mut HashMap<(String, usize), f64>,
    group_pos: Option<usize>,
    unstacked_count: usize,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    // Per-x-slot pixel width. For category axis, slot width = plot_width / count.
    let slot_w = layout.plot_rect.width() / s.data.len().max(1) as f32;
    let bar_w_total = slot_w * s.bar_width_ratio;
    let (bar_w, offset_x) = if let Some(gp) = group_pos {
        let w = bar_w_total / unstacked_count as f32;
        let off = (gp as f32 - unstacked_count as f32 / 2.0 + 0.5) * w;
        (w, off)
    } else {
        (bar_w_total, 0.0)
    };

    let baseline_y = layout
        .to_screen(DataPoint { x: 0.0, y: 0.0 })
        .y
        .clamp(layout.plot_rect.min.y, layout.plot_rect.max.y);

    let mut tip: Option<TooltipDatum> = None;
    let hovered_idx = hover.map(|h| h.x.round() as i64);

    for (i, &v) in s.data.iter().enumerate() {
        if !v.is_finite() {
            continue;
        }

        let (y0, y1) = if let Some(stack) = &s.stack {
            let key = (stack.clone(), i);
            if v >= 0.0 {
                let base = *bar_stack_pos.get(&key).unwrap_or(&0.0);
                let top = base + v;
                bar_stack_pos.insert(key, top);
                (base, top)
            } else {
                let base = *bar_stack_neg.get(&key).unwrap_or(&0.0);
                let top = base + v;
                bar_stack_neg.insert(key, top);
                (base, top)
            }
        } else {
            (0.0, v)
        };

        let center = layout.to_screen(DataPoint {
            x: i as f64,
            y: 0.0,
        });
        let p0 = layout.to_screen(DataPoint { x: i as f64, y: y0 });
        let p1 = layout.to_screen(DataPoint { x: i as f64, y: y1 });

        let rect = if s.horizontal {
            // Swap: bars stretch in x direction, slot stacks in y.
            // For Phase 0 horizontal bars assume y is category, x is value.
            let slot_h = layout.plot_rect.height() / s.data.len().max(1) as f32;
            let bar_h = slot_h * s.bar_width_ratio;
            let cy = layout
                .to_screen(DataPoint {
                    x: 0.0,
                    y: i as f64,
                })
                .y;
            Rect::from_min_max(
                egui::Pos2::new(p0.x.min(p1.x), cy - bar_h * 0.5),
                egui::Pos2::new(p0.x.max(p1.x), cy + bar_h * 0.5),
            )
        } else {
            Rect::from_min_max(
                egui::Pos2::new(center.x - bar_w * 0.5 + offset_x, p0.y.min(p1.y)),
                egui::Pos2::new(center.x + bar_w * 0.5 + offset_x, p0.y.max(p1.y)),
            )
        };

        let is_hovered = hovered_idx == Some(i as i64);
        let fill = if is_hovered {
            color
        } else {
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                if hovered_idx.is_some() { 200 } else { 255 },
            )
        };
        p.rect_filled(rect, fill);

        if is_hovered {
            p.painter
                .rect_stroke(rect, 0.0, Stroke::new(1.5, color), StrokeKind::Inside);
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: s.name.clone(),
                data_index: i,
                value: v,
                color,
                screen_pos: Some(egui::Pos2::new(rect.center().x, rect.min.y)),
            });
        }
    }

    let _ = baseline_y;
    tip
}
