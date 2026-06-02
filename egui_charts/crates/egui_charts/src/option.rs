//! ECharts-shaped, strongly-typed chart description.
//!
//! Mirrors `option` objects users of Apache ECharts already know — `title`,
//! `legend`, `xAxis`/`yAxis`, `series[]`. Constructed via [`Chart::new()`].

use egui::Align2;

// ── ChartKind enum (single source of truth for the catalog) ─────────────────

/// Every chart type in the catalog. Drives both registry + docs so nothing
/// silently falls out of sync.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ChartKind {
    // 3.1 Cartesian
    Line,
    Bar,
    Scatter,
    EffectScatter,
    Candlestick,
    BoxPlot,
    Heatmap,
    PictorialBar,
    ThemeRiver,
    LinesCartesian,
    Custom,
    // 3.2 Polar
    Pie,
    Doughnut,
    Rose,
    Radar,
    Gauge,
    PolarBar,
    // 3.3 Hierarchical
    Tree,
    Treemap,
    Sunburst,
    // 3.4 Relational
    Graph,
    Sankey,
    Chord,
    // 3.5 Statistical
    Parallel,
    Funnel,
    CalendarHeatmap,
    // 3.6 Geographic
    Map,
    LinesGeo,
    ScatterGeo,
    // 3.7 3D (planned, optional)
    Bar3D,
    Line3D,
    Scatter3D,
    Surface3D,
    Map3D,
    Lines3D,
    Globe,
    // 3.8 Extensions
    WordCloud,
    LiquidFill,
}

impl ChartKind {
    pub fn label(self) -> &'static str {
        match self {
            ChartKind::Line => "Line",
            ChartKind::Bar => "Bar",
            ChartKind::Scatter => "Scatter",
            ChartKind::EffectScatter => "Effect scatter",
            ChartKind::Candlestick => "Candlestick",
            ChartKind::BoxPlot => "Box plot",
            ChartKind::Heatmap => "Heatmap",
            ChartKind::PictorialBar => "Pictorial bar",
            ChartKind::ThemeRiver => "Theme river",
            ChartKind::LinesCartesian => "Lines (cartesian)",
            ChartKind::Custom => "Custom",
            ChartKind::Pie => "Pie",
            ChartKind::Doughnut => "Doughnut",
            ChartKind::Rose => "Rose / Nightingale",
            ChartKind::Radar => "Radar",
            ChartKind::Gauge => "Gauge",
            ChartKind::PolarBar => "Polar bar",
            ChartKind::Tree => "Tree",
            ChartKind::Treemap => "Treemap",
            ChartKind::Sunburst => "Sunburst",
            ChartKind::Graph => "Graph",
            ChartKind::Sankey => "Sankey",
            ChartKind::Chord => "Chord",
            ChartKind::Parallel => "Parallel coordinates",
            ChartKind::Funnel => "Funnel",
            ChartKind::CalendarHeatmap => "Calendar heatmap",
            ChartKind::Map => "Map",
            ChartKind::LinesGeo => "Lines (geo)",
            ChartKind::ScatterGeo => "Scatter (geo)",
            ChartKind::Bar3D => "Bar 3D",
            ChartKind::Line3D => "Line 3D",
            ChartKind::Scatter3D => "Scatter 3D",
            ChartKind::Surface3D => "Surface 3D",
            ChartKind::Map3D => "Map 3D",
            ChartKind::Lines3D => "Lines 3D",
            ChartKind::Globe => "Globe",
            ChartKind::WordCloud => "Word cloud",
            ChartKind::LiquidFill => "Liquid fill",
        }
    }

    pub fn category(self) -> &'static str {
        match self {
            ChartKind::Line
            | ChartKind::Bar
            | ChartKind::Scatter
            | ChartKind::EffectScatter
            | ChartKind::Candlestick
            | ChartKind::BoxPlot
            | ChartKind::Heatmap
            | ChartKind::PictorialBar
            | ChartKind::ThemeRiver
            | ChartKind::LinesCartesian
            | ChartKind::Custom => "Cartesian",
            ChartKind::Pie
            | ChartKind::Doughnut
            | ChartKind::Rose
            | ChartKind::Radar
            | ChartKind::Gauge
            | ChartKind::PolarBar => "Polar",
            ChartKind::Tree | ChartKind::Treemap | ChartKind::Sunburst => "Hierarchical",
            ChartKind::Graph | ChartKind::Sankey | ChartKind::Chord => "Relational",
            ChartKind::Parallel | ChartKind::Funnel | ChartKind::CalendarHeatmap => "Statistical",
            ChartKind::Map | ChartKind::LinesGeo | ChartKind::ScatterGeo => "Geographic",
            ChartKind::Bar3D
            | ChartKind::Line3D
            | ChartKind::Scatter3D
            | ChartKind::Surface3D
            | ChartKind::Map3D
            | ChartKind::Lines3D
            | ChartKind::Globe => "3D",
            ChartKind::WordCloud | ChartKind::LiquidFill => "Extensions",
        }
    }

    /// Whether implementation exists. Gallery uses this to grey-out
    /// the planned-but-not-yet entries instead of crashing.
    pub fn is_implemented(self) -> bool {
        matches!(
            self,
            ChartKind::Line
                | ChartKind::Bar
                | ChartKind::Scatter
                | ChartKind::EffectScatter
                | ChartKind::Candlestick
                | ChartKind::BoxPlot
                | ChartKind::Heatmap
                | ChartKind::PictorialBar
                | ChartKind::ThemeRiver
                | ChartKind::LinesCartesian
                | ChartKind::Pie
                | ChartKind::Doughnut
                | ChartKind::Rose
                | ChartKind::Radar
                | ChartKind::Gauge
                | ChartKind::PolarBar
                | ChartKind::Sunburst
                | ChartKind::Treemap
                | ChartKind::Tree
                | ChartKind::Sankey
                | ChartKind::Graph
                | ChartKind::Parallel
                | ChartKind::Funnel
                | ChartKind::CalendarHeatmap
                | ChartKind::WordCloud
                | ChartKind::LiquidFill
                | ChartKind::Custom
                | ChartKind::Map
                | ChartKind::LinesGeo
                | ChartKind::ScatterGeo
                | ChartKind::Bar3D
                | ChartKind::Line3D
                | ChartKind::Scatter3D
                | ChartKind::Surface3D
                | ChartKind::Lines3D
                | ChartKind::Map3D
                | ChartKind::Globe
                | ChartKind::Chord
        )
    }

    pub fn all() -> &'static [ChartKind] {
        &[
            ChartKind::Line,
            ChartKind::Bar,
            ChartKind::Scatter,
            ChartKind::EffectScatter,
            ChartKind::Candlestick,
            ChartKind::BoxPlot,
            ChartKind::Heatmap,
            ChartKind::PictorialBar,
            ChartKind::ThemeRiver,
            ChartKind::LinesCartesian,
            ChartKind::Custom,
            ChartKind::Pie,
            ChartKind::Doughnut,
            ChartKind::Rose,
            ChartKind::Radar,
            ChartKind::Gauge,
            ChartKind::PolarBar,
            ChartKind::Tree,
            ChartKind::Treemap,
            ChartKind::Sunburst,
            ChartKind::Graph,
            ChartKind::Sankey,
            ChartKind::Chord,
            ChartKind::Parallel,
            ChartKind::Funnel,
            ChartKind::CalendarHeatmap,
            ChartKind::Map,
            ChartKind::LinesGeo,
            ChartKind::ScatterGeo,
            ChartKind::Bar3D,
            ChartKind::Line3D,
            ChartKind::Scatter3D,
            ChartKind::Surface3D,
            ChartKind::Map3D,
            ChartKind::Lines3D,
            ChartKind::Globe,
            ChartKind::WordCloud,
            ChartKind::LiquidFill,
        ]
    }
}

