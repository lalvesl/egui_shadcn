# Build pipeline and packaging

Design rationale in RFC 0008 (assets) and RFC 0009 (i18n).

## Generated assets — everything goes to OUT_DIR

`build.rs` writes **nothing** into the workspace. A `nix build` sandbox has a
read-only source tree, so writing there does not merely dirty the tree — it
fails the build.

```rust
include!(concat!(env!("OUT_DIR"), "/icon_consts.rs"));   // icons.rs
```

Generated into `OUT_DIR`:

- `MaterialIcons-Regular.ttf` — downloaded at build time
- `icon_consts.rs` — icon glyph constants, from the upstream codepoints file
- `custom_font_name.rs` — only with `EGUI_SHADCN_CUSTOM_FONT_URL`

Emitted cfgs: `has_material_icons`, `has_custom_font`. A build without network
access turns the first off and falls back to a stub font family, so it degrades
instead of breaking.

To confirm an icon constant name exists, grep the generated file under
`target/`. Do not guess names.

## Network provider: native-tls, not rustls

`ureq` is configured with `default-features = false` and an explicit `NativeTls`
provider — leaving it to auto-detect panics at runtime. rustls is deliberately
avoided: it pulls `ring` into the build graph of every consumer, and `ring` is
the crate most likely to break on an off-pin toolchain. This is only a font
downloader; it should never be the reason someone's build fails.

Under Nix the fonts are fixed-output derivations with pinned hashes, so the
sandbox never reaches the network. Those hashes must be updated by hand when an
upstream asset moves.

## wasm and i18n catalogs

`linkme` has no wasm backend, so distributed-slice registration is compiled out
on `wasm32` and the catalogs must ship as assets. **Without them every string on
the web build renders blank.**

They are generated from the native binary:

```sh
cargo run -p demo --bin demo-native --features lang-en,lang-pt -- --gen-i18n <dir>
```

`nix build .#web` runs this before the wasm build and emits per-language bundles
into `wasm_assets/i18n/`. The demo binary statically reaches every section — and
therefore every `traductions!` enum — so `linkme` collects every catalog,
embedded and component, into the bundles.

All runtime fetches use relative paths, so the bundle works unchanged under the
`/egui_shadcn/` GitHub Pages subpath.

## Release profile — tuned for wasm size

```toml
opt-level = 3      # beats "s"/"z": inlines enough that wasm-opt -Oz DCEs more after
lto = "fat"
codegen-units = 1  # dominant lever: ~0.6 MB vs the default 16
panic = "abort"
strip = true
```

Measured across `opt ∈ {3, s, z} × lto ∈ {off, thin, fat}` post-`wasm-opt`.
Re-measure before changing any of it; the intuitive choices are not the winners
here.

The web bundle also ships a **stripped** MaterialIcons font (unused glyphs
removed).

## CI

Everything CI does wraps the flake, so the same derivations run locally.

| Workflow | Trigger | Runs |
|---|---|---|
| `ci.yml` | push / PR | Gate: `nix build .#checks.<sys>.{clippy,test,web}`. Plus non-blocking `rustfmt --check` and the full `nix run .#e2e`. |
| `pages.yml` | push to `main` | `nix build .#web` → GitHub Pages |

Reproduce the gate:

```sh
nix build .#checks.x86_64-linux.clippy .#checks.x86_64-linux.test .#checks.x86_64-linux.web -L
```

## Crate boundaries worth preserving

- **`e2e/` is its own workspace**, excluded from the root, so its async and
  browser dependencies stay out of the main lockfile and out of the `wasm32`
  build of `demo`.
- **`demo/` is a separate crate** from the library: its size dominated build
  times and its dependencies leaked to consumers when it lived inside.
- **`DemoApp::show(ui)` / `GalleryApp::render(ui)`** are split out of
  `eframe::App::update` so headless tests can step them without an
  `eframe::Frame`. Folding them back in breaks the smoke tests.
