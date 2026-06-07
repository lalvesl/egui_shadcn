# Implementation Plan — `egui_charts`: an ECharts-parity charting library for egui

A Rust workspace that delivers a charting crate built on **egui / epaint**, aiming for **feature parity with Apache ECharts' chart catalog**, with first-class **dark-mode + primary-color theming** and **color-wheel (harmony) palette generation**, plus a separate **examples/gallery crate**.

> Crate names below (`egui_charts`, `egui_charts_gallery`) are placeholders. Note that "ECharts" is an Apache trademark — for a published crate prefer a distinct name (e.g. `eplot_charts`, `egcharts`) and describe it as "ECharts-inspired" rather than using the mark.

---

## 1. Goals & non-goals

**Goals**

- Idiomatic egui widget API: `ChartWidget::new(option).show(ui)` and a builder for assembling charts in Rust.
- Cover **every chart type ECharts ships** (full catalog in §3), grouped by the coordinate system each needs.
- A **theming layer** with light/dark token sets derived from a single **primary color**, including a **color-wheel harmony** palette generator (complementary, analogous, triadic, etc.) and evenly-distributed categorical palettes for N series.
- Optional **ECharts-JSON compatibility** (a subset): deserialize a familiar `option` object so existing configs can be reused.
- A **gallery app** (eframe) that renders every chart and exposes live theme controls (dark toggle, primary-color picker, harmony scheme, series count).

**Non-goals (initially)**

- Pixel-identical reproduction of ECharts styling.
- Full GL/3D parity in v1 (3D is a later, optional phase — see §3 and §8).
- Server-side rendering.

---

## 2. Workspace layout

Two crates as requested (core + examples), with theming as a well-isolated module inside the core crate so it can later be split into its own crate without API churn.

```
egui_charts/                      # workspace root
├── Cargo.toml                    # [workspace] members
├── crates/
│   ├── egui_charts/              # ← the charting crate
│   │   ├── Cargo.toml
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── option.rs         # ECharts-like data/option model (+ serde)
│   │       ├── widget.rs         # ChartWidget: egui::Widget impl + show()
│   │       ├── coord/            # coordinate systems
│   │       │   ├── mod.rs        # CoordKind, CoordLayout, data→screen transform
│   │       │   ├── cartesian.rs  # grid (rectangular)
│   │       │   ├── polar.rs      # angle/radius
│   │       │   ├── radar.rs
│   │       │   ├── single.rs     # single-axis (themeRiver)
│   │       │   ├── calendar.rs
│   │       │   ├── parallel.rs
│   │       │   └── geo.rs        # GeoJSON projection
│   │       ├── render/
│   │       │   ├── painter.rs    # ChartPainter wraps egui::Painter + transform
│   │       │   ├── shapes.rs     # sectors, ribbons, smooth paths, symbols
│   │       │   └── text.rs       # labels, ellipsis, anchored placement
│   │       ├── series/           # one module per chart type (§3)
│   │       │   ├── line.rs  bar.rs  scatter.rs  pie.rs  radar.rs ...
│   │       ├── interaction/
│   │       │   ├── tooltip.rs    hover/emphasis state, hit testing
│   │       │   ├── legend.rs
│   │       │   └── datazoom.rs   pan/zoom/brush
│   │       └── theme/            # ← theming + color (splittable later)
│   │           ├── mod.rs        # ChartTheme, ThemeMode, tokens
│   │           ├── color.rs      # Color32 ⇄ Oklch helpers
│   │           ├── harmony.rs    # color-wheel schemes
│   │           └── palette.rs    # categorical / sequential / diverging
│   └── egui_charts_gallery/      # ← the examples crate (eframe app)
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs           # eframe entry
│           ├── registry.rs       # ChartKind → sample builder
│           ├── samples/          # one sample dataset per chart type
│           └── controls.rs       # theme/primary/harmony UI panel
```

**Optional third crate later:** `egui_charts_theme` (split `theme/` out) if the palette logic proves reusable outside charting. Keep its public surface (`ChartTheme`, `Harmony`, palette fns) stable from day one to make the split painless.

---

## 3. Complete chart catalog (ECharts parity target)

Every ECharts `series.type` (plus components that behave like charts), grouped by the **coordinate system** the implementation needs. **Everything renders on `epaint` directly — there is no `egui_plot` dependency** (see §4.4). This grouping drives the engine design and the roadmap (§8).

### 3.1 Cartesian / grid coordinate (rectangular)

