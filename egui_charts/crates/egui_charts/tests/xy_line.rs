//! XY line series, decimation, axis fitting and interactive zoom/pan, driven
//! through a real headless `egui::Context` (RFC 0010 harness pattern).

use egui::{
    Event, Modifiers, MouseWheelUnit, PointerButton, Pos2, RawInput, Rect,
    TouchPhase, pos2, vec2,
};
use egui_charts::coord::cartesian::CartesianCoord;
use egui_charts::{
    Axis, Chart, ChartTheme, ChartView, ChartWidget, Series, SymbolKind, XyData,
};
use std::sync::Arc;

const SCREEN: Rect = Rect {
    min: Pos2::ZERO,
    max: Pos2::new(800.0, 500.0),
};

fn input(time: f64, events: Vec<Event>) -> RawInput {
    RawInput {
        screen_rect: Some(SCREEN),
        time: Some(time),
        events,
        ..Default::default()
    }
}

/// Run one frame; returns the chart response rect and a geometry measure:
/// total path points plus one per shape (a proxy for what was emitted).
fn frame(
    ctx: &egui::Context,
    raw: RawInput,
    chart: &Chart,
    id: &'static str,
    interactive: bool,
) -> (Rect, usize) {
    let mut rect = Rect::NOTHING;
    let mut output = ctx.run_ui(raw, |ui| {
        rect = ChartWidget::new(chart)
            .id(id)
            .interactive(interactive)
            .min_size(vec2(760.0, 440.0))
            .show(ui)
            .rect;
    });
    output.textures_delta.clear();
    let points = output
        .shapes
        .iter()
        .map(|c| match &c.shape {
            egui::Shape::Path(p) => p.points.len() + 1,
            _ => 1,
        })
        .sum();
    (rect, points)
}

fn theme() -> ChartTheme {
    ChartTheme::follow_egui(
        &egui::Context::default(),
        egui::Color32::from_rgb(0x4f, 0x8c, 0xff),
        egui_charts::Harmony::Square,
        6,
    )
}

fn resolved(chart: &Chart) -> ((f64, f64), (f64, f64)) {
    let layout = CartesianCoord::new(chart).layout(
        Rect::from_min_size(Pos2::ZERO, vec2(600.0, 400.0)),
        &theme(),
    );
    let sc = layout.cartesian().expect("cartesian layout");
    (sc.x.data_extent(), sc.y.data_extent())
}

fn sine(n: usize, dt: f64) -> Arc<[[f64; 2]]> {
    (0..n)
        .map(|i| {
            let t = i as f64 * dt;
            [t, (t * 7.0).sin()]
        })
        .collect::<Vec<_>>()
        .into()
}

// ── Rendering ──────────────────────────────────────────────────────────────

#[test]
fn xy_line_renders_on_every_axis_combination() {
    let ctx = egui::Context::default();
    let pts: Vec<(f64, f64)> =
        (1..200).map(|i| (i as f64, (i as f64).sqrt())).collect();
    for (xa, ya) in [
        (Axis::value(), Axis::value()),
        (Axis::log(), Axis::value()),
        (Axis::value(), Axis::log()),
        (Axis::log(), Axis::log()),
    ] {
        let chart = Chart::new()
            .x_axis(xa.name("Frequency (Hz)"))
            .y_axis(ya.name("Gain"))
            .series(Series::xy_line("a").points(pts.clone()))
            .series(
                Series::xy_line("b")
                    .points(pts.iter().map(|(x, y)| (*x, y * 2.0)))
                    .dashed()
                    .markers(SymbolKind::Diamond)
                    .color(egui::Color32::RED),
            )
            .series(
                Series::xy_line("c")
                    .points(pts.iter().map(|(x, y)| (*x, y * 3.0)))
                    .dotted(),
            );
        let (r, n) = frame(&ctx, input(0.0, vec![]), &chart, "combo", false);
        assert!(r.width() > 0.0 && n > 0);
    }
}

