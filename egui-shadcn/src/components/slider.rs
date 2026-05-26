use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};
use crate::ShadcnTheme;

pub struct Slider<'a> {
    value:  &'a mut f32,
    min:    f32,
    max:    f32,
    step:   Option<f32>,
    height: f32,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, min: f32, max: f32) -> Self {
        Self { value, min, max, step: None, height: 20.0 }
    }

    pub fn step(mut self, s: f32)   -> Self { self.step   = Some(s); self }
    pub fn height(mut self, h: f32) -> Self { self.height = h; self }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = ShadcnTheme::get(ui.ctx());

        let track_h = 6.0;
        let thumb_r = 9.0;
        let width   = ui.available_width();

        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(width, self.height.max(thumb_r * 2.0)),
            Sense::drag(),
        );

        let track_rect = egui::Rect::from_center_size(
            rect.center(),
            Vec2::new(width, track_h),
        );

        if resp.dragged() {
            let delta = resp.drag_delta().x;
            let range  = self.max - self.min;
            let px_per_unit = (width - thumb_r * 2.0) / range;
            *self.value = (*self.value + delta / px_per_unit).clamp(self.min, self.max);

            if let Some(step) = self.step {
                *self.value = (*self.value / step).round() * step;
            }
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let cr      = CornerRadius::same(255);

            painter.rect_filled(track_rect, cr, theme.secondary);

            let t = (*self.value - self.min) / (self.max - self.min);
            let fill_w = (track_rect.width() * t).max(0.0);

            if fill_w > 0.0 {
                let fill_rect = egui::Rect::from_min_size(
                    track_rect.min,
                    Vec2::new(fill_w, track_h),
                );
                painter.rect_filled(fill_rect, cr, theme.primary);
            }

            let thumb_x = track_rect.left() + track_rect.width() * t;
            let thumb_center = egui::Pos2::new(thumb_x, rect.center().y);

            painter.circle_filled(thumb_center, thumb_r, Color32::WHITE);
            painter.circle_stroke(thumb_center, thumb_r, Stroke::new(2.0, theme.primary));
        }

        resp
    }
}
