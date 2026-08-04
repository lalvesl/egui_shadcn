# egui-shadcn

[![CI](https://github.com/lalvesl/egui_shadcn/actions/workflows/ci.yml/badge.svg)](https://github.com/lalvesl/egui_shadcn/actions/workflows/ci.yml)
[![Pages](https://github.com/lalvesl/egui_shadcn/actions/workflows/pages.yml/badge.svg)](https://github.com/lalvesl/egui_shadcn/actions/workflows/pages.yml)

Rust [egui](https://github.com/emilk/egui) implementation of [Shadcn/ui](https://ui.shadcn.com/) components. Not pixel-perfect, aesthetically faithful.

**🌐 Live demo:** <https://lalvesl.github.io/egui_shadcn/>

## Components

| Component       | Status | Notes                                                           |
| --------------- | ------ | --------------------------------------------------------------- |
| Accordion       | ✅     | Collapsible sections                                            |
| Alert           | ✅     | Default / destructive / warning variants                        |
| Avatar          | ✅     | Initials fallback, colored circles, no image assets             |
| Badge           | ✅     | Default / secondary / destructive / outline                     |
| Button          | ✅     | 6 variants × 4 sizes, enabled/disabled state                    |
| Calendar        | ✅     | Single date, date range (2-month), custom cell content          |
| Card            | ✅     | card_header helper, configurable padding                        |
| Checkbox        | ✅     | Checked / unchecked / disabled                                  |
| Dialog          | ✅     | Modal overlay with dim backdrop                                 |
| Input           | ✅     | Label, placeholder, password mode, focus ring                   |
| Label           | ✅     | Required indicator                                              |
| Progress        | ✅     | Determinate progress bar                                        |
| Radio           | ✅     | Radio group via generic value binding                           |
| Select          | ✅     | Dropdown with manual Area popup                                 |
| Separator       | ✅     | Horizontal / vertical                                           |
| Skeleton        | ✅     | Rect and circle variants with animated shimmer                  |
| Slider          | ✅     | Drag, optional step snapping                                    |
| Spinner         | ✅     | Animated arc with gradient fade                                 |
| Switch          | ✅     | Toggle, enabled/disabled                                        |
| Tabs            | ✅     | Tab bar with content callback                                   |
| Textarea        | ✅     | Multiline, configurable rows                                    |
| Tooltip         | ✅     | On-hover popup via `on_hover_ui`                                |
| Typography      | ✅     | heading1–4, lead, body, muted, small, code helpers              |
| Alert Dialog    | ✅     | Confirmation modal with cancel/confirm, destructive variant     |
| Breadcrumb      | ✅     | Path navigation with separator, returns clicked index           |
| Button Group    | ✅     | Connected button strip, default/outline variants, single-select |
| Carousel        | ✅     | Slide content with prev/next buttons and dot indicators         |
| Chart           | ✅     | Bar and line charts with grid, legend, auto-scale               |
| Collapsible     | ✅     | Expandable section with trigger and content callback            |
| Combobox        | ✅     | Searchable select with filter input                             |
| Command         | ✅     | Command palette with search, groups, keyboard navigation        |
| Context Menu    | ✅     | Right-click popup with items, separators, shortcuts             |
| Data Table      | ✅     | Sorting, filtering, pagination                                  |
| Date Picker     | ✅     | Popover wrapper around Calendar                                 |
| Drawer          | ✅     | Bottom slide-in panel with drag handle                          |
| Dropdown Menu   | ✅     | Button-triggered menu with items, separators, disabled states   |
| Hover Card      | ✅     | Delayed hover popup with configurable content                   |
| Input OTP       | ✅     | N-digit boxes with separator, keyboard capture                  |
| Menubar         | ✅     | Horizontal menubar with dropdown menus and shortcuts            |
| Navigation Menu | ✅     | Horizontal/vertical nav links with active state and badges      |
| Pagination      | ✅     | Page nav with ellipsis, returns new page on click               |
| Popover         | ✅     | Generic floating content popup with click-outside dismiss       |
| Resizable       | ✅     | Two-panel split with draggable divider, horizontal/vertical     |
| Sheet           | ✅     | Side-drawer sliding in from left or right edge                  |
| Table           | ✅     | Striped table with fixed/flexible columns                       |
| Toast           | ✅     | Global notification queue, auto-dismiss, 4 variants             |
| Toggle          | ✅     | Two-state button with label/icon and enabled state              |
| Toggle Group    | ✅     | Segmented control, single-select, generic value binding         |

> ✅ Implemented &nbsp; 🚧 Planned

## Features

- **HSL color system** — pick any primary hue; all Shadcn tokens derive from it
- **Dark / light mode** — toggle in toolbar
- **Material Icons** — downloaded at build time via `build.rs`, embedded natively, served separately for web
- **WASM / web** — demo compiles to both native and WASM (via [Trunk](https://trunkrs.dev/))

## Running

Requires [Nix](https://nixos.org/) with flakes enabled.

```sh
git add .              # Nix flakes require tracked files
nix develop            # enter dev shell (Rust + all GUI libs)
cargo run              # native demo
```

### Web (WASM)

```sh
nix develop
cd demo
trunk serve            # dev server at http://localhost:8080
trunk build --release  # production bundle
```

## Testing

All tests are pure Rust — no Python, no Playwright.

```sh
cargo test --workspace   # unit + interaction + headless smoke tests
nix run .#test           # same, inside the pinned toolchain
nix run .#e2e            # full gate: tests → run examples → WASM browser test
```

Three layers:

| Layer                  | Where                                                             | What it checks                                                                                                                                                                                                                                                                                          |
| ---------------------- | ----------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| **Unit / interaction** | `egui_components/tests/`, `egui_charts/.../tests/`                | Every component renders without panicking across variants, sizes and light/dark; clicks toggle state, sliders drag, inputs type; theme/token math. Driven by a real headless `egui::Context` (no window, no GPU).                                                                                       |
| **Example smoke**      | `demo/tests/smoke.rs`, gallery `#[cfg(test)]`, `i18n/example-app` | The real `DemoApp` and chart `GalleryApp` update loops are stepped frame-by-frame over every section / chart kind / locale — "the examples run" is itself a confidence test.                                                                                                                            |
| **Web e2e**            | `e2e/` (standalone crate)                                         | Builds the WASM demo, serves it, and drives **headless Chromium** over the DevTools Protocol ([`chromiumoxide`](https://github.com/mattsse/chromiumoxide)). Scans the console for crash signatures and screenshots the canvas, asserting it rendered a real (multi-color) UI rather than a blank frame. |

The browser harness self-serves `demo/dist` and talks CDP directly, so it needs
only a Chromium binary (provided by the flake; override with `E2E_CHROME`). It
replaces the former `e2e/test.py` Playwright script.

## CI & deployment

Everything CI does is a thin wrapper over the flake — the same derivations run
locally and on GitHub Actions, so "green on my machine" means green in CI.

| Workflow                                   | Trigger        | What runs                                                                                                                                                                                                                                      |
| ------------------------------------------ | -------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| [`ci.yml`](.github/workflows/ci.yml)       | push / PR      | **Gate:** `nix build .#checks.<sys>.{clippy,test,web}` — clippy `-D warnings`, workspace tests, and the optimized WASM build. Plus two non-blocking jobs: `rustfmt --check` (informational) and the full `nix run .#e2e` headless-browser run. |
| [`pages.yml`](.github/workflows/pages.yml) | push to `main` | `nix build .#web` → publish to GitHub Pages.                                                                                                                                                                                                   |

Reproduce the gate locally:

```sh
nix build .#checks.x86_64-linux.clippy .#checks.x86_64-linux.test .#checks.x86_64-linux.web -L
```

### GitHub Pages build

`nix build .#web` produces a fully self-contained, offline, size-optimized
bundle (`wasm-opt -Oz`): the JS glue, the WASM binary, the stripped MaterialIcons
font, **and the per-language i18n catalog bundles** under `wasm_assets/i18n/`
(generated from the native binary — `linkme` has no wasm backend, so the catalogs
_must_ ship as assets or every string renders blank). All runtime fetches use
relative paths, so it works unchanged under the `/egui_shadcn/` project-pages
subpath.

> **One-time setup:** in the repo, **Settings → Pages → Build and deployment →
> Source: GitHub Actions**. After that, every push to `main` redeploys.

## Workspace structure

```
egui_components/       — Shadcn-style component library (formerly `egui-shadcn`)
  src/
    lib.rs             — re-exports
    theme.rs           — ShadcnTheme, HSL helpers, design tokens
    icons.rs           — Material Icons constants + font registration
    components/        — one file per component
  assets/
    MaterialIcons-Regular.ttf   (downloaded by build.rs)
  build.rs             — downloads font, emits has_material_icons cfg

egui_charts/crates/    — egui_charts (charting lib) + egui_charts_gallery (bin)
i18n/                  — i18n facade, i18n-format, i18n-macros, example-app
egui_sc/               — umbrella crate; re-exports egui_components + egui_charts + i18n

demo/                  — showcase app (native + WASM); depends on egui_sc + i18n
  src/
    app/               — DemoApp + component sections
    main.rs            — native entry
    lib.rs             — WASM entry (wasm_bindgen)
  index.html           — Trunk config

e2e/                   — standalone crate: headless-Chromium web e2e (chromiumoxide)
```

## Design records

Non-trivial changes are written up as RFCs in [`rfcs/`](rfcs/) — what the change
solves, what was decided, and which alternatives lost. Start at the
[index](rfcs/README.md), which also carries the roadmap: each RFC is `WIP`,
`PLANNED`, `PLANNED-LOW` or `IMPLEMENTED`.

## Tech stack

- Rust 1.95+
- [egui](https://github.com/emilk/egui) 0.34 / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.34
- Nix flakes (reproducible dev environment)
- Trunk (WASM builds)
