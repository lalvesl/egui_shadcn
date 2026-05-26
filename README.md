# egui-shadcn

Rust [egui](https://github.com/emilk/egui) implementation of [Shadcn/ui](https://ui.shadcn.com/) components. Not pixel-perfect, aesthetically faithful.

## Components

| Component       | Status | Notes                                                              |
| --------------- | ------ | ------------------------------------------------------------------ |
| Accordion       | ✅     | Collapsible sections                                               |
| Alert           | ✅     | Default / destructive / warning variants                           |
| Avatar          | ✅     | Initials fallback, colored circles, no image assets                |
| Badge           | ✅     | Default / secondary / destructive / outline                        |
| Button          | ✅     | 6 variants × 4 sizes, enabled/disabled state                       |
| Calendar        | ✅     | Single date, date range (2-month), custom cell content             |
| Card            | ✅     | card_header helper, configurable padding                           |
| Checkbox        | ✅     | Checked / unchecked / disabled                                     |
| Dialog          | ✅     | Modal overlay with dim backdrop                                    |
| Input           | ✅     | Label, placeholder, password mode, focus ring                      |
| Label           | ✅     | Required indicator                                                 |
| Progress        | ✅     | Determinate progress bar                                           |
| Radio           | ✅     | Radio group via generic value binding                              |
| Select          | ✅     | Dropdown with manual Area popup                                    |
| Separator       | ✅     | Horizontal / vertical                                              |
| Skeleton        | ✅     | Rect and circle variants with animated shimmer                     |
| Slider          | ✅     | Drag, optional step snapping                                       |
| Spinner         | ✅     | Animated arc with gradient fade                                    |
| Switch          | ✅     | Toggle, enabled/disabled                                           |
| Tabs            | ✅     | Tab bar with content callback                                      |
| Textarea        | ✅     | Multiline, configurable rows                                       |
| Tooltip         | ✅     | On-hover popup via `on_hover_ui`                                   |
| Typography      | ✅     | heading1–4, lead, body, muted, small, code helpers                 |
| Alert Dialog    | 🚧     |                                                                    |
| Breadcrumb      | 🚧     |                                                                    |
| Button Group    | 🚧     |                                                                    |
| Carousel        | 🚧     |                                                                    |
| Chart           | 🚧     |                                                                    |
| Collapsible     | 🚧     |                                                                    |
| Combobox        | 🚧     |                                                                    |
| Command         | 🚧     |                                                                    |
| Context Menu    | 🚧     |                                                                    |
| Data Table      | 🚧     | Sorting, filtering, pagination                                     |
| Date Picker     | 🚧     | Popover wrapper around Calendar                                    |
| Drawer          | 🚧     |                                                                    |
| Dropdown Menu   | 🚧     |                                                                    |
| Hover Card      | 🚧     |                                                                    |
| Input OTP       | 🚧     |                                                                    |
| Menubar         | 🚧     |                                                                    |
| Navigation Menu | 🚧     |                                                                    |
| Pagination      | 🚧     |                                                                    |
| Popover         | 🚧     |                                                                    |
| Resizable       | 🚧     |                                                                    |
| Sheet           | 🚧     | Side-drawer                                                        |
| Table           | 🚧     |                                                                    |
| Toast           | 🚧     |                                                                    |
| Toggle          | 🚧     |                                                                    |
| Toggle Group    | 🚧     |                                                                    |

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

## Workspace structure

```
egui-shadcn/           — library crate
  src/
    lib.rs             — re-exports
    theme.rs           — ShadcnTheme, HSL helpers, design tokens
    icons.rs           — Material Icons constants + font registration
    components/        — one file per component
  assets/
    MaterialIcons-Regular.ttf   (downloaded by build.rs)
  build.rs             — downloads font, emits has_material_icons cfg

demo/                  — showcase app (native + WASM)
  src/
    app.rs             — DemoApp, all component sections
    main.rs            — native entry
    lib.rs             — WASM entry (wasm_bindgen)
  index.html           — Trunk config
```

## Tech stack

- Rust 1.95+
- [egui](https://github.com/emilk/egui) 0.34 / [eframe](https://github.com/emilk/egui/tree/master/crates/eframe) 0.34
- Nix flakes (reproducible dev environment)
- Trunk (WASM builds)
