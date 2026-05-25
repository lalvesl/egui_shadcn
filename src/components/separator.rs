use egui::{Sense, Stroke, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub enum Orientation { Horizontal, Vertical }

pub struct Separator {
    orientation: Orientation,
    length:      Option<f32>,
}

impl Separator {
    pub fn horizontal() -> Self { Self { orientation: Orientation::Horizontal, length: None } }
    pub fn vertical()   -> Self { Self { orientation: Orientation::Vertical,   length: None } }

    pub fn length(mut self, l: f32) -> Self { self.length = Some(l); self }

    pub fn show(self, ui: &mut Ui) {
        let theme  = ShadcnTheme::get(ui.ctx());
        let stroke = Stroke::new(1.0, theme.border);

        match self.orientation {
            Orientation::Horizontal => {
                ui.add_space(4.0);
                let w = ui.available_width();
                let (rect, _) = ui.allocate_exact_size(Vec2::new(w, 1.0), Sense::hover());
                ui.painter().hline(rect.x_range(), rect.center().y, stroke);
                ui.add_space(4.0);
            }

            Orientation::Vertical => {
                let h = self.length.unwrap_or(ui.available_height());
                let (rect, _) = ui.allocate_exact_size(Vec2::new(1.0, h), Sense::hover());
                ui.painter().vline(rect.center().x, rect.y_range(), stroke);
            }
        }
    }
}
