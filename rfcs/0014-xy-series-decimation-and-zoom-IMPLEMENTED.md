# RFC 0014 — XY series, decimation and interactive zoom

|             |                                                   |
| ----------- | ------------------------------------------------- |
| **Status**  | IMPLEMENTED                                       |
| **Area**    | `egui_charts`, `egui_charts_gallery`, `demo`      |
| **Created** | 2026-09-26                                        |
| **Updated** | 2026-09-27                                        |

## Summary

Add a line series over explicit `(x, y)` pairs (`Series::xy_line`) that plots
on value and log axes on both x and y. It holds its points in a shared `Arc`
buffer, draws only the visible x-range, and decimates to min/max per pixel
column. Cartesian charts get opt-in wheel zoom, drag pan and double-click
reset. The zoom/pan view is persisted in egui memory and can be read and
driven by the caller through `ChartView`. Log-axis ticks become readable at
any zoom level, and axis titles get their own gutter line.

## Motivation

Lab-instrument GUIs (the first consumer is a steam-plant characterisation rig)
need two chart shapes that `egui_charts` could not draw:

- **Acquisition time series**: several channels at 1 kHz, 1e5–1e6 samples
  each, rebuilt every frame while streaming. `LineSeries` only takes `y` over a
  category axis, copies its data into a `Vec`, and draws one vertex per
  sample.
- **Bode diagrams**: magnitude (dB) and phase (deg) over a log frequency axis.
  The log axis existed, but no series could put numeric `x` on it. Its ticks
  were decade-only, so a zoom inside one decade had no labels at all, and
  labels read `1e-2`.

Both need zoom/pan, a "follow latest N seconds" mode where the application
sets the x range every frame, a "reset view" button, and fixed per-channel
colours so a channel looks the same in every chart.

Done means: 3 × 1e6 points render at interactive frame rates in release,
decimation provably keeps extrema, NaN leaves gaps, log axes ignore
non-positive values, and existing charts keep their API and behaviour.

## Design

```rust
use egui_charts::{Axis, Chart, ChartView, ChartWidget, Series, XyData};

// One XyData per data snapshot; cloning it into each frame's Chart is an Arc
// bump. `assume_sorted` skips the O(n) sortedness scan for timestamps.
let ch1 = XyData::from(samples).assume_sorted();

let chart = Chart::new()
    .x_axis(Axis::value().name("Time (s)"))
    .y_axis(Axis::value().name("Pressure (bar)"))
    .series(Series::xy_line("P1").data(ch1.clone()).color(theme.primary))
    .series(Series::xy_line("P2").data(ch2.clone()).dashed());

// Follow mode: the caller pins x; y still auto-fits the visible window.
ChartView::update(ui.ctx(), "scope", |v| v.x = Some((now - 10.0, now)));
ChartWidget::new(&chart).id("scope").interactive(true).show(ui);
if reset { ChartView::reset(ui.ctx(), "scope"); }
```

New public items:

- `XyLineSeries` (`Series::xy_line`, `Series::XyLine`) with `data(impl
  Into<XyData>)`, `data_arc`, `points`, `columns`, `color`, `width`, `dash`,
  `dashed`, `dotted`, `markers`, `marker_size`.
- `XyData`: a shared buffer built from `Arc<[[f64; 2]]>`, `Arc<Vec<[f64; 2]>>`,
  `Vec<[f64; 2]>`, pairs or columns. It lazily caches `XyStats` (sortedness and
  extents), shared by every clone.
- `DashStyle { Solid, Dashed, Dotted }`.
- `ChartView { x, y, drawn_x, drawn_y }` with `load`, `store`, `update`,
  `reset`, `with_x`, `with_y`, `is_auto`.
- `ChartWidget::interactive(bool)`, default `false`.
- `Series::color_override()`. The legend and renderers use it ahead of the
  palette.
- `ChartKind::XyLine` and `ChartKind::Bode` for the gallery catalogue.

Interaction (only with `interactive(true)`):

| Input               | Effect                              |
| ------------------- | ----------------------------------- |
| wheel               | zoom both axes around the cursor    |
| Shift + wheel       | zoom x only                         |
| Ctrl / Cmd + wheel  | zoom y only                         |
| pinch               | zoom both axes                      |
| primary drag        | pan                                 |
| double-click        | reset (drop both overrides)         |

Per axis, the range is resolved as: view override, then `Axis::min`/`max`,
then auto-fit. A stored view is honoured even on a non-interactive chart, so
follow mode does not require mouse interaction.

## Reference-level detail

- `option.rs` — `XyData`, `XyStats`, `XyLineSeries`, `DashStyle`, the
  `Series::XyLine` variant, and the `ChartKind` entries.
- `series/xy_line.rs` — the renderer:
  - `visible_range` binary-searches sorted `x` and adds one neighbour on each
    side, so the line reaches the plot edge.
  - `decimate_minmax` keeps first/min/max/last per pixel column (physical
    pixels, `pixels_per_point` aware). It finds a column's end with a
    gallop-plus-bisect, so the inner loop compares `y` only. NaN `y`, and `x`
    outside a log domain, emit one gap index per run.
  - Decimation kicks in when the visible points exceed 2 × columns. Unsorted
    data falls back to drawing every point.
  - Markers are skipped while decimating or above 4000 visible points.
  - Screen coordinates are clamped to ±1e5 px around the plot, so deep zooms
    stay finite in `f32`. The series is clipped to the plot rect.
