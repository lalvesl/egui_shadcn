# Theme, sizing, icons, fonts

## ShadcnTheme

Stored in `egui::Memory`, so any component can read it without plumbing:

```rust
let theme = ShadcnTheme::get(ui.ctx());
ui.painter().rect_filled(rect, cr, theme.card);
```

Construction and application:

```rust
ShadcnTheme::light()
ShadcnTheme::dark()
ShadcnTheme::build(dark: bool, primary_hue: Option<f32>)   // None → zinc default

ShadcnTheme::set(ctx, theme.clone());   // publish to Memory
theme.apply(ctx);                       // push into egui's own Style/Visuals
```

Both calls belong in `update()` every frame, not only at startup — otherwise a
runtime theme change only takes effect on the next cold start.

### Tokens

```
dark: bool                 primary_hue: Option<f32>

background      foreground
card            card_foreground
primary         primary_foreground
secondary       secondary_foreground
muted           muted_foreground
accent          accent_foreground
destructive     destructive_foreground
success         success_foreground
warning         warning_foreground
border          input            ring
radius: f32
```

Helpers:

```rust
ShadcnTheme::DISABLED_OPACITY       // 0.5 — mirrors Shadcn opacity-50
ShadcnTheme::disabled(color)        // dim one color
ShadcnTheme::with_alpha(color, a)
ShadcnTheme::icon_font(size)
```

To fade a whole widget, prefer the UI-level form so caller content fades too:

```rust
let prev = ui.opacity();
ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY);
// … paint …
ui.set_opacity(prev);
```

Everything derives from one hue — never hardcode a color, or the "pick a primary
and the whole UI follows" property breaks.

## Size

`Size { Sm, Default, Lg }`, used by every sized component except `Button`
(which has `ButtonSize { Sm, Default, Lg, Icon }`).

Resolvers: `.height()` `.font_size()` `.h_pad()` `.v_pad()` `.icon_size()`
`.diameter()` `.box_size()` `.switch_track()` `.spinner()` `.slider_track()`.

`Size::Default.height() == 36.0`.

## Spacing

`Spacing { Xs, Sm, Md, Lg, Xl, Xl2, Xl3 }`.

```rust
Spacing::Md.show(ui);            // emit the gap — use this, never ui.add_space
let p: f32 = Spacing::Lg.into(); // or Spacing::Lg.px()
```

## Animations

```rust
Animations { enabled: bool, speed: f32 }   // speed 1.0 = default

Animations::get(ctx) / Animations::set(ctx, a)
Animations::duration(ctx, base_secs) -> f32   // scaled, or 0.0 when disabled
```

Always route a duration through `Animations::duration`:

```rust
let t = ctx.animate_bool_with_time(id, open, Animations::duration(ctx, 0.15));
```

Because it returns `0.0` when disabled, no component needs an `if enabled` branch.

## Icons

Constants are **generated at build time** into `OUT_DIR` from the upstream
MaterialIcons codepoints and pulled in by `include!`. They are `&'static str`
glyphs, screaming snake case (`icons::SEARCH`, `icons::CHECK`,
`icons::CHEVRON_RIGHT`). Grep the generated `icon_consts.rs` in `target/` to
confirm a name before using it — do not guess.

```rust
Icon::new(icons::SEARCH).size(18.0).color(theme.foreground).show(ui) -> Response
Icon::new(icons::CLOSE).clickable().show(ui)
Icon::new(g).paint(ui, pos, egui::Align2::CENTER_CENTER, color);   // manual painting
icon_font_id(size) -> FontId
```

Components that accept an icon take the glyph directly: `.icon(icons::CHECK)`.

## Fonts

```rust
register_font(ctx);                 // = ctx.set_fonts(font_definitions())
font_definitions() -> FontDefinitions   // extend this to add your own fonts
register_font_bytes(ctx, bytes);        // wasm: after fetching the font
register_custom_font(ctx);              // only with EGUI_SHADCN_CUSTOM_FONT_URL
```

**This is mandatory.** Without it the `MaterialIcons` family is missing and the
first icon paint panics. On wasm and on builds without the font, a stub family
aliases the proportional font so egui does not panic; the real font is fetched
at runtime.
