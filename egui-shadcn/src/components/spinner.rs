use crate::ShadcnTheme;
use egui::{Sense, Stroke, Ui, Vec2};
use std::f32::consts::TAU;

pub struct Spinner {
    size: f32,
    thickness: f32,
}

impl Default for Spinner {
    fn default() -> Self {
        Self {
            size: 24.0,
            thickness: 2.5,
        }
    }
}

impl Spinner {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }
    pub fn thickness(mut self, t: f32) -> Self {
        self.thickness = t;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let (rect, _) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.ctx().request_repaint();

            let t = ui.input(|i| i.time) as f32;
            let angle = t * TAU * 0.8;
            let arc_len = TAU * 0.75;

            let center = rect.center();
            let radius = self.size / 2.0 - self.thickness;

            let n = 48usize;
            let points: Vec<egui::Pos2> = (0..=n)
                .map(|i| {
                    let a = angle + (i as f32 / n as f32) * arc_len;
                    egui::Pos2::new(center.x + a.cos() * radius, center.y + a.sin() * radius)
                })
                .collect();

            let painter = ui.painter();
            for i in 0..points.len().saturating_sub(1) {
                // gradient fade
                let alpha = (i as f32 / n as f32 * 255.0) as u8;
                let color = egui::Color32::from_rgba_unmultiplied(
                    theme.primary.r(),
                    theme.primary.g(),
                    theme.primary.b(),
                    alpha,
                );
                painter.line_segment(
                    [points[i], points[i + 1]],
                    Stroke::new(self.thickness, color),
                );
            }
        }
    }
}