// ── Axis ─────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AxisKind {
    /// Continuous numeric range. Auto-fit when bounds omitted.
    Value,
    /// Discrete categories (strings).
    Category,
    /// Logarithmic — base 10. Auto-fit when bounds omitted.
    Log,
}

#[derive(Clone, Debug)]
pub struct Axis {
    pub kind: AxisKind,
    pub name: Option<String>,
    pub categories: Vec<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub show_grid: bool,
    pub show_axis_line: bool,
    pub show_tick_labels: bool,
    pub label_rotation: f32,
    /// Inverted axis (top→bottom for Y, right→left for X).
    pub inverse: bool,
}

impl Axis {
    pub fn value() -> Self {
        Self {
            kind: AxisKind::Value,
            name: None,
            categories: Vec::new(),
            min: None,
            max: None,
            show_grid: true,
            show_axis_line: true,
            show_tick_labels: true,
            label_rotation: 0.0,
            inverse: false,
        }
    }

    pub fn log() -> Self {
        Self {
            kind: AxisKind::Log,
            ..Self::value()
        }
    }

    pub fn category<I, S>(items: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        Self {
            kind: AxisKind::Category,
            categories: items.into_iter().map(Into::into).collect(),
            ..Self::value()
        }
    }

    pub fn name(mut self, name: impl Into<String>) -> Self {
        self.name = Some(name.into());
        self
    }

    pub fn min(mut self, v: f64) -> Self {
        self.min = Some(v);
        self
    }

    pub fn max(mut self, v: f64) -> Self {
        self.max = Some(v);
        self
    }

    pub fn rotate_labels(mut self, deg: f32) -> Self {
        self.label_rotation = deg;
        self
    }

    pub fn inverse(mut self, inverse: bool) -> Self {
        self.inverse = inverse;
        self
    }

    pub fn hide_grid(mut self) -> Self {
        self.show_grid = false;
        self
    }
}

// ── Title ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Title {
    pub text: String,
    pub subtext: Option<String>,
    pub align: Align2,
}

impl Title {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            subtext: None,
            align: Align2::LEFT_TOP,
        }
    }
}

// ── Legend ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LegendPosition {
    Top,
    Bottom,
    Left,
    Right,
    Hidden,
}

#[derive(Clone, Debug)]
pub struct Legend {
    pub position: LegendPosition,
}

impl Default for Legend {
    fn default() -> Self {
        Self {
            position: LegendPosition::Top,
        }
    }
}

// ── Series ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LineStyle {
    Solid,
    Smooth,
    Step,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SymbolKind {
    Circle,
    Square,
    Diamond,
    Triangle,
    Cross,
}

#[derive(Clone, Debug)]
pub struct LineSeries {
    pub name: String,
    pub data: Vec<f64>,
    pub style: LineStyle,
    pub fill_area: bool,
    pub area_alpha: u8,
    pub stack: Option<String>,
    pub line_width: f32,
    pub show_symbols: bool,
    pub symbol: SymbolKind,
}

impl LineSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            style: LineStyle::Solid,
            fill_area: false,
            area_alpha: 64,
            stack: None,
            line_width: 2.0,
            show_symbols: true,
            symbol: SymbolKind::Circle,
        }
    }

    pub fn data<I: IntoIterator<Item = f64>>(mut self, data: I) -> Self {
        self.data = data.into_iter().collect();
        self
    }

    pub fn smooth(mut self, smooth: bool) -> Self {
        self.style = if smooth {
            LineStyle::Smooth
        } else {
            LineStyle::Solid
        };
        self
    }

    pub fn step(mut self) -> Self {
        self.style = LineStyle::Step;
        self
    }

    pub fn area(mut self) -> Self {
        self.fill_area = true;
        self
    }

    pub fn stack(mut self, group: impl Into<String>) -> Self {
        self.stack = Some(group.into());
        self
    }

    pub fn hide_symbols(mut self) -> Self {
        self.show_symbols = false;
        self
    }
}

#[derive(Clone, Debug)]
pub struct BarSeries {
    pub name: String,
    pub data: Vec<f64>,
    pub stack: Option<String>,
    pub horizontal: bool,
    pub bar_width_ratio: f32,
}

impl BarSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            stack: None,
            horizontal: false,
            bar_width_ratio: 0.6,
        }
    }

    pub fn data<I: IntoIterator<Item = f64>>(mut self, data: I) -> Self {
        self.data = data.into_iter().collect();
        self
    }

    pub fn stack(mut self, group: impl Into<String>) -> Self {
        self.stack = Some(group.into());
        self
    }

    pub fn horizontal(mut self) -> Self {
        self.horizontal = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct ScatterSeries {
    pub name: String,
    /// Each datum: (x, y, optional size in pixels override).
    pub data: Vec<(f64, f64, Option<f32>)>,
    pub symbol: SymbolKind,
    pub symbol_size: f32,
}

impl ScatterSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            symbol: SymbolKind::Circle,
            symbol_size: 8.0,
        }
    }

    pub fn data<I: IntoIterator<Item = (f64, f64)>>(mut self, pts: I) -> Self {
        self.data = pts.into_iter().map(|(x, y)| (x, y, None)).collect();
        self
    }

    /// Bubble form — each datum carries its own size override.
    pub fn bubbles<I: IntoIterator<Item = (f64, f64, f32)>>(mut self, pts: I) -> Self {
        self.data = pts.into_iter().map(|(x, y, s)| (x, y, Some(s))).collect();
        self
    }

    pub fn symbol(mut self, sym: SymbolKind) -> Self {
        self.symbol = sym;
        self
    }

    pub fn size(mut self, px: f32) -> Self {
        self.symbol_size = px;
        self
    }
}

// ── Pie / Doughnut / Rose ────────────────────────────────────────────────────

/// One slice of a pie / doughnut / rose chart.
#[derive(Clone, Debug)]
pub struct PieDatum {
    pub name: String,
    pub value: f64,
}

