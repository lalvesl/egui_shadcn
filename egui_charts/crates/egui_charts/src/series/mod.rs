//! Per-chart-type renderers. Each module owns the rendering + hit-testing of
//! one ECharts series type.

pub mod bar;
pub mod line;
pub mod scatter;

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::{Chart, Series};
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::Pos2;

/// Index of a series inside `Chart::series`.
pub type SeriesIndex = usize;

/// Per-series state held by the widget across frames (visibility, hover).
#[derive(Clone, Debug, Default)]
pub struct SeriesState {
    pub visible: bool,
    pub hovered_index: Option<usize>,
}

impl SeriesState {
    pub fn new() -> Self {
        Self { visible: true, hovered_index: None }
    }
}

/// Render every visible series in `chart`, in declaration order.
pub fn render_all(
    p: &ChartPainter,
    chart: &Chart,
    layout: &CoordLayout,
    theme: &ChartTheme,
    states: &[SeriesState],
    hover_data: Option<DataPoint>,
) -> Vec<TooltipDatum> {
    let mut tips = Vec::new();

    // Stack accumulators per (stack_group, x_index).
    use std::collections::HashMap;
    let mut bar_stack_pos: HashMap<(String, usize), f64> = HashMap::new();
    let mut bar_stack_neg: HashMap<(String, usize), f64> = HashMap::new();
    let mut line_stack: HashMap<(String, usize), f64> = HashMap::new();

    // Count bar groups (unstacked bars share x slot side-by-side).
    let bar_indices: Vec<usize> = chart
        .series
        .iter()
        .enumerate()
        .filter(|(_, s)| matches!(s, Series::Bar(_)))
        .map(|(i, _)| i)
        .collect();
    let unstacked_bars: Vec<usize> = bar_indices
        .iter()
        .copied()
        .filter(|i| {
            if let Series::Bar(b) = &chart.series[*i] {
                b.stack.is_none()
            } else {
                false
            }
        })
        .collect();
    let unstacked_count = unstacked_bars.len().max(1);

    for (idx, series) in chart.series.iter().enumerate() {
        let state = states.get(idx).cloned().unwrap_or_else(SeriesState::new);
        if !state.visible {
            continue;
        }
        let color = theme.series_color(idx);
        match series {
            Series::Line(l) => {
                if let Some(t) = line::render(
                    p,
                    l,
                    idx,
                    color,
                    layout,
                    theme,
                    &mut line_stack,
                    hover_data,
                ) {
                    tips.push(t);
                }
            }
            Series::Bar(b) => {
                let group_pos = unstacked_bars.iter().position(|&i| i == idx);
                if let Some(t) = bar::render(
                    p,
                    b,
                    idx,
                    color,
                    layout,
                    theme,
                    &mut bar_stack_pos,
                    &mut bar_stack_neg,
                    group_pos,
                    unstacked_count,
                    hover_data,
                ) {
                    tips.push(t);
                }
            }
            Series::Scatter(s) => {
                if let Some(t) = scatter::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
        }
    }
    tips
}

/// Convert a screen point hit into a `DataPoint` if inside plot area.
pub fn hovered_data(layout: &CoordLayout, screen: Pos2) -> Option<DataPoint> {
    if layout.plot_rect.contains(screen) {
        Some(layout.to_data(screen))
    } else {
        None
    }
}
