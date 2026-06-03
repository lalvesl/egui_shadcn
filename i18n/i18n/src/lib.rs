//! `i18n` — the runtime facade. Apps depend on this crate.
//!
//! It owns the *registry* (one `linkme` distributed slice per language, filled
//! by `#[traductions]` expansions), the currently selected language, and the
//! [`translate`] dispatcher that `t!` lowers to.
//!
//! Memory model, by request:
//! * No `HashMap`. The per-language registry is reached by a `match` on the
//!   language discriminant; within a language it is a small slice of catalogs;
//!   within a catalog it is a binary search over a packed key table.
//! * Embedded catalogs are `&'static [u8]` consts in `.rodata` — zero runtime
//!   allocation, zero fragmentation. A plain (no-placeholder) string is returned
//!   as `Cow::Borrowed`, so reading it allocates nothing either.
//! * On wasm you compile with no language features; every lookup misses, the
//!   registered async [`Source`] is asked to fetch the string from a server, and
//!   a fallback is shown until it arrives.

use std::borrow::Cow;
use std::sync::atomic::{AtomicU8, Ordering};
use std::sync::OnceLock;

// Re-exports so the generated code (and users) have a single import root.
pub use i18n_format as format;
pub use i18n_format::Languages;
pub use i18n_format::{
    composite_key, Arg, ArgValue, Catalog, LanguagesWithValue, Plural, Translate,
};
pub use i18n_macros::{t, traductions};
#[doc(hidden)]
pub use linkme;

// ---------------------------------------------------------------------------
// Registry: one distributed slice per language. `#[traductions]` pushes a
// `Catalog` into the slice for each language it embedded. The slices always
// exist; they are simply empty when nothing was compiled in for that language
// (e.g. on wasm, or for languages whose feature is off).
// ---------------------------------------------------------------------------
pub mod registry {
    use super::Catalog;
    #[cfg(not(target_arch = "wasm32"))]
    use linkme::distributed_slice;

