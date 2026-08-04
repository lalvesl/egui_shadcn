# RFC 0002 — Sizing and spacing primitives

|             |                     |
| ----------- | ------------------- |
| **Status**  | IMPLEMENTED         |
| **Area**    | `egui_components`   |
| **Created** | 2026-08-03          |
| **Updated** | 2026-08-03          |

## Summary

Introduce two shared scales — a `Size` enum for widget dimensions and a `Spacing`
enum for gaps — and apply them across the library so that magic numbers stop
appearing at call sites. Add a `Separator` component covering both orientations.

## Motivation

`Button` had a size scale. Nothing else did. Every other component picked heights
by hand, so a `Button`, a `Select` and an `Input` placed on one row did not line
up. Vertical rhythm had the same problem: `ui.add_space(8.0)` appeared throughout
the library and demo with 8 chosen by feel, per site.

Two symptoms made this concrete:

- A form row of mixed widgets was visibly ragged.
- Changing the base rhythm meant grepping for float literals — unreviewable.

## Design

```rust
pub enum Size { Sm, #[default] Default, Lg }

pub enum Spacing { Xs, Sm, Md, Lg, Xl, Xl2, Xl3 }
```

`Size` resolves to concrete dimensions (`Size::Default.height() == 36.0`) and is
accepted by every sized component:

```rust
Badge::new("new").size(Size::Sm).show(ui);
Select::new(…).size(Size::Lg).show(ui);
```

`Spacing` renders itself and converts to numbers, so it works both as a widget
and as a value:

```rust
Spacing::Md.show(ui);          // emits the gap
let pad: f32 = Spacing::Lg.into();  // or Spacing::Lg.px()
```

`Separator` covers the divider case that spacing alone cannot:

```rust
Separator::horizontal().show(ui);
Separator::vertical().show(ui);
```

## Reference-level detail

- `components/size.rs` — `Size`, with `height()` and the derived paddings and
  font sizes each component needs.
- `components/spacing.rs` — `Spacing`, with `px()`, `show()`, and `From<Spacing>`
  for `f32` and the other numeric types so it drops into arithmetic unchanged.
- `Size` was threaded through `Badge`, `Select`, `Slider`, `Spinner`,
  `RadioGroup`, `Checkbox`, `Avatar`, `Switch`, `Toggle`, `ToggleGroup`,
  `ButtonGroup`, `Combobox` and `DropdownMenu`, matching the pre-existing
  `Button` scale.
- Every `add_space` call in the library and the demo was replaced by `Spacing`.
  This is the part that keeps the scale honest: as long as no `add_space`
  survives, no site can invent its own rhythm.

## Drawbacks

- Three sizes is a coarse scale. A component that genuinely needs a fourth has to
  either extend the enum for everyone or take an escape-hatch float.
- `From<Spacing> for f32` makes it easy to leak spacing into arithmetic where a
  layout constraint would have been the better tool.

## Alternatives

- **Free constants (`pub const SPACE_MD: f32`).** Rejected: constants cannot
  render themselves, so call sites still write `ui.add_space(SPACE_MD)` and the
  `add_space` grep-gate above stops working.
- **Per-component size types.** Rejected: it is the *shared* scale that makes
  mixed rows align.

## Implementation status

- [x] `Size` enum with default variant, applied to 13 components
- [x] `Spacing` enum with `show`, `px`, and numeric conversions
- [x] All `add_space` call sites migrated to `Spacing`
- [x] `Separator` component (horizontal + vertical)
- [x] Audit pass: components rebuilt on existing primitives (`Boxed`, `Spacing`,
      `Separator`, `Card`, `Typography`) instead of hand-rolled equivalents —
      see [RFC 0001](0001-boxed-surface-primitive-IMPLEMENTED.md)
