use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};

pub struct Card {
    padding: f32,
    shadow: bool,
}

impl Default for Card {
    fn default() -> Self {
        Self {
            padding: 24.0,
            shadow: true,
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
    pub fn shadow(mut self, s: bool) -> Self {
        self.shadow = s;
        self
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());

        let frame = Frame::new()
            .fill(theme.card)
            .corner_radius(CornerRadius::same(theme.radius as u8))
            .stroke(Stroke::new(1.0, theme.border))
            .inner_margin(Margin::same(self.padding as i8));

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
        ui.add_space(4.0);
        ui.label(
            egui::RichText::new(desc)
                .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                .color(theme.muted_foreground),
        );
    }

    ui.add_space(8.0);
}

pub fn card_footer(ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.separator();
    ui.add_space(4.0);

    Frame::new()
        .fill(Color32::TRANSPARENT)
        .inner_margin(Margin::ZERO)
        .show(ui, |ui| {
            ui.visuals_mut().override_text_color = Some(theme.muted_foreground);
            content(ui);
        });
}
