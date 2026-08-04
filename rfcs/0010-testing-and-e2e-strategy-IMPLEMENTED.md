# RFC 0010 — Testing and e2e strategy

|             |                    |
| ----------- | ------------------ |
| **Status**  | IMPLEMENTED        |
| **Area**    | workspace, `e2e`   |
| **Created** | 2026-08-03         |
| **Updated** | 2026-08-03         |

## Summary

Three test layers, all pure Rust: a headless `egui::Context` harness for
component rendering and interaction, frame-stepped smoke tests of the real
example apps, and a standalone `e2e` crate driving headless Chromium over the
DevTools Protocol. The Python/Playwright script is deleted.

## Motivation

A component library whose only verification is "it looked fine when I ran the
demo" cannot be refactored with confidence — and [RFC 0001](0001-boxed-surface-primitive-IMPLEMENTED.md)
through [RFC 0004](0004-global-animation-directive-IMPLEMENTED.md) are exactly large,
cross-cutting refactors.

The obvious tool, `egui_kittest`, does not apply: these components paint directly
through `Painter` and emit **no AccessKit nodes**, so a kittest harness has
nothing to query. A different approach was required rather than retrofitting
accessibility trees purely to satisfy a test framework.

The web build needed its own layer for a failure mode the Rust tests cannot see:
a wasm binary that loads, initializes, and then renders a blank canvas.

## Design

### Layer 1 — unit and interaction

A real headless `egui::Context` fed synthetic `RawInput`. Shared harness in
`egui_components/tests/common/mod.rs`: `test_ctx`, `frame`, `render`,
`click_input`.

The pattern is two frames — the first to capture a widget's `Rect`, the second to
deliver a click at its center:

```rust
let ctx = test_ctx();
let rect = render(&ctx, |ui| { /* widget */ });   // frame 1: learn geometry
frame(&ctx, click_input(rect.center()), |ui| { /* widget */ });  // frame 2: click it
assert!(state_changed);
```

> Bind fonts with `ctx.set_fonts(egui_components::font_definitions())` or the
> MaterialIcons family panics.

Coverage: every component across variants, sizes and light/dark
(`render_simple`, `render_complex`); clicks, drags and typing (`interaction`);
theme and token math (`theme`).

### Layer 2 — example smoke

`DemoApp::show(ui)` and `GalleryApp::render(ui)` were split out of
`eframe::App::update` **precisely so tests can step them without an
`eframe::Frame`**. The real apps are then driven frame by frame over every
section, chart kind and locale. "The examples still run" is itself a confidence
test, and it catches integration breakage that per-component tests cannot.

### Layer 3 — web e2e

`e2e/` is a **standalone crate** with its own `[workspace]`, excluded from the
root — its async and browser dependencies stay out of the main lockfile and out
of the `wasm32` build of `demo`. It self-serves `demo/dist`, drives headless
Chromium over CDP via [`chromiumoxide`](https://github.com/mattsse/chromiumoxide),
scans the console for crash signatures, and screenshots the canvas.

The screenshot assertion checks **color diversity**, not merely non-background
pixels — a lost WebGL context or a half-initialized frame can paint a uniform
non-background color and would otherwise pass.

## Reference-level detail

```sh
cargo test --workspace   # layers 1 and 2
nix run .#test           # same, pinned toolchain
nix run .#e2e            # tests → run examples → wasm build → browser test
```

CI runs `nix build .#checks.<sys>.{clippy,test,web}` as the blocking gate, plus
`rustfmt --check` and the full e2e run as non-blocking jobs. Everything CI does
is a thin wrapper over the flake, so the same derivations run locally.

Chromium comes from the flake; `E2E_CHROME` overrides the binary.

### Known hazard: global state

The active i18n language is global ([RFC 0009](0009-compile-time-i18n-IMPLEMENTED.md)), so
tests asserting specific translations race under the parallel runner. They are
serialized behind a `Mutex` in the `i18n` and `example-app` test modules. Any new
language-dependent test must join that mutex.

## Drawbacks

- The synthetic-input harness encodes egui's frame semantics into the tests. An
  egui upgrade that changes when hover or click is resolved will break tests in
  ways that look like product bugs.
- No pixel regression testing. Layer 1 proves a component does not panic and
  that state changes; it does not prove it looks right.
- The e2e layer needs a Chromium binary, which is why it is non-blocking in CI.

## Alternatives

- **`egui_kittest`.** Rejected: no AccessKit nodes to query (see Motivation).
- **Keep the Playwright script.** Rejected: a Python toolchain and a browser
  driver for one screenshot check, in a workspace that is otherwise pure Rust.
- **Snapshot/golden images.** Deferred: valuable, but font rendering differs
  across platforms and would need a pinned rasterizer to be stable.

## Implementation status

- [x] Headless `egui::Context` harness (`tests/common`) — no `egui_kittest`
- [x] Render coverage across variants / sizes / light + dark
- [x] Interaction coverage: clicks, drags, typing
- [x] Theme and token math tests
- [x] `egui_charts` render smoke for every chart kind, both modes, plus palette tests
- [x] `DemoApp::show` / `GalleryApp::render` extracted and stepped headlessly
- [x] `i18n` example binary executed as part of the gate
- [x] Standalone `e2e` crate on `chromiumoxide`; `e2e/test.py` deleted
- [x] Screenshot check asserts color diversity
- [x] `nix run .#test` and `nix run .#e2e`
- [x] i18n language tests serialized behind a `Mutex`
