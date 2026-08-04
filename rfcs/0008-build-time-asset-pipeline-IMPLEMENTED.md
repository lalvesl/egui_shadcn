# RFC 0008 — Build-time asset pipeline

|             |                          |
| ----------- | ------------------------ |
| **Status**  | IMPLEMENTED              |
| **Area**    | `egui_components`, `nix` |
| **Created** | 2026-08-03               |
| **Updated** | 2026-08-03               |

## Summary

Fetch and generate all font and icon assets from `build.rs` into `OUT_DIR`,
include the generated icon registry with `include!`, and remove the checked-in
`assets/` dependency from `index.html`. Ship a simple icon API on top.

## Motivation

The build wrote downloaded fonts and a generated icon registry **into the
workspace**. That is wrong for the usual reasons — dirty working trees, files
that must be gitignored but are load-bearing, and builds that are not idempotent
— and for one specific to this project: a `nix build` sandbox has a read-only
source tree, so writing into the workspace is not merely untidy, it fails.

`index.html` also referenced `assets/` directly, so the wasm build only worked if
someone had run a native build first to populate that directory.

## Design

Everything generated goes to `OUT_DIR` and is pulled back in by path:

```rust
include!(concat!(env!("OUT_DIR"), "/icon_registry.rs"));
```

`build.rs` downloads MaterialIcons at build time and emits a
`has_material_icons` cfg, so a build without network access degrades instead of
breaking.

Icon rendering is one call with sensible defaults:

```rust
icon(ui, icons::CHECK);                        // default size + color
icon(ui, icons::CHECK).size(20.0).color(red);  // when you need control
```

Custom text fonts use the same strategy as the icon font, which is what let the
demo pick up Nerd Fonts without a second mechanism.

## Reference-level detail

- **Network provider**: `ureq` with **native-tls**, not rustls. rustls pulls
  `ring` into the build graph of every consumer, and `ring` is the crate most
  likely to break on an off-pin toolchain. This is a build-time font downloader —
  it does not deserve to be the reason someone's build fails. The `ureq` agent is
  configured with the `NativeTls` provider explicitly, since leaving it to
  auto-detect panics at runtime.
- **Nix**: fonts and codepoints are fetched as fixed-output derivations with
  pinned hashes, so the sandboxed build never reaches the network itself.
  `pkg-config` and the native GUI libs are wired into the build inputs.
- **Web bundle** (`nix build .#web`): a fully self-contained, offline,
  size-optimized bundle — JS glue, wasm binary, the **stripped** MaterialIcons
  font (unused glyphs removed), and the per-language i18n catalogs under
  `wasm_assets/i18n/`. All runtime fetches use relative paths so the bundle works
  unchanged under the `/egui_shadcn/` project-pages subpath.
- **Release profile** is tuned for wasm size: `opt-level = 3`, `lto = "fat"`,
  `codegen-units = 1`, `panic = "abort"`, `strip = true`. `codegen-units = 1` is
  the dominant lever (~0.6 MB against the default 16); `opt-level = 3` beats
  `"s"`/`"z"` here because it inlines enough for `wasm-opt -Oz` to dead-code
  more afterwards. Measured across `opt ∈ {3, s, z} × lto ∈ {off, thin, fat}`
  post-`wasm-opt`.

## Drawbacks

- A build-time network fetch means a cold, offline `cargo build` cannot produce
  the icon font. The `has_material_icons` cfg keeps that a degradation rather
  than a failure, but it is still a footgun outside Nix.
- Pinned hashes in `flake.nix` must be updated by hand when an upstream asset
  moves.

## Alternatives

- **Vendor the fonts into the repo.** Rejected: a multi-megabyte binary blob in
  git, and the license/redistribution question that comes with it.
- **rustls instead of native-tls.** Rejected: see the `ring` argument above.

## Implementation status

- [x] `build.rs` writes downloaded fonts to `OUT_DIR`, not the workspace
- [x] Generated icon registry moved to `OUT_DIR`, pulled in with `include!`
- [x] `index.html` no longer depends on a checked-in `assets/` folder
- [x] Custom text fonts supported via the icon-font mechanism; Nerd Fonts in the demo
- [x] Simplified icon helper with optional size / color and defaults
- [x] `nix run .#web` builds and runs
- [x] `ureq` pinned to native-tls with an explicit `NativeTls` provider
- [x] Size-tuned release profile, empirically chosen
- [x] Clippy warnings across the workspace cleared (CI runs `-D warnings`)
