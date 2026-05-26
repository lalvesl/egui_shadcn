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

        egui::Tooltip::for_enabled(&response)
            .show(|ui| {
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
            });
    }
}
