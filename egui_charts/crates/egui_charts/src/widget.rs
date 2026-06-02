//! `ChartWidget` — the egui-facing chart renderer.

use crate::coord::cartesian::CartesianCoord;
use crate::interaction::{legend, tooltip};
use crate::option::{Axis, AxisKind, Chart};
use crate::render::ChartPainter;
use crate::render::text::{label_font, title_font};
use crate::series::{
    SeriesState, hovered_data, is_all_cartesian, render_all, render_non_cartesian,
};
use crate::theme::ChartTheme;
use egui::{Align2, Id, Response, Sense, Stroke, StrokeKind, Ui, Vec2, vec2};

/// Persistent per-chart state (visibility toggles, etc.) keyed by widget id.
#[derive(Clone, Debug, Default)]
pub struct ChartState {
    pub series: Vec<SeriesState>,
}

impl ChartState {
    fn ensure(&mut self, n: usize) {
        while self.series.len() < n {
            self.series.push(SeriesState::new());
        }
    }
}

pub struct ChartWidget<'a> {
    chart: &'a Chart,
    theme: Option<ChartTheme>,
    desired_size: Vec2,
    id_source: Option<Id>,
}

impl<'a> ChartWidget<'a> {
    pub fn new(chart: &'a Chart) -> Self {
        Self {
            chart,
            theme: None,
            desired_size: vec2(480.0, 320.0),
            id_source: None,
        }
    }

    pub fn theme(mut self, theme: ChartTheme) -> Self {
        self.theme = Some(theme);
        self
    }

    pub fn min_size(mut self, size: Vec2) -> Self {
        self.desired_size = size;
        self
    }

    pub fn id(mut self, id: impl Into<Id>) -> Self {
        self.id_source = Some(id.into());
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let resolved_theme = self
            .theme
            .clone()
            .unwrap_or_else(|| ChartTheme::follow_egui(ui.ctx(), egui::Color32::from_rgb(0x4f, 0x8c, 0xff), crate::theme::Harmony::Square, self.chart.series.len().max(6)));

        let available = ui.available_size_before_wrap();
        let size = vec2(
            available.x.max(self.desired_size.x).min(available.x),
            self.desired_size.y.max(160.0).min(available.y.max(self.desired_size.y)),
        );
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        let id = self.id_source.unwrap_or(response.id);

        // Load/store persistent state.
        let mut state: ChartState =
            ui.data_mut(|d| d.get_temp::<ChartState>(id).unwrap_or_default());
        state.ensure(self.chart.series.len());

        let painter = ui.painter_at(rect);
        let chart_p = ChartPainter::new(&painter, rect);

        // Background card.
        chart_p.rounded_rect_filled(rect, 8.0, resolved_theme.background);
        chart_p
            .painter
            .rect_stroke(rect, 8.0, Stroke::new(1.0, resolved_theme.grid_line), StrokeKind::Inside);

        // Title bar.
        let mut content = rect.shrink2(vec2(8.0, 8.0));
        if let Some(title) = &self.chart.title {
            chart_p.text(
                content.min + vec2(0.0, 0.0),
                Align2::LEFT_TOP,
                title.text.clone(),
                title_font(),
                resolved_theme.text,
            );
            content.min.y += 22.0;
        }

        // Legend reserves space first.
        let (plot_area, legend_rect) = legend::reserve(content, self.chart);

        let hover_pos = ui.input(|i| i.pointer.hover_pos());

        let (tips, tooltip_rect, x_label) = if is_all_cartesian(self.chart) {
            // Build coord layout from the remaining area.
            let coord = CartesianCoord::new(self.chart);
            let layout = coord.layout(plot_area, &resolved_theme);

            // Draw split areas (alternating bands across y-axis).
            let y_axis = self
                .chart
                .y_axis
                .clone()
                .unwrap_or_else(Axis::value);
            if y_axis.show_grid
                && let Some(y_layout) = layout.axes.iter().find(|a| !a.is_x) {
                    let mut last_px = layout.plot_rect.min.y;
                    for (i, t) in y_layout.ticks.iter().enumerate() {
                        let band = egui::Rect::from_min_max(
                            egui::Pos2::new(layout.plot_rect.min.x, last_px.min(t.pixel)),
                            egui::Pos2::new(layout.plot_rect.max.x, last_px.max(t.pixel)),
                        );
                        if band.height() > 0.0 {
                            chart_p.rect_filled(band, resolved_theme.split_area[i % 2]);
                        }
                        last_px = t.pixel;
                    }
                    if last_px < layout.plot_rect.max.y {
                        let band = egui::Rect::from_min_max(
                            egui::Pos2::new(layout.plot_rect.min.x, last_px),
                            egui::Pos2::new(layout.plot_rect.max.x, layout.plot_rect.max.y),
                        );
                        chart_p
                            .rect_filled(band, resolved_theme.split_area[y_layout.ticks.len() % 2]);
                    }
                }

            // Grid lines + axes.
            draw_axes(&chart_p, &layout, &resolved_theme, self.chart);

            let hover_data = hover_pos.and_then(|p| hovered_data(&layout, p));
            let tips = render_all(
                &chart_p,
                self.chart,
                &layout,
                &resolved_theme,
                &state.series,
                hover_data,
            );
            let x_label = build_x_label(self.chart, &tips);
            (tips, layout.plot_rect, x_label)
        } else {
            // Polar / radar / boxed series: no axes, split rect horizontally.
            let tips = render_non_cartesian(
                &chart_p,
                self.chart,
                plot_area,
                &resolved_theme,
                &state.series,
                hover_pos,
            );
            (tips, plot_area, None)
        };

        // Tooltip.
        if self.chart.show_tooltip
            && let (Some(cursor), false) = (hover_pos, tips.is_empty()) {
                tooltip::draw(
                    &chart_p,
                    ui.ctx(),
                    cursor,
                    tooltip_rect,
                    &resolved_theme,
                    &tips,
                    x_label,
                );
            }

        // Legend (after plot so it overlays cleanly).
        legend::draw_and_handle(
            ui,
            &chart_p,
            legend_rect,
            self.chart,
            &resolved_theme,
            &mut state.series,
        );

        // Persist state.
        ui.data_mut(|d| d.insert_temp(id, state));

        response
    }
}

