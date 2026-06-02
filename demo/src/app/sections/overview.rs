use crate::i18n as t;
use ::i18n::t as tr;
use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ShadcnTheme,
    badge::{Badge, BadgeVariant},
    card::{Card, card_header},
    typography::{
        body_text, code_text, heading1, heading2, heading3, heading4, lead_text, muted_text,
        small_text,
    },
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_overview(&mut self, ui: &mut egui::Ui) {
        let subtitle = tr!(t::OverviewSec::Subtitle);
        self.section_title(ui, "egui-shadcn", subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::OverviewSec::HWelcomeDesc);
            card_header(
                ui,
                tr!(t::OverviewSec::HWelcome).as_ref(),
                Some(desc.as_ref()),
            );
            muted_text(ui, tr!(t::OverviewSec::Body).as_ref());
            Spacing::Lg.show(ui);
            ui.horizontal(|ui| {
                Badge::new("v0.1.0").show(ui);
                Spacing::Xs.show(ui);
                Badge::new("egui 0.34")
                    .variant(BadgeVariant::Secondary)
                    .show(ui);
                Spacing::Xs.show(ui);
                Badge::new("Rust").variant(BadgeVariant::Outline).show(ui);
            });
        });

        Spacing::Xl.show(ui);
        let theme = ShadcnTheme::get(ui.ctx());

        let comps_label = tr!(t::OverviewSec::Components);
        let themes_label = tr!(t::OverviewSec::Themes);
        let row1: &[(&str, &str)] = &[("56", comps_label.as_ref()), ("2", themes_label.as_ref())];

        ui.horizontal(|ui| {
            for (n, desc) in row1 {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(*n)
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

        let presets_label = tr!(t::OverviewSec::ColorPresets);
        let custom_label = tr!(t::OverviewSec::Customizable);
        let row2: &[(&str, &str)] = &[("8", presets_label.as_ref()), ("∞", custom_label.as_ref())];

        ui.horizontal(|ui| {
            for (n, desc) in row2 {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(*n)
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
        let title = t::section_name(50);
        let subtitle = tr!(t::TypographySec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());
        Card::new().show(ui, |ui| {
            heading1(ui, tr!(t::TypographySec::H1).as_ref());
            Spacing::Sm.show(ui);
            heading2(ui, tr!(t::TypographySec::H2).as_ref());
            Spacing::Sm.show(ui);
            heading3(ui, tr!(t::TypographySec::H3).as_ref());
            Spacing::Sm.show(ui);
            heading4(ui, tr!(t::TypographySec::H4).as_ref());
            Spacing::Sm.show(ui);
            lead_text(ui, tr!(t::TypographySec::Lead).as_ref());
            Spacing::Sm.show(ui);
            body_text(ui, tr!(t::TypographySec::Body).as_ref());
            Spacing::Sm.show(ui);
            muted_text(ui, tr!(t::TypographySec::Muted).as_ref());
            Spacing::Sm.show(ui);
            small_text(ui, tr!(t::TypographySec::Small).as_ref());
            Spacing::Sm.show(ui);
            ui.horizontal(|ui| {
                body_text(ui, tr!(t::TypographySec::InlinePre).as_ref());
                code_text(ui, "code_snippet");
                body_text(ui, tr!(t::TypographySec::InlinePost).as_ref());
            });
        });
    }
}
