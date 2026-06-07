//! Sample dataset builders, one per `ChartKind`.

use egui_charts::{
    Axis, Bar3DSeries, BarSeries, BoxDatum, BoxPlotSeries,
    CalendarHeatmapSeries, Candle, CandlestickSeries, Chart, ChartKind,
    ChordSeries, CustomSeries, EffectScatterSeries, FunnelSeries, GaugeSeries,
    GeoLine, GeoPoint, GlobeSeries, GraphLayout, GraphLink, GraphNode,
    GraphSeries, HeatmapSeries, Line3DSeries, LineSegment, LineSeries,
    Lines3DSeries, LinesCartesianSeries, LinesGeoSeries, LiquidFillSeries,
    Map3DSeries, MapRegion, MapSeries, ParallelAxis, ParallelLine,
    ParallelSeries, PictorialBarSeries, PieSeries, PolarBarSeries,
    RadarDataset, RadarSeries, SankeyLink, SankeySeries, Scatter3DSeries,
    ScatterGeoSeries, ScatterSeries, Series, SunburstNode, SunburstSeries,
    Surface3DSeries, SymbolKind, ThemeRiverBand, ThemeRiverSeries, TreeNode,
    TreeOrientation, TreeSeries, TreemapNode, TreemapSeries, WordCloudSeries,
};

/// Build a sample chart for `kind`. Returns `None` for kinds not yet
/// implemented.
pub fn build(kind: ChartKind) -> Option<Chart> {
    match kind {
        ChartKind::Line => Some(line_sample()),
        ChartKind::Bar => Some(bar_sample()),
        ChartKind::Scatter => Some(scatter_sample()),
        ChartKind::EffectScatter => Some(effect_scatter_sample()),
        ChartKind::Candlestick => Some(candlestick_sample()),
        ChartKind::BoxPlot => Some(boxplot_sample()),
        ChartKind::Heatmap => Some(heatmap_sample()),
        ChartKind::PictorialBar => Some(pictorial_bar_sample()),
        ChartKind::ThemeRiver => Some(theme_river_sample()),
        ChartKind::LinesCartesian => Some(lines_cartesian_sample()),
        ChartKind::Pie => Some(pie_sample()),
        ChartKind::Doughnut => Some(doughnut_sample()),
        ChartKind::Rose => Some(rose_sample()),
        ChartKind::Radar => Some(radar_sample()),
        ChartKind::Gauge => Some(gauge_sample()),
        ChartKind::PolarBar => Some(polar_bar_sample()),
        ChartKind::Sunburst => Some(sunburst_sample()),
        ChartKind::Treemap => Some(treemap_sample()),
        ChartKind::Tree => Some(tree_sample()),
        ChartKind::Sankey => Some(sankey_sample()),
        ChartKind::Graph => Some(graph_sample()),
        ChartKind::Parallel => Some(parallel_sample()),
        ChartKind::Funnel => Some(funnel_sample()),
        ChartKind::CalendarHeatmap => Some(calendar_heatmap_sample()),
        ChartKind::WordCloud => Some(word_cloud_sample()),
        ChartKind::LiquidFill => Some(liquid_fill_sample()),
        ChartKind::Custom => Some(custom_sample()),
        ChartKind::Map => Some(map_sample()),
        ChartKind::LinesGeo => Some(lines_geo_sample()),
        ChartKind::ScatterGeo => Some(scatter_geo_sample()),
        ChartKind::Bar3D => Some(bar_3d_sample()),
        ChartKind::Line3D => Some(line_3d_sample()),
        ChartKind::Scatter3D => Some(scatter_3d_sample()),
        ChartKind::Surface3D => Some(surface_3d_sample()),
        ChartKind::Lines3D => Some(lines_3d_sample()),
        ChartKind::Map3D => Some(map_3d_sample()),
        ChartKind::Globe => Some(globe_sample()),
        ChartKind::Chord => Some(chord_sample()),
    }
}