| Chart                    | ECharts `type`  | Variants                       | Render path                    |
| ------------------------ | --------------- | ------------------------------ | ------------------------------ |
| Line                     | `line`          | area, stacked, step, smooth    | custom (epaint paths/area)     |
| Bar                      | `bar`           | stacked, horizontal, polar bar | custom (epaint rects)          |
| Scatter / bubble         | `scatter`       | size-encoded (bubble)          | custom (epaint symbols)        |
| Effect scatter           | `effectScatter` | ripple animation               | custom (animated symbols)      |
| Candlestick / K-line     | `candlestick`   | —                              | custom (OHLC rects + wicks)    |
| Box plot                 | `boxplot`       | —                              | custom (whisker + box)         |
| Heatmap (grid)           | `heatmap`       | —                              | custom (cell grid + visualMap) |
| Pictorial bar            | `pictorialBar`  | repeat / clip symbols          | custom                         |
| ThemeRiver (streamgraph) | `themeRiver`    | —                              | custom on **single-axis**      |
| Lines (edges/paths)      | `lines`         | on cartesian or geo            | custom (poly/bezier)           |
| Custom render            | `custom`        | user `render_item` callback    | custom (callback)              |

### 3.2 Polar / angular

| Chart               | `type`       | Variants                       | Coord |
| ------------------- | ------------ | ------------------------------ | ----- |
| Pie / doughnut      | `pie`        | rose / nightingale             | polar |
| Radar               | `radar`      | —                              | radar |
| Gauge               | `gauge`      | progress, grade, multi-pointer | polar |
| Bar / line on polar | `bar`/`line` | —                              | polar |

### 3.3 Hierarchical

| Chart    | `type`     | Coord             |
| -------- | ---------- | ----------------- |
| Tree     | `tree`     | none (laid out)   |
| Treemap  | `treemap`  | none (squarified) |
| Sunburst | `sunburst` | polar (radial)    |

### 3.4 Relational / flow

| Chart             | `type`                 | Notes                           |
| ----------------- | ---------------------- | ------------------------------- |
| Graph (node-link) | `graph`                | force / circular / fixed layout |
| Sankey            | `sankey`               | layered ribbons                 |
| Chord             | `graph` (chord layout) | circular relationships          |

### 3.5 Statistical / specialized

| Chart                                      | `type`                          | Coord                     |
| ------------------------------------------ | ------------------------------- | ------------------------- |
| Parallel coordinates                       | `parallel`                      | parallel                  |
| Funnel                                     | `funnel`                        | none (stacked trapezoids) |
| Calendar heatmap                           | `heatmap`/`scatter` on calendar | calendar                  |
| (Boxplot, candlestick, heatmap — see §3.1) |                                 |                           |

### 3.6 Geographic

| Chart                          | `type`                    | Notes                                |
| ------------------------------ | ------------------------- | ------------------------------------ |
| Map (choropleth)               | `map`                     | GeoJSON polygons + visualMap         |
| Lines on geo                   | `lines`                   | migration / flight paths             |
| Scatter / effectScatter on geo | `scatter`/`effectScatter` | points on a projection               |
| Basemap overlays               | `bmap`/`gmap`             | external tiles (extension, optional) |

### 3.7 3D / WebGL (ECharts GL) — **optional later phase**

egui is a 2D library, so these require a **wgpu paint callback** (`egui_wgpu`) or the **`three-d`** crate embedded via a paint callback.
`bar3D`, `line3D`, `scatter3D`, `surface` (3D surface), `map3D`, `lines3D`, `graphGL` (large force graph), `flowGL` (vector field), `scatterGL` (massive scatter), `globe` (3D earth component).

### 3.8 Community extensions (parity for completeness)

`wordCloud` (echarts-wordcloud) and `liquidFill` (echarts-liquidfill) — implement as native series so the catalog is complete.

A single `ChartKind` enum will enumerate all of the above; the gallery's registry and the docs are generated from it so nothing silently falls out of sync.

---

## 4. Core architecture

### 4.1 Two-layer API

1. **Option / data model** (`option.rs`) — an ECharts-shaped, strongly-typed, `serde`-(de)serializable description: `title`, `legend`, `tooltip`, `grid`, `xAxis`/`yAxis`, `polar`/`radiusAxis`/`angleAxis`, `radar`, `visualMap`, `series: Vec<Series>`. A Rust builder wraps it ergonomically. Optional JSON ingestion lets users paste a (supported subset of) ECharts config.
2. **Widget** (`widget.rs`) — `ChartWidget` implements `egui::Widget` and offers `show(&self, ui) -> Response`. It resolves the active `ChartTheme`, computes the `CoordLayout` from the available rect, dispatches each series to its renderer, then runs interaction (legend, tooltip, datazoom).

