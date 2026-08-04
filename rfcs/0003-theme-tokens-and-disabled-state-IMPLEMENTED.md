# RFC 0003 — Theme tokens and a uniform disabled state

|             |                     |
| ----------- | ------------------- |
| **Status**  | IMPLEMENTED         |
| **Area**    | `egui_components`   |
| **Created** | 2026-08-03          |
| **Updated** | 2026-08-03          |

## Summary

Make the disabled appearance a single theme token applied uniformly through
`ui.multiply_opacity`, replacing the per-component alpha hacks; and make the
`theme.rs` accessors cheap enough to call every frame from every widget.

## Motivation

Two problems, same file.

**Disabled state was inconsistent.** Each component dimmed itself by tweaking its
own colors — `Checkbox` and `Button` disagreed on how faded "disabled" was, and
some components dimmed a color that was *already* dimmed, producing near-invisible
widgets. Shadcn expresses this as one rule (`opacity-50`); the library expressed
it as a dozen approximations.

**Token access was hot.** `ShadcnTheme::get(ctx)` and its color helpers run once
per widget per frame. Hue math and font descriptors were being recomputed at
runtime for values that never change.

## Design

One token, one helper, one application site per widget:

```rust
impl ShadcnTheme {
    /// Mirrors Shadcn `opacity-50`.
    pub const DISABLED_OPACITY: f32 = 0.5;

    /// Dim a single color. Prefer `ui.multiply_opacity` when dimming a whole widget.
    pub fn disabled(c: Color32) -> Color32 { c.gamma_multiply(Self::DISABLED_OPACITY) }
}
```

A disabled widget fades *as a whole* and restores the opacity afterwards, so the
dimming cannot leak into siblings:

```rust
let prev = ui.opacity();
if !enabled { ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY); }
// … paint the widget …
ui.set_opacity(prev);
```

## Reference-level detail

- `theme.rs` accessors are `#[inline]`; the pure helpers are `const fn` where the
  pinned toolchain allows it: `hsl`, `hue2rgb`, `text_style_body`, `body_font`,
  `small_font`, `heading_font`. Const float arithmetic and float→int casts were
  verified to compile on the pinned Rust version — this is toolchain-sensitive
  and worth re-checking on a bump.
- Uniform fade applied to `Button`, `Checkbox`, `Input`, `Textarea`, `Radio`,
  `Switch`, `Toggle`, and `ToggleGroup` (inherited through `Toggle`).
- The old ad-hoc per-color alpha adjustments were **removed**, not layered on
  top. Leaving them would double-dim; the checkbox bug was exactly that.

## Drawbacks

- `multiply_opacity` fades everything a widget paints, including any custom
  content a caller injects. That is usually right, but it is not opt-out.
- `const fn` on the theme helpers ties the crate to toolchain support for const
  float ops. A downgrade of the pinned toolchain could break the build.

## Alternatives

- **A `disabled` color per token (`primary_disabled`, …).** Rejected: it doubles
  the token surface and still cannot dim caller-supplied content.
- **Let each component keep its own rule.** Rejected — that was the bug.

## Implementation status

- [x] `ShadcnTheme::DISABLED_OPACITY` + `ShadcnTheme::disabled(color)`
- [x] All inputs fade via `multiply_opacity` with the previous value restored
- [x] Ad-hoc per-color alpha hacks removed (no double-dimming)
- [x] `Checkbox` disabled rendering fixed as a consequence
- [x] `theme.rs` marked `#[inline]`; pure helpers promoted to `const fn`