fn line_sample() -> Chart {
    Chart::new()
        .title("Website traffic — last 12 weeks")
        .x_axis(Axis::category((1..=12).map(|i| format!("W{i}"))))
        .y_axis(Axis::value().name("Visitors (k)"))
        .series(LineSeries::new("Direct").smooth(true).area().data([
            22.0, 25.0, 30.0, 28.0, 35.0, 38.0, 42.0, 45.0, 50.0, 48.0, 55.0,
            62.0,
        ]))
        .series(LineSeries::new("Search").smooth(true).data([
            18.0, 19.0, 22.0, 20.0, 24.0, 27.0, 26.0, 30.0, 35.0, 33.0, 40.0,
            45.0,
        ]))
        .series(LineSeries::new("Referral").step().data([
            8.0, 9.0, 11.0, 12.0, 12.0, 14.0, 16.0, 17.0, 17.0, 19.0, 21.0,
            22.0,
        ]))
}

fn bar_sample() -> Chart {
    Chart::new()
        .title("Quarterly revenue by region")
        .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
        .y_axis(Axis::value().name("Revenue ($M)"))
        .series(
            BarSeries::new("EMEA")
                .stack("rev")
                .data([12.0, 18.0, 9.0, 22.0]),
        )
        .series(
            BarSeries::new("AMER")
                .stack("rev")
                .data([15.0, 14.0, 19.0, 17.0]),
        )
        .series(
            BarSeries::new("APAC")
                .stack("rev")
                .data([7.0, 9.0, 13.0, 11.0]),
        )
}

fn scatter_sample() -> Chart {
    let pts: Vec<(f64, f64)> = (0..40)
        .map(|i| {
            let x = i as f64 * 0.25;
            let y = (x * 0.6).sin() * 5.0 + x * 0.5 + ((i as f64 * 13.7) % 3.0);
            (x, y)
        })
        .collect();

    let bubbles: Vec<(f64, f64, f32)> = (0..14)
        .map(|i| {
            let x = i as f64 * 0.7 + 1.0;
            let y = ((i as f64 * 1.3).cos() * 3.0) + 4.0 + (i as f64 * 0.15);
            let s = 8.0 + ((i * 5) % 16) as f32;
            (x, y, s)
        })
        .collect();

    Chart::new()
        .title("Scatter + bubble overlay")
        .x_axis(Axis::value().name("x"))
        .y_axis(Axis::value().name("y"))
        .series(
            ScatterSeries::new("series A")
                .data(pts)
                .symbol(SymbolKind::Circle)
                .size(6.0),
        )
        .series(Series::Scatter(
            ScatterSeries::new("bubbles")
                .bubbles(bubbles)
                .symbol(SymbolKind::Diamond),
        ))
}

fn effect_scatter_sample() -> Chart {
    let pts: Vec<(f64, f64)> = (0..12)
        .map(|i| {
            let x = (i as f64 + 1.0) * 0.7;
            let y = (x * 0.8).sin() * 3.0 + x;
            (x, y)
        })
        .collect();

    Chart::new()
        .title("Effect scatter — highlighted observations")
        .x_axis(Axis::value().name("time"))
        .y_axis(Axis::value().name("signal"))
        .series(EffectScatterSeries::new("anomalies").data(pts))
}

fn candlestick_sample() -> Chart {
    // Toy OHLC sequence — random walk with bullish/bearish alternation.
    let mut price = 100.0;
    let candles: Vec<Candle> = (0..20)
        .map(|i| {
            let drift = ((i as f64 * 0.7).sin() * 4.0) + 0.3;
            let open = price;
            let close = price + drift;
            let high =
                open.max(close) + (((i as f64 * 1.7).cos().abs()) * 2.0 + 1.0);
            let low =
                open.min(close) - (((i as f64 * 2.1).sin().abs()) * 1.5 + 1.0);
            price = close;
            Candle::ohlc(open, high, low, close)
        })
        .collect();

    Chart::new()
        .title("Candlestick — synthetic ticker")
        .x_axis(Axis::category((1..=20).map(|i| format!("d{i}"))))
        .y_axis(Axis::value().name("Price"))
        .series(CandlestickSeries::new("XYZ").data(candles))
}

