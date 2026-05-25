use egui::{Sense, Ui, Vec2};
use crate::theme::ShadcnTheme;

const PILL: u8 = 255;

pub struct Progress {
    value: f32,
    label: Option<String>,
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self { value: value.clamp(0.0, 1.0), label: None }
    }

    pub fn label(mut self, l: impl Into<String>) -> Self { self.label = Some(l.into()); self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        if let Some(ref lbl) = self.label {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new(lbl).size(14.0).color(theme.foreground));
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let pct = format!("{}%", (self.value * 100.0) as u32);
                    ui.label(egui::RichText::new(pct).size(13.0).color(theme.muted_foreground));
                });
            });
            ui.add_space(4.0);
        }

        let width  = ui.available_width();
        let height = 8.0;
        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, PILL, theme.secondary);

            if self.value > 0.0 {
                let fill = egui::Rect::from_min_size(
                    rect.min,
                    Vec2::new(rect.width() * self.value, height),
                );
                ui.painter().rect_filled(fill, PILL, theme.primary);
            }
        }
    }
}