#[test]
fn degenerate_xy_data_does_not_panic() {
    let ctx = egui::Context::default();
    let cases: Vec<Vec<(f64, f64)>> = vec![
        vec![],
        vec![(1.0, 1.0)],
        vec![(1.0, f64::NAN), (2.0, f64::NAN)],
        vec![(3.0, 1.0), (1.0, 2.0), (2.0, 0.5)], // unsorted
        vec![(0.0, 0.0), (-1.0, -5.0), (f64::NAN, 1.0)],
        vec![(1.0, f64::INFINITY), (2.0, 1.0)],
    ];
    let mut t = 0.0;
    for (i, pts) in cases.into_iter().enumerate() {
        for log in [false, true] {
            t += 0.2;
            let ax = if log { Axis::log() } else { Axis::value() };
            let chart = Chart::new().x_axis(ax.clone()).y_axis(ax).series(
                Series::xy_line(format!("case {i}")).points(pts.clone()),
            );
            frame(&ctx, input(t, vec![]), &chart, "degenerate", false);
            // Hover across the plot too.
            frame(
                &ctx,
                input(t + 0.1, vec![Event::PointerMoved(pos2(400.0, 250.0))]),
                &chart,
                "degenerate",
                false,
            );
        }
    }
}

#[test]
fn hover_on_xy_series_renders_tooltip() {
    let ctx = egui::Context::default();
    let chart = Chart::new()
        .x_axis(Axis::value().name("Time (s)"))
        .series(Series::xy_line("ch1").data_arc(sine(5_000, 0.001)))
        .series(Series::xy_line("ch2").data_arc(sine(3_000, 0.0017)));
    frame(&ctx, input(0.0, vec![]), &chart, "hover", false);
    let (_, with_hover) = frame(
        &ctx,
        input(0.1, vec![Event::PointerMoved(pos2(400.0, 250.0))]),
        &chart,
        "hover",
        false,
    );
    let (_, without) = frame(
        &ctx,
        input(0.2, vec![Event::PointerGone]),
        &chart,
        "hover",
        false,
    );
    // Crosshair + highlight rings + tooltip add geometry.
    assert!(with_hover > without, "{with_hover} vs {without}");
}

#[test]
fn large_series_is_decimated_to_pixel_columns() {
    let ctx = egui::Context::default();
    let data = XyData::from_arc(sine(1_000_000, 0.001));
    let chart = Chart::new()
        .x_axis(Axis::value())
        .hide_legend()
        .series(Series::xy_line("big").data(data.clone()));
    let (r, points) = frame(&ctx, input(0.0, vec![]), &chart, "big", false);
    // ≤ 4 kept points per pixel column (+ axis/grid geometry).
    let budget = (r.width() as usize) * 4 + 2_000;
    assert!(
        points < budget,
        "{points} path points for a {}px chart",
        r.width()
    );
    assert!(data.stats().sorted);
}

#[test]
fn per_series_color_override_is_used() {
    let chart = Chart::new().series(
        Series::xy_line("fixed")
            .points([(0.0, 0.0), (1.0, 1.0)])
            .color(egui::Color32::from_rgb(1, 2, 3)),
    );
    assert_eq!(
        chart.series[0].color_override(),
        Some(egui::Color32::from_rgb(1, 2, 3))
    );
    assert_eq!(Series::from(Series::line("l")).color_override(), None);
}

// ── Auto-fit ───────────────────────────────────────────────────────────────

#[test]
fn autofit_includes_xy_and_ignores_nan() {
    let chart = Chart::new()
        .x_axis(Axis::value())
        .y_axis(Axis::value())
        .series(Series::xy_line("s").points([
            (10.0, 1.0),
            (20.0, f64::NAN),
            (30.0, 3.0),
            (40.0, 2.0),
        ]));
    let ((x0, x1), (y0, y1)) = resolved(&chart);
    assert!(x0 <= 10.0 && (40.0..100.0).contains(&x1), "x {x0}..{x1}");
    assert!(y0 <= 1.0 && (3.0..10.0).contains(&y1), "y {y0}..{y1}");
}

#[test]
fn explicit_axis_bounds_win_over_autofit() {
    let chart = Chart::new()
        .x_axis(Axis::value().min(15.0).max(35.0))
        .y_axis(Axis::value().min(-10.0).max(10.0))
        .series(Series::xy_line("s").points([(10.0, 1.0), (40.0, 2.0)]));
    let ((x0, x1), (y0, y1)) = resolved(&chart);
    assert_eq!((x0, x1), (15.0, 35.0));
    assert_eq!((y0, y1), (-10.0, 10.0));
}

