use std::sync::Arc;

#[cfg(has_custom_font)]
include!(concat!(env!("OUT_DIR"), "/custom_font_name.rs"));

/// Build the base `FontDefinitions` with MaterialIcons registered.
///
/// Use this when you need to extend the definitions before calling
/// `ctx.set_fonts` (e.g. to add your own fonts on top).
pub fn font_definitions() -> egui::FontDefinitions {
    let mut fonts = egui::FontDefinitions::default();

    #[cfg(has_material_icons)]
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

    #[cfg(not(has_material_icons))]
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

/// Replace the MaterialIcons font with real bytes at runtime (WASM only).
/// Call from inside your `ehttp::fetch` completion callback.
#[cfg(target_arch = "wasm32")]
pub fn register_font_bytes(ctx: &egui::Context, bytes: Vec<u8>) {
    let mut fonts = ctx.fonts(|f| f.definitions().clone());
    fonts.font_data.insert(
        "MaterialIcons".to_owned(),
        Arc::new(egui::FontData::from_owned(bytes)),
    );
    fonts.families.insert(
        egui::FontFamily::Name("MaterialIcons".into()),
        vec!["MaterialIcons".to_owned()],
    );
    ctx.set_fonts(fonts);
    ctx.request_repaint();
}