fn boxplot_sample() -> Chart {
    let data: Vec<BoxDatum> = [
        (3.0, 8.0, 12.0, 18.0, 26.0),
        (5.0, 12.0, 17.0, 22.0, 32.0),
        (6.0, 10.0, 14.0, 18.0, 24.0),
        (4.0, 11.0, 16.0, 23.0, 35.0),
        (2.0, 7.0, 11.0, 15.0, 22.0),
    ]
    .into_iter()
    .map(|(lo, q1, med, q3, hi)| BoxDatum::new(lo, q1, med, q3, hi))
    .collect();

    Chart::new()
        .title("Latency distribution by deployment")
        .x_axis(Axis::category(["v1.0", "v1.1", "v1.2", "v1.3", "v1.4"]))
        .y_axis(Axis::value().name("ms"))
        .series(BoxPlotSeries::new("p50/p25/p75").data(data))
}

fn heatmap_sample() -> Chart {
    let mut cells: Vec<(usize, usize, f64)> = Vec::new();
    for hour in 0..24 {
        for day in 0..7 {
            let v = ((hour as f64 - 12.0).abs() + day as f64 * 0.7).sin()
                * 30.0
                + 50.0
                + ((hour * day) as f64 % 7.0);
            cells.push((hour, day, v));
        }
    }

    Chart::new()
        .title("Active users by weekday / hour")
        .x_axis(Axis::category((0..24).map(|h| format!("{h:02}"))))
        .y_axis(Axis::category([
            "Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun",
        ]))
        .series(HeatmapSeries::new("users").data(cells))
}

fn pictorial_bar_sample() -> Chart {
    Chart::new()
        .title("Releases shipped per quarter")
        .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
        .y_axis(Axis::value().name("Releases"))
        .series(
            PictorialBarSeries::new("Releases")
                .symbol(SymbolKind::Triangle)
                .data([4.0, 6.0, 9.0, 7.0]),
        )
}

fn theme_river_sample() -> Chart {
    let n = 20;
    let band = |amp: f64, phase: f64| -> Vec<f64> {
        (0..n)
            .map(|i| {
                let x = i as f64 * 0.6 + phase;
                (x.sin() * 0.5 + 1.0) * amp
            })
            .collect()
    };

    Chart::new()
        .title("Stream graph — topic share over time")
        .x_axis(Axis::category((1..=n).map(|i| i.to_string())))
        .y_axis(Axis::value().name("Volume"))
        .series(
            ThemeRiverSeries::new("Topics")
                .band(ThemeRiverBand::new("AI").data(band(40.0, 0.0)))
                .band(ThemeRiverBand::new("Rust").data(band(28.0, 1.7)))
                .band(ThemeRiverBand::new("Web").data(band(20.0, 3.0)))
                .band(ThemeRiverBand::new("Mobile").data(band(15.0, 4.5))),
        )
}

fn lines_cartesian_sample() -> Chart {
    let segments: Vec<LineSegment> = (0..20)
        .map(|i| {
            let t = i as f64 * 0.3;
            let from = (t.sin() * 4.0 + 5.0, t.cos() * 4.0 + 5.0);
            let to = ((t + 0.6).sin() * 4.0 + 5.0, (t + 0.6).cos() * 4.0 + 5.0);
            LineSegment {
                from,
                to,
                value: (t * 1.3).sin() * 0.5 + 0.5,
            }
        })
        .collect();

    Chart::new()
        .title("Lines (cartesian) — value-colored segments")
        .x_axis(Axis::value().min(0.0).max(10.0))
        .y_axis(Axis::value().min(0.0).max(10.0))
        .series(LinesCartesianSeries::new("flow").data(segments))
}

fn pie_sample() -> Chart {
    Chart::new().title("Browser market share").series(
        PieSeries::new("Browsers")
            .data([
                ("Chrome", 64.0),
                ("Safari", 19.0),
                ("Edge", 5.0),
                ("Firefox", 3.0),
                ("Other", 9.0),
            ])
            .pad_angle(1.0),
    )
}

