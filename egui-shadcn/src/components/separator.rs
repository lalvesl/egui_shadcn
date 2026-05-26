use egui::{Sense, Ui, Vec2};
use crate::ShadcnTheme;

#[derive(Clone, Copy, PartialEq)]
pub enum Orientation {
    Horizontal,
    Vertical,
}

pub struct Separator {
    orientation: Orientation,
    thickness:   f32,
}

impl Default for Separator {
    fn default() -> Self {
        Self { orientation: Orientation::Horizontal, thickness: 1.0 }
    }
}

impl Separator {
    pub fn horizontal() -> Self { Self::default() }

    pub fn vertical() -> Self {
        Self { orientation: Orientation::Vertical, thickness: 1.0 }
    }

    pub fn thickness(mut self, t: f32) -> Self { self.thickness = t; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let size = match self.orientation {
            Orientation::Horizontal => Vec2::new(ui.available_width(), self.thickness),
            Orientation::Vertical   => Vec2::new(self.thickness, ui.available_height()),
        };

        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, 0.0, theme.border);
        }
    }
}
