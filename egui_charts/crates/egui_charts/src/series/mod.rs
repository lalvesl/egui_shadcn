//! Per-chart-type renderers. Each module owns the rendering + hit-testing of
//! one ECharts series type.

pub mod bar;
pub mod bar_3d;
pub mod boxplot;
pub mod calendar_heatmap;
pub mod candlestick;
pub mod chord;
pub mod custom;
pub mod effect_scatter;
pub mod funnel;
pub mod gauge;
pub mod globe;
pub mod graph;
pub mod heatmap;
pub mod line;
pub mod line_3d;
pub mod lines;
pub mod lines_3d;
pub mod lines_geo;
pub mod liquid_fill;
pub mod map;
pub mod map_3d;
pub mod parallel;
pub mod pictorial_bar;
pub mod pie;
pub mod polar_bar;
pub mod radar;
pub mod sankey;
pub mod scatter;
pub mod scatter_3d;
pub mod scatter_geo;
pub mod sunburst;
pub mod surface_3d;
pub mod theme_river;
pub mod tree;
pub mod treemap;
pub mod word_cloud;

use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::{Chart, Series, SeriesCoord};
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Pos2, Rect};

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
        Self {
            visible: true,
            hovered_index: None,
        }
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
                if let Some(t) =
                    line::render(p, l, idx, color, layout, theme, &mut line_stack, hover_data)
                {
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
            Series::Heatmap(s) => {
                if let Some(t) = heatmap::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
            Series::Candlestick(s) => {
                if let Some(t) = candlestick::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
            Series::BoxPlot(s) => {
                if let Some(t) = boxplot::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
            Series::EffectScatter(s) => {
                if let Some(t) = effect_scatter::render(p, s, idx, color, layout, theme, hover_data)
                {
                    tips.push(t);
                }
            }
            Series::LinesCartesian(s) => {
                if let Some(t) = lines::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
            Series::PictorialBar(s) => {
                if let Some(t) = pictorial_bar::render(p, s, idx, color, layout, theme, hover_data)
                {
                    tips.push(t);
                }
            }
            Series::ThemeRiver(s) => {
                if let Some(t) = theme_river::render(p, s, idx, color, layout, theme, hover_data) {
                    tips.push(t);
                }
            }
            // Non-cartesian series are dispatched separately by the widget.
            _ => {}
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

/// True when any series in `chart` uses a non-cartesian coordinate system.
pub fn has_non_cartesian(chart: &Chart) -> bool {
    chart
        .series
        .iter()
        .any(|s| !matches!(s.coord(), SeriesCoord::Cartesian))
}

/// True when every visible series is cartesian.
pub fn is_all_cartesian(chart: &Chart) -> bool {
    chart
        .series
        .iter()
        .all(|s| matches!(s.coord(), SeriesCoord::Cartesian))
}

/// Render every visible non-cartesian series, splitting `area` evenly across
/// them in a single row. Returns the tooltip data for the hovered datum (if
/// any).
pub fn render_non_cartesian(
    p: &ChartPainter,
    chart: &Chart,
    area: Rect,
    theme: &ChartTheme,
    states: &[SeriesState],
    hover_pos: Option<Pos2>,
) -> Vec<TooltipDatum> {
    let visible: Vec<(usize, &Series)> = chart
        .series
        .iter()
        .enumerate()
        .filter(|(i, s)| {
            !matches!(s.coord(), SeriesCoord::Cartesian)
                && states.get(*i).map(|st| st.visible).unwrap_or(true)
        })
        .collect();

    if visible.is_empty() {
        return Vec::new();
    }

    let n = visible.len();
    let cell_w = area.width() / n as f32;
    let mut tips = Vec::new();

    for (slot, (idx, series)) in visible.iter().enumerate() {
        let cell = Rect::from_min_size(
            egui::pos2(area.min.x + cell_w * slot as f32, area.min.y),
            egui::vec2(cell_w, area.height()),
        );
        match series {
            Series::Pie(s) => {
                if let Some(t) = pie::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Radar(s) => {
                if let Some(t) = radar::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Gauge(s) => {
                if let Some(t) = gauge::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Funnel(s) => {
                if let Some(t) = funnel::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::PolarBar(s) => {
                if let Some(t) = polar_bar::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Sunburst(s) => {
                if let Some(t) = sunburst::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Treemap(s) => {
                if let Some(t) = treemap::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Tree(s) => {
                if let Some(t) = tree::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Sankey(s) => {
                if let Some(t) = sankey::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Graph(s) => {
                if let Some(t) = graph::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Parallel(s) => {
                if let Some(t) = parallel::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::CalendarHeatmap(s) => {
                if let Some(t) = calendar_heatmap::render(p, s, *idx, cell, theme, *idx, hover_pos)
                {
                    tips.push(t);
                }
            }
            Series::WordCloud(s) => {
                if let Some(t) = word_cloud::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::LiquidFill(s) => {
                if let Some(t) = liquid_fill::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Custom(s) => {
                if let Some(t) = custom::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Map(s) => {
                if let Some(t) = map::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::LinesGeo(s) => {
                if let Some(t) = lines_geo::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::ScatterGeo(s) => {
                if let Some(t) = scatter_geo::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Bar3D(s) => {
                if let Some(t) = bar_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Line3D(s) => {
                if let Some(t) = line_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Scatter3D(s) => {
                if let Some(t) = scatter_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Surface3D(s) => {
                if let Some(t) = surface_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Lines3D(s) => {
                if let Some(t) = lines_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Map3D(s) => {
                if let Some(t) = map_3d::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Globe(s) => {
                if let Some(t) = globe::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            Series::Chord(s) => {
                if let Some(t) = chord::render(p, s, *idx, cell, theme, *idx, hover_pos) {
                    tips.push(t);
                }
            }
            _ => {}
        }
    }
    tips
}