fn doughnut_sample() -> Chart {
    Chart::new().title("Quarterly revenue split").series(
        PieSeries::new("Revenue")
            .data([
                ("EMEA", 42.0),
                ("AMER", 58.0),
                ("APAC", 31.0),
                ("LATAM", 14.0),
            ])
            .doughnut(0.55)
            .pad_angle(2.0),
    )
}

fn rose_sample() -> Chart {
    Chart::new()
        .title("Nightingale — issues opened by day")
        .series(
            PieSeries::new("Issues")
                .data([
                    ("Mon", 18.0),
                    ("Tue", 25.0),
                    ("Wed", 31.0),
                    ("Thu", 22.0),
                    ("Fri", 17.0),
                    ("Sat", 8.0),
                    ("Sun", 5.0),
                ])
                .rose()
                .doughnut(0.18)
                .pad_angle(1.0),
        )
}

fn radar_sample() -> Chart {
    Chart::new().title("Product fit by segment").series(
        RadarSeries::new("Segments")
            .indicators([
                ("Performance", 100.0),
                ("Ergonomics", 100.0),
                ("Cost", 100.0),
                ("Reliability", 100.0),
                ("Power efficiency", 100.0),
                ("Aesthetics", 100.0),
            ])
            .dataset(
                RadarDataset::new("Current")
                    .values([78.0, 88.0, 64.0, 92.0, 70.0, 55.0]),
            )
            .dataset(
                RadarDataset::new("Competitor")
                    .values([65.0, 70.0, 80.0, 60.0, 85.0, 78.0]),
            ),
    )
}

fn gauge_sample() -> Chart {
    Chart::new().title("Cluster CPU usage").series(
        GaugeSeries::new("CPU")
            .range(0.0, 100.0)
            .value(67.4)
            .unit("%"),
    )
}

fn polar_bar_sample() -> Chart {
    Chart::new()
        .title("Polar bar — pull requests merged by weekday")
        .series(PolarBarSeries::new("PRs").data([
            ("Mon", 32.0),
            ("Tue", 41.0),
            ("Wed", 28.0),
            ("Thu", 38.0),
            ("Fri", 22.0),
            ("Sat", 7.0),
            ("Sun", 4.0),
        ]))
}

fn sunburst_sample() -> Chart {
    Chart::new().title("Subscription tier breakdown").series(
        SunburstSeries::new("Tiers").root(SunburstNode::parent(
            "All",
            vec![
                SunburstNode::parent(
                    "Free",
                    vec![
                        SunburstNode::leaf("Active", 60.0),
                        SunburstNode::leaf("Idle", 22.0),
                    ],
                ),
                SunburstNode::parent(
                    "Pro",
                    vec![
                        SunburstNode::leaf("Solo", 32.0),
                        SunburstNode::leaf("Team", 18.0),
                    ],
                ),
                SunburstNode::parent(
                    "Enterprise",
                    vec![
                        SunburstNode::leaf("Annual", 8.0),
                        SunburstNode::leaf("Custom", 5.0),
                    ],
                ),
            ],
        )),
    )
}

fn treemap_sample() -> Chart {
    Chart::new()
        .title("Treemap — language distribution")
        .series(
            TreemapSeries::new("Code")
                .root(TreemapNode::parent(
                    "Frontend",
                    vec![
                        TreemapNode::leaf("TypeScript", 220.0),
                        TreemapNode::leaf("HTML/CSS", 80.0),
                        TreemapNode::leaf("Svelte", 60.0),
                    ],
                ))
                .root(TreemapNode::parent(
                    "Backend",
                    vec![
                        TreemapNode::leaf("Rust", 320.0),
                        TreemapNode::leaf("Python", 110.0),
                        TreemapNode::leaf("Go", 70.0),
                    ],
                ))
                .root(TreemapNode::parent(
                    "Infra",
                    vec![
                        TreemapNode::leaf("Terraform", 40.0),
                        TreemapNode::leaf("Shell", 25.0),
                    ],
                )),
        )
}

