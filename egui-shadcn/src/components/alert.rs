use crate::{ICON_ERROR_OUTLINE, ICON_INFO, ICON_WARNING, ShadcnTheme};
use egui::{CornerRadius, FontFamily, FontId, Frame, Margin, Stroke, Ui};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
    Warning,
}

pub struct Alert<'a> {
    title: &'a str,
    description: Option<&'a str>,
    variant: AlertVariant,
}

impl<'a> Alert<'a> {
    pub fn new(title: &'a str) -> Self {
        Self {
            title,
            description: None,
            variant: AlertVariant::Default,
        }
    }

    pub fn description(mut self, d: &'a str) -> Self {
        self.description = Some(d);
        self
    }
    pub fn variant(mut self, v: AlertVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let (border_color, icon_color, icon) = match self.variant {
            AlertVariant::Default => (theme.border, theme.foreground, ICON_INFO),
            AlertVariant::Destructive => (theme.destructive, theme.destructive, ICON_ERROR_OUTLINE),
            AlertVariant::Warning => (
                egui::Color32::from_rgb(234, 179, 8),
                egui::Color32::from_rgb(234, 179, 8),
                ICON_WARNING,
            ),
        };

        let bg = if self.variant == AlertVariant::Destructive {
            ShadcnTheme::with_alpha(theme.destructive, 15)
        } else {
            theme.card
        };

        Frame::new()
            .fill(bg)
            .corner_radius(CornerRadius::same(theme.radius as u8))
            .stroke(Stroke::new(1.0, border_color))
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    // Icon
                    ui.label(
                        egui::RichText::new(icon)
                            .font(FontId::new(18.0, FontFamily::Name("MaterialIcons".into())))
                            .color(icon_color),
                    );

                    ui.add_space(8.0);

                    ui.vertical(|ui| {
                        ui.label(
                            egui::RichText::new(self.title)
                                .font(FontId::new(14.0, FontFamily::Proportional))
                                .color(theme.foreground)
                                .strong(),
                        );

                        if let Some(desc) = self.description {
                            ui.add_space(2.0);
                            ui.label(
                                egui::RichText::new(desc)
                                    .font(FontId::new(13.0, FontFamily::Proportional))
                                    .color(theme.muted_foreground),
                            );
                        }
                    });
                });
            });
    }
}
