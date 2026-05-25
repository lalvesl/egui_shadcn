use egui::{Color32, Margin, Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct Select<'a> {
    selected:    &'a mut usize,
    options:     &'a [&'a str],
    placeholder: &'a str,
    disabled:    bool,
}

impl<'a> Select<'a> {
    pub fn new(selected: &'a mut usize, options: &'a [&'a str]) -> Self {
        Self { selected, options, placeholder: "Select...", disabled: false }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self { self.placeholder = p; self }
    pub fn disabled(mut self, d: bool) -> Self        { self.disabled = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme   = ShadcnTheme::get(ui.ctx());
        let width   = ui.available_width();
        let height  = 36.0;

        let label = if *self.selected < self.options.len() {
            self.options[*self.selected]
        } else {
            self.placeholder
        };

        let is_placeholder = *self.selected >= self.options.len();
        let text_color = if is_placeholder || self.disabled {
            theme.muted_foreground
        } else {
            theme.foreground
        };

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
        let hovered = response.hovered() && !self.disabled;

        if ui.is_rect_visible(rect) {
            let bg     = if self.disabled { theme.muted } else { theme.background };
            let border = if hovered { Stroke::new(1.0, theme.ring) } else { Stroke::new(1.0, theme.border) };

            ui.painter().rect_filled(rect, theme.radius, bg);
            ui.painter().rect_stroke(rect, theme.radius, border, StrokeKind::Inside);

            let galley   = ui.painter().layout_no_wrap(label.to_owned(), egui::FontId::proportional(14.0), text_color);
            let text_pos = egui::pos2(rect.left() + 12.0, rect.center().y - galley.size().y * 0.5);
            ui.painter().galley(text_pos, galley, text_color);

            // Chevron ▾
            let cx = rect.right() - 16.0;
            let cy = rect.center().y;
            ui.painter().add(egui::Shape::line(vec![
                egui::pos2(cx - 4.0, cy - 2.0),
                egui::pos2(cx,       cy + 2.0),
                egui::pos2(cx + 4.0, cy - 2.0),
            ], Stroke::new(1.5, theme.muted_foreground)));
        }

        if response.clicked() && !self.disabled {
            ui.memory_mut(|m| m.toggle_popup(response.id));
        }

        let popup_id = response.id;
        egui::popup::popup_below_widget(
            ui,
            popup_id,
            &response,
            egui::PopupCloseBehavior::CloseOnClickOutside,
            |ui| {
                ui.set_min_width(width);
                egui::Frame::new()
                    .fill(theme.card)
                    .stroke(Stroke::new(1.0, theme.border))
                    .corner_radius(theme.radius)
                    .inner_margin(Margin::same(4))
                    .show(ui, |ui| {
                        for (i, opt) in self.options.iter().enumerate() {
                            let selected = *self.selected == i;
                            let row_fill = if selected { theme.accent } else { Color32::TRANSPARENT };

                            let item = ui.add(
                                egui::Button::new(
                                    egui::RichText::new(*opt).size(14.0).color(theme.foreground)
                                )
                                .frame(false)
                                .fill(row_fill)
                                .min_size(Vec2::new(width - 8.0, 32.0)),
                            );

                            if item.clicked() {
                                *self.selected = i;
                                ui.memory_mut(|m| m.close_popup());
                            }
                        }
                    });
            },
        );

        response
    }
}