fn tree_sample() -> Chart {
    Chart::new().title("Org tree — engineering").series(
        TreeSeries::new(
            "Eng",
            TreeNode::parent(
                "VP Eng",
                vec![
                    TreeNode::parent(
                        "Platform",
                        vec![
                            TreeNode::leaf("Storage"),
                            TreeNode::leaf("Networking"),
                            TreeNode::leaf("Identity"),
                        ],
                    ),
                    TreeNode::parent(
                        "Product",
                        vec![TreeNode::leaf("Web"), TreeNode::leaf("Mobile")],
                    ),
                    TreeNode::parent(
                        "Data",
                        vec![TreeNode::leaf("Pipelines"), TreeNode::leaf("ML")],
                    ),
                ],
            ),
        )
        .orientation(TreeOrientation::LeftRight),
    )
}

fn sankey_sample() -> Chart {
    let names = [
        "Visits", "Signups", "Free", "Trial", "Paid", "Churned", "Retained",
    ];

    Chart::new().title("Funnel as Sankey flow").series(
        SankeySeries::new("Flow")
            .nodes(names.iter().copied())
            .links([
                SankeyLink::new(0, 1, 100.0),
                SankeyLink::new(1, 2, 60.0),
                SankeyLink::new(1, 3, 40.0),
                SankeyLink::new(2, 5, 30.0),
                SankeyLink::new(2, 6, 30.0),
                SankeyLink::new(3, 4, 25.0),
                SankeyLink::new(3, 5, 15.0),
                SankeyLink::new(4, 6, 20.0),
                SankeyLink::new(4, 5, 5.0),
            ]),
    )
}

fn graph_sample() -> Chart {
    let nodes: Vec<GraphNode> = [
        ("Auth", 5.0),
        ("API", 12.0),
        ("Cache", 7.0),
        ("DB", 14.0),
        ("Queue", 6.0),
        ("Worker", 8.0),
        ("Notifier", 4.0),
    ]
    .into_iter()
    .map(|(n, v)| GraphNode::new(n, v))
    .collect();

    let links = [
        GraphLink::new(0, 1, 4.0),
        GraphLink::new(1, 2, 6.0),
        GraphLink::new(1, 3, 5.0),
        GraphLink::new(2, 3, 3.0),
        GraphLink::new(1, 4, 2.0),
        GraphLink::new(4, 5, 4.0),
        GraphLink::new(5, 3, 3.0),
        GraphLink::new(5, 6, 2.0),
    ];

    Chart::new().title("Service graph").series(
        GraphSeries::new("Services")
            .nodes(nodes)
            .links(links)
            .layout(GraphLayout::Force),
    )
}

fn parallel_sample() -> Chart {
    Chart::new().title("Parallel — laptop specs").series(
        ParallelSeries::new("Models")
            .axes([
                ParallelAxis::new("Battery (h)", 6.0, 20.0),
                ParallelAxis::new("Weight (kg)", 0.9, 2.2),
                ParallelAxis::new("RAM (GB)", 8.0, 64.0),
                ParallelAxis::new("Price ($)", 800.0, 3200.0),
                ParallelAxis::new("Score", 50.0, 100.0),
            ])
            .line(ParallelLine::new("A", vec![14.0, 1.3, 16.0, 1600.0, 82.0]))
            .line(ParallelLine::new("B", vec![10.5, 1.5, 32.0, 2400.0, 89.0]))
            .line(ParallelLine::new("C", vec![18.0, 1.0, 8.0, 1100.0, 70.0]))
            .line(ParallelLine::new("D", vec![8.0, 2.0, 64.0, 3100.0, 96.0]))
            .line(ParallelLine::new("E", vec![12.0, 1.7, 16.0, 1900.0, 78.0])),
    )
}

fn funnel_sample() -> Chart {
    Chart::new().title("Acquisition funnel").series(
        FunnelSeries::new("Conversion")
            .data([
                ("Visits", 100_000.0),
                ("Signups", 32_500.0),
                ("Activated", 14_200.0),
                ("Paying", 4_100.0),
                ("Retained", 1_850.0),
            ])
            .gap(3.0),
    )
}

