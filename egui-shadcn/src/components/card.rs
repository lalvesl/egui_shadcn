use super::spacing::Spacing;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};

pub struct Card {
    padding: f32,
    shadow: Option<egui::Shadow>,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            padding: 24.0,
            shadow: None,
        }
    }
}

impl Card {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn padding(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }
    pub fn shadow(mut self, s: egui::Shadow) -> Self {
        self.shadow = Some(s);
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());

        let mut frame = Frame::new()
            .fill(theme.card)
            .corner_radius(CornerRadius::same(theme.radius as u8))
            .stroke(Stroke::new(1.0, theme.border))
            .inner_margin(Margin::same(self.padding as i8));

        if let Some(shadow) = self.shadow {
            frame = frame.shadow(shadow);
        }

        frame.show(ui, content);
    }
}

// ── Card sub-sections ────────────────────────────────────────────────────────

pub fn card_header(ui: &mut Ui, title: &str, description: Option<&str>) {
    let theme = ShadcnTheme::get(ui.ctx());

    ui.label(
        egui::RichText::new(title)
            .font(egui::FontId::new(20.0, egui::FontFamily::Proportional))
            .color(theme.card_foreground)
            .strong(),
    );

    if let Some(desc) = description {
        Spacing::Xs.show(ui);
        ui.label(
            egui::RichText::new(desc)
                .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                .color(theme.muted_foreground),
        );
    }

    Spacing::Sm.show(ui);
}

pub fn card_footer(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.separator();
    Spacing::Xs.show(ui);

    Frame::new()
        .fill(Color32::TRANSPARENT)
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(theme.muted_foreground);
            content(ui);
        });
}
