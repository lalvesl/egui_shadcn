# RFC 0005 — Calendar and date picker behaviour

|             |                     |
| ----------- | ------------------- |
| **Status**  | IMPLEMENTED         |
| **Area**    | `egui_components`   |
| **Created** | 2026-08-03          |
| **Updated** | 2026-08-03          |

## Summary

Bring `Calendar` up to Shadcn behaviour: a live range preview while picking the
second endpoint, a compact single-month range mode, custom per-day cell content,
and correct alignment of the navigation arrows and day numbers. `DatePicker`
stays a thin popover wrapper around it.

## Motivation

The calendar is the most stateful component in the library and it accumulated the
most defects:

- Selecting a range gave no feedback between the first and second click — the
  user could not see what they were about to select.
- The two-month range layout was the only option, and it does not fit narrow
  containers or mobile.
- The next-month arrow sat near the middle of the header instead of at the right
  edge, while the previous-month arrow was correctly at the left.
- With custom cell content, the day number stopped being centered because the
  interior layout pushed it off-axis — selection and today-highlight circles then
  looked wrong.

## Design

```rust
// Single date
Calendar::single("my_cal", &mut selected).show(ui);

// Range — two months side by side
Calendar::range("rng", &mut start, &mut end).show(ui);

// Range in one month, both arrows on the same header
Calendar::range("rng", &mut start, &mut end).compact().show(ui);

// Custom content per day
Calendar::single("cal", &mut date)
    .cell_height(52.0)
    .cell_width(54.0)
    .cell_content(|ui, date| { /* price, badge, dot … */ })
    .show(ui);
```

### Range hover preview

While the range has a start but no end, hovering a day paints the prospective
`start → hovered` range: in-range cells shaded, the hovered endpoint circled in
the primary color. It is driven by the *previous* frame's hovered day (egui
resolves hover after layout), and is inert in single-date mode.

## Reference-level detail

- State (`view` month, `hover` day) lives in `ctx.data_mut` keyed by the
  component `Id`, so several calendars coexist without sharing state.
- `fit_to_width` clamps `cell_w` / `cell_h` and collapses a two-month range into
  the compact layout when `ui.available_width()` cannot hold it. This is what
  keeps the grid on screen in small windows and on Android, independent of
  whether the caller opted into `.compact()`.
- Cell interactions are namespaced under the calendar `Id` so two calendars on
  one screen do not steal each other's clicks.
- The custom-cell centering fix restores the day number to the cell center, with
  the selection / today circle drawn against that same center — the number and
  its highlight can no longer disagree.
- `DatePicker` composes `Popover` + `Calendar`; its calendar box is a `Boxed`
  surface, per [RFC 0001](0001-boxed-surface-primitive-IMPLEMENTED.md).

## Drawbacks

- `cell_content` is a `Fn(&mut Ui, CalDate)` invoked for every visible day —
  up to 42 calls per month, doubled in the two-month layout. Expensive closures
  are paid for on every frame.
- Cell dimensions are still caller-supplied numbers; a caller who forgets to
  enlarge them gets overflowing content. Addressed by
  [RFC 0011](0011-intrinsic-calendar-cell-sizing-WIP.md).

## Alternatives

- **Commit the range on hover.** Rejected: hovering is not intent; only the
  preview is speculative, the commit stays on click.
- **Two independent single calendars for ranges.** Rejected: the shared view
  state (keeping the two months adjacent, and the cross-month in-range shading)
  is the whole feature.

## Implementation status

- [x] Range hover preview with shaded in-range cells and a primary endpoint
- [x] `.compact()` single-month range mode with both nav arrows
- [x] Next-month arrow aligned to the right edge of the header
- [x] `cell_content` custom cells restored, with `cell_width` / `cell_height`
- [x] Day-number centering fixed for custom cells (selection + today highlight)
- [x] Range navigation buttons fixed — the two-month layout advances again
- [x] Width-driven responsive fallback (`fit_to_width`)