fn word_cloud_sample() -> Chart {
    Chart::new()
        .title("Word cloud — search terms this month")
        .series(WordCloudSeries::new("Terms").data([
            ("rust", 240.0),
            ("egui", 180.0),
            ("charts", 160.0),
            ("shadcn", 130.0),
            ("epaint", 110.0),
            ("typescript", 95.0),
            ("react", 85.0),
            ("vite", 70.0),
            ("trunk", 65.0),
            ("svelte", 60.0),
            ("axum", 55.0),
            ("postgres", 50.0),
            ("wasm", 45.0),
            ("hot reload", 40.0),
            ("docs", 38.0),
            ("ide", 35.0),
            ("tokio", 30.0),
            ("serde", 28.0),
            ("clippy", 24.0),
            ("ci", 22.0),
            ("linker", 18.0),
            ("perf", 16.0),
            ("logs", 14.0),
            ("storage", 12.0),
            ("api", 10.0),
        ]))
}

fn liquid_fill_sample() -> Chart {
    Chart::new()
        .title("Disk capacity")
        .series(LiquidFillSeries::new("Used").value(0.68).waves(3))
}

fn chord_sample() -> Chart {
    let labels = ["NA", "EU", "APAC", "LATAM", "MEA"];
    let matrix = vec![
        vec![0.0, 24.0, 18.0, 12.0, 6.0],
        vec![21.0, 0.0, 16.0, 8.0, 14.0],
        vec![17.0, 14.0, 0.0, 6.0, 9.0],
        vec![10.0, 7.0, 5.0, 0.0, 3.0],
        vec![5.0, 12.0, 8.0, 4.0, 0.0],
    ];
    Chart::new()
        .title("Chord — regional trade flows")
        .series(ChordSeries::new("Flows").labels(labels).matrix(matrix))
}

fn custom_sample() -> Chart {
    Chart::new()
        .title("Custom — sketched dashboard tile")
        .series(CustomSeries::new("Sketch", |p, rect, theme, _hover| {
            let inner = rect.shrink(16.0);
            p.rounded_rect_filled(inner, 6.0, theme.surface);
            // Sparkline.
            let series_color = theme.series_color(0);
            let pts: Vec<egui::Pos2> = (0..24)
                .map(|i| {
                    let t = i as f32 / 23.0;
                    let y = (t * 8.0).sin() * 0.5 + 0.5;
                    egui::pos2(
                        inner.min.x + 12.0 + t * (inner.width() - 24.0),
                        inner.max.y - 14.0 - y * (inner.height() - 60.0),
                    )
                })
                .collect();
            p.path(pts, egui::Stroke::new(2.0, series_color));
            // Caption.
            p.text(
                egui::pos2(inner.min.x + 12.0, inner.min.y + 10.0),
                egui::Align2::LEFT_TOP,
                "Anything you draw with ChartPainter",
                egui::FontId::proportional(13.0),
                theme.text,
            );
        }))
}

