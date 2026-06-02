//! `egui_charts` — ECharts-inspired charting library for egui, built on
//! `epaint` only.
//!
//! Phase 0 catalog: **line** (solid/smooth/step/area/stacked), **bar**
//! (vertical/horizontal/stacked/grouped), **scatter** (bubble). All consume
//! the centralized [`ChartTheme`] so a single primary color reskins the
//! whole canvas. See [`theme`] for the color-wheel palette generator.
//!
//! ```no_run
//! use egui_charts::{Chart, ChartWidget, Axis, Series};
//!
//! # let ctx: egui::Context = unimplemented!();
//! # egui::CentralPanel::default().show(&ctx, |ui| {
//! let chart = Chart::new()
//!     .title("Quarterly revenue")
//!     .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
//!     .y_axis(Axis::value())
//!     .series(Series::bar("2025").data([12.0, 18.0, 9.0, 22.0]))
//!     .series(Series::line("trend").smooth(true).data([10.0, 15.0, 14.0, 20.0]));
//!
//! ChartWidget::new(&chart).show(ui);
//! # });
//! ```

pub mod coord;
pub mod interaction;
pub mod option;
pub mod render;
pub mod series;
pub mod theme;
pub mod widget;

pub use coord::geo::GeoBbox;
pub use coord::three_d::{Camera3D, ThreeDLayout, Vec3};
pub use coord::{CoordKind, CoordLayout, DataPoint};
pub use option::{
    Axis, AxisKind, Bar3DSeries, BarSeries, BoxDatum, BoxPlotSeries, CalendarHeatmapSeries, Candle,
    CandlestickSeries, Chart, ChartKind, ChordSeries, CustomRenderFn, CustomSeries,
    EffectScatterSeries, FunnelSeries, GaugeSeries, GeoLine, GeoPath, GeoPoint, GlobeSeries,
    GraphLayout, GraphLink, GraphNode, GraphSeries, HeatmapSeries, Legend, LegendPosition,
    Line3DSeries, LineSegment, LineSeries, LineStyle, Lines3DSeries, LinesCartesianSeries,
    LinesGeoSeries, LiquidFillSeries, Map3DSeries, MapRegion, MapSeries, ParallelAxis,
    ParallelLine, ParallelSeries, PictorialBarSeries, PieDatum, PieSeries, PolarBarDatum,
    PolarBarSeries, RadarDataset, RadarIndicator, RadarSeries, SankeyLink, SankeyNode,
    SankeySeries, Scatter3DSeries, ScatterGeoSeries, ScatterSeries, Series, SeriesCoord,
    SunburstNode, SunburstSeries, Surface3DSeries, SymbolKind, ThemeRiverBand, ThemeRiverSeries,
    Title, TreeNode, TreeOrientation, TreeSeries, TreemapNode, TreemapSeries, WordCloudSeries,
};
pub use theme::{ChartTheme, Distribution, Harmony, ThemeMode};
pub use widget::ChartWidget;
