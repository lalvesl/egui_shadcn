//! Floating tooltip panel drawn on a top layer.

use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke, StrokeKind, vec2};

#[derive(Clone, Debug)]
pub struct TooltipDatum {
    pub series_index: usize,
    pub series_name: String,
    pub data_index: usize,
    pub value: f64,
    pub color: Color32,
    pub screen_pos: Option<Pos2>,
}

/// Draw a tooltip card listing every hit datum at the current cursor.
pub fn draw(
    p: &ChartPainter,
    ctx: &egui::Context,
    cursor: Pos2,
    plot_rect: Rect,
    theme: &ChartTheme,
    tips: &[TooltipDatum],
    x_axis_label: Option<String>,
) {
    if tips.is_empty() {
        return;
    }
    let font = label_font();

    // Lay out lines: optional header (x label) + one row per series.
    let mut lines: Vec<(String, Color32)> = Vec::with_capacity(tips.len() + 1);
    if let Some(xl) = x_axis_label {
        lines.push((xl, theme.text_dim));
    }
    for t in tips {
        let val = format_value(t.value);
        lines.push((format!("{}  {}", t.series_name, val), t.color));
    }

    // Measure.
    let mut max_w: f32 = 0.0;
    let mut total_h: f32 = 0.0;
    let mut row_h: f32 = 0.0;
    let measured: Vec<egui::Vec2> = lines
        .iter()
        .map(|(s, _)| {
            let g = ctx.fonts_mut(|f| f.layout_no_wrap(s.clone(), font.clone(), theme.text));
            let size = g.size();
            max_w = max_w.max(size.x);
            row_h = row_h.max(size.y);
            total_h += size.y + 4.0;
            size
        })
        .collect();

    let pad = vec2(10.0, 8.0);
    let panel_size = vec2(max_w + pad.x * 2.0 + 14.0, total_h + pad.y * 2.0);

    // Pin near cursor, clamp to plot bounds.
    let mut origin = cursor + vec2(14.0, 14.0);
    if origin.x + panel_size.x > plot_rect.max.x {
        origin.x = cursor.x - panel_size.x - 14.0;
    }
    if origin.y + panel_size.y > plot_rect.max.y {
        origin.y = plot_rect.max.y - panel_size.y - 2.0;
    }
    origin.x = origin.x.max(plot_rect.min.x);
    origin.y = origin.y.max(plot_rect.min.y);

    let panel = Rect::from_min_size(origin, panel_size);
    p.rounded_rect_filled(panel, 6.0, theme.surface);
    p.painter.rect_stroke(
        panel,
        6.0,
        Stroke::new(1.0, theme.grid_line),
        StrokeKind::Inside,
    );

    let mut y = origin.y + pad.y;
    for ((line, color), size) in lines.iter().zip(measured.iter()) {
        let dot = Pos2::new(origin.x + pad.x, y + size.y * 0.5);
        p.circle_filled(dot, 4.0, *color);
        p.text(
            Pos2::new(origin.x + pad.x + 10.0, y + size.y * 0.5),
            Align2::LEFT_CENTER,
            line.clone(),
            font.clone(),
            theme.text,
        );
        y += size.y + 4.0;
    }
}

fn format_value(v: f64) -> String {
    if v == 0.0 {
        "0".into()
    } else if v.abs() >= 1000.0 {
        format!("{:.0}", v)
    } else if v.abs() >= 10.0 {
        format!("{:.1}", v)
    } else {
        format!("{:.2}", v)
    }
}
