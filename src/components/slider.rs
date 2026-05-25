use egui::{Response, Sense, Stroke, Ui, Vec2};
use crate::theme::ShadcnTheme;

const PILL: u8 = 255;

pub struct Slider<'a> {
    value:    &'a mut f32,
    min:      f32,
    max:      f32,
    label:    Option<&'a str>,
    disabled: bool,
}

impl<'a> Slider<'a> {
    pub fn new(value: &'a mut f32, min: f32, max: f32) -> Self {
        Self { value, min, max, label: None, disabled: false }
    }

    pub fn label(mut self, l: &'a str) -> Self    { self.label = Some(l); self }
    pub fn disabled(mut self, d: bool) -> Self     { self.disabled = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme   = ShadcnTheme::get(ui.ctx());
        let width   = ui.available_width();
        let track_h = 4.0;
        let thumb_r = 10.0;
        let total_h = thumb_r * 2.0 + 4.0;

        if let Some(lbl) = self.label {
            ui.label(egui::RichText::new(lbl).size(14.0).color(theme.foreground));
        }

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, total_h), Sense::click_and_drag());

        let margin_x = thumb_r;
        let track = egui::Rect::from_center_size(
            rect.center(),
            Vec2::new(rect.width() - margin_x * 2.0, track_h),
        );

        let t       = (*self.value - self.min) / (self.max - self.min);
        let thumb_x = track.left() + t * track.width();
        let thumb_c = egui::pos2(thumb_x, rect.center().y);

        if !self.disabled {
            if response.dragged() || response.clicked() {
                if let Some(pos) = ui.input(|i| i.pointer.interact_pos()) {
                    let raw = (pos.x - track.left()) / track.width();
                    *self.value = self.min + raw.clamp(0.0, 1.0) * (self.max - self.min);
                }
            }
        }

        if ui.is_rect_visible(rect) {
            // Track background
            ui.painter().rect_filled(track, PILL, theme.secondary);

            // Track fill
            let filled = egui::Rect::from_min_max(track.left_top(), egui::pos2(thumb_x, track.bottom()));
            ui.painter().rect_filled(filled, PILL, if self.disabled { theme.muted_foreground } else { theme.primary });

            // Thumb ring glow on hover
            if response.hovered() && !self.disabled {
                let glow_color = ShadcnTheme::with_alpha(theme.primary, 30);
                ui.painter().circle_filled(thumb_c, thumb_r + 3.0, glow_color);
            }

            // Thumb
            ui.painter().circle_filled(thumb_c, thumb_r, theme.background);
            ui.painter().circle_stroke(thumb_c, thumb_r - 1.0, Stroke::new(2.0, if self.disabled { theme.muted_foreground } else { theme.primary }));
        }

        response
    }
}
