use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};

pub struct Dialog<'a> {
    title: &'a str,
    open: &'a mut bool,
    width: f32,
}

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            open,
            width: 448.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(self, ctx: &egui::Context, content: impl FnOnce(&mut Ui)) {
        if !*self.open {
            return;
        }

        let theme = ShadcnTheme::get(ctx);

        // Dim overlay
        let overlay_painter = ctx.layer_painter(egui::LayerId::new(
            egui::Order::Background,
            egui::Id::new("dialog_overlay"),
        ));
        overlay_painter.rect_filled(egui::Rect::EVERYTHING, 0.0, Color32::from_black_alpha(128));

        egui::Window::new(self.title)
            .id(egui::Id::new("shadcn_dialog").with(self.title))
            .collapsible(false)
            .resizable(false)
            .default_width(self.width)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                Frame::new()
                    .fill(theme.card)
                    .corner_radius(CornerRadius::same(theme.radius as u8))
                    .stroke(Stroke::new(1.0, theme.border))
                    .inner_margin(Margin::same(24)),
            )
            .title_bar(false)
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(egui::FontId::new(18.0, egui::FontFamily::Proportional))
                            .color(theme.foreground)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if ui.small_button("✕").clicked() {
                            *self.open = false;
                        }
                    });
                });

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(8.0);

                content(ui);
            });
    }
}
