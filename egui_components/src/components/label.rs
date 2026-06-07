use crate::ShadcnTheme;
use egui::Ui;

pub struct Label<'a> {
    text: &'a str,
    required: bool,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self {
            text,
            required: false,
        }
    }
    pub fn required(mut self, r: bool) -> Self {
        self.required = r;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(self.text)
                    .font(egui::FontId::new(
                        14.0,
                        egui::FontFamily::Proportional,
                    ))
                    .color(theme.foreground),
            );
            if self.required {
                ui.label(
                    egui::RichText::new("*")
                        .font(egui::FontId::new(
                            14.0,
                            egui::FontFamily::Proportional,
                        ))
                        .color(theme.destructive),
                );
            }
        });
    }
}