impl PieDatum {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PieSeries {
    pub name: String,
    pub data: Vec<PieDatum>,
    /// 0.0 = pie, > 0 = doughnut (fraction of outer radius, 0..1).
    pub inner_ratio: f32,
    /// Outer radius as fraction of available radius (0..1).
    pub outer_ratio: f32,
    /// Rose / nightingale variant — radius scales with value.
    pub rose: bool,
    /// Show category labels with connector lines outside slices.
    pub show_labels: bool,
    /// Padding angle (degrees) between adjacent slices.
    pub pad_angle: f32,
    /// Corner radius (px) applied to slice tips.
    pub corner_radius: f32,
}

impl PieSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            inner_ratio: 0.0,
            outer_ratio: 0.75,
            rose: false,
            show_labels: true,
            pad_angle: 0.0,
            corner_radius: 0.0,
        }
    }

    pub fn data<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.data = items
            .into_iter()
            .map(|(n, v)| PieDatum::new(n, v))
            .collect();
        self
    }

    pub fn doughnut(mut self, inner_ratio: f32) -> Self {
        self.inner_ratio = inner_ratio.clamp(0.0, 0.95);
        self
    }

    pub fn rose(mut self) -> Self {
        self.rose = true;
        self
    }

    pub fn outer_ratio(mut self, r: f32) -> Self {
        self.outer_ratio = r.clamp(0.1, 1.0);
        self
    }

    pub fn pad_angle(mut self, deg: f32) -> Self {
        self.pad_angle = deg.max(0.0);
        self
    }

    pub fn hide_labels(mut self) -> Self {
        self.show_labels = false;
        self
    }
}

// ── Radar ────────────────────────────────────────────────────────────────────

/// One axis of a radar chart.
#[derive(Clone, Debug)]
pub struct RadarIndicator {
    pub name: String,
    pub max: f64,
}

impl RadarIndicator {
    pub fn new(name: impl Into<String>, max: f64) -> Self {
        Self {
            name: name.into(),
            max,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RadarDataset {
    pub name: String,
    pub values: Vec<f64>,
    pub fill_alpha: u8,
    pub line_width: f32,
}

impl RadarDataset {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            values: Vec::new(),
            fill_alpha: 50,
            line_width: 2.0,
        }
    }

    pub fn values<I: IntoIterator<Item = f64>>(mut self, vs: I) -> Self {
        self.values = vs.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct RadarSeries {
    pub name: String,
    pub indicators: Vec<RadarIndicator>,
    pub datasets: Vec<RadarDataset>,
    /// Concentric rings to draw as gridlines.
    pub rings: usize,
    /// Polygon (true) or circle (false) gridline shape.
    pub polygon_grid: bool,
}

impl RadarSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            indicators: Vec::new(),
            datasets: Vec::new(),
            rings: 4,
            polygon_grid: true,
        }
    }

    pub fn indicators<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.indicators = items
            .into_iter()
            .map(|(n, m)| RadarIndicator::new(n, m))
            .collect();
        self
    }

    pub fn dataset(mut self, ds: RadarDataset) -> Self {
        self.datasets.push(ds);
        self
    }

    pub fn rings(mut self, n: usize) -> Self {
        self.rings = n.max(1);
        self
    }

    pub fn circular_grid(mut self) -> Self {
        self.polygon_grid = false;
        self
    }
}

// ── Gauge ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct GaugeSeries {
    pub name: String,
    pub value: f64,
    pub min: f64,
    pub max: f64,
    /// Start angle of the arc (degrees, 0 = right, sweeping CCW per math convention).
    /// Default places the gauge as a "speedometer" bottom half.
    pub start_angle_deg: f32,
    pub end_angle_deg: f32,
    /// Thickness of the progress ring (fraction of radius, 0..1).
    pub thickness: f32,
    pub unit: Option<String>,
}

impl GaugeSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: 0.0,
            min: 0.0,
            max: 100.0,
            start_angle_deg: 225.0,
            end_angle_deg: -45.0,
            thickness: 0.18,
            unit: None,
        }
    }

    pub fn value(mut self, v: f64) -> Self {
        self.value = v;
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = min;
        self.max = max;
        self
    }

    pub fn arc(mut self, start_deg: f32, end_deg: f32) -> Self {
        self.start_angle_deg = start_deg;
        self.end_angle_deg = end_deg;
        self
    }

    pub fn thickness(mut self, t: f32) -> Self {
        self.thickness = t.clamp(0.02, 0.5);
        self
    }

    pub fn unit(mut self, unit: impl Into<String>) -> Self {
        self.unit = Some(unit.into());
        self
    }
}

// ── Funnel ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FunnelSeries {
    pub name: String,
    pub data: Vec<PieDatum>,
    /// Inverted = pyramid (small on top, large on bottom).
    pub inverted: bool,
    /// Gap between stacked trapezoids (px).
    pub gap: f32,
}

impl FunnelSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            inverted: false,
            gap: 2.0,
        }
    }

    pub fn data<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.data = items
            .into_iter()
            .map(|(n, v)| PieDatum::new(n, v))
            .collect();
        self
    }

    pub fn inverted(mut self) -> Self {
        self.inverted = true;
        self
    }

    pub fn gap(mut self, px: f32) -> Self {
        self.gap = px.max(0.0);
        self
    }
}

// ── Heatmap ──────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct HeatmapSeries {
    pub name: String,
    /// Each datum is `(x_index, y_index, value)`.
    pub data: Vec<(usize, usize, f64)>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    pub show_values: bool,
}

impl HeatmapSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            min: None,
            max: None,
            show_values: false,
        }
    }

    pub fn data<I: IntoIterator<Item = (usize, usize, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub fn show_values(mut self) -> Self {
        self.show_values = true;
        self
    }
}

// ── Candlestick ──────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct Candle {
    pub open: f64,
    pub close: f64,
    pub low: f64,
    pub high: f64,
}

impl Candle {
    pub fn ohlc(open: f64, high: f64, low: f64, close: f64) -> Self {
        Self {
            open,
            high,
            low,
            close,
        }
    }
}

#[derive(Clone, Debug)]
pub struct CandlestickSeries {
    pub name: String,
    pub data: Vec<Candle>,
    pub up_color: Option<egui::Color32>,
    pub down_color: Option<egui::Color32>,
    pub body_ratio: f32,
}

impl CandlestickSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            up_color: None,
            down_color: None,
            body_ratio: 0.6,
        }
    }

    pub fn data<I: IntoIterator<Item = Candle>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }

    pub fn up_color(mut self, c: egui::Color32) -> Self {
        self.up_color = Some(c);
        self
    }

    pub fn down_color(mut self, c: egui::Color32) -> Self {
        self.down_color = Some(c);
        self
    }
}

// ── BoxPlot ──────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug)]
pub struct BoxDatum {
    pub min: f64,
    pub q1: f64,
    pub median: f64,
    pub q3: f64,
    pub max: f64,
}

