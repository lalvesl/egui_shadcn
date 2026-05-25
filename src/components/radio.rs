use egui::{Response, Sense, Stroke, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct RadioGroup<'a, T: PartialEq + Clone> {
    value:    &'a mut T,
    disabled: bool,
}

impl<'a, T: PartialEq + Clone> RadioGroup<'a, T> {
    pub fn new(value: &'a mut T) -> Self {
        Self { value, disabled: false }
    }

    pub fn disabled(mut self, d: bool) -> Self { self.disabled = d; self }

    pub fn item(&mut self, ui: &mut Ui, item_value: T, label: &str) -> Response {
        let theme    = ShadcnTheme::get(ui.ctx());
        let size     = 18.0;
        let selected = *self.value == item_value;

        ui.horizontal(|ui| {
            let (rect, response) = ui.allocate_exact_size(Vec2::splat(size), Sense::click());

            if response.clicked() && !self.disabled {
                *self.value = item_value.clone();
            }

            if ui.is_rect_visible(rect) {
                let hovered = response.hovered() && !self.disabled;
                let border  = if selected || hovered { theme.primary } else { theme.border };

                ui.painter().circle_stroke(rect.center(), size * 0.5 - 1.0, Stroke::new(1.5, border));

                if selected {
                    ui.painter().circle_filled(rect.center(), size * 0.27, theme.primary);
                }
            }

            let color = if self.disabled { theme.muted_foreground } else { theme.foreground };
            let r = ui.label(egui::RichText::new(label).size(14.0).color(color));
            response | r
        }).inner
    }
}
