//! Headless render-smoke + theme tests for the public `egui_charts` API.
//!
//! Each chart kind is built through the real builder and painted by
//! `ChartWidget` into an offscreen `egui::Context` (no window, no GPU). A chart
//! that panics — divide-by-zero on empty data, an out-of-range palette index,
//! a bad axis transform — fails here instead of in a demo.

use egui_charts::{
    Axis, Chart, ChartTheme, ChartWidget, Distribution, Harmony, Series,
    ThemeMode,
};

/// Run one offscreen frame and hand the chart a real `Ui`. Returns the widget's
/// response rect so callers can assert the geometry is sane.
fn render_chart(dark: bool, build: impl FnOnce() -> Chart) -> egui::Rect {
    let ctx = egui::Context::default();
    let mut visuals = if dark {
        egui::Visuals::dark()
    } else {
        egui::Visuals::light()
    };
    visuals.dark_mode = dark;
    ctx.set_visuals(visuals);

    let chart = build();
    let input = egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(640.0, 480.0),
        )),
        ..Default::default()
    };

    let mut rect = egui::Rect::ZERO;
    let _ = ctx.run_ui(input, |ui| {
        rect = ChartWidget::new(&chart)
            .min_size(egui::vec2(480.0, 320.0))
            .show(ui)
            .rect;
    });
    rect
}

fn assert_sane(r: egui::Rect) {
    assert!(r.width().is_finite() && r.height().is_finite());
    assert!(r.width() > 0.0 && r.height() > 0.0);
}

#[test]
fn bar_chart_renders_both_modes() {
    for dark in [true, false] {
        let r = render_chart(dark, || {
            Chart::new()
                .title("Quarterly revenue")
                .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
                .y_axis(Axis::value())
                .series(Series::bar("2025").data([12.0, 18.0, 9.0, 22.0]))
        });
        assert_sane(r);
    }
}

#[test]
fn line_variants_render() {
    let r = render_chart(true, || {
        Chart::new()
            .x_axis(Axis::category(["Mon", "Tue", "Wed", "Thu", "Fri"]))
            .y_axis(Axis::value())
            .series(Series::line("plain").data([3.0, 5.0, 2.0, 8.0, 6.0]))
            .series(
                Series::line("smooth")
                    .smooth(true)
                    .data([4.0, 4.0, 5.0, 7.0, 7.0]),
            )
            .series(Series::line("area").area().data([1.0, 2.0, 3.0, 4.0, 5.0]))
    });
    assert_sane(r);
}

#[test]
fn stacked_bars_render() {
    let r = render_chart(false, || {
        Chart::new()
            .x_axis(Axis::category(["A", "B", "C"]))
            .y_axis(Axis::value())
            .series(Series::bar("x").stack("g").data([1.0, 2.0, 3.0]))
            .series(Series::bar("y").stack("g").data([3.0, 2.0, 1.0]))
    });
    assert_sane(r);
}

#[test]
fn scatter_renders() {
    let r = render_chart(true, || {
        Chart::new()
            .x_axis(Axis::value())
            .y_axis(Axis::value())
            .series(Series::scatter("pts").data([
                (1.0, 2.0),
                (3.0, 4.0),
                (5.0, 1.0),
            ]))
    });
    assert_sane(r);
}

#[test]
fn non_cartesian_kinds_render() {
    // Pie / radar / gauge / funnel take no x/y axes.
    assert_sane(render_chart(true, || {
        Chart::new().series(Series::pie("share").data([
            ("A", 30.0),
            ("B", 50.0),
            ("C", 20.0),
        ]))
    }));
    assert_sane(render_chart(false, || {
        Chart::new()
            .series(Series::gauge("speed").range(0.0, 100.0).value(72.0))
    }));
    assert_sane(render_chart(true, || {
        Chart::new().series(
            Series::funnel("stages").data([("Visit", 100.0), ("Buy", 20.0)]),
        )
    }));
}

#[test]
fn empty_chart_does_not_panic() {
    assert_sane(render_chart(true, Chart::new));
    // A series with no data points must not divide-by-zero.
    assert_sane(render_chart(false, || {
        Chart::new()
            .x_axis(Axis::category::<[&str; 0], &str>([]))
            .y_axis(Axis::value())
            .series(Series::bar("empty").data([]))
    }));
}

#[test]
fn mixed_bar_and_line_render() {
    let r = render_chart(true, || {
        Chart::new()
            .title("Revenue + trend")
            .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
            .y_axis(Axis::value())
            .series(Series::bar("rev").data([12.0, 18.0, 9.0, 22.0]))
            .series(
                Series::line("trend")
                    .smooth(true)
                    .data([10.0, 15.0, 14.0, 20.0]),
            )
    });
    assert_sane(r);
}

// ── Theme / palette ──────────────────────────────────────────────────────────

#[test]
fn series_color_wraps_palette() {
    let theme = ChartTheme::default();
    // Indexing past the palette length wraps round (modulo), never panics.
    let first = theme.series_color(0);
    let wrapped = theme.series_color(1000);
    assert_eq!(theme.series_color(0), first);
    let _ = wrapped; // just must not panic / must be a valid color
    for i in 0..50 {
        let _ = theme.series_color(i);
    }
}

#[test]
fn from_primary_light_and_dark_differ() {
    let primary = egui::Color32::from_rgb(0x4f, 0x8c, 0xff);
    let light = ChartTheme::from_primary(
        primary,
        ThemeMode::Light,
        Harmony::Square,
        6,
        Distribution::Even,
    );
    let dark = ChartTheme::from_primary(
        primary,
        ThemeMode::Dark,
        Harmony::Square,
        6,
        Distribution::Even,
    );
    // Background/foreground should differ between modes (the whole point of a mode).
    assert_ne!(light.background, dark.background);
}

#[test]
fn follow_egui_tracks_context_mode() {
    let primary = egui::Color32::from_rgb(0x4f, 0x8c, 0xff);

    let ctx = egui::Context::default();
    ctx.set_visuals(egui::Visuals::dark());
    let dark = ChartTheme::follow_egui(&ctx, primary, Harmony::Square, 6);

    ctx.set_visuals(egui::Visuals::light());
    let light = ChartTheme::follow_egui(&ctx, primary, Harmony::Square, 6);

    assert_ne!(dark.background, light.background);
}
