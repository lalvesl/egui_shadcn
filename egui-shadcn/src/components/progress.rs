use crate::ShadcnTheme;
use egui::{CornerRadius, Sense, Ui, Vec2};

pub struct Progress {
    value: f32, // 0.0..=1.0
    height: f32,
}

impl Default for Progress {
    fn default() -> Self {
        Self {
            value: 0.0,
            height: 8.0,
        }
    }
}

impl Progress {
    pub fn new(value: f32) -> Self {
        Self {
            value: value.clamp(0.0, 1.0),
            ..Self::default()
        }
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let width = ui.available_width();
        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, self.height), Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let cr = CornerRadius::same(255);

            painter.rect_filled(rect, cr, theme.secondary);

            if self.value > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    rect.min,
                    Vec2::new(rect.width() * self.value, rect.height()),
                );
                painter.rect_filled(fill_rect, cr, theme.primary);
            }
        }
    }
}
