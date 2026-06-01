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

    /// Whether Phase 0 implementation exists. Gallery uses this to grey-out
    /// the planned-but-not-yet entries instead of crashing.
    pub fn is_implemented(self) -> bool {
        matches!(self, ChartKind::Line | ChartKind::Bar | ChartKind::Scatter)
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
        self.style = if smooth { LineStyle::Smooth } else { LineStyle::Solid };
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

#[derive(Clone, Debug)]
pub enum Series {
    Line(LineSeries),
    Bar(BarSeries),
    Scatter(ScatterSeries),
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

    pub fn name(&self) -> &str {
        match self {
            Series::Line(s) => &s.name,
            Series::Bar(s) => &s.name,
            Series::Scatter(s) => &s.name,
        }
    }

    pub fn stack(&self) -> Option<&str> {
        match self {
            Series::Line(s) => s.stack.as_deref(),
            Series::Bar(s) => s.stack.as_deref(),
            Series::Scatter(_) => None,
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
