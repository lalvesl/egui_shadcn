use std::sync::Arc;

pub fn register_demo_fonts(ctx: &egui::Context) {
    let mut fonts = ctx.fonts(|f| f.definitions().clone());

    #[cfg(has_fallback_font)]
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

    #[cfg(has_nerd_font)]
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