impl BoxDatum {
    pub fn new(min: f64, q1: f64, median: f64, q3: f64, max: f64) -> Self {
        Self {
            min,
            q1,
            median,
            q3,
            max,
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxPlotSeries {
    pub name: String,
    pub data: Vec<BoxDatum>,
    pub box_ratio: f32,
}

impl BoxPlotSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            box_ratio: 0.5,
        }
    }

    pub fn data<I: IntoIterator<Item = BoxDatum>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }
}

// ── EffectScatter ────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct EffectScatterSeries {
    pub name: String,
    pub data: Vec<(f64, f64, Option<f32>)>,
    pub symbol: SymbolKind,
    pub symbol_size: f32,
    pub ripple_count: usize,
}

impl EffectScatterSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            symbol: SymbolKind::Circle,
            symbol_size: 10.0,
            ripple_count: 2,
        }
    }

    pub fn data<I: IntoIterator<Item = (f64, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().map(|(x, y)| (x, y, None)).collect();
        self
    }
}

// ── LinesCartesian (arbitrary segments) ──────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LineSegment {
    pub from: (f64, f64),
    pub to: (f64, f64),
    pub value: f64,
}

impl LineSegment {
    pub fn new(from: (f64, f64), to: (f64, f64)) -> Self {
        Self {
            from,
            to,
            value: 1.0,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinesCartesianSeries {
    pub name: String,
    pub segments: Vec<LineSegment>,
    pub line_width: f32,
    pub show_endpoints: bool,
}

impl LinesCartesianSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            segments: Vec::new(),
            line_width: 1.6,
            show_endpoints: true,
        }
    }

    pub fn data<I: IntoIterator<Item = LineSegment>>(mut self, items: I) -> Self {
        self.segments = items.into_iter().collect();
        self
    }
}

// ── PictorialBar ─────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PictorialBarSeries {
    pub name: String,
    pub data: Vec<f64>,
    pub symbol: SymbolKind,
    pub symbol_size: f32,
    pub unit_value: f64,
}

impl PictorialBarSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            symbol: SymbolKind::Square,
            symbol_size: 14.0,
            unit_value: 1.0,
        }
    }

    pub fn data<I: IntoIterator<Item = f64>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }

    pub fn symbol(mut self, sym: SymbolKind) -> Self {
        self.symbol = sym;
        self
    }
}

// ── ThemeRiver ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ThemeRiverBand {
    pub name: String,
    pub data: Vec<f64>,
}

impl ThemeRiverBand {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
        }
    }

    pub fn data<I: IntoIterator<Item = f64>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct ThemeRiverSeries {
    pub name: String,
    pub bands: Vec<ThemeRiverBand>,
}

impl ThemeRiverSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            bands: Vec::new(),
        }
    }

    pub fn band(mut self, b: ThemeRiverBand) -> Self {
        self.bands.push(b);
        self
    }
}

// ── PolarBar ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct PolarBarDatum {
    pub name: String,
    pub value: f64,
}

impl PolarBarDatum {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct PolarBarSeries {
    pub name: String,
    pub data: Vec<PolarBarDatum>,
    pub inner_ratio: f32,
    pub outer_ratio: f32,
    pub pad_angle: f32,
}

impl PolarBarSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            inner_ratio: 0.18,
            outer_ratio: 0.85,
            pad_angle: 2.0,
        }
    }

    pub fn data<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.data = items
            .into_iter()
            .map(|(n, v)| PolarBarDatum::new(n, v))
            .collect();
        self
    }
}

// ── Sunburst ─────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SunburstNode {
    pub name: String,
    pub value: f64,
    pub children: Vec<SunburstNode>,
}

impl SunburstNode {
    pub fn leaf(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            children: Vec::new(),
        }
    }

    pub fn parent(name: impl Into<String>, children: Vec<SunburstNode>) -> Self {
        let value = children.iter().map(|c| c.subtree_value()).sum();
        Self {
            name: name.into(),
            value,
            children,
        }
    }

    pub fn subtree_value(&self) -> f64 {
        if self.children.is_empty() {
            self.value
        } else {
            self.children.iter().map(|c| c.subtree_value()).sum()
        }
    }
}

#[derive(Clone, Debug)]
pub struct SunburstSeries {
    pub name: String,
    pub roots: Vec<SunburstNode>,
    pub inner_ratio: f32,
    pub outer_ratio: f32,
}

impl SunburstSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            roots: Vec::new(),
            inner_ratio: 0.18,
            outer_ratio: 0.95,
        }
    }

    pub fn root(mut self, node: SunburstNode) -> Self {
        self.roots.push(node);
        self
    }
}

// ── Treemap ──────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct TreemapNode {
    pub name: String,
    pub value: f64,
    pub children: Vec<TreemapNode>,
}

impl TreemapNode {
    pub fn leaf(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            children: Vec::new(),
        }
    }

    pub fn parent(name: impl Into<String>, children: Vec<TreemapNode>) -> Self {
        let value = children.iter().map(|c| c.subtree_value()).sum();
        Self {
            name: name.into(),
            value,
            children,
        }
    }

    pub fn subtree_value(&self) -> f64 {
        if self.children.is_empty() {
            self.value
        } else {
            self.children.iter().map(|c| c.subtree_value()).sum()
        }
    }
}

#[derive(Clone, Debug)]
pub struct TreemapSeries {
    pub name: String,
    pub roots: Vec<TreemapNode>,
}

impl TreemapSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            roots: Vec::new(),
        }
    }

    pub fn root(mut self, node: TreemapNode) -> Self {
        self.roots.push(node);
        self
    }
}

// ── Tree (node-link) ─────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TreeOrientation {
    LeftRight,
    RightLeft,
    TopBottom,
    BottomTop,
    Radial,
}

#[derive(Clone, Debug)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
}

impl TreeNode {
    pub fn leaf(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
        }
    }

    pub fn parent(name: impl Into<String>, children: Vec<TreeNode>) -> Self {
        Self {
            name: name.into(),
            children,
        }
    }
}

#[derive(Clone, Debug)]
pub struct TreeSeries {
    pub name: String,
    pub root: TreeNode,
    pub orientation: TreeOrientation,
}

impl TreeSeries {
    pub fn new(name: impl Into<String>, root: TreeNode) -> Self {
        Self {
            name: name.into(),
            root,
            orientation: TreeOrientation::LeftRight,
        }
    }

    pub fn orientation(mut self, o: TreeOrientation) -> Self {
        self.orientation = o;
        self
    }
}

// ── Sankey ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct SankeyNode {
    pub name: String,
}

impl SankeyNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into() }
    }
}

#[derive(Clone, Debug)]
pub struct SankeyLink {
    pub source: usize,
    pub target: usize,
    pub value: f64,
}

