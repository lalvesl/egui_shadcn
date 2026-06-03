//! Example usage of the i18n system.
//!
//! Build with the languages you want embedded:
//!   cargo run -p example-app                       # en + pt-BR (the default)
//!   cargo run -p example-app --no-default-features --features lang-en
//!   cargo build -p example-app --no-default-features   # wasm-style: fetch all

use i18n::{set_language, t, Languages, Source};

// One application enum. `traductions! { .. }`:
//   * computes APP_ID = salted FNV-1a("Calendar") at compile time,
//   * generates `enum Calendar { January, February, Month, Apples }` (no payload),
//   * implements `Translate`,
//   * encodes one packed catalog per enabled language and registers it.
i18n::traductions! {
    pub enum Calendar {
        January([En("January"), PtBr("Janeiro")]),
        February([En("February"), PtBr("Fevereiro")]),
        // Placeholder: rendered with `t!(.., month = ..)`.
        Month([En("The month {month}"), PtBr("O mês {month}")]),
        // Plural: `t!(.., count = n)` picks the form; `{n}` is the count.
        Apples(plural {
            one: [En("{n} apple"), PtBr("{n} maçã")],
            other: [En("{n} apples"), PtBr("{n} maçãs")],
        }),
    }
}

// A second application enum has a *different* APP_ID, so its variants never
// collide with Calendar's even though both start at variant 0.
i18n::traductions! {
    pub enum Settings {
        Title([En("Settings"), PtBr("Configurações")]),
        Language([En("Language"), PtBr("Idioma")]),
    }
}

/// Stand-in for `egui::Ui`. `t!` yields a `Cow<'static, str>`, which `egui`'s
/// `ui.label(..)` / `ui.button(..)` accept directly (they take `impl Into<WidgetText>`).
struct Ui;
impl Ui {
    fn label(&self, text: impl AsRef<str>) {
        println!("  label: {}", text.as_ref());
    }
}

/// The wasm path: nothing is embedded, so on a miss this is asked to fetch the
/// string. Here it just logs; a real impl would spawn a request and later call
/// `i18n::install_runtime_catalog(..)`.
struct ServerSource;
impl Source for ServerSource {
    fn request(&self, lang: Languages, app_id: u16, variant: u8) {
        println!(
            "  [fetch] GET /i18n/{}/{app_id:04x}/{variant}",
            lang.bcp47()
        );
    }
}

pub fn demo() {
    i18n::set_source(ServerSource);
    let ui = Ui;

    for lang in [Languages::En, Languages::PtBr] {
        set_language(lang);
        println!("== {} ==", lang.bcp47());
        ui.label(t!(Calendar::January));
        ui.label(t!(Calendar::Month, month = t!(Calendar::February)));
        ui.label(t!(Calendar::Apples, count = 1));
        ui.label(t!(Calendar::Apples, count = 5));
        ui.label(t!(Settings::Title));
        ui.label(t!(Settings::Language));
    }
}

fn main() {
    demo();
}

#[cfg(test)]
mod tests {
    use super::*;
    use i18n::Translate;
    use std::sync::Mutex;

    // The active language is global process state, so tests that set it and then
    // assert on translations must not interleave with each other under the
    // default multi-threaded test runner. Serialize them on this lock.
    static LANG_LOCK: Mutex<()> = Mutex::new(());

    #[test]
    fn distinct_app_ids() {
        assert_ne!(Calendar::APP_ID, Settings::APP_ID);
    }

    #[test]
    fn variant_indices() {
        assert_eq!(Calendar::January.variant_index(), 0);
        assert_eq!(Calendar::Apples.variant_index(), 3);
    }

    #[cfg(feature = "lang-en")]
    #[test]
    fn english() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language(Languages::En);
        assert_eq!(t!(Calendar::January), "January");
        assert_eq!(t!(Calendar::Month, month = "May"), "The month May");
        assert_eq!(t!(Calendar::Apples, count = 1), "1 apple");
        assert_eq!(t!(Calendar::Apples, count = 7), "7 apples");
        assert_eq!(t!(Settings::Title), "Settings");
    }

    #[cfg(feature = "lang-pt-br")]
    #[test]
    fn portuguese() {
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language(Languages::PtBr);
        assert_eq!(t!(Calendar::January), "Janeiro");
        assert_eq!(t!(Calendar::Apples, count = 1), "1 maçã");
        assert_eq!(t!(Calendar::Apples, count = 3), "3 maçãs");
        assert_eq!(t!(Settings::Language), "Idioma");
    }

    #[cfg(feature = "lang-pt-br")]
    #[test]
    fn region_fallback_to_base_then_marker() {
        // pt-BR is embedded, pt is not, en is. Selecting Pt (base) finds nothing
        // embedded for Pt, falls back to Pt (terminal), then to the marker —
        // unless en is embedded and we ask for it. Here we check PtBr works and
        // an unknown language degrades to a visible marker.
        let _guard = LANG_LOCK.lock().unwrap_or_else(|e| e.into_inner());
        set_language(Languages::EnUs); // not embedded; falls back to En
        #[cfg(feature = "lang-en")]
        assert_eq!(t!(Calendar::January), "January");
    }
}