#[test]
fn log_axes_skip_non_positive_values() {
    let chart = Chart::new().x_axis(Axis::log()).y_axis(Axis::log()).series(
        Series::xy_line("s").points([
            (0.0, 5.0),
            (-3.0, 5.0),
            (10.0, 0.0),
            (100.0, 0.5),
            (1000.0, 50.0),
        ]),
    );
    let ((x0, x1), (y0, y1)) = resolved(&chart);
    assert_eq!((x0, x1), (10.0, 1000.0));
    assert_eq!((y0, y1), (0.5, 50.0));
}

#[test]
fn y_autofit_follows_pinned_x_range() {
    // y = x: pinning x to [0, 10] must fit y to ~[0, 10], not [0, 1000].
    let chart = Chart::new()
        .x_axis(Axis::value().min(0.0).max(10.0))
        .series(
            Series::xy_line("ramp")
                .points((0..=1000).map(|i| (i as f64, i as f64))),
        );
    let (_, (_, y1)) = resolved(&chart);
    assert!(y1 <= 20.0, "y max {y1}");
}

#[test]
fn xy_default_x_axis_is_numeric() {
    let chart = Chart::new()
        .series(Series::xy_line("s").points([(100.0, 1.0), (200.0, 2.0)]));
    let ((x0, x1), _) = resolved(&chart);
    assert!(x0 <= 100.0 && x1 >= 200.0 && x0 > 0.0, "x {x0}..{x1}");
}

#[test]
fn named_axes_reserve_their_own_line() {
    let rect = Rect::from_min_size(Pos2::ZERO, vec2(600.0, 400.0));
    let plain = Chart::new()
        .series(Series::xy_line("s").points([(0.0, 0.0), (1.0, 1.0)]));
    let named = plain
        .clone()
        .x_axis(Axis::value().name("Time (s)"))
        .y_axis(Axis::value().name("Pressure (bar)"));
    let a = CartesianCoord::new(&plain).layout(rect, &theme()).plot_rect;
    let b = CartesianCoord::new(&named).layout(rect, &theme()).plot_rect;
    assert!(b.min.y >= a.min.y + 12.0, "top gutter for the y title");
    assert!(b.max.y <= a.max.y - 10.0, "bottom gutter for the x title");
}

// ── View + interaction ─────────────────────────────────────────────────────

fn scope_chart() -> Chart {
    Chart::new()
        .x_axis(Axis::value().name("Time (s)"))
        .y_axis(Axis::value())
        .series(Series::xy_line("ch").data_arc(sine(10_000, 0.001)))
}

fn span(r: (f64, f64)) -> f64 {
    r.1 - r.0
}

#[test]
fn stored_view_overrides_the_drawn_range() {
    let ctx = egui::Context::default();
    let chart = scope_chart();
    ChartView::default().with_x(2.0, 3.0).store(&ctx, "scope");
    frame(&ctx, input(0.0, vec![]), &chart, "scope", false);
    let v = ChartView::load(&ctx, "scope");
    assert_eq!(v.drawn_x, Some((2.0, 3.0)));
    ChartView::reset(&ctx, "scope");
    frame(&ctx, input(0.1, vec![]), &chart, "scope", false);
    let v = ChartView::load(&ctx, "scope");
    assert!(v.is_auto());
    assert!(span(v.drawn_x.unwrap()) >= 9.0);
}

fn wheel(pos: Pos2, dy: f32, modifiers: Modifiers) -> Vec<Event> {
    vec![
        Event::PointerMoved(pos),
        Event::MouseWheel {
            unit: MouseWheelUnit::Point,
            delta: vec2(0.0, dy),
            phase: TouchPhase::Move,
            modifiers,
        },
    ]
}

