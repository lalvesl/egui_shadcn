//! Box-and-whisker series. Each datum draws min/Q1/median/Q3/max.

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::BoxPlotSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Color32, Pos2, Rect, Stroke, StrokeKind};

pub fn render(
    p: &ChartPainter,
    s: &BoxPlotSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    if s.data.is_empty() {
        return None;
    }

    let slot_w = layout.plot_rect.width() / s.data.len().max(1) as f32;
    let box_w = (slot_w * s.box_ratio).max(4.0);
    let cap = box_w * 0.5;
    let stroke = Stroke::new(1.4, color);
    let mut tip: Option<TooltipDatum> = None;
    let hovered_idx = hover.map(|h| h.x.round() as i64);

    for (i, b) in s.data.iter().enumerate() {
        let center_x = layout.to_screen(DataPoint { x: i as f64, y: 0.0 }).x;
        let y_min = layout.to_screen(DataPoint { x: i as f64, y: b.min }).y;
        let y_q1 = layout.to_screen(DataPoint { x: i as f64, y: b.q1 }).y;
        let y_med = layout.to_screen(DataPoint { x: i as f64, y: b.median }).y;
        let y_q3 = layout.to_screen(DataPoint { x: i as f64, y: b.q3 }).y;
        let y_max = layout.to_screen(DataPoint { x: i as f64, y: b.max }).y;

        // Whisker line min → max.
        p.line(Pos2::new(center_x, y_min), Pos2::new(center_x, y_max), stroke);
        // Caps.
        p.line(
            Pos2::new(center_x - cap * 0.5, y_min),
            Pos2::new(center_x + cap * 0.5, y_min),
            stroke,
        );
        p.line(
            Pos2::new(center_x - cap * 0.5, y_max),
            Pos2::new(center_x + cap * 0.5, y_max),
            stroke,
        );

        // Box body (Q1 → Q3).
        let rect = Rect::from_min_max(
            Pos2::new(center_x - box_w * 0.5, y_q3.min(y_q1)),
            Pos2::new(center_x + box_w * 0.5, y_q1.max(y_q3)),
        );
        let fill = Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 60);
        p.rect_filled(rect, fill);
        p.painter.rect_stroke(rect, 0.0, stroke, StrokeKind::Inside);

        // Median line.
        p.line(
            Pos2::new(rect.min.x, y_med),
            Pos2::new(rect.max.x, y_med),
            Stroke::new(1.8, color),
        );

        if hovered_idx == Some(i as i64) {
            p.painter
                .rect_stroke(rect, 0.0, Stroke::new(2.0, theme.text), StrokeKind::Inside);
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: format!("{} (Q1/med/Q3)", s.name),
                data_index: i,
                value: b.median,
                color,
                screen_pos: Some(Pos2::new(center_x, rect.min.y)),
            });
        }
    }

    tip
}
