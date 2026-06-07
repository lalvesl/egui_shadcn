//! Line series — variants: solid / smooth / step, optional area fill, stacked.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::{LineSeries, LineStyle};
use crate::render::ChartPainter;
use crate::render::shapes::{
    fill_under, smooth_polyline, step_polyline, symbol,
};
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Stroke};
use std::collections::HashMap;

#[allow(clippy::too_many_arguments)]
pub fn render(
    p: &ChartPainter,
    s: &LineSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    _theme: &ChartTheme,
    line_stack: &mut HashMap<(String, usize), f64>,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    // Project data → screen, applying stack offset if any.
    let mut pts = Vec::with_capacity(s.data.len());
    for (i, &v) in s.data.iter().enumerate() {
        if !v.is_finite() {
            continue;
        }
        let y = if let Some(stack) = &s.stack {
            let key = (stack.clone(), i);
            let prev = *line_stack.get(&key).unwrap_or(&0.0);
            let cur = prev + v;
            line_stack.insert(key, cur);
            cur
        } else {
            v
        };
        pts.push(layout.to_screen(DataPoint { x: i as f64, y }));
    }

    if pts.len() < 2 {
        // Single point — just draw the symbol.
        if let Some(pt) = pts.first().copied() {
            symbol(p, s.symbol, pt, 6.0, color);
        }
        return None;
    }

    let render_pts: Vec<Pos2> = match s.style {
        LineStyle::Solid => pts.clone(),
        LineStyle::Smooth => smooth_polyline(&pts, 16),
        LineStyle::Step => step_polyline(&pts),
    };

    if s.fill_area {
        let baseline_y = layout
            .to_screen(DataPoint { x: 0.0, y: 0.0 })
            .y
            .clamp(layout.plot_rect.min.y, layout.plot_rect.max.y);
        let fill = Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            s.area_alpha,
        );
        fill_under(p, &render_pts, baseline_y, fill);
    }

    p.path(render_pts, Stroke::new(s.line_width, color));

    if s.show_symbols {
        for pt in &pts {
            symbol(p, s.symbol, *pt, 5.0, color);
        }
    }

    // Tooltip: nearest-x match.
    if let Some(h) = hover {
        let idx = h.x.round() as i64;
        if idx >= 0 && (idx as usize) < s.data.len() {
            let i = idx as usize;
            let raw = s.data[i];
            if raw.is_finite() {
                return Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.name.clone(),
                    data_index: i,
                    value: raw,
                    color,
                    screen_pos: pts.get(i).copied(),
                });
            }
        }
    }

    None
}