```rust
// Builder usage
let chart = Chart::new()
    .title("Quarterly revenue")
    .x_axis(Axis::category(["Q1", "Q2", "Q3", "Q4"]))
    .y_axis(Axis::value())
    .series(Series::bar("2025").data([12.0, 18.0, 9.0, 22.0]))
    .series(Series::line("trend").smooth(true).data([10.0, 15.0, 14.0, 20.0]));

egui::CentralPanel::default().show(ctx, |ui| {
    ChartWidget::new(&chart).show(ui);
});
```

### 4.2 Coordinate systems

ECharts' power comes from separating _coordinate systems_ from _series_. Mirror that. A `CoordKind` selects the layout engine; a `CoordLayout` exposes `data_to_screen(point) -> Pos2` (and inverse for hit-testing).

```rust
pub enum CoordKind { Cartesian2D, Polar, Radar, Single, Calendar, Parallel, Geo, None, ThreeD }

pub trait Coordinate {
    fn layout(&self, rect: Rect, option: &Option_, theme: &ChartTheme) -> CoordLayout;
}
pub struct CoordLayout {
    pub plot_rect: Rect,
    pub to_screen: Box<dyn Fn(DataPoint) -> Pos2>,
    pub to_data:   Box<dyn Fn(Pos2) -> DataPoint>,
    pub axes:      Vec<AxisLayout>,   // ticks, labels, gridlines
}
```

### 4.3 Series trait

Each chart type implements one trait. Rendering goes through `ChartPainter`, never raw screen math, so theming and coordinate transforms stay centralized.

```rust
pub trait ChartType {
    fn coord_kind(&self) -> CoordKind;
    fn render(&self, p: &mut ChartPainter, layout: &CoordLayout, theme: &ChartTheme, idx: SeriesIndex);
    fn legend_entries(&self) -> Vec<LegendItem>;
    fn hit_test(&self, pos: Pos2, layout: &CoordLayout) -> Option<TooltipDatum>;
    fn animate(&mut self, t: f32) {}   // optional (ripples, grow-in)
}
```

### 4.4 Render backend — single custom engine on `epaint`

