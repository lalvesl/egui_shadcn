---
name: egui-shadcn
description: Build application UI with the egui-shadcn component library (crates egui_sc, egui_components, egui_charts, i18n) — Shadcn/ui components for egui in Rust. Use when writing egui UI that uses Button, Card, Dialog, Input, Calendar, DataTable, Toast or any component from this library, when setting up the theme (ShadcnTheme) or fonts, when adding the dependency to a Rust project, or when translating UI strings with t!/traductions!. Triggers on "egui_sc", "egui_components", "egui-shadcn", "ShadcnTheme", "Boxed", "Spacing::", "ButtonVariant". For contributing to the library itself, use egui-shadcn-contributing instead.
---

# egui-shadcn — using the library

Shadcn/ui components for egui. Aesthetically faithful, not pixel-perfect.
This skill covers **consuming** the library in an application.

## Adding the dependency

Not published to crates.io — depend on it by git (or a local path):

```toml
[dependencies]
egui_sc = { git = "https://github.com/lalvesl/egui_shadcn" }
i18n    = { git = "https://github.com/lalvesl/egui_shadcn" }  # only if you call t!
egui    = "0.35"
eframe  = "0.35"
```

**Your `egui` version must match the library's (0.35).** A mismatch produces
type errors on `&mut Ui` that look unrelated to versions.

Language features (default `lang-en-us` + `lang-pt-br`):

```toml
egui_sc = { git = "…", default-features = false, features = ["lang-en"] }
```

Two build-time facts worth knowing:

- `build.rs` **downloads MaterialIcons on first build**. Offline builds set a
  `has_material_icons` cfg off and fall back to a stub family instead of failing.
- On Linux you need the usual GUI libs (`libxkbcommon`, `wayland`, …). The
  repo's `flake.nix` provides them if you use Nix.

## Imports

```rust
use egui_sc::egui_components::*;   // components + theme + icons + fonts
use egui_sc::egui_charts;          // charts
use egui_sc::i18n;                 // i18n runtime API only
```

If your crate invokes `t!` or `traductions!`, it must **also depend on `i18n`
directly** — those macros expand to absolute `::i18n::…` paths and the
re-export does not satisfy them.

## Bootstrap (required, in this order)

```rust
impl MyApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        register_font(&cc.egui_ctx);                  // MUST come first
        let theme = ShadcnTheme::build(true, None);   // (dark, primary_hue)
        ShadcnTheme::set(&cc.egui_ctx, theme.clone());
        theme.apply(&cc.egui_ctx);
        Self { theme }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _f: &mut eframe::Frame) {
        ShadcnTheme::set(ctx, self.theme.clone());   // re-set every frame
        self.theme.apply(ctx);

        egui::CentralPanel::default().show(ctx, |ui| { /* … */ });

        Toaster::show(ctx);                          // once per frame, last
    }
}
```

Skipping `register_font` (or `ctx.set_fonts(font_definitions())`) **panics** the
moment any component paints an icon — the `MaterialIcons` family will not exist.

Re-setting the theme every frame is what makes a runtime theme switch take
effect; doing it only in `new` means changes appear after a restart.

## The universal pattern

Every component is a builder ending in `.show()`. Bind state with `&mut`; read
the outcome from the return value.

```rust
let resp = Button::new("Save").variant(ButtonVariant::Destructive).show(ui);
if resp.clicked() { … }
```

Return values are **not uniform** — check before assuming:

| Returns | Components |
|---|---|
| `Response` | `Button`, `Checkbox`, `Switch`, `Radio`, `Toggle`, `Slider`, `Input`, `Textarea`, `InputOtp`, `Icon` |
| `bool` (changed / confirmed) | `Select`, `Combobox`, `DatePicker`, `AlertDialog` |
| `Option<usize>` (clicked index) | `Breadcrumb`, `ButtonGroup`, `DropdownMenu`, `NavigationMenu`, `Pagination`, `ContextMenu` |
| `Option<(usize, usize)>` | `Menubar`, `Command` (group, item) |
| `Option<(usize, SortDir)>` | `DataTable` |
| `usize` (current index) | `Carousel` |
| `()` | `Alert`, `Avatar`, `Badge`, `Calendar`, `Card`, `Label`, `Progress`, `Separator`, `Skeleton`, `Spinner`, `Table`, `Tabs`, `ToggleGroup`, typography fns |

## Overlays take `&Context`, not `&mut Ui`

The most common mistake. These render at viewport level:

```rust
Dialog::new("Title", &mut self.open).show(ctx, |ui| { … });
AlertDialog::new("Delete?", "This cannot be undone", &mut self.open)
    .destructive(true).show(ctx, || { /* on_confirm */ });
Sheet::new("Title", &mut self.open).side(SheetSide::Right).show(ctx, |ui| { … });
Drawer::new("Title", &mut self.open).show(ctx, |ui| { … });
Command::new("cmd", &groups, &mut self.open).show(ctx);
Toaster::push(ctx, "Saved", ToastVariant::Success);
```

Everything else takes `&mut Ui`.

## Conventions that keep your UI consistent

1. **Never `ui.add_space(n)`.** Use `Spacing::Md.show(ui)` — scale is
   `Xs Sm Md Lg Xl Xl2 Xl3`, convert with `.px()` or `.into()`.
2. **Never hand-roll a bordered/rounded panel.** Use `Boxed` (named `Boxed`, not
   `Box`, to avoid the prelude type) or something built on it — `Card`,
   `Popover`, `Dialog`, `Sheet`, `Drawer`.
3. **Never hardcode colors.** Read `ShadcnTheme::get(ui.ctx())` and use its
   tokens, so your UI follows the user's chosen primary hue and dark/light mode.
4. **Sizing is `Size { Sm, Default, Lg }`** everywhere — except `Button`, which
   uses `ButtonSize { Sm, Default, Lg, Icon }`.
5. **Disabled state**: pass `.enabled(false)`. The component fades itself via
   `ShadcnTheme::DISABLED_OPACITY`. Do not dim colors yourself.
6. **Animation durations** come from `Animations::duration(ctx, base_secs)`,
   never a literal — it returns `0.0` when the user disables motion.

## Not implemented — do not assume these exist

- **No accessibility.** Components emit no `widget_info`/AccessKit nodes, so
  screen readers see nothing. If you need a11y, budget for it yourself.
- **Keyboard navigation** exists only in `Input`, `Textarea`, `InputOtp`,
  `Button` and `Command`. Menus, selects and tabs are mouse-only — no tab order,
  no arrow keys.
- **No form/validation layer.** Build validation in your own application code.
- **No Scroll Area, Sidebar, Form, or Aspect Ratio** component.
- `Calendar::cell_content` is silently dropped if the cell is too short — always
  set `.cell_height()` when you use it.

## References

- `references/components.md` — full catalog: every component's constructor,
  properties, `.show()` arguments and return type, plus usage snippets.
- `references/theming.md` — tokens, `Size`/`Spacing`, animations, icons, fonts.
- `references/i18n.md` — `traductions!` / `t!`, languages, the wasm caveat.