impl SankeyLink {
    pub fn new(source: usize, target: usize, value: f64) -> Self {
        Self {
            source,
            target,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct SankeySeries {
    pub name: String,
    pub nodes: Vec<SankeyNode>,
    pub links: Vec<SankeyLink>,
    pub node_gap: f32,
    pub node_width: f32,
}

impl SankeySeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            links: Vec::new(),
            node_gap: 8.0,
            node_width: 14.0,
        }
    }

    pub fn nodes<I, S>(mut self, ns: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.nodes = ns.into_iter().map(|n| SankeyNode::new(n)).collect();
        self
    }

    pub fn links<I: IntoIterator<Item = SankeyLink>>(mut self, ls: I) -> Self {
        self.links = ls.into_iter().collect();
        self
    }
}

// ── Chord ────────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ChordSeries {
    pub name: String,
    /// Node labels (one entry per row/column of the matrix).
    pub labels: Vec<String>,
    /// `matrix[i][j]` = flow from node `i` to node `j`. Should be square.
    pub matrix: Vec<Vec<f64>>,
    /// Padding angle (degrees) between adjacent arcs.
    pub pad_angle: f32,
    /// Thickness of the outer label ring as a fraction of the outer radius.
    pub ring_thickness: f32,
}

impl ChordSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            labels: Vec::new(),
            matrix: Vec::new(),
            pad_angle: 2.0,
            ring_thickness: 0.06,
        }
    }

    pub fn labels<I, S>(mut self, ls: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        self.labels = ls.into_iter().map(Into::into).collect();
        self
    }

    pub fn matrix(mut self, m: Vec<Vec<f64>>) -> Self {
        self.matrix = m;
        self
    }
}

// ── Graph ────────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GraphLayout {
    Circular,
    Force,
}

#[derive(Clone, Debug)]
pub struct GraphNode {
    pub name: String,
    pub value: f64,
    /// Optional fixed position (data coords). `None` means layout decides.
    pub position: Option<(f64, f64)>,
}

impl GraphNode {
    pub fn new(name: impl Into<String>, value: f64) -> Self {
        Self {
            name: name.into(),
            value,
            position: None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GraphLink {
    pub source: usize,
    pub target: usize,
    pub value: f64,
}

impl GraphLink {
    pub fn new(source: usize, target: usize, value: f64) -> Self {
        Self {
            source,
            target,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct GraphSeries {
    pub name: String,
    pub nodes: Vec<GraphNode>,
    pub links: Vec<GraphLink>,
    pub layout: GraphLayout,
    pub node_size_min: f32,
    pub node_size_max: f32,
}

impl GraphSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            nodes: Vec::new(),
            links: Vec::new(),
            layout: GraphLayout::Circular,
            node_size_min: 8.0,
            node_size_max: 22.0,
        }
    }

    pub fn nodes<I: IntoIterator<Item = GraphNode>>(mut self, ns: I) -> Self {
        self.nodes = ns.into_iter().collect();
        self
    }

    pub fn links<I: IntoIterator<Item = GraphLink>>(mut self, ls: I) -> Self {
        self.links = ls.into_iter().collect();
        self
    }

    pub fn layout(mut self, l: GraphLayout) -> Self {
        self.layout = l;
        self
    }
}

// ── Parallel coordinates ─────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ParallelAxis {
    pub name: String,
    pub min: f64,
    pub max: f64,
}

impl ParallelAxis {
    pub fn new(name: impl Into<String>, min: f64, max: f64) -> Self {
        Self {
            name: name.into(),
            min,
            max,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParallelLine {
    pub name: String,
    pub values: Vec<f64>,
}

impl ParallelLine {
    pub fn new(name: impl Into<String>, values: Vec<f64>) -> Self {
        Self {
            name: name.into(),
            values,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ParallelSeries {
    pub name: String,
    pub axes: Vec<ParallelAxis>,
    pub lines: Vec<ParallelLine>,
}

impl ParallelSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            axes: Vec::new(),
            lines: Vec::new(),
        }
    }

    pub fn axes<I: IntoIterator<Item = ParallelAxis>>(mut self, axes: I) -> Self {
        self.axes = axes.into_iter().collect();
        self
    }

    pub fn line(mut self, l: ParallelLine) -> Self {
        self.lines.push(l);
        self
    }
}

// ── CalendarHeatmap ──────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct CalendarHeatmapSeries {
    pub name: String,
    /// `(day_of_year [1..=366], value)`. Day-of-year keeps the type
    /// independent of a real date library.
    pub data: Vec<(u32, f64)>,
    /// Year label rendered above the grid.
    pub year: i32,
    /// Optional weekday the year starts on (0 = Sunday … 6 = Saturday).
    /// Defaults to 0.
    pub start_weekday: u8,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

impl CalendarHeatmapSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            year: 2026,
            start_weekday: 0,
            min: None,
            max: None,
        }
    }

    pub fn year(mut self, y: i32) -> Self {
        self.year = y;
        self
    }

    pub fn start_weekday(mut self, w: u8) -> Self {
        self.start_weekday = w % 7;
        self
    }

    pub fn data<I: IntoIterator<Item = (u32, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }
}

// ── WordCloud ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct WordCloudSeries {
    pub name: String,
    /// Each word with its weight; weights drive font size.
    pub words: Vec<(String, f64)>,
    pub min_size: f32,
    pub max_size: f32,
}

impl WordCloudSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            words: Vec::new(),
            min_size: 12.0,
            max_size: 36.0,
        }
    }

    pub fn data<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64)>,
        S: Into<String>,
    {
        self.words = items.into_iter().map(|(t, w)| (t.into(), w)).collect();
        self
    }

    pub fn size_range(mut self, min: f32, max: f32) -> Self {
        self.min_size = min;
        self.max_size = max;
        self
    }
}

// ── LiquidFill ───────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct LiquidFillSeries {
    pub name: String,
    /// Fill level in `[0, 1]`.
    pub value: f64,
    /// Number of stacked waves (each at a different phase).
    pub waves: usize,
    pub wave_amplitude: f32,
    pub unit: Option<String>,
}

impl LiquidFillSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: 0.5,
            waves: 2,
            wave_amplitude: 6.0,
            unit: Some("%".into()),
        }
    }

    pub fn value(mut self, v: f64) -> Self {
        self.value = v.clamp(0.0, 1.0);
        self
    }

    pub fn waves(mut self, n: usize) -> Self {
        self.waves = n.max(1);
        self
    }

    pub fn unit(mut self, u: impl Into<String>) -> Self {
        self.unit = Some(u.into());
        self
    }
}

// ── Geo (Map / LinesGeo / ScatterGeo) ────────────────────────────────────────

/// One polygon path of a map region. Points are `(longitude, latitude)` in
/// **degrees**.
pub type GeoPath = Vec<(f64, f64)>;

