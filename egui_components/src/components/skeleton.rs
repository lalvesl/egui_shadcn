use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Ui, Vec2};

pub struct Skeleton {
    width: f32,
    height: f32,
    circle: bool,
    radius: Option<f32>,
}

impl Skeleton {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            width,
            height,
            circle: false,
            radius: None,
        }
    }

    pub fn circle(size: f32) -> Self {
        Self {
            width: size,
            height: size,
            circle: true,
            radius: None,
        }
    }

    pub fn radius(mut self, r: f32) -> Self {
        self.radius = Some(r);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let (rect, _) = ui.allocate_exact_size(
            Vec2::new(self.width, self.height),
            Sense::hover(),
        );

        if !ui.is_rect_visible(rect) {
            return;
        }

        ui.ctx().request_repaint();

        let t = ui.input(|i| i.time) as f32;
        let pulse = (t * 1.5).sin() * 0.5 + 0.5;

        let base = if theme.dark {
            Color32::from_rgb(39, 39, 42)
        } else {
            Color32::from_rgb(228, 228, 231)
        };

        let highlight = if theme.dark {
            Color32::from_rgb(63, 63, 70)
        } else {
            Color32::from_rgb(244, 244, 245)
        };

        let r =
            base.r() as f32 + (highlight.r() as f32 - base.r() as f32) * pulse;
        let g =
            base.g() as f32 + (highlight.g() as f32 - base.g() as f32) * pulse;
        let b =
            base.b() as f32 + (highlight.b() as f32 - base.b() as f32) * pulse;
        let color = Color32::from_rgb(r as u8, g as u8, b as u8);

        let painter = ui.painter();

        if self.circle {
            painter.circle_filled(rect.center(), self.width / 2.0, color);
        } else {
            let cr = if let Some(r) = self.radius {
                CornerRadius::same(r as u8)
            } else {
                CornerRadius::same(4)
            };
            painter.rect_filled(rect, cr, color);
        }
    }
}
