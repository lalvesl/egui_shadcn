# RFC 0001 — Boxed as the shared surface primitive

|             |                     |
| ----------- | ------------------- |
| **Status**  | IMPLEMENTED         |
| **Area**    | `egui_components`   |
| **Created** | 2026-08-03          |
| **Updated** | 2026-08-03          |

## Summary

Promote `Boxed` from a convenience wrapper into *the* box-surface primitive, and
route every component that draws a bordered, rounded, elevated container through
it — `Card`, `Popover`, `HoverCard`, `Dialog`, `Sheet`, `Drawer`, `Toast`, the
calendar box inside `DatePicker`, and the popup surfaces of `Select`, `Combobox`,
`DropdownMenu`, `ContextMenu` and `Menubar`.

## Motivation

Every one of those components had grown its own copy of "rounded rect + border +
padding". The consequences were the ones duplication always produces:

- A visual change had to be applied a dozen times, and inevitably was not. The
  primary-colored bottom accent introduced on `Boxed` did not exist anywhere
  else, so surfaces looked like they came from two different libraries.
- Padding drifted per component. `Drawer` in particular had padding that matched
  nothing else on screen.
- Corner radius and shadow were literal numbers at each call site rather than
  theme tokens, so they could not track the theme.

## Design

`Boxed` takes the full surface description and renders children inside it:

```rust
Boxed::new()
    .fill(theme.card)
    .corner_radius(theme.radius)
    .shadow(true)
    .accent(true)        // primary-colored bottom border
    .padding_px(16.0)
    .show(ui, |ui| {
        // children
    });
```

Named `Boxed`, not `Box`, because `Box` is in the Rust prelude and shadowing it
in a library that users glob-import (`use egui_components::*`) is hostile.

Components compose it rather than re-implementing it:

```rust
// Card is Boxed plus card semantics, not a second surface implementation.
Card::new().show(ui, |ui| { … });
```

## Reference-level detail

- `egui_components/src/components/boxed.rs` owns the painting: background fill,
  border stroke, corner radius, optional shadow, optional primary bottom accent,
  and uniform inner padding.
- The bottom border uses the theme's **primary** color (not `border`), which is
  the signature that visually unifies every surface in the library.
- Consumers pass only what differs from the default. `Toast` keeps its own
  variant-colored left stripe drawn *on top of* the `Boxed` surface, because that
  stripe encodes severity and is not a generic surface feature.

### Deliberate exceptions

Not every bordered thing is a box surface. These stay outside `Boxed`:

| Component      | Why                                                          |
| -------------- | ------------------------------------------------------------ |
| `Alert`        | Multi-color per variant; the border *is* the semantic signal. |
| `Input`, `Textarea` | Focus ring and inline validation states, not a surface. |
| Buttons        | Interactive fill/hover states dominate the visual.            |

## Drawbacks

- One more indirection between a component and its pixels. Debugging a padding
  problem now means reading `boxed.rs` first.
- `Boxed` accumulates knobs as consumers need them; without discipline it drifts
  toward being a generic style bag.

## Alternatives

- **A free function `paint_surface(ui, style)`.** Rejected: components need to
  lay children out *inside* the padded region, which means owning a child `Ui`,
  which is exactly what the closure form of `Boxed` provides.
- **Leave the duplication, fix visuals per component.** Rejected on the evidence
  — the accent border had already failed to propagate once.

## Implementation status

- [x] `Boxed` gains `fill`, `corner_radius`, `shadow`, `accent`, `padding_px`
- [x] Bottom border switched to the primary theme color
- [x] `Card`, `Popover`, `HoverCard`, `Dialog` (and `AlertDialog` transitively),
      `Sheet`, `Drawer`, `Toast` routed through `Boxed`
- [x] Popup surfaces of `Select`, `Combobox`, `DropdownMenu`, `ContextMenu`,
      `Menubar` and the `DatePicker` calendar box routed through `Boxed`
- [x] `Drawer` padding fixed as a consequence of the migration
- [x] `AlertDialog` rebuilt on `Dialog` instead of duplicating modal logic