#[test]
fn wheel_zoom_is_opt_in() {
    let ctx = egui::Context::default();
    let chart = scope_chart();
    let (r, _) = frame(&ctx, input(0.0, vec![]), &chart, "passive", false);
    frame(
        &ctx,
        input(0.1, wheel(r.center(), 120.0, Modifiers::NONE)),
        &chart,
        "passive",
        false,
    );
    assert!(ChartView::load(&ctx, "passive").is_auto());
}

#[test]
fn wheel_zooms_around_cursor_and_modifiers_pick_axes() {
    let ctx = egui::Context::default();
    let chart = scope_chart();
    let (r, _) = frame(&ctx, input(0.0, vec![]), &chart, "zoom", true);
    let full = ChartView::load(&ctx, "zoom");
    let (fx, fy) = (full.drawn_x.unwrap(), full.drawn_y.unwrap());

    // Plain wheel: both axes shrink.
    frame(
        &ctx,
        input(0.1, wheel(r.center(), 120.0, Modifiers::NONE)),
        &chart,
        "zoom",
        true,
    );
    let v = ChartView::load(&ctx, "zoom");
    let (vx, vy) = (v.x.expect("x zoomed"), v.y.expect("y zoomed"));
    assert!(span(vx) < span(fx) && span(vy) < span(fy));

    // Shift: x only.
    ChartView::reset(&ctx, "zoom");
    frame(&ctx, input(0.2, vec![]), &chart, "zoom", true);
    frame(
        &ctx,
        input(0.3, wheel(r.center(), 120.0, Modifiers::SHIFT)),
        &chart,
        "zoom",
        true,
    );
    let v = ChartView::load(&ctx, "zoom");
    assert!(v.x.is_some() && v.y.is_none());

    // Ctrl: y only.
    ChartView::reset(&ctx, "zoom");
    frame(&ctx, input(0.4, vec![]), &chart, "zoom", true);
    frame(
        &ctx,
        input(0.5, wheel(r.center(), 120.0, Modifiers::CTRL)),
        &chart,
        "zoom",
        true,
    );
    let v = ChartView::load(&ctx, "zoom");
    assert!(v.x.is_none() && v.y.is_some());
}

#[test]
fn wheel_zoom_on_log_axis_stays_positive() {
    let ctx = egui::Context::default();
    let chart = Chart::new().x_axis(Axis::log()).series(
        Series::xy_line("bode").points((0..400).map(|i| {
            let f = 10f64.powf(i as f64 / 100.0);
            (f, -20.0 * (1.0 + f / 100.0).log10())
        })),
    );
    let (r, _) = frame(&ctx, input(0.0, vec![]), &chart, "log", true);
    for k in 0..5 {
        frame(
            &ctx,
            input(
                0.1 * (k + 1) as f64,
                wheel(r.center(), -200.0, Modifiers::SHIFT),
            ),
            &chart,
            "log",
            true,
        );
    }
    let (a, b) = ChartView::load(&ctx, "log").x.unwrap();
    assert!(a > 0.0 && b > a, "{a}..{b}");
    // Zoomed out (negative wheel) around the centre in log space.
    assert!(b / a > 1e4, "{a}..{b}");
}

