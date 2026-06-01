//! Sample dataset builders, one per `ChartKind`.

use egui_charts::{Axis, BarSeries, Chart, ChartKind, LineSeries, ScatterSeries, Series, SymbolKind};

/// Build a sample chart for `kind`. Returns `None` for kinds not yet
/// implemented in Phase 0.
pub fn build(kind: ChartKind) -> Option<Chart> {
    match kind {
        ChartKind::Line => Some(line_sample()),
        ChartKind::Bar => Some(bar_sample()),
        ChartKind::Scatter => Some(scatter_sample()),
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
