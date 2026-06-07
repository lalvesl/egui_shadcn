use crate::ShadcnTheme;
use egui::{Response, Ui};

pub struct Tooltip<'a> {
    text: &'a str,
}

impl<'a> Tooltip<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text }
    }

    /// Show `content` and attach tooltip on hover.
    pub fn wrap(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> Response,
    ) -> Response {
        let text = self.text;

        let inner = content(ui);

        inner.clone().on_hover_ui(|ui| {
            let theme = ShadcnTheme::get(ui.ctx());
            ui.visuals_mut().override_text_color = Some(theme.background);

            egui::Frame::new()
                .fill(theme.foreground)
                .corner_radius(egui::CornerRadius::same(6))
                .inner_margin(egui::Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(text)
                            .font(egui::FontId::new(
                                12.0,
                                egui::FontFamily::Proportional,
                            ))
                            .color(theme.background),
                    );
                });
        });

        inner
    }
}
