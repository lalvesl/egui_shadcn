# RFC 0007 — Charts integration and theming

|             |                       |
| ----------- | --------------------- |
| **Status**  | IMPLEMENTED           |
| **Area**    | `egui_charts`, `demo` |
| **Created** | 2026-08-03            |
| **Updated** | 2026-08-03            |

## Summary

Replace the demo's local `Chart` component with `egui_charts`' `ChartWidget`, and
derive the chart theme from the active Shadcn `primary` so charts recolor when
the theme does.

## Motivation

The library shipped a minimal `Chart` component (bar + line, grid, legend,
auto-scale) *and* a separate `egui_charts` crate that did the same thing better.
Two chart implementations in one workspace means the demo showed the weaker one.

The theming was also wrong in a way that undercut the project's main selling
point: the demo lets you pick any primary hue and watch the whole UI follow, but
the charts stayed fixed. They read as pasted-in images rather than part of the
theme.

## Design

Charts take a theme built from the Shadcn primary:

```rust
let theme = ShadcnTheme::get(ui.ctx());
let chart_theme = ChartTheme::from_primary(theme.primary, mode, harmony, series, distribution);
```

`from_primary` derives a series palette from one hue using a harmony rule and a
distribution across the color wheel, so an N-series chart gets N distinguishable
colors that all belong to the chosen hue family. Text, grid and axis colors are
taken from the corresponding Shadcn tokens directly.

Charts sit inside the demo's `Card`s, so `ChartWidget` skips its own background
card and outer border when the supplied background is fully transparent — the
chart blends into the surface that already frames it instead of drawing a second
box inside the first.

## Reference-level detail

- `egui_charts/crates/egui_charts/src/theme/mod.rs` — `ChartTheme::from_primary`
  and its convenience wrappers.
- Demo usage: Bar (Desktop + Mobile) and Line (Desktop, smooth) in the charts
  section, passing a transparent background.
- The legacy `Chart` component's plot fill was made transparent for the same
  reason; it remains in the library for callers who want the small built-in, but
  the demo no longer showcases it.
- `serde` was dropped from `egui_charts` — chart configuration is built in code,
  and the dependency only inflated the wasm bundle.

## Drawbacks

- Two chart paths still exist (`components/chart.rs` and `egui_charts`). Until
  one is deprecated outright, contributors must be told which to touch.
- "Transparent background disables the card" is implicit behaviour keyed off an
  alpha value. It is convenient and it is also a hidden mode switch.

## Alternatives

- **Delete `components/chart.rs`.** Deferred, not rejected: it is a public API
  and removing it is a breaking change that deserves its own RFC.
- **Hardcode a chart palette.** Rejected: it breaks the "pick a hue, everything
  follows" property that the theme system exists for.

## Implementation status

- [x] Demo bar and line examples migrated to `ChartWidget`
- [x] `ChartTheme::from_primary` drives series colors from the Shadcn primary
- [x] Text / grid / axis colors tracked to Shadcn tokens
- [x] `ChartWidget` skips its background card and border when the background is
      fully transparent; the demo passes transparent
- [x] Legacy `Chart` plot fill made transparent
- [x] `serde` removed from `egui_charts`