fn map_sample() -> Chart {
    let regions = vec![
        MapRegion::new(
            "North America",
            vec![
                (-168.0, 65.0),
                (-141.0, 70.0),
                (-110.0, 73.0),
                (-90.0, 80.0),
                (-60.0, 82.0),
                (-55.0, 73.0),
                (-55.0, 50.0),
                (-65.0, 45.0),
                (-80.0, 27.0),
                (-95.0, 18.0),
                (-105.0, 20.0),
                (-115.0, 30.0),
                (-125.0, 40.0),
                (-130.0, 55.0),
                (-150.0, 58.0),
                (-168.0, 65.0),
            ],
        )
        .with_value(82.0),
        MapRegion::new(
            "South America",
            vec![
                (-80.0, 12.0),
                (-60.0, 12.0),
                (-50.0, 5.0),
                (-35.0, -5.0),
                (-40.0, -20.0),
                (-55.0, -35.0),
                (-65.0, -55.0),
                (-72.0, -50.0),
                (-78.0, -10.0),
                (-80.0, 12.0),
            ],
        )
        .with_value(48.0),
        MapRegion::new(
            "Europe",
            vec![
                (-10.0, 36.0),
                (0.0, 43.0),
                (10.0, 38.0),
                (28.0, 41.0),
                (40.0, 47.0),
                (60.0, 60.0),
                (60.0, 70.0),
                (30.0, 71.0),
                (5.0, 60.0),
                (-10.0, 50.0),
                (-10.0, 36.0),
            ],
        )
        .with_value(94.0),
        MapRegion::new(
            "Africa",
            vec![
                (-17.0, 14.0),
                (0.0, 35.0),
                (12.0, 35.0),
                (35.0, 30.0),
                (43.0, 12.0),
                (52.0, 0.0),
                (40.0, -15.0),
                (30.0, -28.0),
                (18.0, -34.0),
                (12.0, -22.0),
                (8.0, -2.0),
                (-5.0, 5.0),
                (-15.0, 12.0),
                (-17.0, 14.0),
            ],
        )
        .with_value(36.0),
        MapRegion::new(
            "Asia",
            vec![
                (30.0, 50.0),
                (45.0, 60.0),
                (70.0, 75.0),
                (110.0, 78.0),
                (140.0, 75.0),
                (170.0, 65.0),
                (155.0, 50.0),
                (140.0, 30.0),
                (120.0, 18.0),
                (100.0, 12.0),
                (95.0, 5.0),
                (78.0, 8.0),
                (65.0, 25.0),
                (50.0, 30.0),
                (40.0, 35.0),
                (30.0, 50.0),
            ],
        )
        .with_value(76.0),
        MapRegion::new(
            "Australia",
            vec![
                (113.0, -22.0),
                (130.0, -12.0),
                (143.0, -10.0),
                (153.0, -25.0),
                (148.0, -38.0),
                (135.0, -36.0),
                (118.0, -33.0),
                (113.0, -22.0),
            ],
        )
        .with_value(58.0),
    ];

    Chart::new()
        .title("Choropleth — engagement index by region")
        .series(MapSeries::new("Engagement").regions(regions))
}

fn lines_geo_sample() -> Chart {
    let cities = [
        ("NYC", -74.0, 40.7),
        ("London", -0.1, 51.5),
        ("Tokyo", 139.7, 35.7),
        ("Sydney", 151.2, -33.9),
        ("São Paulo", -46.6, -23.6),
        ("Cairo", 31.2, 30.0),
    ];
    let mut lines: Vec<GeoLine> = Vec::new();
    for i in 0..cities.len() {
        for j in (i + 1)..cities.len() {
            let from = (cities[i].1, cities[i].2);
            let to = (cities[j].1, cities[j].2);
            lines.push(GeoLine::arc(
                format!("{}→{}", cities[i].0, cities[j].0),
                from,
                to,
                ((i + j) as f64).sqrt() * 10.0,
            ));
        }
    }
    Chart::new()
        .title("Flight network — sample arcs")
        .series(LinesGeoSeries::new("flights").data(lines))
}

fn scatter_geo_sample() -> Chart {
    let pts = [
        ("NYC", -74.0, 40.7, 38.0),
        ("London", -0.1, 51.5, 28.0),
        ("Paris", 2.3, 48.9, 22.0),
        ("Tokyo", 139.7, 35.7, 35.0),
        ("Singapore", 103.8, 1.4, 18.0),
        ("Sydney", 151.2, -33.9, 16.0),
        ("São Paulo", -46.6, -23.6, 24.0),
        ("Cairo", 31.2, 30.0, 12.0),
        ("Mumbai", 72.9, 19.1, 30.0),
        ("Moscow", 37.6, 55.8, 14.0),
        ("Lagos", 3.4, 6.5, 20.0),
        ("Mexico City", -99.1, 19.4, 26.0),
    ]
    .into_iter()
    .map(|(n, lon, lat, v)| GeoPoint::new(n, lon, lat, v))
    .collect::<Vec<_>>();

    Chart::new()
        .title("Scatter on world map")
        .series(ScatterGeoSeries::new("cities").data(pts))
}

fn bar_3d_sample() -> Chart {
    let mut data: Vec<(usize, usize, f64)> = Vec::new();
    for x in 0..8 {
        for z in 0..6 {
            let v = ((x as f64 * 0.7).sin() * (z as f64 * 0.5).cos() + 1.0)
                * 5.0
                + (x as f64 * z as f64 * 0.1);
            data.push((x, z, v));
        }
    }
    Chart::new()
        .title("3D bar — sensor matrix")
        .series(Bar3DSeries::new("Sensors").data(data))
}

