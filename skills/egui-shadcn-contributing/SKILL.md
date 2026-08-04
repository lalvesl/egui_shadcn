---
name: egui-shadcn-contributing
description: Contribute to the egui-shadcn library itself — add or modify a component in egui_components, work on egui_charts, the i18n crates, the demo app, the nix/flake build, or the RFC design records. Use when editing files under egui_components/src/, egui_charts/, i18n/, demo/, e2e/ or flake.nix, when adding a new component to the library, when writing or updating an RFC, or when running the test/e2e gate. For merely *using* the library in an application, use the egui-shadcn skill instead.
---

# Contributing to egui-shadcn

Shadcn/ui components for egui. Aesthetically faithful, not pixel-perfect.
This skill covers **working on the library**, not consuming it — for API usage
see the sibling `egui-shadcn` skill and its `references/components.md`.

## Dev environment

```bash
nix develop            # Rust 1.96.0 + all GUI libs (libxkbcommon, wayland, …)
cargo run              # native demo
cargo clippy           # CI runs with -D warnings — a new warning fails the build
cargo test --workspace
nix run .#test         # tests, pinned toolchain
nix run .#e2e          # tests → run examples → wasm build → headless browser
nix run .#fmt
```

## Workspace

```
egui_components/       component library
  src/theme.rs         ShadcnTheme + Animations (design tokens)
  src/components/      one file per component
  src/fonts.rs         font_definitions / register_font
  src/icons.rs         include!(OUT_DIR/icon_consts.rs) — generated
  build.rs             downloads MaterialIcons, emits cfgs
egui_charts/crates/    egui_charts (lib) + egui_charts_gallery (bin)
i18n/                  i18n facade + i18n-format + i18n-macros + example-app
egui_sc/               umbrella crate re-exporting the three above
demo/                  showcase app (native + wasm)
e2e/                   standalone crate (own [workspace]) — headless Chromium
rfcs/                  design records
```

## Read the RFC first

`rfcs/` explains why each subsystem is shaped the way it is and which
alternatives were already rejected. Before changing a subsystem, read its record
— it will save you from re-proposing something already declined.

| Touching | Read |
|---|---|
| Any bordered/rounded surface | RFC 0001 — `Boxed` primitive |
| Sizes, spacing, separators | RFC 0002 |
| Theme tokens, disabled state | RFC 0003 |
| Anything animated | RFC 0004 |
| Calendar / DatePicker | RFC 0005, RFC 0011 (**WIP**) |
| The demo app | RFC 0006 |
| Charts | RFC 0007 |
| build.rs, fonts, nix, wasm size | RFC 0008 |
| i18n | RFC 0009 |
| Tests, e2e | RFC 0010 |

Statuses live in **both** the header table and the filename suffix
(`NNNN-title-STATUS.md`), so changing one means `git mv` plus an index update.
Values: `WIP`, `PLANNED`, `PLANNED-LOW`, `IMPLEMENTED`, `DEPRECATED`.
Full process in `rfcs/README.md`.

Write an RFC when a change introduces a new primitive, alters a public API
shape, or commits the project to a direction that is expensive to reverse. Bug
fixes and one more enum variant do not need one.

## Adding a new component

1. `egui_components/src/components/mycomp.rs` — builder struct + `show`.
2. `pub mod mycomp;` in `components/mod.rs`.
3. Demo section under `demo/src/app/sections/`.
4. Tests in `egui_components/tests/` — see `references/testing.md`.
5. Row in the root `README.md` component table.

**Compose existing primitives.** A component that draws its own rounded border,
invents its own spacing, or writes its own disabled fade is a bug, not a
component. The invariants:

- Surfaces go through `Boxed` — it owns fill, border, radius, shadow, the
  primary bottom accent, and padding. Exceptions are deliberate and listed in
  RFC 0001 (`Alert`, text inputs, buttons).
- Spacing comes from `Spacing`, never `ui.add_space`. As long as no `add_space`
  survives in the library, no call site can invent its own rhythm.
- Sizes come from `Size` (`Button` alone uses `ButtonSize`, which adds `Icon`).
- Colors come from `ShadcnTheme::get(ui.ctx())`. Never a literal.
- Disabled fades the whole widget:

  ```rust
  let prev = ui.opacity();
  ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY);
  // … paint …
  ui.set_opacity(prev);
  ```

  Do not also dim individual colors — that double-dims, which was a real bug.
- Durations go through `Animations::duration(ctx, base)`, which returns `0.0`
  when motion is disabled, so no component needs an `if enabled` branch.
- Component state lives in `ctx.data_mut` keyed by the component `Id`, and
  interactions are namespaced under that `Id` so two instances don't collide.

## User-visible strings

Every string a user reads goes through `traductions!` / `t!`. See the
`egui-shadcn` skill's `references/i18n.md` for the DSL. Note that any crate
invoking those macros needs a direct `i18n` dependency — the `egui_sc::i18n`
re-export only covers the runtime API.

## Contribution roadmap — the real gaps

Measured against the current tree, ranked:

1. **Accessibility + keyboard navigation** (one project). Zero occurrences of
   `widget_info`/AccessKit in `egui_components`; only 5 of 55 component files
   handle focus or keys. This is the only gap that is a correctness problem
   rather than a convenience one, and fixing it unlocks `egui_kittest` —
   currently rejected in RFC 0010 *because* there are no AccessKit nodes.
2. **Form / validation layer.** Nothing exists; every consumer rebuilds it.
3. **Publishability.** `egui_components` has no `description`/`license`/
   `repository`, there is no `LICENSE` file or `CHANGELOG`, and all deps are
   path-only — `cargo publish` cannot work. 43 of 55 component files have zero
   doc comments.
4. **Theming depth** — one hue, one radius, no font token, no presets.
5. Missing components: Scroll Area, Sidebar, Form, Aspect Ratio.

Undocumented subsystems (shipping, but no RFC): responsive layout
(`breakpoint.rs`/`grid.rs`, overlay sidebar, `Calendar::fit_to_width`) and
Android packaging (`cargo-apk` in `flake.nix`).

## References

- `references/testing.md` — the headless harness, the two-frame click pattern,
  the CI gate.
- `references/architecture.md` — build pipeline, generated assets, wasm/i18n
  packaging, size tuning.