    // Native: linkme distributed slices, filled by `#[traductions]` expansions.
    #[cfg(not(target_arch = "wasm32"))]
    #[distributed_slice]
    pub static __I18N_CATALOGS_EN: [Catalog<'static>] = [..];
    #[cfg(not(target_arch = "wasm32"))]
    #[distributed_slice]
    pub static __I18N_CATALOGS_ENUS: [Catalog<'static>] = [..];
    #[cfg(not(target_arch = "wasm32"))]
    #[distributed_slice]
    pub static __I18N_CATALOGS_PT: [Catalog<'static>] = [..];
    #[cfg(not(target_arch = "wasm32"))]
    #[distributed_slice]
    pub static __I18N_CATALOGS_PTBR: [Catalog<'static>] = [..];

    // wasm: `linkme` has no wasm backend (`distributed_slice is not implemented
    // for this platform`), and nothing is embedded there anyway — every lookup
    // misses and the async `Source` is asked instead. So the registry is just
    // four empty slices.
    #[cfg(target_arch = "wasm32")]
    pub static __I18N_CATALOGS_EN: [Catalog<'static>; 0] = [];
    #[cfg(target_arch = "wasm32")]
    pub static __I18N_CATALOGS_ENUS: [Catalog<'static>; 0] = [];
    #[cfg(target_arch = "wasm32")]
    pub static __I18N_CATALOGS_PT: [Catalog<'static>; 0] = [];
    #[cfg(target_arch = "wasm32")]
    pub static __I18N_CATALOGS_PTBR: [Catalog<'static>; 0] = [];
}

/// The catalogs compiled in for a language — selected without a map.
fn catalogs_for(lang: Languages) -> &'static [Catalog<'static>] {
    match lang {
        Languages::En => &registry::__I18N_CATALOGS_EN,
        Languages::EnUs => &registry::__I18N_CATALOGS_ENUS,
        Languages::Pt => &registry::__I18N_CATALOGS_PT,
        Languages::PtBr => &registry::__I18N_CATALOGS_PTBR,
    }
}

// ---------------------------------------------------------------------------
// Currently-selected language
// ---------------------------------------------------------------------------

static CURRENT: AtomicU8 = AtomicU8::new(Languages::En as u8);

/// Set the active UI language (e.g. from a settings dropdown).
pub fn set_language(lang: Languages) {
    CURRENT.store(lang as u8, Ordering::Relaxed);
}

/// The active UI language. `t!` reads this on every call.
pub fn current_language() -> Languages {
    Languages::from_index(CURRENT.load(Ordering::Relaxed)).unwrap_or(Languages::En)
}

// ---------------------------------------------------------------------------
// Async source (the wasm / on-demand path)
// ---------------------------------------------------------------------------

/// Implemented by whatever fetches strings that were not compiled into the
/// binary. On a miss the dispatcher calls [`Source::request`] (fire-and-forget)
/// and shows a fallback; when the fetch completes you insert the result with
/// [`install_runtime_catalog`] and request a repaint.
pub trait Source: Send + Sync {
    fn request(&self, lang: Languages, app_id: u16, variant: u8);
}

static SOURCE: OnceLock<Box<dyn Source>> = OnceLock::new();

/// Register the async source once at startup (typically only on wasm).
pub fn set_source<S: Source + 'static>(source: S) {
    let _ = SOURCE.set(Box::new(source));
}

fn notify_missing(lang: Languages, app_id: u16, variant: u8) {
    if let Some(src) = SOURCE.get() {
        src.request(lang, app_id, variant);
    }
}

// ---------------------------------------------------------------------------
// Runtime-installed catalogs (filled by the async source after a fetch).
// Kept separate from the linkme slices, which are immutable. One Vec per
// language; appended to, never fragmented per-string.
// ---------------------------------------------------------------------------

use std::sync::RwLock;

static RUNTIME: OnceLock<[RwLock<Vec<&'static [u8]>>; Languages::COUNT]> = OnceLock::new();

fn runtime() -> &'static [RwLock<Vec<&'static [u8]>>; Languages::COUNT] {
    RUNTIME.get_or_init(|| {
        [
            RwLock::new(Vec::new()),
            RwLock::new(Vec::new()),
            RwLock::new(Vec::new()),
            RwLock::new(Vec::new()),
        ]
    })
}

/// Install a catalog fetched at runtime. The bytes must outlive the program
/// (leak them, or keep them in a `OnceLock`); this keeps lookups allocation-free
/// and lets us hand back `Cow::Borrowed`. Returns `false` if the bytes are not a
/// valid catalog.
pub fn install_runtime_catalog(lang: Languages, bytes: &'static [u8]) -> bool {
    if Catalog::new(bytes).is_none() {
        return false;
    }
    runtime()[lang.as_index()].write().unwrap().push(bytes);
    true
}

// ---------------------------------------------------------------------------
// The dispatcher `t!` lowers to.
// ---------------------------------------------------------------------------

fn lookup(
    lang: Languages,
    app_id: u16,
    variant: u8,
    count: Option<i64>,
    args: &[Arg<'_>],
) -> Option<Cow<'static, str>> {
    // 1. Embedded catalogs.
    for cat in catalogs_for(lang) {
        if let Some(entry) = cat.lookup(app_id, variant) {
            return Some(entry.render(lang, count, args));
        }
    }
    // 2. Runtime-fetched catalogs.
    if let Some(slots) = RUNTIME.get() {
        let guard = slots[lang.as_index()].read().unwrap();
        for bytes in guard.iter() {
            if let Some(cat) = Catalog::new(bytes) {
                if let Some(entry) = cat.lookup(app_id, variant) {
                    return Some(entry.render(lang, count, args));
                }
            }
        }
    }
    None
}

/// Resolve a translation for the *current* language, with placeholder/plural
/// rendering. `t!` calls this; you can also call it directly.
///
/// Resolution order: current language (embedded, then runtime) → kick the async
/// source on a miss → fallback language → a visible `⟦app:variant⟧` marker so a
/// missing key is obvious rather than silently empty.
pub fn translate(
    app_id: u16,
    variant: u8,
    count: Option<i64>,
    args: &[Arg<'_>],
) -> Cow<'static, str> {
    let lang = current_language();

    if let Some(s) = lookup(lang, app_id, variant, count, args) {
        return s;
    }

    // Not present for the active language: ask the server (if any) and fall back.
    notify_missing(lang, app_id, variant);

    let fb = lang.fallback();
    if fb != lang {
        if let Some(s) = lookup(fb, app_id, variant, count, args) {
            return s;
        }
    }

    Cow::Owned(format!("\u{27E6}{app_id:04x}:{variant}\u{27E7}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    // The active language is global state; serialize the tests that mutate it so
    // the default parallel runner can't interleave a `set_language` from one test
    // into another's assertions.
    static LANG_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn current_language_roundtrip() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language(Languages::PtBr);
        assert_eq!(current_language(), Languages::PtBr);
        set_language(Languages::En);
        assert_eq!(current_language(), Languages::En);
    }

    #[test]
    fn missing_shows_marker() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language(Languages::En);
        // App id/variant that nothing registered.
        let s = translate(0xDEAD, 7, None, &[]);
        assert_eq!(s, "\u{27E6}dead:7\u{27E7}");
    }

    #[test]
    fn runtime_catalog_lookup() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        // Simulate the wasm path: fetch bytes, install, then resolve.
        let bytes = i18n_format::encode_catalog(
            Languages::En,
            vec![i18n_format::EntrySpec {
                app_id: 0x1234,
                variant: 0,
                forms: vec![i18n_format::FormSpec {
                    category: Plural::Other,
                    template: "Hello".into(),
                }],
            }],
        );
        let leaked: &'static [u8] = Box::leak(bytes.into_boxed_slice());
        assert!(install_runtime_catalog(Languages::En, leaked));
        set_language(Languages::En);
        assert_eq!(translate(0x1234, 0, None, &[]), "Hello");
    }
}
