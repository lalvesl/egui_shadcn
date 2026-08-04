# AGENTS.md

Instructions for LLM coding agents working in this repository. Tool-agnostic —
`CLAUDE.md` is a symlink to this file.

## What this is

`egui-shadcn` — a Rust implementation of [Shadcn/ui](https://ui.shadcn.com/)
components for [egui](https://github.com/emilk/egui). Aesthetically faithful,
not pixel-perfect.

## Dev environment

```bash
nix develop            # dev shell: Rust 1.96.0 + GUI libs
cargo run              # native demo
cargo build
cargo clippy           # CI runs with -D warnings
cargo test --workspace
nix run .#test         # tests, pinned toolchain
nix run .#e2e          # tests → examples → wasm build → headless browser
nix run .#fmt
```

Linux GUI libs (`libxkbcommon`, `wayland`, …) come from `flake.nix` buildInputs.

## Architecture

Cargo workspace:

```
egui_components/       component library
  src/lib.rs           pub use re-exports
  src/theme.rs         ShadcnTheme + Animations — design tokens
  src/components/      one file per component
  src/fonts.rs         font_definitions / register_font
  src/icons.rs         include!(OUT_DIR/icon_consts.rs) — generated
  build.rs             downloads MaterialIcons, emits cfgs
egui_charts/crates/    egui_charts (lib) + egui_charts_gallery (bin)
i18n/                  i18n facade + i18n-format + i18n-macros + example-app
egui_sc/               umbrella: re-exports egui_components + egui_charts + i18n
demo/                  showcase app (native + wasm); depends on egui_sc + i18n
e2e/                   standalone crate (own [workspace]) — headless Chromium
rfcs/                  design records
skills/                agent reference docs
```

## Read these first

| If you are… | Read |
| ----------- | ---- |
| Building an app **with** the library | [`skills/egui-shadcn/SKILL.md`](skills/egui-shadcn/SKILL.md) |
| Changing the library **itself** | [`skills/egui-shadcn-contributing/SKILL.md`](skills/egui-shadcn-contributing/SKILL.md) |
| Touching a subsystem's design | the matching record in [`rfcs/`](rfcs/README.md) |

Each `SKILL.md` links to `references/` files with the full component catalog,
theming tokens, i18n, testing and build details. Load them on demand.

## Component API pattern

Every component is a builder ending in `.show()`, taking `&mut egui::Ui` and
returning `egui::Response` where applicable:

```rust
let resp = Button::new("Click me")
    .variant(ButtonVariant::Destructive)
    .show(ui);
if resp.clicked() { … }
```

Return types are **not uniform** (`Response`, `bool`, `Option<usize>`,
`Option<(usize, usize)>`, `usize`, `()`) — the table in
`skills/egui-shadcn/SKILL.md` lists which is which.

**Overlays take `&egui::Context`, not `&mut Ui`**: `Dialog`, `AlertDialog`,
`Sheet`, `Drawer`, `Command`, `Toaster`.

## Design tokens (`theme.rs`)

Mirrors Shadcn CSS variables: `background`, `foreground`, `card`, `primary`,
`primary_foreground`, `secondary`, `muted`, `accent`, `destructive`, `success`,
`warning`, `border`, `input`, `ring`, `radius` — each with its `_foreground`
pair where applicable. Everything derives from one hue.

## Adding a new component

1. Create `egui_components/src/components/mycomp.rs`
2. Add `pub mod mycomp;` to `egui_components/src/components/mod.rs`
3. Add a demo section under `demo/src/app/sections/`
4. Add tests in `egui_components/tests/`
5. Add a row to the `README.md` component table

## Ground rules

- **Read the RFC before redesigning a subsystem.** `rfcs/` records what was
  decided and which alternatives were already rejected — including several that
  look obviously better until you read why they lost.
- **Compose existing primitives.** Surfaces go through `Boxed`, spacing through
  `Spacing` (never `ui.add_space`), sizes through `Size` (`Button` alone uses
  `ButtonSize`), colors through `ShadcnTheme` tokens, durations through
  `Animations::duration`. A component that reimplements any of those is a bug.
- **Disabled state** is `ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY)`
  with the previous value restored — never also dim individual colors, that
  double-dims.
- **Verify before you assert.** Icon constants are generated into `OUT_DIR` at
  build time — grep the generated file rather than guessing a name.
- **Do not claim work is done without running the gate**: `cargo test
  --workspace` and `cargo clippy`.

## Design records (RFCs)

`rfcs/` holds one record per non-trivial change — index in `rfcs/README.md`,
template in `rfcs/0000-template.md`.

Statuses are `WIP`, `PLANNED`, `PLANNED-LOW`, `IMPLEMENTED`, `DEPRECATED`, and
live in **both** the header table and the filename suffix
(`NNNN-title-STATUS.md`) — changing one means `git mv` plus an index update.
RFC 0011 (calendar cell sizing) is the only `WIP` one. There is no tasklist;
new work gets an RFC.

## Notes

- Theme lives in `egui::Memory` via `egui::Id` — `ShadcnTheme::get(ctx)` /
  `ShadcnTheme::set(ctx, theme)`. Re-set and `apply` it every frame, or a
  runtime theme change only takes effect after a restart.
- `register_font(ctx)` is mandatory before any component paints, or the
  `MaterialIcons` family is missing and it panics.
- No external image assets — avatars are colored circles with initials.
- `egui_sc` re-exports under namespaces: `egui_sc::egui_components`,
  `egui_sc::egui_charts`, `egui_sc::i18n`.
- **i18n caveat**: `t!` / `traductions!` expand to absolute `::i18n::…` paths,
  so any crate that *invokes* those macros (e.g. `demo`) must depend on `i18n`
  directly — the `egui_sc::i18n` re-export only covers the runtime API.

## Testing

Full detail in `skills/egui-shadcn-contributing/references/testing.md`.

- Components paint via `Painter` and emit **no AccessKit nodes**, so
  `egui_kittest` does not apply. Tests drive a real headless `egui::Context`
  with synthetic `RawInput`. Shared harness: `egui_components/tests/common/mod.rs`
  (`test_ctx`, `frame`, `render`, `click_input`). Pattern: render one frame to
  capture a widget `Rect`, then a second frame with `click_input(rect.center())`
  to assert the state change. Bind fonts with
  `ctx.set_fonts(egui_components::font_definitions())` or MaterialIcons panics.
- Example apps are smoke-tested headlessly: `DemoApp::show(ui)` and
  `GalleryApp::render(ui)` are split out of `eframe::App::update` precisely so
  tests can step them without an `eframe::Frame`. Keep that split.
- The active i18n language is global state — tests asserting specific
  translations must serialize on a `Mutex` or they race under the parallel runner.
- `e2e/` is a standalone crate using `chromiumoxide` (CDP). It self-serves
  `demo/dist` and needs a Chromium binary via `E2E_CHROME` (the flake provides
  `pkgs.chromium`). The screenshot check requires **color diversity**, not just
  non-background pixels, so a blank or lost-context canvas cannot pass.

## Known gaps — do not assume these exist

No accessibility (no AccessKit nodes), keyboard navigation in only 5 of 55
component files, no form/validation layer, and no Scroll Area, Sidebar, Form or
Aspect Ratio component. `skills/egui-shadcn-contributing/SKILL.md` has the
ranked roadmap.