- `coord/cartesian.rs`:
  - `CartesianCoord::layout_view(rect, theme, view_x, view_y)`; `layout()`
    delegates with no overrides.
  - `CartesianScales` and `AxisMap`: a precomputed affine data→pixel map in
    log10 space for log axes, attached to `CoordLayout` via `cartesian()`.
    This avoids one boxed-closure call per sample.
  - XY auto-fit: `x` from the cached stats, or O(log n) for sorted data;
    positive-only on log axes. `y` is fitted to the points inside the resolved
    x range, so a pinned or zoomed x auto-fits y to what is visible.
  - An empty or degenerate log extent pads by a decade.
  - With no `x_axis` given, a chart that has XY series defaults to
    `Axis::value()` instead of a category axis.
- Ticks:
  - Linear ticks are filtered to the visible range. Explicit and zoomed ranges
    are not "nice", and the old code drew ticks outside the plot.
  - `format_tick` takes its decimals from the step, so zoomed labels
    (`1000.125`, `1000.250`) stay distinct.
  - Long x labels trigger one re-spacing pass.
  - `ticks_log_nice` uses decades (thinned to fit), then 1-2-5 or 1..9
    multiples, then linear nice numbers inside one mantissa step.
  - `format_log_tick` prints `0.001 … 1 10 100 1k 10k 100k`, and exponent form
    outside that span.
- Axis titles: a named x axis adds a bottom gutter line (the title no longer
  overlaps the tick labels). A named y axis adds a top gutter line (the title
  is no longer clipped). The left gutter grows with the widest y tick label.
- `view.rs` holds `ChartView` and the `zoom_range`/`pan_range` math, in log10
  space for log axes. `interaction/zoom.rs` reads raw `Event::MouseWheel` and
  `Event::Zoom`, so egui's own Shift→horizontal and Ctrl→zoom conversions do
  not interfere. It zeroes `smooth_scroll_delta` when it consumes the wheel, so
  an enclosing `ScrollArea` does not scroll too.
- Widget: the view lives under `id.with("egui_charts::view")`. After input
  that changes the view, the chart is laid out again in the same frame (no
  one-frame lag). A non-auto view clips every cartesian series to the plot
  rect.
- Hover on XY series: nearest point by x per series (binary search when
  sorted), a ring on each hit, a vertical crosshair, and a tooltip header
  `"<x axis name>: <x>"` with one `y` row per series.
- The tooltip now prints values below 0.01 in exponent form instead of `0.00`.

Measured (release, 760×440 widget, 3 × 1e6 points, full frame incl.
tessellation, `tests/xy_line.rs::perf_three_channels_of_a_million_points`):
≈11 ms full view with cached stats, ≈1 ms with a 10 s follow window, ≈27 ms
full view when the buffer is new every frame (the O(n) y-extent scan
dominates).

## Drawbacks

- `Series`, `ChartKind` are exhaustive public enums; adding variants breaks a
  downstream exhaustive `match` (none exists in this workspace).
- Auto-fitting y over a *full* view of a buffer that changes every frame is an
  O(n) scan. Pin the y range (`Axis::min/max` or `ChartView::y`) or pin x to a
  window to avoid it.
- The `XyData` stats cache only helps if the caller keeps the `XyData`.
  Re-wrapping a raw `Arc` every frame rescans it.
- Dashed/dotted strokes restart their pattern at every gap and look busy on a
  decimated trace.
- `ChartState` (legend toggles) and `ChartView` are stored under different
  keys. That is deliberate (it keeps `ChartState`'s public fields unchanged),
  but it means two memory entries per chart.

## Alternatives

- **Extend `LineSeries` with an optional x vector.** Rejected: its stacking,
  smoothing and category-index tooltip semantics do not carry over to numeric
  x, and its `data: Vec<f64>` field is public, so the `Arc` storage would have
  been a breaking change.
- **LTTB decimation.** Rejected: it picks visually representative points but
  can drop a single-sample spike. Min/max per column keeps every extremum,
  which matters more for fault-finding on instrument data.
- **Store the view inside `ChartState`.** Rejected: `ChartState` has public
  fields and derives `Default`, so adding one breaks struct literals
  downstream.
- **`egui_plot`.** Rejected: a second plotting stack with its own look,
  outside the `ChartTheme`/Shadcn token pipeline (see RFC 0007).

## Unresolved questions

- Box (rubber-band) zoom and axis-only drag on the gutters.
- Secondary (right-hand) y axis.
- A ring-buffer `XyData` source for streaming without linearising a snapshot.

## Implementation status

- [x] `XyLineSeries` / `XyData` / `DashStyle`, legend + colour override
- [x] NaN gaps, log-domain handling, auto-fit (explicit bounds win)
- [x] Visible-range culling + min/max-per-column decimation, unit-tested
- [x] Opt-in zoom/pan/reset, `ChartView` load/store/update/reset
- [x] Readable log ticks, step-aware linear labels, axis-title gutters
- [x] XY hover: crosshair, nearest-by-x per series, x header in tooltip
- [x] Tests (`tests/xy_line.rs`, unit tests in `cartesian.rs`, `view.rs`,
      `series/xy_line.rs`), gallery kinds, demo cards, README row