#[derive(Clone, Debug)]
pub struct MapRegion {
    pub name: String,
    /// One or more rings making up the region (multi-polygon support).
    pub paths: Vec<GeoPath>,
    /// Choropleth value; `None` keeps the region neutral.
    pub value: Option<f64>,
}

impl MapRegion {
    pub fn new(name: impl Into<String>, ring: Vec<(f64, f64)>) -> Self {
        Self {
            name: name.into(),
            paths: vec![ring],
            value: None,
        }
    }

    pub fn with_value(mut self, v: f64) -> Self {
        self.value = Some(v);
        self
    }
}

#[derive(Clone, Debug)]
pub struct MapSeries {
    pub name: String,
    pub regions: Vec<MapRegion>,
    pub min: Option<f64>,
    pub max: Option<f64>,
    /// Optional bounding box override; auto-fits when `None`.
    pub bbox: Option<(f64, f64, f64, f64)>,
}

impl MapSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            regions: Vec::new(),
            min: None,
            max: None,
            bbox: None,
        }
    }

    pub fn regions<I: IntoIterator<Item = MapRegion>>(mut self, rs: I) -> Self {
        self.regions = rs.into_iter().collect();
        self
    }

    pub fn range(mut self, min: f64, max: f64) -> Self {
        self.min = Some(min);
        self.max = Some(max);
        self
    }

    pub fn bbox(mut self, min_lon: f64, max_lon: f64, min_lat: f64, max_lat: f64) -> Self {
        self.bbox = Some((min_lon, max_lon, min_lat, max_lat));
        self
    }
}

#[derive(Clone, Debug)]
pub struct GeoLine {
    pub name: String,
    /// Polyline through these `(lon, lat)` waypoints.
    pub points: Vec<(f64, f64)>,
    pub value: f64,
    /// Arc height as a fraction of segment length (0 = straight, ~0.3 looks
    /// like a great-circle arc on a small screen).
    pub arc_height: f32,
}

impl GeoLine {
    pub fn arc(name: impl Into<String>, from: (f64, f64), to: (f64, f64), value: f64) -> Self {
        Self {
            name: name.into(),
            points: vec![from, to],
            value,
            arc_height: 0.18,
        }
    }
}

#[derive(Clone, Debug)]
pub struct LinesGeoSeries {
    pub name: String,
    pub lines: Vec<GeoLine>,
    pub line_width: f32,
    pub show_endpoints: bool,
    pub bbox: Option<(f64, f64, f64, f64)>,
}

impl LinesGeoSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            lines: Vec::new(),
            line_width: 1.6,
            show_endpoints: true,
            bbox: None,
        }
    }

    pub fn data<I: IntoIterator<Item = GeoLine>>(mut self, ls: I) -> Self {
        self.lines = ls.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct GeoPoint {
    pub name: String,
    pub lon: f64,
    pub lat: f64,
    pub value: f64,
}

impl GeoPoint {
    pub fn new(name: impl Into<String>, lon: f64, lat: f64, value: f64) -> Self {
        Self {
            name: name.into(),
            lon,
            lat,
            value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct ScatterGeoSeries {
    pub name: String,
    pub points: Vec<GeoPoint>,
    pub symbol: SymbolKind,
    pub symbol_size_min: f32,
    pub symbol_size_max: f32,
    pub bbox: Option<(f64, f64, f64, f64)>,
}

impl ScatterGeoSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
            symbol: SymbolKind::Circle,
            symbol_size_min: 4.0,
            symbol_size_max: 18.0,
            bbox: None,
        }
    }

    pub fn data<I: IntoIterator<Item = GeoPoint>>(mut self, ps: I) -> Self {
        self.points = ps.into_iter().collect();
        self
    }
}

// ── 3D family ────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Bar3DSeries {
    pub name: String,
    /// `(x_index, z_index, height)` — heights extruded along Y.
    pub data: Vec<(usize, usize, f64)>,
    pub bar_size: f32,
}

impl Bar3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            bar_size: 0.6,
        }
    }

    pub fn data<I: IntoIterator<Item = (usize, usize, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Line3DSeries {
    pub name: String,
    pub data: Vec<(f64, f64, f64)>,
    pub line_width: f32,
}

impl Line3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            line_width: 2.0,
        }
    }

    pub fn data<I: IntoIterator<Item = (f64, f64, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Scatter3DSeries {
    pub name: String,
    pub data: Vec<(f64, f64, f64, Option<f32>)>,
    pub symbol: SymbolKind,
    pub symbol_size: f32,
}

impl Scatter3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            data: Vec::new(),
            symbol: SymbolKind::Circle,
            symbol_size: 8.0,
        }
    }

    pub fn data<I: IntoIterator<Item = (f64, f64, f64)>>(mut self, items: I) -> Self {
        self.data = items.into_iter().map(|(x, y, z)| (x, y, z, None)).collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Surface3DSeries {
    pub name: String,
    /// Heights in row-major order; rows = Z slices, columns = X slices.
    pub heights: Vec<Vec<f64>>,
    pub wireframe: bool,
}

impl Surface3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            heights: Vec::new(),
            wireframe: false,
        }
    }

    pub fn data(mut self, heights: Vec<Vec<f64>>) -> Self {
        self.heights = heights;
        self
    }

    pub fn wireframe(mut self) -> Self {
        self.wireframe = true;
        self
    }
}

#[derive(Clone, Debug)]
pub struct Lines3DSeries {
    pub name: String,
    /// `(from, to, value)`.
    pub segments: Vec<((f64, f64, f64), (f64, f64, f64), f64)>,
    pub line_width: f32,
}

impl Lines3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            segments: Vec::new(),
            line_width: 1.6,
        }
    }

    pub fn data<I: IntoIterator<Item = ((f64, f64, f64), (f64, f64, f64), f64)>>(
        mut self,
        items: I,
    ) -> Self {
        self.segments = items.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct Map3DSeries {
    pub name: String,
    /// Same shape as `MapSeries::regions`; heights drive extrusion.
    pub regions: Vec<MapRegion>,
    pub height_scale: f32,
}

impl Map3DSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            regions: Vec::new(),
            height_scale: 0.5,
        }
    }

    pub fn regions<I: IntoIterator<Item = MapRegion>>(mut self, rs: I) -> Self {
        self.regions = rs.into_iter().collect();
        self
    }
}

#[derive(Clone, Debug)]
pub struct GlobeSeries {
    pub name: String,
    /// `(lon, lat, value)` markers placed on the surface.
    pub points: Vec<(f64, f64, f64, String)>,
    /// Hemisphere shading off-axis intensity.
    pub yaw: f32,
    pub pitch: f32,
}

impl GlobeSeries {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            points: Vec::new(),
            yaw: 0.6,
            pitch: 0.3,
        }
    }

    pub fn data<I, S>(mut self, items: I) -> Self
    where
        I: IntoIterator<Item = (S, f64, f64, f64)>,
        S: Into<String>,
    {
        self.points = items
            .into_iter()
            .map(|(n, lon, lat, v)| (lon, lat, v, n.into()))
            .collect();
        self
    }
}