fn build_x_label(chart: &Chart, tips: &[crate::interaction::tooltip::TooltipDatum]) -> Option<String> {
    let idx = tips.first()?.data_index;
    let axis = chart.x_axis.as_ref()?;
    match axis.kind {
        AxisKind::Category => axis.categories.get(idx).cloned(),
        _ => None,
    }
}

fn draw_axes(p: &ChartPainter, layout: &crate::coord::CoordLayout, theme: &ChartTheme, chart: &Chart) {
    let font = label_font();
    for axis in &layout.axes {
        // Axis line.
        if axis.is_x {
            if chart
                .x_axis
                .as_ref()
                .map(|a| a.show_axis_line)
                .unwrap_or(true)
            {
                p.line(axis.line_start, axis.line_end, Stroke::new(1.0, theme.axis_line));
            }
        } else if chart
            .y_axis
            .as_ref()
            .map(|a| a.show_axis_line)
            .unwrap_or(true)
        {
            p.line(axis.line_start, axis.line_end, Stroke::new(1.0, theme.axis_line));
        }

        // Gridlines + tick labels.
        for tick in &axis.ticks {
            if axis.is_x {
                let show_grid = chart.x_axis.as_ref().map(|a| a.show_grid).unwrap_or(true);
                if show_grid {
                    p.line(
                        egui::Pos2::new(tick.pixel, layout.plot_rect.min.y),
                        egui::Pos2::new(tick.pixel, layout.plot_rect.max.y),
                        Stroke::new(1.0, theme.grid_line),
                    );
                }
                // Tick mark.
                p.line(
                    egui::Pos2::new(tick.pixel, layout.plot_rect.max.y),
                    egui::Pos2::new(tick.pixel, layout.plot_rect.max.y + 4.0),
                    Stroke::new(1.0, theme.axis_line),
                );
                if chart.x_axis.as_ref().map(|a| a.show_tick_labels).unwrap_or(true) {
                    p.text(
                        egui::Pos2::new(tick.pixel, layout.plot_rect.max.y + 6.0),
                        Align2::CENTER_TOP,
                        tick.label.clone(),
                        font.clone(),
                        theme.text_dim,
                    );
                }
            } else {
                let show_grid = chart.y_axis.as_ref().map(|a| a.show_grid).unwrap_or(true);
                if show_grid {
                    p.line(
                        egui::Pos2::new(layout.plot_rect.min.x, tick.pixel),
                        egui::Pos2::new(layout.plot_rect.max.x, tick.pixel),
                        Stroke::new(1.0, theme.grid_line),
                    );
                }
                p.line(
                    egui::Pos2::new(layout.plot_rect.min.x - 4.0, tick.pixel),
                    egui::Pos2::new(layout.plot_rect.min.x, tick.pixel),
                    Stroke::new(1.0, theme.axis_line),
                );
                if chart.y_axis.as_ref().map(|a| a.show_tick_labels).unwrap_or(true) {
                    p.text(
                        egui::Pos2::new(layout.plot_rect.min.x - 6.0, tick.pixel),
                        Align2::RIGHT_CENTER,
                        tick.label.clone(),
                        font.clone(),
                        theme.text_dim,
                    );
                }
            }
        }

        // Axis name.
        if let Some(name) = &axis.name {
            if axis.is_x {
                p.text(
                    egui::Pos2::new(
                        (axis.line_start.x + axis.line_end.x) * 0.5,
                        layout.plot_rect.max.y + 20.0,
                    ),
                    Align2::CENTER_CENTER,
                    name.clone(),
                    font.clone(),
                    theme.text_dim,
                );
            } else {
                p.text(
                    egui::Pos2::new(layout.plot_rect.min.x - 44.0, layout.plot_rect.min.y - 6.0),
                    Align2::LEFT_BOTTOM,
                    name.clone(),
                    font.clone(),
                    theme.text_dim,
                );
            }
        }
    }
}

impl egui::Widget for ChartWidget<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        self.show(ui)
    }
}