**No `egui_plot`.** The library is built from zero on **`epaint`** (egui's 2D paint layer: paths, convex polygons, bezier, text, images), so every chart — cartesian and non-cartesian alike — shares one coordinate, render, and interaction engine with a consistent look. This is more upfront work but avoids two mismatched coordinate/interaction models and a later migration.

Because nothing is inherited from `egui_plot`, the cartesian engine must implement the pieces it would otherwise have provided for free. Build these once in `coord/cartesian.rs` + `render/`, then every cartesian chart reuses them:

- **Data→screen transform** and its inverse (for hit-testing), with linear/log/category/time scales.
- **Axis layout & tick generation** — a "nice numbers" algorithm (e.g. Wilkinson / 1-2-5 step selection) picking human-friendly ticks for the visible range; category and time-axis tick strategies.
- **Axis & gridline rendering** — axis lines, major/minor ticks, tick labels (with rotation + ellipsis), split lines and split areas, all theme-styled.
- **Pan / zoom / box-zoom** plumbed through egui's `Response` (drag to pan, scroll/pinch to zoom), feeding back into the transform.

`render/shapes.rs` provides the primitives most ECharts charts reduce to:

- **Sectors / annular sectors** (pie, doughnut, rose, gauge, sunburst) — fan of triangles between two radii/angles, rounded-corner option.
- **Smooth paths** (smooth line, area boundaries) — Catmull-Rom → cubic bezier.
- **Ribbons** (sankey, chord) — two bezier edges filled.
- **Symbols** (scatter/effectScatter/legend) — circle, rect, triangle, diamond, pin, arrow, custom SVG-path.
- **Concave polygon fill** (geo, treemap with holes) — triangulate via `lyon` or `earcutr` before feeding epaint (which only fills convex shapes natively).

### 4.5 Interaction

- **Tooltip:** per-frame hit test of the hovered series item; floating panel drawn in a top layer; theme-styled surface.
- **Legend:** toggles series visibility; entries pull color from the theme palette by series index.
- **DataZoom / brush:** pan & zoom for cartesian; box-zoom; range slider component.
- **Emphasis/blur states:** mirror ECharts' hover highlighting (raise hovered, dim siblings).
- **Animations:** a frame clock drives grow-in transitions and `effectScatter` ripples; request repaint while animating.

---

## 5. Theming & color system (dark mode + primary color + color wheel)

This is the headline feature. One **primary color** + a **harmony scheme** + a **mode** (light/dark) produces a complete, balanced token set and an N-color categorical palette distributed around the color wheel.

### 5.1 Why OKLCH, not HSL

Hue rotation in HSL produces uneven perceived brightness (yellow looks far lighter than blue at the same `L`). Generating palettes in **OKLCH** (the cylindrical form of OKLab) keeps lightness and chroma perceptually constant as hue rotates, so categorical colors look equally weighted and sequential ramps look smooth. Use the **`palette`** crate for the conversions rather than hand-rolling matrices.

```rust
// theme/color.rs — Color32 ⇄ Oklch
use palette::{Oklch, Srgb, IntoColor, FromColor};
use egui::Color32;

pub fn to_oklch(c: Color32) -> Oklch {
    let srgb = Srgb::new(c.r() as f32 / 255.0, c.g() as f32 / 255.0, c.b() as f32 / 255.0);
    Oklch::from_color(srgb.into_linear())
}
pub fn from_oklch(c: Oklch) -> Color32 {
    let srgb: Srgb = Srgb::from_linear(c.into_color());
    Color32::from_rgb((srgb.red * 255.0) as u8, (srgb.green * 255.0) as u8, (srgb.blue * 255.0) as u8)
}
```

### 5.2 Color-wheel harmony schemes

Hue offsets (in degrees) applied to the primary's hue. These are the classic color-wheel relationships.

```rust
// theme/harmony.rs
#[derive(Clone, Copy)]
pub enum Harmony {
    Monochromatic, Complementary, Analogous, Triadic,
    SplitComplementary, Tetradic, Square,
}

impl Harmony {
    pub fn hue_offsets(self) -> &'static [f32] {
        match self {
            Harmony::Monochromatic      => &[0.0],
            Harmony::Complementary      => &[0.0, 180.0],
            Harmony::Analogous          => &[-30.0, 0.0, 30.0],
            Harmony::Triadic            => &[0.0, 120.0, 240.0],
            Harmony::SplitComplementary => &[0.0, 150.0, 210.0],
            Harmony::Tetradic           => &[0.0, 60.0, 180.0, 240.0], // rectangle
            Harmony::Square             => &[0.0, 90.0, 180.0, 270.0],
        }
    }
}

/// Anchor colors for a scheme (constant L & C, rotated hue).
pub fn harmony_palette(primary: Color32, scheme: Harmony) -> Vec<Color32> {
    let base = to_oklch(primary);
    scheme.hue_offsets().iter()
        .map(|d| from_oklch(Oklch::new(base.l, base.chroma, base.hue.into_degrees() + d)))
        .collect()
}
```

### 5.3 Categorical palette for N series (even color-wheel distribution)

The harmony schemes give 1–4 anchor hues; charts often need more. Distribute `n` series evenly around the wheel from the primary hue, holding lightness/chroma constant (tuned per mode so colors read on the active background).

```rust
// theme/palette.rs
pub fn categorical(primary: Color32, n: usize, dark: bool) -> Vec<Color32> {
    let base = to_oklch(primary);
    let l = if dark { (base.l + 0.12).min(0.85) } else { base.l.clamp(0.45, 0.70) };
    let c = if dark { base.chroma * 0.90 } else { base.chroma };
    (0..n).map(|i| {
        let hue = base.hue.into_degrees() + 360.0 * (i as f32) / (n as f32);
        from_oklch(Oklch::new(l, c, hue))
    }).collect()
}
```

For very large `n`, swap even spacing for the **golden angle (~137.5°)** to maximize separation between consecutive series. Expose both via a `Distribution { Even, GoldenAngle }` option.

### 5.4 Sequential & diverging scales (heatmap / visualMap / choropleth)

Interpolate in OKLab for smooth, banding-free ramps.

```rust
/// Light tint of primary → saturated primary (sequential).
pub fn sequential(primary: Color32, steps: usize) -> Vec<Color32> {
    let base = to_oklch(primary);
    (0..steps).map(|i| {
        let t = i as f32 / (steps.max(2) - 1) as f32;
        from_oklch(Oklch::new(0.95 - 0.55 * t, base.chroma * (0.25 + 0.75 * t), base.hue))
    }).collect()
}
/// Primary ↔ neutral ↔ complement (diverging).
pub fn diverging(primary: Color32, steps: usize) -> Vec<Color32> { /* lerp two ramps through gray */ }
```

### 5.5 Theme tokens (light + dark)

A `ChartTheme` carries everything the renderer needs; building it from a primary color produces both the chrome (background, grid, axis, text, tooltip surface) and the data palettes.

```rust
// theme/mod.rs
#[derive(Clone, Copy, PartialEq)] pub enum ThemeMode { Light, Dark }

pub struct ChartTheme {
    pub mode: ThemeMode,
    pub primary: Color32,
    pub background: Color32,
    pub surface:    Color32,   // tooltip / card
    pub axis_line:  Color32,
    pub grid_line:  Color32,
    pub text:       Color32,
    pub text_dim:   Color32,
    pub palette:    Vec<Color32>,   // categorical series colors
    pub sequential: Vec<Color32>,   // visualMap / heatmap
}

impl ChartTheme {
    pub fn from_primary(primary: Color32, mode: ThemeMode, harmony: Harmony, series: usize) -> Self {
        let dark = mode == ThemeMode::Dark;
        // Anchor by harmony first, then extend evenly to `series` colors.
        let mut palette = harmony_palette(primary, harmony);
        if palette.len() < series { palette = categorical(primary, series, dark); }
        Self {
            mode, primary,
            background: if dark { Color32::from_rgb(0x14,0x16,0x1a) } else { Color32::from_rgb(0xff,0xff,0xff) },
            surface:    if dark { Color32::from_rgb(0x22,0x25,0x2b) } else { Color32::from_rgb(0xf6,0xf7,0xf9) },
            axis_line:  if dark { Color32::from_gray(0x55) }          else { Color32::from_gray(0x99) },
            grid_line:  if dark { Color32::from_gray(0x33) }          else { Color32::from_gray(0xe2) },
            text:       if dark { Color32::from_gray(0xea) }          else { Color32::from_gray(0x20) },
            text_dim:   if dark { Color32::from_gray(0x9a) }          else { Color32::from_gray(0x70) },
            palette,
            sequential: sequential(primary, 9),
        }
    }

    /// Follow egui's own light/dark setting automatically.
    pub fn follow_egui(ctx: &egui::Context, primary: Color32, harmony: Harmony, series: usize) -> Self {
        let mode = if ctx.style().visuals.dark_mode { ThemeMode::Dark } else { ThemeMode::Light };
        Self::from_primary(primary, mode, harmony, series)
    }

    pub fn series_color(&self, i: usize) -> Color32 { self.palette[i % self.palette.len()] }
}
```

**Dark-mode behavior:** `follow_egui` ties chart mode to `ctx.style().visuals.dark_mode`, so toggling egui's theme reskins charts automatically; `from_primary` allows overriding independently. Per-series color always comes from `series_color(i)` — never hard-coded — which is what makes the whole catalog reskin from one primary color.

---

## 6. The examples crate (`egui_charts_gallery`)

An **eframe** app that doubles as living documentation and as the manual test surface for theming.

- **Layout:** left sidebar = collapsible tree of categories → chart types (mirrors §3 and ECharts' examples page); center = the selected chart at full size; right panel = **theme controls**.
- **Theme controls (the live demo of §5):**
  - Light/Dark toggle (drives `ctx.set_visuals` + `follow_egui`).
  - **Primary color picker** (egui's built-in `color_edit_button_srgba`).
  - **Harmony scheme** dropdown (`Harmony` variants).
  - **Series count** slider → recomputes `categorical` palette live.
  - A swatch strip rendering the current palette so the color-wheel distribution is visible.
- **"View option" toggle:** show the chart's `option` as pretty JSON (demonstrates serde round-trip / ECharts-config compatibility).
- **Registry:** `registry.rs` maps each `ChartKind` to a `fn() -> Chart` sample builder with a small dataset in `samples/`. Because both the gallery and docs iterate the same `ChartKind` enum, adding a chart automatically adds a gallery entry.
- **Targets:** runs natively and compiles to **WASM** (`trunk`) so the gallery can ship as a web demo like ECharts' own.

---

## 7. Dependencies & Cargo setup

Core crate:

- `egui`, `epaint` — UI + 2D painting (track latest; e.g. egui `0.32`). **The only rendering dependency** — no `egui_plot`.
- `palette` — OKLab/OKLCH color math.
- `serde`, `serde_json` _(feature `serde`)_ — option model (de)serialization.
- `lyon` **or** `earcutr` _(feature `geo`/`maps`)_ — concave polygon tessellation for geo/treemap.
- `geo`, `geojson` _(feature `maps`)_ — map ingestion & projection.
- `three-d` or raw `egui_wgpu` paint callbacks _(feature `gl3d`, optional)_ — 3D/GL charts.

Examples crate: `eframe`, the core crate, plus sample-data helpers.

Feature flags keep the dependency surface small by default: `default = ["serde"]`; `full = ["serde", "maps", "gl3d"]`.

---

## 8. Phased roadmap (every chart mapped to a milestone)

**Phase 0 — Foundations: engine + theming + first charts**

- Workspace, `ChartKind` enum, `option.rs` + builder, `ChartWidget` skeleton.
- **The `epaint` engine itself:** cartesian coordinate system (scales, data↔screen transform), nice-number tick generation, axis/gridline/label rendering, and pan/zoom plumbed through egui's `Response` (the work `egui_plot` would otherwise have done — see §4.4).
- `render/` primitives: paths, area fills, rects, symbols, text with rotation/ellipsis.
- Full **theme/color system** (§5) end-to-end — needed early so every later chart consumes it.
- Gallery shell with live theme controls.
- Charts: **line (incl. area/stacked/step/smooth), bar (incl. stacked/horizontal), scatter**. ✅ Shippable, fully themeable demo on a self-owned engine.

**Phase 1 — Polar family**

- Polar/radar coordinate systems; sector & annular-sector primitives; legend + tooltip.
- Charts: **pie / doughnut / rose, radar, gauge, funnel**.

**Phase 2 — Hierarchical & relational**

- Layout algorithms (squarified treemap, tidy-tree, force/circular graph, sankey layering).
- Charts: **tree, treemap, sunburst, graph, sankey, chord**.

**Phase 3 — Statistical & specialized**

- Charts: **boxplot, candlestick, heatmap, calendar heatmap, parallel, themeRiver, pictorialBar, effectScatter, custom**.

**Phase 4 — Geographic**

- GeoJSON loading + projection, polygon tessellation, visualMap binding.
- Charts: **map (choropleth), lines on geo, scatter/effectScatter on geo**.

**Phase 5 — Extensions & 3D (optional)**

- Native **wordCloud, liquidFill**.
- **GL/3D** via wgpu/`three-d`: **bar3D, line3D, scatter3D, surface, map3D, lines3D, graphGL, flowGL, scatterGL, globe**.

**Cross-cutting (continuous):** animations/emphasis states, dataZoom/brush, accessibility (focus, keyboard), docs generated from `ChartKind`.

---

## 9. Testing & quality

- **Unit tests** for color math: `Color32 → Oklch → Color32` round-trip tolerance; harmony offsets land on expected hues; palette length/uniqueness.
- **Snapshot rendering tests** with **`egui_kittest`** (egui's official test harness) — render each chart at a fixed size and compare against committed reference images so regressions are caught visually. Run light and dark variants.
- **Layout property tests** (e.g. treemap areas sum to the rect; sankey flows conserve mass).
- **WASM smoke build** in CI so the gallery's web target never breaks.
- **`cargo doc` + doctests** on the builder API; deny broken intra-doc links.

---

## 10. Risks & open questions

- **Scope.** ECharts is enormous; the phased plan front-loads the engine + theming so each later chart is incremental. 3D is explicitly optional.
- **Building cartesian from scratch.** Skipping `egui_plot` means the axis/tick/gridline + pan-zoom layer is on us (Phase 0). The highest-risk piece is robust **nice-number tick generation** and **log/time scales**; budget time there and lock it down with snapshot tests before building charts on top. The payoff is one consistent engine and zero migration debt.
- **Concave fills.** epaint fills convex shapes only — geo and some treemap/funnel cases need tessellation (`lyon`/`earcutr`); budget for it in Phase 4.
- **Text layout parity.** ECharts label placement (rotation, ellipsis, rich text) is intricate; `render/text.rs` should be built once and reused everywhere.
- **ECharts-JSON compatibility** is a _subset_ promise — document exactly which `option` fields are honored to avoid surprising users.
- **Naming/trademark** (see header) before any crates.io publish.
- **Open questions to confirm with you:** Is interactive pan/zoom/tooltip in-scope for v1 or can v1 be static (this matters more now that we own the interaction layer)? Is the GL/3D phase actually wanted, or is the 2D catalog sufficient? Should theming ship as its own crate from the start?