// ── Custom (escape hatch) ────────────────────────────────────────────────────

/// Function called by `CustomSeries::render`. Receives the chart painter,
/// the rect allocated to this series, the resolved theme, and the current
/// pointer position (when hovering). Implementations should produce shapes
/// directly via `painter`.
pub type CustomRenderFn = std::sync::Arc<
    dyn for<'a> Fn(
            &crate::render::ChartPainter<'a>,
            egui::Rect,
            &crate::theme::ChartTheme,
            Option<egui::Pos2>,
        ) + Send
        + Sync,
>;

pub struct CustomSeries {
    pub name: String,
    pub render: CustomRenderFn,
}

impl CustomSeries {
    pub fn new<F>(name: impl Into<String>, render: F) -> Self
    where
        F: for<'a> Fn(
                &crate::render::ChartPainter<'a>,
                egui::Rect,
                &crate::theme::ChartTheme,
                Option<egui::Pos2>,
            ) + Send
            + Sync
            + 'static,
    {
        Self {
            name: name.into(),
            render: std::sync::Arc::new(render),
        }
    }
}

impl Clone for CustomSeries {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            render: std::sync::Arc::clone(&self.render),
        }
    }
}

impl std::fmt::Debug for CustomSeries {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CustomSeries")
            .field("name", &self.name)
            .finish()
    }
}

// ── Series enum ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Series {
    Line(LineSeries),
    Bar(BarSeries),
    Scatter(ScatterSeries),
    Pie(PieSeries),
    Radar(RadarSeries),
    Gauge(GaugeSeries),
    Funnel(FunnelSeries),
    Heatmap(HeatmapSeries),
    Candlestick(CandlestickSeries),
    BoxPlot(BoxPlotSeries),
    EffectScatter(EffectScatterSeries),
    LinesCartesian(LinesCartesianSeries),
    PictorialBar(PictorialBarSeries),
    ThemeRiver(ThemeRiverSeries),
    PolarBar(PolarBarSeries),
    Sunburst(SunburstSeries),
    Treemap(TreemapSeries),
    Tree(TreeSeries),
    Sankey(SankeySeries),
    Graph(GraphSeries),
    Parallel(ParallelSeries),
    CalendarHeatmap(CalendarHeatmapSeries),
    WordCloud(WordCloudSeries),
    LiquidFill(LiquidFillSeries),
    Custom(CustomSeries),
    Map(MapSeries),
    LinesGeo(LinesGeoSeries),
    ScatterGeo(ScatterGeoSeries),
    Bar3D(Bar3DSeries),
    Line3D(Line3DSeries),
    Scatter3D(Scatter3DSeries),
    Surface3D(Surface3DSeries),
    Lines3D(Lines3DSeries),
    Map3D(Map3DSeries),
    Globe(GlobeSeries),
    Chord(ChordSeries),
}

/// Which coordinate system a series wants. Drives widget dispatch.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SeriesCoord {
    Cartesian,
    Polar,
    Radar,
    /// Lays out in the content rect directly (no axes).
    Boxed,
}

impl Series {
    pub fn line(name: impl Into<String>) -> LineSeries {
        LineSeries::new(name)
    }

    pub fn bar(name: impl Into<String>) -> BarSeries {
        BarSeries::new(name)
    }

    pub fn scatter(name: impl Into<String>) -> ScatterSeries {
        ScatterSeries::new(name)
    }

    pub fn pie(name: impl Into<String>) -> PieSeries {
        PieSeries::new(name)
    }

    pub fn radar(name: impl Into<String>) -> RadarSeries {
        RadarSeries::new(name)
    }

    pub fn gauge(name: impl Into<String>) -> GaugeSeries {
        GaugeSeries::new(name)
    }

    pub fn funnel(name: impl Into<String>) -> FunnelSeries {
        FunnelSeries::new(name)
    }

    pub fn name(&self) -> &str {
        match self {
            Series::Line(s) => &s.name,
            Series::Bar(s) => &s.name,
            Series::Scatter(s) => &s.name,
            Series::Pie(s) => &s.name,
            Series::Radar(s) => &s.name,
            Series::Gauge(s) => &s.name,
            Series::Funnel(s) => &s.name,
            Series::Heatmap(s) => &s.name,
            Series::Candlestick(s) => &s.name,
            Series::BoxPlot(s) => &s.name,
            Series::EffectScatter(s) => &s.name,
            Series::LinesCartesian(s) => &s.name,
            Series::PictorialBar(s) => &s.name,
            Series::ThemeRiver(s) => &s.name,
            Series::PolarBar(s) => &s.name,
            Series::Sunburst(s) => &s.name,
            Series::Treemap(s) => &s.name,
            Series::Tree(s) => &s.name,
            Series::Sankey(s) => &s.name,
            Series::Graph(s) => &s.name,
            Series::Parallel(s) => &s.name,
            Series::CalendarHeatmap(s) => &s.name,
            Series::WordCloud(s) => &s.name,
            Series::LiquidFill(s) => &s.name,
            Series::Custom(s) => &s.name,
            Series::Map(s) => &s.name,
            Series::LinesGeo(s) => &s.name,
            Series::ScatterGeo(s) => &s.name,
            Series::Bar3D(s) => &s.name,
            Series::Line3D(s) => &s.name,
            Series::Scatter3D(s) => &s.name,
            Series::Surface3D(s) => &s.name,
            Series::Lines3D(s) => &s.name,
            Series::Map3D(s) => &s.name,
            Series::Globe(s) => &s.name,
            Series::Chord(s) => &s.name,
        }
    }

    pub fn stack(&self) -> Option<&str> {
        match self {
            Series::Line(s) => s.stack.as_deref(),
            Series::Bar(s) => s.stack.as_deref(),
            _ => None,
        }
    }

    pub fn coord(&self) -> SeriesCoord {
        match self {
            Series::Line(_)
            | Series::Bar(_)
            | Series::Scatter(_)
            | Series::Heatmap(_)
            | Series::Candlestick(_)
            | Series::BoxPlot(_)
            | Series::EffectScatter(_)
            | Series::LinesCartesian(_)
            | Series::PictorialBar(_)
            | Series::ThemeRiver(_) => SeriesCoord::Cartesian,
            Series::Pie(_)
            | Series::Gauge(_)
            | Series::PolarBar(_)
            | Series::Sunburst(_)
            | Series::LiquidFill(_) => SeriesCoord::Polar,
            Series::Radar(_) => SeriesCoord::Radar,
            Series::Funnel(_)
            | Series::Treemap(_)
            | Series::Tree(_)
            | Series::Sankey(_)
            | Series::Graph(_)
            | Series::Parallel(_)
            | Series::CalendarHeatmap(_)
            | Series::WordCloud(_)
            | Series::Custom(_)
            | Series::Map(_)
            | Series::LinesGeo(_)
            | Series::ScatterGeo(_)
            | Series::Bar3D(_)
            | Series::Line3D(_)
            | Series::Scatter3D(_)
            | Series::Surface3D(_)
            | Series::Lines3D(_)
            | Series::Map3D(_)
            | Series::Globe(_)
            | Series::Chord(_) => SeriesCoord::Boxed,
        }
    }
}

