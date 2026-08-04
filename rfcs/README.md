# egui-shadcn RFCs

Design records for `egui-shadcn`. Each RFC captures **one coherent change** to the
library, the demo, or the build — what problem it solves, what was decided, and
why the alternatives were rejected.

These records replace the flat tasklist this project used to keep: shipped work
became `IMPLEMENTED` records, in-flight work became `WIP`, and the parking-lot
ideas became `PLANNED-LOW`.

## Status values

| Status          | Meaning                                                                        |
| --------------- | ------------------------------------------------------------------------------ |
| **WIP**         | Work in progress. Being implemented right now; partial work may be on `main`.  |
| **PLANNED**     | Agreed and queued. Will be built; nobody has started.                          |
| **PLANNED-LOW** | Low priority. Worth doing eventually, nothing depends on it. May never be built. |
| **IMPLEMENTED** | Landed. The RFC is now a historical record of *why* the code looks so.         |
| **DEPRECATED**  | No longer useful — declined, abandoned, or replaced. Kept so the argument is not re-litigated; the header says what replaced it, if anything. |

The status lives in **two places that must agree**: the `Status` row in the
RFC's header table, and the filename suffix. Changing a status therefore means
renaming the file (`git mv`) and updating the link in the index below.

## Index

| #                                            | Title                                       | Status      | Area                     |
| -------------------------------------------- | ------------------------------------------- | ----------- | ------------------------ |
| [0001](0001-boxed-surface-primitive-IMPLEMENTED.md)      | Boxed as the shared surface primitive       | IMPLEMENTED | `egui_components`        |
| [0002](0002-sizing-and-spacing-primitives-IMPLEMENTED.md)| Sizing and spacing primitives               | IMPLEMENTED | `egui_components`        |
| [0003](0003-theme-tokens-and-disabled-state-IMPLEMENTED.md)| Theme tokens and a uniform disabled state | IMPLEMENTED | `egui_components`        |
| [0004](0004-global-animation-directive-IMPLEMENTED.md)   | Global animation directive                  | IMPLEMENTED | `egui_components`, demo  |
| [0005](0005-calendar-and-date-picker-IMPLEMENTED.md)     | Calendar and date picker behaviour          | IMPLEMENTED | `egui_components`        |
| [0006](0006-demo-application-shell-IMPLEMENTED.md)       | Demo application shell                      | IMPLEMENTED | `demo`                   |
| [0007](0007-charts-integration-IMPLEMENTED.md)           | Charts integration and theming              | IMPLEMENTED | `egui_charts`, demo      |
| [0008](0008-build-time-asset-pipeline-IMPLEMENTED.md)    | Build-time asset pipeline                   | IMPLEMENTED | `egui_components`, nix   |
| [0009](0009-compile-time-i18n-IMPLEMENTED.md)            | Compile-time i18n catalogs                  | IMPLEMENTED | `i18n/*`                 |
| [0010](0010-testing-and-e2e-strategy-IMPLEMENTED.md)     | Testing and e2e strategy                    | IMPLEMENTED | workspace, `e2e`         |
| [0011](0011-intrinsic-calendar-cell-sizing-WIP.md)| Intrinsic sizing for custom calendar cells | **WIP**     | `egui_components`        |
| [0012](0012-demo-source-snippet-macro-PLANNED-LOW.md)    | Demo source-snippet macro crate             | PLANNED-LOW | `demo`, new crate        |

## Writing a new RFC

Filenames carry the status, so `ls` alone shows the roadmap:

```
NNNN-kebab-case-title-STATUS.md
0013-accessibility-and-keyboard-nav-PLANNED.md
```

1. Copy [`0000-template.md`](0000-template.md) to `NNNN-kebab-case-title-PLANNED.md`,
   taking the next free number. (The template itself has no status suffix — it
   is not an RFC.)
2. Fill in the header table and the sections. Delete sections that genuinely do
   not apply rather than writing "N/A" in them.
3. Add a row to the index above.
4. Open it as `PLANNED` (or `PLANNED-LOW` if nothing depends on it). Flip to
   `WIP` when work starts and `IMPLEMENTED` when it lands — renaming the file
   each time. An RFC that ships without its status updated is a stale RFC.

Small, obvious changes (a bug fix, a typo, one more variant on an existing enum)
do not need an RFC. Write one when the change introduces a new primitive,
changes a public API shape, or commits the project to a direction that is
expensive to reverse.

## Known documentation gaps

These areas ship in the codebase but were never tracked, so they have no RFC
yet. They are listed here so the gap is explicit, not forgotten:

- **Responsive layout** — `breakpoint.rs` / `grid.rs`, the overlay sidebar, and
  the width-driven fallbacks in `Calendar::fit_to_width`.
- **Android packaging** — the `cargo-apk` derivation and `android_main` entry
  point in `flake.nix`.
