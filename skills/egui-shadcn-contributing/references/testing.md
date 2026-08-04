# Testing

`egui_kittest` does **not** work here: components paint directly through
`Painter` and emit no AccessKit nodes, so there is nothing for kittest to query.
Tests drive a real headless `egui::Context` with synthetic `RawInput` instead.

See RFC 0010 for the full strategy.

## Harness

`egui_components/tests/common/mod.rs` provides `test_ctx`, `frame`, `render`,
`click_input`.

```rust
mod common;
use common::*;

#[test]
fn checkbox_toggles_on_click() {
    let ctx = test_ctx();
    let mut checked = false;

    // Frame 1: render to learn the widget's geometry.
    let rect = render(&ctx, |ui| {
        Checkbox::new(&mut checked).label("Accept").show(ui).rect
    });

    // Frame 2: deliver a click at its center.
    frame(&ctx, click_input(rect.center()), |ui| {
        Checkbox::new(&mut checked).label("Accept").show(ui);
    });

    assert!(checked);
}
```

Two frames are required: egui resolves interaction against the *previous*
frame's layout, so a click in frame 1 lands nowhere.

**Bind fonts or the test panics:**

```rust
ctx.set_fonts(egui_components::font_definitions());
```

Without it the `MaterialIcons` family is missing and any icon paint aborts.

## Test files

| File | Covers |
|---|---|
| `tests/render_simple.rs` | every component renders across variants and sizes |
| `tests/render_complex.rs` | composed / nested cases, light + dark |
| `tests/interaction.rs` | clicks, drags, typing |
| `tests/theme.rs` | token and HSL math |
| `demo/tests/smoke.rs` | the real `DemoApp` stepped over every section |

## Example smoke tests

`DemoApp::show(ui)` and `GalleryApp::render(ui)` are split out of
`eframe::App::update` **so tests can step them without an `eframe::Frame`**.
Keep that split intact when editing either app — folding the body back into
`update` breaks the smoke tests.

## Commands

```sh
cargo test --workspace   # unit + interaction + example smoke
nix run .#test           # same, pinned toolchain
nix run .#e2e            # tests → run examples → wasm build → headless browser
```

CI gate: `nix build .#checks.<sys>.{clippy,test,web}` — clippy runs with
`-D warnings`, so a new warning fails the build.

## Web e2e

`e2e/` is a standalone crate (own `[workspace]`, excluded from the root) using
`chromiumoxide` over CDP. It self-serves `demo/dist`, scans the console for
crash signatures, and screenshots the canvas asserting **color diversity** — a
uniform non-background frame means a lost context, not a rendered UI. Needs a
Chromium binary; the flake provides one, `E2E_CHROME` overrides.

## Gaps

No pixel/snapshot regression testing. Tests prove a component does not panic and
that state changes — not that it looks right.