impl From<LineSeries> for Series {
    fn from(s: LineSeries) -> Self {
        Series::Line(s)
    }
}

impl From<BarSeries> for Series {
    fn from(s: BarSeries) -> Self {
        Series::Bar(s)
    }
}

impl From<ScatterSeries> for Series {
    fn from(s: ScatterSeries) -> Self {
        Series::Scatter(s)
    }
}

impl From<PieSeries> for Series {
    fn from(s: PieSeries) -> Self {
        Series::Pie(s)
    }
}

impl From<RadarSeries> for Series {
    fn from(s: RadarSeries) -> Self {
        Series::Radar(s)
    }
}

impl From<GaugeSeries> for Series {
    fn from(s: GaugeSeries) -> Self {
        Series::Gauge(s)
    }
}

impl From<FunnelSeries> for Series {
    fn from(s: FunnelSeries) -> Self {
        Series::Funnel(s)
    }
}

impl From<HeatmapSeries> for Series {
    fn from(s: HeatmapSeries) -> Self {
        Series::Heatmap(s)
    }
}

impl From<CandlestickSeries> for Series {
    fn from(s: CandlestickSeries) -> Self {
        Series::Candlestick(s)
    }
}

impl From<BoxPlotSeries> for Series {
    fn from(s: BoxPlotSeries) -> Self {
        Series::BoxPlot(s)
    }
}

impl From<EffectScatterSeries> for Series {
    fn from(s: EffectScatterSeries) -> Self {
        Series::EffectScatter(s)
    }
}

impl From<LinesCartesianSeries> for Series {
    fn from(s: LinesCartesianSeries) -> Self {
        Series::LinesCartesian(s)
    }
}

impl From<PictorialBarSeries> for Series {
    fn from(s: PictorialBarSeries) -> Self {
        Series::PictorialBar(s)
    }
}

impl From<ThemeRiverSeries> for Series {
    fn from(s: ThemeRiverSeries) -> Self {
        Series::ThemeRiver(s)
    }
}

impl From<PolarBarSeries> for Series {
    fn from(s: PolarBarSeries) -> Self {
        Series::PolarBar(s)
    }
}

impl From<SunburstSeries> for Series {
    fn from(s: SunburstSeries) -> Self {
        Series::Sunburst(s)
    }
}

impl From<TreemapSeries> for Series {
    fn from(s: TreemapSeries) -> Self {
        Series::Treemap(s)
    }
}

impl From<TreeSeries> for Series {
    fn from(s: TreeSeries) -> Self {
        Series::Tree(s)
    }
}

impl From<SankeySeries> for Series {
    fn from(s: SankeySeries) -> Self {
        Series::Sankey(s)
    }
}

impl From<GraphSeries> for Series {
    fn from(s: GraphSeries) -> Self {
        Series::Graph(s)
    }
}

impl From<ParallelSeries> for Series {
    fn from(s: ParallelSeries) -> Self {
        Series::Parallel(s)
    }
}

impl From<CalendarHeatmapSeries> for Series {
    fn from(s: CalendarHeatmapSeries) -> Self {
        Series::CalendarHeatmap(s)
    }
}

impl From<WordCloudSeries> for Series {
    fn from(s: WordCloudSeries) -> Self {
        Series::WordCloud(s)
    }
}

impl From<LiquidFillSeries> for Series {
    fn from(s: LiquidFillSeries) -> Self {
        Series::LiquidFill(s)
    }
}

impl From<CustomSeries> for Series {
    fn from(s: CustomSeries) -> Self {
        Series::Custom(s)
    }
}

impl From<MapSeries> for Series {
    fn from(s: MapSeries) -> Self {
        Series::Map(s)
    }
}

impl From<LinesGeoSeries> for Series {
    fn from(s: LinesGeoSeries) -> Self {
        Series::LinesGeo(s)
    }
}

impl From<ScatterGeoSeries> for Series {
    fn from(s: ScatterGeoSeries) -> Self {
        Series::ScatterGeo(s)
    }
}

impl From<Bar3DSeries> for Series {
    fn from(s: Bar3DSeries) -> Self {
        Series::Bar3D(s)
    }
}

impl From<Line3DSeries> for Series {
    fn from(s: Line3DSeries) -> Self {
        Series::Line3D(s)
    }
}

impl From<Scatter3DSeries> for Series {
    fn from(s: Scatter3DSeries) -> Self {
        Series::Scatter3D(s)
    }
}

impl From<Surface3DSeries> for Series {
    fn from(s: Surface3DSeries) -> Self {
        Series::Surface3D(s)
    }
}

impl From<Lines3DSeries> for Series {
    fn from(s: Lines3DSeries) -> Self {
        Series::Lines3D(s)
    }
}

impl From<Map3DSeries> for Series {
    fn from(s: Map3DSeries) -> Self {
        Series::Map3D(s)
    }
}

impl From<GlobeSeries> for Series {
    fn from(s: GlobeSeries) -> Self {
        Series::Globe(s)
    }
}

impl From<ChordSeries> for Series {
    fn from(s: ChordSeries) -> Self {
        Series::Chord(s)
    }
}

// ── Chart (top-level option) ─────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct Chart {
    pub title: Option<Title>,
    pub legend: Legend,
    pub x_axis: Option<Axis>,
    pub y_axis: Option<Axis>,
    pub series: Vec<Series>,
    pub show_tooltip: bool,
    pub grid_padding: egui::Margin,
}

impl Chart {
    pub fn new() -> Self {
        Self {
            title: None,
            legend: Legend::default(),
            x_axis: None,
            y_axis: None,
            series: Vec::new(),
            show_tooltip: true,
            grid_padding: egui::Margin::symmetric(12, 12),
        }
    }

    pub fn title(mut self, text: impl Into<String>) -> Self {
        self.title = Some(Title::new(text));
        self
    }

    pub fn x_axis(mut self, axis: Axis) -> Self {
        self.x_axis = Some(axis);
        self
    }

    pub fn y_axis(mut self, axis: Axis) -> Self {
        self.y_axis = Some(axis);
        self
    }

    pub fn series(mut self, s: impl Into<Series>) -> Self {
        self.series.push(s.into());
        self
    }

    pub fn legend(mut self, legend: Legend) -> Self {
        self.legend = legend;
        self
    }

    pub fn hide_legend(mut self) -> Self {
        self.legend.position = LegendPosition::Hidden;
        self
    }

    pub fn tooltip(mut self, on: bool) -> Self {
        self.show_tooltip = on;
        self
    }
}
