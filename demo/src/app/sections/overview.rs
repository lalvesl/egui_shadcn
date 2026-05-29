use egui_shadcn::spacing::Spacing;
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
            Spacing::Lg.show(ui);
            ui.horizontal(|ui| {
                Badge::new("v0.1.0").show(ui);
                Spacing::Xs.show(ui);
                Badge::new("egui 0.34").variant(BadgeVariant::Secondary).show(ui);
                Spacing::Xs.show(ui);
                Badge::new("Rust").variant(BadgeVariant::Outline).show(ui);
            });
        });

        Spacing::Xl.show(ui);
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
                Spacing::Md.show(ui);
            }
        });
        Spacing::Md.show(ui);
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
                Spacing::Md.show(ui);
            }
        });
    }

    pub(in crate::app) fn section_typography(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Typography", "Text styles and type scale.");
        Card::new().show(ui, |ui| {
            heading1(ui, "Heading 1");
            Spacing::Sm.show(ui);
            heading2(ui, "Heading 2");
            Spacing::Sm.show(ui);
            heading3(ui, "Heading 3");
            Spacing::Sm.show(ui);
            heading4(ui, "Heading 4");
            Spacing::Sm.show(ui);
            lead_text(ui, "Lead paragraph — larger, muted text for introductions.");
            Spacing::Sm.show(ui);
            body_text(ui, "Body text — the default paragraph size.");
            Spacing::Sm.show(ui);
            muted_text(ui, "Muted text — de-emphasised, helper copy.");
            Spacing::Sm.show(ui);
            small_text(ui, "Small text — footnotes and captions.");
            Spacing::Sm.show(ui);
            ui.horizontal(|ui| {
                body_text(ui, "Inline ");
                code_text(ui, "code_snippet");
                body_text(ui, " inside a sentence.");
            });
        });
    }
}
