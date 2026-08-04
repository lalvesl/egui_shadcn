# RFC 0011 — Intrinsic sizing for custom calendar cells

|             |                     |
| ----------- | ------------------- |
| **Status**  | **WIP**          |
| **Area**    | `egui_components`   |
| **Created** | 2026-08-03          |
| **Updated** | 2026-08-03          |

> Supersedes the former tasklist entry: *"The Calendar with custom values, need
> to increase size of each day, this create aspect of over text of each cell."*

## Summary

Make `Calendar` size its cells from the content they actually hold instead of
from caller-supplied constants: measure `cell_content` in an egui sizing pass,
grow the cell to fit, clip whatever still overflows, and stop silently dropping
the content when the cell gets small.

## Motivation

`cell_width` / `cell_height` landed as the first fix and they do work — the demo
prices calendar sets `52 × 54` and renders correctly. But they only work *if the
caller guesses right*, and three failure modes remain in the code as written:

1. **Silent disappearance.** The content region is computed as
   `rect.left()+1 → rect.max`, starting `12 px` below the day-number center, and
   the closure is skipped entirely when `inner.height() <= 4.0`. At the default
   `cell_h = 32.0` that region is about `9 px`; one careless default and custom
   content simply is not drawn, with no warning and nothing to debug.

2. **Responsive shrink eats the content.** `fit_to_width` scales `cell_h`
   proportionally with `cell_w` when seven columns will not fit. On a narrow
   window or on Android, a calendar that renders fine on desktop crosses the
   `4 px` threshold and drops its content — the layer most likely to be tested
   least.

3. **Horizontal spill is unguarded.** The child `Ui` is created with
   `max_rect(inner)` but is not clipped, so a closure that paints wider than the
   cell bleeds over its neighbours. The demo avoids this only by hand-tuning a
   `9 pt` font — which is the "over text" symptom the original task described,
   worked around rather than fixed.

The magic numbers `10.0` (number offset from the top) and `12.0` (content offset
below it) are also hardcoded rather than derived from the `13 pt` day-number
font, so they do not survive a font-size or theme change.

## Design

The builder keeps its explicit setters — they stay useful as a *floor* — and
gains automatic sizing:

```rust
Calendar::single("cal", &mut date)
    .cell_content(|ui, d| { … })   // measured; the cell grows to fit
    .show(ui);

Calendar::single("cal", &mut date)
    .cell_content(|ui, d| { … })
    .cell_height(52.0)             // explicit values act as a minimum, not the size
    .cell_width(54.0)
    .show(ui);
```

Overflow behaviour becomes explicit rather than emergent:

```rust
pub enum CellOverflow {
    /// Default. Clip to the cell — never bleed over neighbours.
    Clip,
    /// Grow every cell in the grid to the largest measured content.
    Grow,
}
```

## Reference-level detail

### Measuring

egui 0.35 provides `UiBuilder::sizing_pass()` (and `invisible()`), which is
exactly the mechanism for this: run `cell_fn` once into an invisible sizing-pass
`Ui` and read back `min_rect()`.

- Measure **once per frame**, not once per cell: run the closure for a single
  representative date and cache the result in `ctx.data_mut` keyed by the
  calendar `Id`. A 42-cell grid must not pay 42 extra layout passes, and the
  two-month range layout would double that.
- Caveat to verify during implementation: content whose size varies by date (a
  badge on some days only) will be under-measured by a single-date probe. The
  fallback is to measure the widest of *n* sampled dates, or to accumulate the
  max across the previous frame's real draws and apply it on the next — one
  frame of lag, no extra passes. **Prefer the accumulate-from-last-frame
  approach**; it is both cheaper and exact.

### Deriving the anchors

Replace the hardcoded `10.0` / `12.0` with values derived from the day-number
`FontId` via `ctx.fonts(|f| f.row_height(&font_id))`, so the layout tracks the
font instead of assuming `13 pt`.

### Clipping

Create the child `Ui` with the content rect **and** set its clip rect to that
same region, so `CellOverflow::Clip` is enforced by the painter rather than by
caller discipline.

### Responsive interaction

`fit_to_width` currently scales `cell_h` by the same ratio as `cell_w`. It should
instead scale the *day-number* region and preserve the measured content height,
with a hard floor below which the calendar switches to compact mode (already
implemented for ranges) rather than shrinking further. See
[RFC 0005](0005-calendar-and-date-picker-IMPLEMENTED.md) for the existing responsive path.

### Removing the silent skip

The `inner.height() > 4.0` guard goes away once the cell is sized from content.
If a caller forces a cell too small to hold anything, the content is clipped —
visibly wrong, which is far better than invisibly absent.

## Drawbacks

- A sizing pass costs a layout, even amortized to one per frame. The
  accumulate-from-last-frame variant avoids it entirely at the cost of one frame
  of lag on the first draw.
- Auto-growth means cell size depends on content, so a calendar can change
  dimensions as the month changes. Callers who need a stable layout must pin
  both `cell_width` and `cell_height`, and that has to be documented.
- Deriving offsets from font metrics makes the layout correct but harder to
  reason about from the source alone.

## Alternatives

- **Document the required sizes and stop there.** This is the status quo. It
  keeps failing the same way: the caller is asked to compute a number the library
  can measure.
- **Fixed generous default (`cell_h = 52`).** Rejected: it penalizes the common
  case — plain calendars, which are the majority — with oversized cells.
- **Require the caller to declare a content size (`.content_size(vec2)`).**
  Rejected: another number to guess, only more explicit about it.

## Unresolved questions

- Should `CellOverflow::Grow` size cells **uniformly** across the grid (tidy,
  what a calendar should look like) or per-row? Uniform is assumed here.
- Does the two-month range layout share one measurement or measure each month
  separately? Sharing keeps the two grids aligned and is the assumed answer.

## Implementation status

- [x] `cell_width` / `cell_height` builders
- [x] Day number pinned near the top when `cell_content` is present, with all
      highlight geometry anchored to the same point
- [x] Demo prices calendar sized explicitly (`52 × 54`)
- [ ] Measure `cell_content` and size cells from it
- [ ] Explicit `cell_width` / `cell_height` reinterpreted as minimums
- [ ] `CellOverflow` with `Clip` as the default; child `Ui` clip rect set
- [ ] Number / content offsets derived from font metrics instead of `10.0` / `12.0`
- [ ] `fit_to_width` preserves content height; falls back to compact instead of
      shrinking past the floor
- [ ] `inner.height() > 4.0` silent-skip guard removed
- [ ] Test: custom-cell calendar at a narrow available width still paints content
