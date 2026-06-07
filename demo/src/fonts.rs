// Only used by the optional bundled-font blocks below, which compile in only
// when build.rs managed to fetch them (has_*_font). Gate the import to match,
// or an offline build (no fonts fetched) trips `unused_imports`.
#[cfg(all(
    not(target_arch = "wasm32"),
    any(has_fallback_font, has_nerd_font)
))]
use std::sync::Arc;

pub fn register_demo_fonts(ctx: &egui::Context) {
    #[allow(unused_mut)]
    let mut fonts = egui_sc::egui_components::font_definitions();

    #[cfg(all(not(target_arch = "wasm32"), has_fallback_font))]
    {
        fonts.font_data.insert(
            "Roboto".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/fallback_font.ttf"
            )))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .push("Roboto".to_owned());
    }

    #[cfg(all(not(target_arch = "wasm32"), has_nerd_font))]
    {
        fonts.font_data.insert(
            "JetBrainsMonoNF".to_owned(),
            Arc::new(egui::FontData::from_static(include_bytes!(concat!(
                env!("OUT_DIR"),
                "/nerd_font.ttf"
            )))),
        );
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "JetBrainsMonoNF".to_owned());
        fonts.families.insert(
            egui::FontFamily::Monospace,
            vec!["JetBrainsMonoNF".to_owned()],
        );
    }

    ctx.set_fonts(fonts);
}