fn line_3d_sample() -> Chart {
    let pts: Vec<(f64, f64, f64)> = (0..200)
        .map(|i| {
            let t = i as f64 * 0.1;
            let x = (t * 1.1).cos() * (1.0 + t * 0.05);
            let z = (t * 1.1).sin() * (1.0 + t * 0.05);
            let y = t * 0.18;
            (x, y, z)
        })
        .collect();
    Chart::new()
        .title("3D line — helix")
        .series(Line3DSeries::new("helix").data(pts))
}

fn scatter_3d_sample() -> Chart {
    let pts: Vec<(f64, f64, f64)> = (0..120)
        .map(|i| {
            let t = i as f64 * 0.21;
            let x = t.sin() * 2.0 + (t * 1.7).cos();
            let y = (t * 0.7).sin() * 2.0;
            let z = t.cos() * 2.0;
            (x, y, z)
        })
        .collect();
    Chart::new()
        .title("3D scatter — Lissajous cloud")
        .series(Scatter3DSeries::new("samples").data(pts))
}

fn surface_3d_sample() -> Chart {
    let mut heights: Vec<Vec<f64>> = Vec::new();
    let n = 32;
    for zi in 0..n {
        let mut row = Vec::with_capacity(n);
        for xi in 0..n {
            let x = (xi as f64 / n as f64) * 6.0 - 3.0;
            let z = (zi as f64 / n as f64) * 6.0 - 3.0;
            let r = (x * x + z * z).sqrt();
            let v = (r * 1.5).sin() / (1.0 + r * 0.5);
            row.push(v);
        }
        heights.push(row);
    }
    Chart::new()
        .title("3D surface — sinc")
        .series(Surface3DSeries::new("z(x,z)").data(heights))
}

fn lines_3d_sample() -> Chart {
    #[allow(clippy::type_complexity)]
    let segments: Vec<((f64, f64, f64), (f64, f64, f64), f64)> = (0..50)
        .map(|i| {
            let t = i as f64 * 0.13;
            let a = (t.cos() * 2.0, (t * 0.7).sin() * 2.0, t.sin() * 2.0);
            let b = (
                (t + 0.3).cos() * 2.0,
                ((t + 0.3) * 0.7).sin() * 2.0,
                (t + 0.3).sin() * 2.0,
            );
            (a, b, (t * 0.5).sin() * 0.5 + 0.5)
        })
        .collect();
    Chart::new()
        .title("3D lines — knotted ribbon")
        .series(Lines3DSeries::new("knot").data(segments))
}

fn map_3d_sample() -> Chart {
    Chart::new()
        .title("Map 3D — extruded atlas")
        .series(Map3DSeries::new("Atlas"))
}

fn globe_sample() -> Chart {
    let cities = [
        ("NYC", -74.0, 40.7, 28.0),
        ("London", -0.1, 51.5, 22.0),
        ("Tokyo", 139.7, 35.7, 26.0),
        ("Sydney", 151.2, -33.9, 14.0),
        ("São Paulo", -46.6, -23.6, 18.0),
        ("Cairo", 31.2, 30.0, 12.0),
        ("Singapore", 103.8, 1.4, 16.0),
        ("Mumbai", 72.9, 19.1, 24.0),
        ("Moscow", 37.6, 55.8, 10.0),
    ];
    Chart::new()
        .title("Globe — sample cities")
        .series(GlobeSeries::new("cities").data(cities))
}

fn calendar_heatmap_sample() -> Chart {
    let data: Vec<(u32, f64)> = (1..=365)
        .map(|d| {
            let v = ((d as f64 * 0.3).sin().abs() * 30.0)
                + (((d as f64 * 0.1).cos() + 1.0) * 12.0)
                + ((d % 17) as f64);
            (d as u32, v)
        })
        .collect();

    Chart::new()
        .title("Calendar heatmap — daily commits")
        .series(CalendarHeatmapSeries::new("commits").year(2026).data(data))
}
