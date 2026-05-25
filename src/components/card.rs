use egui::{Margin, Stroke, Ui};
use crate::theme::ShadcnTheme;

pub struct Card {
    width: Option<f32>,
}

impl Card {
    pub fn new() -> Self { Self { width: None } }
    pub fn width(mut self, w: f32) -> Self { self.width = Some(w); self }

    pub fn show(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = self.width.unwrap_or(ui.available_width());

        let frame = egui::Frame::new()
            .fill(theme.card)
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(theme.radius)
            .inner_margin(Margin::same(24));

        frame.show(ui, |ui| {
            ui.set_width(width - 48.0);
            add_contents(ui);
        });
    }
}

impl Default for Card {
    fn default() -> Self { Self::new() }
}

pub fn card_header(ui: &mut Ui, title: &str, description: Option<&str>) {
    let theme = ShadcnTheme::get(ui.ctx());

    ui.label(egui::RichText::new(title).size(20.0).strong().color(theme.card_foreground));

    if let Some(desc) = description {
        ui.add_space(4.0);
        ui.label(egui::RichText::new(desc).size(14.0).color(theme.muted_foreground));
    }

    ui.add_space(16.0);
    let (sep, _) = ui.allocate_exact_size(egui::Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(sep, 0u8, theme.border);
    ui.add_space(16.0);
}

pub fn card_footer(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    let theme = ShadcnTheme::get(ui.ctx());

    ui.add_space(16.0);
    let (sep, _) = ui.allocate_exact_size(egui::Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
    ui.painter().rect_filled(sep, 0u8, theme.border);
    ui.add_space(16.0);

    add_contents(ui);
}

pub fn card_content(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.add_space(8.0);
    add_contents(ui);
}
