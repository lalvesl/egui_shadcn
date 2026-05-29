use egui_shadcn::{
    ShadcnTheme,
    badge::{Badge, BadgeVariant},
    card::{Card, card_header},
    typography::{heading1, heading2, heading3, heading4, lead_text, body_text, muted_text, small_text, code_text},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_overview(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "egui-shadcn", "Shadcn/ui components for Rust egui.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Welcome", Some("A beautiful, accessible component library."));
            muted_text(ui, "Use the sidebar to explore all components. Toggle dark mode or pick a primary color with the toolbar icons above.");
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                Badge::new("v0.1.0").show(ui);
                ui.add_space(4.0);
                Badge::new("egui 0.34").variant(BadgeVariant::Secondary).show(ui);
                ui.add_space(4.0);
                Badge::new("Rust").variant(BadgeVariant::Outline).show(ui);
            });
        });

        ui.add_space(24.0);
        let theme = ShadcnTheme::get(ui.ctx());

        ui.horizontal(|ui| {
            for (n, desc) in [("19", "Components"), ("2", "Themes")] {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(n)
                            .font(egui::FontId::new(28.0, egui::FontFamily::Proportional))
                            .color(theme.primary)
                            .strong(),
                    );
                    muted_text(ui, desc);
                });
                ui.add_space(12.0);
            }
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            for (n, desc) in [("8", "Color presets"), ("∞", "Customizable")] {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(n)
                            .font(egui::FontId::new(28.0, egui::FontFamily::Proportional))
                            .color(theme.primary)
                            .strong(),
                    );
                    muted_text(ui, desc);
                });
                ui.add_space(12.0);
            }
        });
    }

    pub(in crate::app) fn section_typography(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Typography", "Text styles and type scale.");
        Card::new().show(ui, |ui| {
            heading1(ui, "Heading 1");
            ui.add_space(8.0);
            heading2(ui, "Heading 2");
            ui.add_space(8.0);
            heading3(ui, "Heading 3");
            ui.add_space(8.0);
            heading4(ui, "Heading 4");
            ui.add_space(8.0);
            lead_text(ui, "Lead paragraph — larger, muted text for introductions.");
            ui.add_space(8.0);
            body_text(ui, "Body text — the default paragraph size.");
            ui.add_space(8.0);
            muted_text(ui, "Muted text — de-emphasised, helper copy.");
            ui.add_space(8.0);
            small_text(ui, "Small text — footnotes and captions.");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                body_text(ui, "Inline ");
                code_text(ui, "code_snippet");
                body_text(ui, " inside a sentence.");
            });
        });
    }
}
