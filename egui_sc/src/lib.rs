//! `egui_sc` — umbrella crate that bundles the project's three building blocks
//! behind a single dependency:
//!
//! * [`egui_components`] — the Shadcn-style egui component library.
//! * [`egui_charts`] — the ECharts-inspired charting library.
//! * [`i18n`] — the compile-time translation runtime.
//!
//! Each is re-exported under its own namespace so call sites stay unambiguous:
//!
//! ```ignore
//! use egui_sc::egui_components::ShadcnTheme;
//! use egui_sc::egui_charts;
//! use egui_sc::i18n;
//! ```
//!
//! ## i18n note
//!
//! The `t!` / `traductions!` macros expand to absolute `::i18n::…` paths, so a
//! crate that *invokes* those macros must still depend on `i18n` directly — the
//! re-export here is only enough to *read* the runtime API
//! (`set_language`, `current_language`, …) through `egui_sc::i18n`.

pub use egui_charts;
pub use egui_components;
pub use i18n;
