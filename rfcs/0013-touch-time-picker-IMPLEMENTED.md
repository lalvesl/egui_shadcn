# RFC 0013 — Touch-first time picker

|             |                                              |
| ----------- | -------------------------------------------- |
| **Status**  | IMPLEMENTED                                  |
| **Area**    | `egui_components`, demo                      |
| **Created** | 2026-08-08                                   |
| **Updated** | 2026-08-08                                   |

## Summary

Adds `TimePicker` — two drag-scrollable wheels (hour `0..=23`, minute
`00..=59`) with flick inertia, snap-to-row and a centered selection band — plus
the plain `CalTime` value type it edits. Usable inline or behind a trigger field
that opens a `Drawer` on narrow viewports and a `Popover` on wide ones.

## Motivation

The library had no way to enter a time. The two obvious workarounds are both
bad on a phone: two `Select` dropdowns means a 60-entry scroll list with 20 px
rows, and a `Slider` gives no readable digits and no minute precision. Both are
also the same widget everywhere, which is exactly the problem — the pointer
target that works with a mouse is too small for a thumb.

Done means: an app can bind a `CalTime` and get 44 px touch rows, continuous
finger tracking, and a wheel that wraps, without writing any of that itself.

## Design

```rust
// Trigger field → Drawer on narrow viewports, Popover on wide ones.
TimePicker::new("id", &mut time).width(240.0).show(ui) -> bool

// Just the wheels, boxed, laid out in place.
TimePicker::new("id", &mut time)
    .minute_step(5)   // 1..=30, default 1
    .rows(3)          // visible rows per wheel, default 5
    .size(Size::Lg)   // default: Lg when narrow, Default otherwise
    .title("…")       // drawer heading on the mobile path
    .inline(ui) -> bool

CalTime::new(9, 30) / CalTime::from_minutes(570) / .total_minutes() / "09:30"
```

Both entry points return `bool` — `true` on the frames the value changed —
matching `DatePicker` and `Select`.

## Reference-level detail

New file `egui_components/src/components/time_picker.rs`; registered in
`components/mod.rs`; strings in `egui_components/src/i18n.rs`
(`TimePicker::{Title, Done}`). Demo section 49 in `demo/src/app/sections/forms.rs`,
which shifted Tooltip/Typography to 50/51 and `SECTION_COUNT` to 52.

- **Wheel state** is one `WheelState { offset, velocity, last_idx }` per column
  in `ctx.data`, keyed `Id::new("shadcn_time_picker").with(id).with("hour"|"minute")`.
  `offset` is the *fractional* item index on the center line, normalized to
  `0..len`, so rows track the finger continuously instead of jumping.
- **Wrapping** falls out of `rem_euclid` on both the offset and the rendered
  row index — 00 sits one row above 23 / 59, in both directions.
- **External writes** are detected by comparing the incoming index against
  `last_idx` (the index this wheel itself last produced) and re-centering, so an
  app-set value moves the wheel but the wheel's own output does not fight it.
- **Motion** goes through `Animations::duration(ctx, 0.20)`: flick decays by
  `exp(-dt * FRICTION)` and snap approaches its target exponentially (so it is
  framerate-independent), and both collapse to an instant jump when the duration
  is `0.0`. `dt` is clamped to `1/240..1/15` so a stalled frame cannot fling the
  wheel across the list.
- **Columns are capped at 110 px** and the pair is centered in the available
  width, while the panel itself claims the full width — otherwise the wheels
  hug the left edge of anything wider than they need, which is what a drawer
  always is.
- **The selection band** is painted into a slot reserved with `Shape::Noop`
  before the row is laid out and filled via `Painter::set` afterwards — the same
  trick `Toggle::show_with` uses — because the band's rect is only known once
  both columns have been allocated.
- **Minutes with `minute_step`** are shown at step granularity but stored as
  real minutes, so an app-set `07` with `minute_step(5)` lands on the `05` row
  rather than being rejected.
- **The mobile trigger opens on the next frame.** `Drawer` dismisses on any
  click outside its rect, and the click that opens it is by definition outside
  it, so the drawer is shown only when the flag was already set at the start of
  the frame. `Popover` already reads its open flag before the trigger, so the
  desktop path needed nothing.

## Drawbacks

- Two more wheels' worth of per-frame repaint while a flick decays. Bounded by
  the friction cutoff, but it is not free.
- The wheels emit no AccessKit nodes and take no keys — the same gap the rest of
  the library has (roadmap item 1), and worse here, since a wheel has no
  keyboard affordance at all.
- `wheel_column` measures its own layout rather than delegating to `Boxed`. The
  band and the digits are painted directly, as `Calendar` does for day cells.

## Alternatives

- **Two `Select` dropdowns.** Rejected: 24 and 60 entries of small rows is the
  problem being solved, not a solution to it.
- **A Material-style tap grid** (hours grid, then minutes grid). Easier to test
  and no drag physics, but two steps to set one value, and it cannot express
  minute precision without a third step.
- **Steppers (`+` / `-`)**. Compact and trivial, but 30 taps to cross a half
  hour.
- **AM/PM as a third column.** Explicitly declined — the values stay `0-23` and
  `00-59`.
- **Mouse-wheel scrolling on the columns.** Dropped: the demo lives inside a
  `ScrollArea` and the event cannot be consumed cleanly, so it would scroll the
  page and the wheel at once. Dragging works with a mouse anyway.

## Unresolved questions

- Seconds are not modelled. `CalTime` would need a third field and the panel a
  third column; nothing needs it yet.
- No min/max clamping (e.g. business hours only). The wheels are cyclic by
  design, and a range would have to break that.

## Implementation status

- [x] `CalTime` + `TimePicker` (`inline` / `show`) in `egui_components`
- [x] i18n strings for the drawer heading and confirm button
- [x] Demo section (trigger, inline, 5-minute steps)
- [x] Tests: geometry across sizes/rows/themes, tap-to-move, wrap, minute step
- [x] README row and `skills/egui-shadcn/references/components.md` entry
