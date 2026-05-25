use egui::{Color32, Response, Sense, Ui, Vec2};
use crate::theme::ShadcnTheme;

const PILL: u8 = 255;

pub struct Switch<'a> {
    checked:  &'a mut bool,
    label:    Option<&'a str>,
    disabled: bool,
}

impl<'a> Switch<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self { checked, label: None, disabled: false }
    }

    pub fn label(mut self, l: &'a str) -> Self   { self.label = Some(l); self }
    pub fn disabled(mut self, d: bool) -> Self    { self.disabled = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme   = ShadcnTheme::get(ui.ctx());
        let track_w = 44.0;
        let track_h = 24.0;
        let thumb_r = 10.0;

        ui.horizontal(|ui| {
            let (rect, response) = ui.allocate_exact_size(Vec2::new(track_w, track_h), Sense::click());

            if response.clicked() && !self.disabled {
                *self.checked = !*self.checked;
            }

            if ui.is_rect_visible(rect) {
                let t   = ui.ctx().animate_bool(response.id, *self.checked);
                let on  = if self.disabled { theme.muted_foreground } else { theme.primary };
                let off = theme.input;

                let track_color = Color32::from_rgb(
                    lerp(off.r(), on.r(), t),
                    lerp(off.g(), on.g(), t),
                    lerp(off.b(), on.b(), t),
                );
                ui.painter().rect_filled(rect, PILL, track_color);

                let travel = track_w - (thumb_r + 2.0) * 2.0;
                let thumb_x = rect.left() + thumb_r + 2.0 + t * travel;
                ui.painter().circle_filled(egui::pos2(thumb_x, rect.center().y), thumb_r, Color32::WHITE);
            }

            if let Some(lbl) = self.label {
                let color = if self.disabled { theme.muted_foreground } else { theme.foreground };
                let r = ui.label(egui::RichText::new(lbl).size(14.0).color(color));
                response | r
            } else {
                response
            }
        }).inner
    }
}

fn lerp(a: u8, b: u8, t: f32) -> u8 {
    (a as f32 + (b as f32 - a as f32) * t) as u8
}
