use egui::{Margin, Stroke, Ui};
use crate::theme::ShadcnTheme;

#[derive(Clone, Copy, Default)]
pub enum AlertVariant {
    #[default]
    Default,
    Destructive,
}

pub struct Alert<'a> {
    title:       &'a str,
    description: Option<&'a str>,
    variant:     AlertVariant,
}

impl<'a> Alert<'a> {
    pub fn new(title: &'a str) -> Self {
        Self { title, description: None, variant: AlertVariant::Default }
    }

    pub fn description(mut self, d: &'a str) -> Self { self.description = Some(d); self }
    pub fn variant(mut self, v: AlertVariant) -> Self { self.variant = v; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let (border_color, accent_color) = match self.variant {
            AlertVariant::Default     => (theme.border, theme.foreground),
            AlertVariant::Destructive => (theme.destructive, theme.destructive),
        };

        egui::Frame::new()
            .fill(theme.background)
            .stroke(Stroke::new(1.0, border_color))
            .corner_radius(theme.radius)
            .inner_margin(Margin::same(16))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    self.draw_icon(ui, accent_color);
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new(self.title).size(14.0).strong().color(accent_color));
                });

                if let Some(desc) = self.description {
                    ui.add_space(6.0);
                    let desc_color = ShadcnTheme::with_alpha(accent_color, 180);
                    ui.label(egui::RichText::new(desc).size(13.0).color(desc_color));
                }
            });
    }

    fn draw_icon(&self, ui: &mut egui::Ui, color: egui::Color32) {
        let sz = 16.0;
        let (r, _) = ui.allocate_exact_size(egui::Vec2::splat(sz), egui::Sense::hover());
        let c      = r.center();

        match self.variant {
            AlertVariant::Default => {
                ui.painter().circle_stroke(c, sz * 0.45, Stroke::new(1.5, color));
                ui.painter().text(c, egui::Align2::CENTER_CENTER, "i", egui::FontId::proportional(10.0), color);
            }
            AlertVariant::Destructive => {
                let pts = vec![
                    egui::pos2(c.x, r.top() + 1.0),
                    egui::pos2(r.right() - 1.0, r.bottom() - 1.0),
                    egui::pos2(r.left() + 1.0, r.bottom() - 1.0),
                ];
                ui.painter().add(egui::Shape::closed_line(pts, Stroke::new(1.5, color)));
                ui.painter().text(c + egui::Vec2::new(0.0, 1.5), egui::Align2::CENTER_CENTER, "!", egui::FontId::proportional(10.0), color);
            }
        }
    }
}
