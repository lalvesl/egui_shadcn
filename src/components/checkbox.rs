use egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct Checkbox<'a> {
    checked:  &'a mut bool,
    label:    Option<&'a str>,
    disabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self { checked, label: None, disabled: false }
    }

    pub fn label(mut self, l: &'a str) -> Self   { self.label = Some(l); self }
    pub fn disabled(mut self, d: bool) -> Self    { self.disabled = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let size  = 18.0;

        ui.horizontal(|ui| {
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

            if response.clicked() && !self.disabled {
                *self.checked = !*self.checked;
            }

            if ui.is_rect_visible(rect) {
                let hovered = response.hovered() && !self.disabled;
                let border_color = if *self.checked || hovered { theme.primary } else { theme.border };
                let bg = if *self.checked { theme.primary } else { Color32::TRANSPARENT };

                ui.painter().rect_filled(rect, 4u8, bg);
                ui.painter().rect_stroke(rect, 4u8, Stroke::new(1.5, border_color), StrokeKind::Inside);

                if *self.checked {
                    let c = rect.center();
                    let check = vec![
                        egui::pos2(c.x - 4.0, c.y),
                        egui::pos2(c.x - 1.5, c.y + 3.0),
                        egui::pos2(c.x + 4.0, c.y - 3.5),
                    ];
                    ui.painter().add(egui::Shape::line(check, Stroke::new(2.0, theme.primary_foreground)));
                }
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
