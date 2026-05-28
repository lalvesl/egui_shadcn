use std::sync::Arc;

#[cfg(has_custom_font)]
include!(concat!(env!("OUT_DIR"), "/custom_font_name.rs"));

/// Build the base `FontDefinitions` with MaterialIcons registered.
///
/// Use this when you need to extend the definitions before calling
/// `ctx.set_fonts` (e.g. to add your own fonts on top).
pub fn font_definitions() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(all(not(target_arch = "wasm32"), has_material_icons))]
    {
        fonts.font_data.insert(
            "MaterialIcons".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/MaterialIcons-Regular.ttf"
            )))),
        );
        fonts.families.insert(
            egui::FontFamily::Name("MaterialIcons".into()),
            vec!["MaterialIcons".to_owned()],
        );
    }

    // WASM and builds without font: stub so egui never panics on missing family.
    // WASM loads the real font via ehttp at runtime (register_font_bytes).
    #[cfg(not(all(not(target_arch = "wasm32"), has_material_icons)))]
    {
        let fallback = fonts
            .families
            .get(&egui::FontFamily::Proportional)
            .cloned()
            .unwrap_or_default();
        fonts
            .families
            .insert(egui::FontFamily::Name("MaterialIcons".into()), fallback);
    }

    fonts
}

/// Register the MaterialIcons font family with egui.
/// Safe to call during `eframe::CreationContext`.
pub fn register_font(ctx: &egui::Context) {
    ctx.set_fonts(font_definitions());
}

/// Register a custom text font as the primary proportional font.
/// Only available when built with `EGUI_SHADCN_CUSTOM_FONT_URL`.
/// Safe to call during `eframe::CreationContext`.
#[cfg(has_custom_font)]
pub fn register_custom_font(ctx: &egui::Context) {
    let mut fonts = font_definitions();
    fonts.font_data.insert(
        CUSTOM_FONT_NAME.to_owned(),
        Arc::new(egui::FontData::from_static(include_bytes!(concat!(
            env!("OUT_DIR"),
            "/custom_font.ttf"
        )))),
    );
    fonts
        .families
        .entry(egui::FontFamily::Proportional)
        .or_default()
        .insert(0, CUSTOM_FONT_NAME.to_owned());
    ctx.set_fonts(fonts);
}

/// Register the MaterialIcons font from raw bytes (WASM only).
/// Safe to call during `eframe::CreationContext` — uses `font_definitions()` as
/// base instead of `ctx.fonts()`, which would panic before `begin_frame`.
#[cfg(target_arch = "wasm32")]
pub fn register_font_bytes(ctx: &egui::Context, bytes: Vec<u8>) {
    let mut fonts = font_definitions();
    fonts.font_data.insert(
        "MaterialIcons".to_owned(),
        Arc::new(egui::FontData::from_owned(bytes)),
    );
    fonts.families.insert(
        egui::FontFamily::Name("MaterialIcons".into()),
        vec!["MaterialIcons".to_owned()],
    );
    ctx.set_fonts(fonts);
}

#[cfg(test)]
mod tests {
    /// Validates that the MaterialIcons font (as downloaded by build.rs) can be parsed
    /// by skrifa using the exact same code path that epaint's FontFace::new uses.
    ///
    /// This test catches WASM-breaking font issues before they reach the browser.
    /// If this test passes natively but WASM still crashes, the bug is 32-bit specific.
    #[cfg(has_material_icons)]
    #[test]
    fn material_icons_parses_with_skrifa() {
        use skrifa::MetadataProvider;
        let font_path = concat!(env!("OUT_DIR"), "/MaterialIcons-Regular.ttf");
        let bytes = std::fs::read(font_path).expect("MaterialIcons font file must exist (run cargo build first)");

        // Step 1: FontRef::from_index — same call as epaint's FontFace::new
        let font = skrifa::FontRef::from_index(&bytes, 0)
            .expect("FontRef::from_index failed for MaterialIcons");

        // Step 2: all non-fallible calls that epaint makes
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

        // Step 3: HintingInstance::new — the autohint path that reads GSUB/GPOS
        // epaint wraps this in .ok() so failure = no hinting, but we want to see the error
        let hinting_result = skrifa::outline::HintingInstance::new(
            &outline_glyphs,
            skrifa::instance::Size::unscaled(),
            skrifa::instance::LocationRef::default(),
            skrifa::outline::Target::default(),
        );
        if let Err(ref e) = hinting_result {
            eprintln!("[test] HintingInstance::new failed (non-fatal, epaint uses .ok()): {e}");
        }
    }
}