#[test]
fn drag_pans_and_double_click_resets() {
    let ctx = egui::Context::default();
    let chart = scope_chart();
    let (r, _) = frame(&ctx, input(0.0, vec![]), &chart, "pan", true);
    let before = ChartView::load(&ctx, "pan").drawn_x.unwrap();
    let c = r.center();
    let press = |p: Pos2, pressed: bool| Event::PointerButton {
        pos: p,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    let mut t = 0.1;
    let mut step = |events: Vec<Event>| {
        frame(&ctx, input(t, events), &chart, "pan", true);
        t += 0.05;
    };
    step(vec![Event::PointerMoved(c), press(c, true)]);
    for k in 1..=5 {
        step(vec![Event::PointerMoved(c + vec2(-20.0 * k as f32, 0.0))]);
    }
    step(vec![press(c + vec2(-100.0, 0.0), false)]);
    let after = ChartView::load(&ctx, "pan").x.expect("panned");
    // Dragging left moves the window to later x.
    assert!(
        after.0 > before.0 && after.1 > before.1,
        "{before:?} → {after:?}"
    );
    assert!((span(after) - span(before)).abs() < 1e-6 * span(before).max(1.0));

    // Double-click resets.
    let p = c;
    step(vec![Event::PointerMoved(p), press(p, true)]);
    step(vec![press(p, false)]);
    step(vec![press(p, true)]);
    step(vec![press(p, false)]);
    step(vec![]);
    assert!(ChartView::load(&ctx, "pan").is_auto());
}

// ── Performance (manual) ───────────────────────────────────────────────────

/// 3 channels × 1e6 points, full frame incl. tessellation. Run with
/// `cargo test --release -p egui_charts --test xy_line -- --ignored --nocapture`.
#[test]
#[ignore = "timing probe; run in release"]
fn perf_three_channels_of_a_million_points() {
    let ctx = egui::Context::default();
    let chans: Vec<XyData> = (0..3)
        .map(|k| {
            XyData::from_arc(
                (0..1_000_000)
                    .map(|i| {
                        let t = i as f64 * 1e-3;
                        [
                            t,
                            (t * (1.0 + k as f64)).sin()
                                + 0.01 * ((i * 7919) % 13) as f64,
                        ]
                    })
                    .collect::<Vec<_>>()
                    .into(),
            )
        })
        .collect();
    let build = |fresh: bool| {
        let mut c = Chart::new().x_axis(Axis::value().name("Time (s)"));
        for (k, d) in chans.iter().enumerate() {
            // `fresh` builds a new buffer each frame, as for a streaming
            // buffer that changes every frame (the copy itself is timed out
            // of the frame below).
            let d = if fresh {
                XyData::from_arc(Arc::from(d.as_slice())).assume_sorted()
            } else {
                d.clone()
            };
            c = c.series(Series::xy_line(format!("ch{k}")).data(d));
        }
        c
    };
    for (label, fresh, view) in [
        ("cached, full view", false, None),
        ("cached, last 10 s", false, Some((990.0, 1000.0))),
        ("fresh, full view", true, None),
        ("fresh, last 10 s", true, Some((990.0, 1000.0))),
    ] {
        let chart = build(fresh);
        let mut t = 0.0;
        let mut total = std::time::Duration::ZERO;
        let n = 10;
        for i in 0..=n {
            if let Some((a, b)) = view {
                ChartView::default().with_x(a, b).store(&ctx, "perf");
            } else {
                ChartView::reset(&ctx, "perf");
            }
            let chart = if fresh { build(true) } else { chart.clone() };
            let start = std::time::Instant::now();
            let out = ctx.run_ui(
                input(t, vec![Event::PointerMoved(pos2(400.0, 250.0))]),
                |ui| {
                    ChartWidget::new(&chart)
                        .id("perf")
                        .interactive(true)
                        .min_size(vec2(760.0, 440.0))
                        .show(ui);
                },
            );
            let _mesh = ctx.tessellate(out.shapes, out.pixels_per_point);
            if i > 0 {
                total += start.elapsed();
            }
            t += 0.016;
        }
        println!("{label:>24}: {:?} / frame", total / n);
    }
}

#[test]
fn legend_toggle_still_works_on_an_interactive_chart() {
    let ctx = egui::Context::default();
    let chart = scope_chart();
    let (r, _) = frame(&ctx, input(0.0, vec![]), &chart, "legend", true);
    // First legend entry sits at the top-left of the content area.
    let p = r.min + vec2(28.0, 20.0);
    let click = |pressed| Event::PointerButton {
        pos: p,
        button: PointerButton::Primary,
        pressed,
        modifiers: Modifiers::NONE,
    };
    frame(
        &ctx,
        input(0.1, vec![Event::PointerMoved(p), click(true)]),
        &chart,
        "legend",
        true,
    );
    frame(
        &ctx,
        input(0.15, vec![click(false)]),
        &chart,
        "legend",
        true,
    );
    let state = ctx
        .data(|d| {
            d.get_temp::<egui_charts::widget::ChartState>(egui::Id::new(
                "legend",
            ))
        })
        .expect("chart state");
    assert!(!state.series[0].visible, "legend click hides the series");
    assert!(
        ChartView::load(&ctx, "legend").is_auto(),
        "no zoom from a legend click"
    );
}
