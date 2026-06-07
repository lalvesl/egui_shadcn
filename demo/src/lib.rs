pub mod app;
pub mod fonts;
pub mod i18n;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

// Embedded only for the WASM unit test. Production loads from wasm_assets/ via fetch
// so the binary doesn't carry 349 KB of font data.
#[cfg(all(test, target_arch = "wasm32", has_wasm_icons))]
const MATERIAL_ICONS_TEST: &[u8] = include_bytes!(env!("WASM_MATERIAL_ICONS"));

// Fetch MaterialIcons from the wasm_assets/ directory served alongside the WASM binary.
// The ?v=2 query param busts any browser cache of the old unstripped font.
// Returns None if the fetch fails; the app starts without icons rather than panicking.
#[cfg(target_arch = "wasm32")]
async fn fetch_material_icons() -> Option<Vec<u8>> {
    use wasm_bindgen::JsCast;
    use wasm_bindgen_futures::JsFuture;

    let window = web_sys::window()?;
    let resp = JsFuture::from(
        window.fetch_with_str("wasm_assets/MaterialIcons-Regular.ttf?v=2"),
    )
    .await
    .ok()?;
    let resp: web_sys::Response = resp.dyn_into().ok()?;
    let array_buf = JsFuture::from(resp.array_buffer().ok()?).await.ok()?;
    Some(js_sys::Uint8Array::new(&array_buf).to_vec())
}

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub fn start() -> Result<(), JsValue> {
    use app::DemoApp;

    let web_options = eframe::WebOptions::default();

    wasm_bindgen_futures::spawn_local(async {
        // Load font before eframe starts: no race condition, no async registration.
        let icon_bytes = fetch_material_icons().await;

        let document = web_sys::window().unwrap().document().unwrap();
        let canvas = document.get_element_by_id("egui_canvas").unwrap();
        let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into().unwrap();

        eframe::WebRunner::new()
            .start(
                canvas,
                web_options,
                Box::new(move |cc| {
                    // DemoApp::new calls register_demo_fonts (sets stub icon family).
                    // register_font_bytes then overwrites with the real bytes.
                    // Order matters: register_font_bytes must come AFTER DemoApp::new.
                    let app = DemoApp::new(cc);
                    if let Some(bytes) = icon_bytes {
                        egui_sc::egui_components::register_font_bytes(
                            &cc.egui_ctx,
                            bytes,
                        );
                    }
                    Ok(Box::new(app))
                }),
            )
            .await
            .expect("failed to start eframe");
    });

    Ok(())
}

// ── Android entry point ─────────────────────────────────────────────────────
// android-activity's NativeActivity glue (pulled in via eframe's
// `android-native-activity` feature) resolves and calls this symbol from
// `ANativeActivity_onCreate`. Packaged into an APK by `nix build .#demo2android`.
// edition 2024 requires `unsafe(no_mangle)`; the param is named `android_app`
// to avoid shadowing the crate's `app` module.
#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    use crate::app::DemoApp;

    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info),
    );

    let options = eframe::NativeOptions {
        android_app: Some(android_app),
        ..Default::default()
    };

    if let Err(err) = eframe::run_native(
        "egui-shadcn",
        options,
        Box::new(|cc| Ok(Box::new(DemoApp::new(cc)))),
    ) {
        log::error!("eframe::run_native failed: {err}");
    }
}

#[cfg(test)]
mod tests {
    #[cfg(all(target_arch = "wasm32", has_wasm_icons))]
    use wasm_bindgen_test::*;

    #[cfg(all(target_arch = "wasm32", has_wasm_icons))]
    wasm_bindgen_test_configure!(run_in_browser);

    /// WASM unit test: verifies the embedded MaterialIcons bytes parse correctly with skrifa,
    /// using the exact same API surface that epaint's FontFace::new calls.
    ///
    /// Run with: wasm-pack test --headless --chrome demo/
    #[cfg(all(target_arch = "wasm32", has_wasm_icons))]
    #[wasm_bindgen_test]
    fn material_icons_parses_on_wasm32() {
        use skrifa::MetadataProvider;
        let bytes = super::MATERIAL_ICONS_TEST;

        let font = skrifa::FontRef::from_index(bytes, 0).expect(
            "skrifa::FontRef::from_index failed on wasm32 — the stripped font bytes are invalid",
        );

        let outline_glyphs = font.outline_glyphs();
        let _charmap = font.charmap();
        let _metrics = font.metrics(
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
        );
        let _glyph_metrics = font.glyph_metrics(
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
        );

        let hinting_result = skrifa::outline::HintingInstance::new(
            &outline_glyphs,
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
            skrifa::outline::Target::default(),
        );
        if let Err(ref e) = hinting_result {
            web_sys::console::warn_1(
                &format!("[test] HintingInstance::new error (non-fatal): {e}")
                    .into(),
            );
        }
    }
}
