//! Legend rendering + click-to-toggle visibility.

use crate::option::{Chart, LegendPosition};
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::series::SeriesState;
use crate::theme::ChartTheme;
use egui::{Align2, Rect, Sense, Stroke, StrokeKind, Ui, Vec2, vec2};

#[derive(Clone, Debug)]
pub struct LegendItem {
    pub series_index: usize,
    pub name: String,
    pub visible: bool,
}

pub fn items(chart: &Chart, states: &[SeriesState]) -> Vec<LegendItem> {
    chart
        .series
        .iter()
        .enumerate()
        .map(|(i, s)| LegendItem {
            series_index: i,
            name: s.name().to_string(),
            visible: states.get(i).map(|s| s.visible).unwrap_or(true),
        })
        .collect()
}

/// Reserve space for the legend, returning the rect available for the plot.
pub fn reserve(rect: Rect, chart: &Chart) -> (Rect, Rect) {
    let h: f32 = 26.0;
    let w: f32 = 120.0;
    match chart.legend.position {
        LegendPosition::Hidden => (rect, Rect::NOTHING),
        LegendPosition::Top => {
            let legend = Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.min.y + h));
            let plot = Rect::from_min_max(egui::pos2(rect.min.x, rect.min.y + h), rect.max);
            (plot, legend)
        }
        LegendPosition::Bottom => {
            let legend = Rect::from_min_max(egui::pos2(rect.min.x, rect.max.y - h), rect.max);
            let plot = Rect::from_min_max(rect.min, egui::pos2(rect.max.x, rect.max.y - h));
            (plot, legend)
        }
        LegendPosition::Left => {
            let legend = Rect::from_min_max(rect.min, egui::pos2(rect.min.x + w, rect.max.y));
            let plot = Rect::from_min_max(egui::pos2(rect.min.x + w, rect.min.y), rect.max);
            (plot, legend)
        }
        LegendPosition::Right => {
            let legend = Rect::from_min_max(egui::pos2(rect.max.x - w, rect.min.y), rect.max);
            let plot = Rect::from_min_max(rect.min, egui::pos2(rect.max.x - w, rect.max.y));
            (plot, legend)
        }
    }
}

/// Draw the legend and handle clicks to toggle visibility.
pub fn draw_and_handle(
    ui: &mut Ui,
    p: &ChartPainter,
    rect: Rect,
    chart: &Chart,
    theme: &ChartTheme,
    states: &mut [SeriesState],
) {
    if rect.area() <= 0.0 || matches!(chart.legend.position, LegendPosition::Hidden) {
        return;
    }

    let items = items(chart, states);
    let font = label_font();

    let horizontal = matches!(
        chart.legend.position,
        LegendPosition::Top | LegendPosition::Bottom
    );

    let pad = vec2(8.0, 4.0);
    let mut cursor = rect.min + pad;
    for it in &items {
        let label = it.name.clone();
        let text_size = ui
            .ctx()
            .fonts_mut(|f| f.layout_no_wrap(label.clone(), font.clone(), theme.text))
            .size();
        let entry_size = vec2(text_size.x + 24.0, text_size.y.max(16.0) + 4.0);

        if horizontal && cursor.x + entry_size.x > rect.max.x - pad.x {
            // wrap to second row
            cursor.x = rect.min.x + pad.x;
            cursor.y += entry_size.y;
        }
        if !horizontal && cursor.y + entry_size.y > rect.max.y - pad.y {
            break;
        }

        let entry_rect = Rect::from_min_size(cursor, entry_size);
        let resp = ui.interact(
            entry_rect,
            ui.id().with(("legend", it.series_index)),
            Sense::click(),
        );

        let swatch_rect = Rect::from_min_size(
            cursor + vec2(2.0, (entry_size.y - 10.0) * 0.5),
            Vec2::splat(10.0),
        );
        let series_color = theme.series_color(it.series_index);
        let fill = if it.visible {
            series_color
        } else {
            theme.text_dim
        };
        p.rounded_rect_filled(swatch_rect, 2.0, fill);

        let text_color = if it.visible {
            theme.text
        } else {
            theme.text_dim
        };
        p.text(
            cursor + vec2(18.0, entry_size.y * 0.5),
            Align2::LEFT_CENTER,
            label,
            font.clone(),
            text_color,
        );

        if resp.hovered() {
            p.painter.rect_stroke(
                entry_rect,
                4.0,
                Stroke::new(1.0, theme.text_dim),
                StrokeKind::Inside,
            );
        }
        if resp.clicked()
            && let Some(state) = states.get_mut(it.series_index)
        {
            state.visible = !state.visible;
        }

        if horizontal {
            cursor.x += entry_size.x + 4.0;
        } else {
            cursor.y += entry_size.y;
        }
    }
}
