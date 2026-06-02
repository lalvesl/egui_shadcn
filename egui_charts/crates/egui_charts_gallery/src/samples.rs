//! Sample dataset builders, one per `ChartKind`.

use egui_charts::{
    Axis, BarSeries, Chart, ChartKind, FunnelSeries, GaugeSeries, LineSeries, PieSeries,
    RadarDataset, RadarSeries, ScatterSeries, Series, SymbolKind,
};

/// Build a sample chart for `kind`. Returns `None` for kinds not yet
/// implemented.
pub fn build(kind: ChartKind) -> Option<Chart> {
    match kind {
        ChartKind::Line => Some(line_sample()),
        ChartKind::Bar => Some(bar_sample()),
        ChartKind::Scatter => Some(scatter_sample()),
        ChartKind::Pie => Some(pie_sample()),
        ChartKind::Doughnut => Some(doughnut_sample()),
        ChartKind::Rose => Some(rose_sample()),
        ChartKind::Radar => Some(radar_sample()),
        ChartKind::Gauge => Some(gauge_sample()),
        ChartKind::Funnel => Some(funnel_sample()),
        _ => None,
    }
}

fn line_sample() -> Chart {
    Chart::new()
        .title("Website traffic — last 12 weeks")
        .x_axis(Axis::category((1..=12).map(|i| format!("W{i}"))))
        .y_axis(Axis::value().name("Visitors (k)"))
        .series(
            LineSeries::new("Direct")
                .smooth(true)
                .area()
                .data([22.0, 25.0, 30.0, 28.0, 35.0, 38.0, 42.0, 45.0, 50.0, 48.0, 55.0, 62.0]),
        )
        .series(
            LineSeries::new("Search")
                .smooth(true)
                .data([18.0, 19.0, 22.0, 20.0, 24.0, 27.0, 26.0, 30.0, 35.0, 33.0, 40.0, 45.0]),
        )
        .series(
            LineSeries::new("Referral")
                .step()
                .data([8.0, 9.0, 11.0, 12.0, 12.0, 14.0, 16.0, 17.0, 17.0, 19.0, 21.0, 22.0]),
        )
}

fn bar_sample() -> Chart {
    Chart::new()
        .title("Quarterly revenue by region")
        .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
        .y_axis(Axis::value().name("Revenue ($M)"))
        .series(BarSeries::new("EMEA").stack("rev").data([12.0, 18.0, 9.0, 22.0]))
        .series(BarSeries::new("AMER").stack("rev").data([15.0, 14.0, 19.0, 17.0]))
        .series(BarSeries::new("APAC").stack("rev").data([7.0, 9.0, 13.0, 11.0]))
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
        .series(
            Series::Scatter(ScatterSeries::new("bubbles").bubbles(bubbles).symbol(SymbolKind::Diamond)),
        )
}

fn pie_sample() -> Chart {
    Chart::new()
        .title("Browser market share")
        .series(
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
    Chart::new()
        .title("Quarterly revenue split")
        .series(
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
    Chart::new()
        .title("Product fit by segment")
        .series(
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
    Chart::new()
        .title("Cluster CPU usage")
        .series(
            GaugeSeries::new("CPU")
                .range(0.0, 100.0)
                .value(67.4)
                .unit("%"),
        )
}

fn funnel_sample() -> Chart {
    Chart::new()
        .title("Acquisition funnel")
        .series(
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
