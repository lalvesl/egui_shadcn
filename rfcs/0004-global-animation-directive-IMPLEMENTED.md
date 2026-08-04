# RFC 0004 — Global animation directive

|             |                           |
| ----------- | ------------------------- |
| **Status**  | IMPLEMENTED               |
| **Area**    | `egui_components`, `demo` |
| **Created** | 2026-08-03                |
| **Updated** | 2026-08-03                |

## Summary

Add an `Animations { enabled, speed }` setting, stored in `egui::Memory` next to
the theme, that every animated component consults for its duration. One switch
disables all motion library-wide; one slider changes its velocity.

## Motivation

Animations were arriving component by component, each with a hardcoded duration.
That leaves the library with no answer to two ordinary requirements:

- **Accessibility / reduced motion.** A user who cannot tolerate motion has to be
  able to turn it off, and a per-component flag is not a usable answer.
- **Tests and screenshots.** Headless tests and the e2e screenshot pass want
  animations resolved instantly, not stepped frame by frame until they settle.

Separately, several components still *popped* instead of transitioning —
accordion bodies appeared instantly, dialogs blinked in, carousel slides swapped
with no travel — which is what made the library read as unpolished next to
Shadcn.

## Design

```rust
pub struct Animations {
    /// Master switch. When `false`, every animation resolves instantly.
    pub enabled: bool,
    /// Speed multiplier: 1.0 = default, 2.0 = twice as fast, 0.5 = half.
    pub speed: f32,
}

impl Animations {
    pub fn get(ctx: &Context) -> Self;
    pub fn set(ctx: &Context, anim: Animations);
    /// Scale a component's base duration — or zero it when disabled.
    pub fn duration(ctx: &Context, base: f32) -> f32;
}
```

A component never hardcodes a duration; it declares its *base* and lets the
directive scale it:

```rust
let t = ctx.animate_bool_with_time(id, open, Animations::duration(ctx, 0.15));
```

Because `duration` returns `0.0` when disabled, egui's animation helpers snap to
the target value and no component needs an `if animations_enabled` branch.

## Reference-level detail

Stored in `egui::Memory` under a dedicated `Id`, mirroring `ShadcnTheme` — see
[RFC 0003](0003-theme-tokens-and-disabled-state-IMPLEMENTED.md). Defaults to
`{ enabled: true, speed: 1.0 }`, so a host that never touches it gets the
designed behaviour.

Components migrated in the same pass:

| Component                 | Transition                                                                    |
| ------------------------- | ----------------------------------------------------------------------------- |
| `Accordion`, `Collapsible`| Body slides by clipping to an animated height; the natural height is measured each frame, so content of unknown size still animates. |
| `Dialog`                  | Eases in sliding up + fading; plays slide-down + fade-out on close. Stays rendered through the close transition and is non-interactive while closing. |
| `Carousel`                | Slides travel horizontally — incoming from the right on *next*, from the left on *prev* — with ease-out cubic. |
| `Switch`                  | Thumb travel respects the global speed.                                        |

The demo's theme-settings popover gained an **Animations** section (an Enabled
switch and a 0.25×–3× speed slider) that calls `Animations::set` each frame,
which is how the directive is exercised by hand.

## Drawbacks

- Global mutable state in `Memory`: a host with two contexts gets two settings,
  and there is no per-component override.
- The measure-then-clip approach used by `Accordion`/`Collapsible` lays content
  out even while collapsed. Cheap for a panel of widgets, wasteful if someone
  puts an expensive subtree inside.
- Closing dialogs stay in the tree during their exit transition. Callers that
  assume "closed means gone" on the very next frame need the non-interactive
  window.

## Alternatives

- **Per-component `.animate(bool)` builders.** Rejected: a reduced-motion user
  would have to set it on every call site, and would miss one.
- **Read the OS `prefers-reduced-motion`.** Not available uniformly across
  native/wasm/Android through egui; the explicit setting is what a host would
  wire such a signal *into*.

## Implementation status

- [x] `Animations` struct in `egui::Memory` with `get` / `set` / `duration`
- [x] `Accordion` / `Collapsible` height-clip slide
- [x] `Dialog` enter and exit transitions, non-interactive while closing
- [x] `Carousel` directional slide with ease-out cubic
- [x] `Switch` thumb travel
- [x] Demo control: Enabled switch + Speed slider in the theme popover
