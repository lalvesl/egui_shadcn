use egui::{Margin, Ui};
use crate::theme::ShadcnTheme;

pub struct Tooltip<'a> {
    content: &'a str,
}

impl<'a> Tooltip<'a> {
    pub fn new(content: &'a str) -> Self { Self { content } }

    pub fn show(self, ui: &mut Ui, add_trigger: impl FnOnce(&mut Ui) -> egui::Response) {
        let theme    = ShadcnTheme::get(ui.ctx());
        let response = add_trigger(ui);

        if response.hovered() {
            let tooltip_id = egui::Id::new("shad_tooltip").with(response.id);
            let pos        = response.rect.center_top() - egui::Vec2::new(0.0, 8.0);

            egui::show_tooltip_at(
                ui.ctx(),
                ui.layer_id(),
                tooltip_id,
                pos,
                |ui| {
                    egui::Frame::new()
                        .fill(theme.foreground)
                        .corner_radius(6u8)
                        .inner_margin(Margin::symmetric(8, 4))
                        .show(ui, |ui| {
                            ui.label(
                                egui::RichText::new(self.content)
                                    .size(12.0)
                                    .color(theme.background),
                            );
                        });
                },
            );
        }
    }
}
