use egui::{Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct Textarea<'a> {
    value:       &'a mut String,
    placeholder: &'a str,
    rows:        usize,
    disabled:    bool,
}

impl<'a> Textarea<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self { value, placeholder: "", rows: 4, disabled: false }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self { self.placeholder = p; self }
    pub fn rows(mut self, r: usize) -> Self           { self.rows = r; self }
    pub fn disabled(mut self, d: bool) -> Self        { self.disabled = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme   = ShadcnTheme::get(ui.ctx());
        let width   = ui.available_width();
        let height  = self.rows as f32 * 20.0 + 24.0;

        let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click_and_drag());

        let focused = ui.memory(|m| m.has_focus(response.id));
        let hovered = response.hovered() && !self.disabled;

        let bg = if self.disabled { theme.muted } else { theme.background };
        let border = if focused {
            Stroke::new(2.0, theme.ring)
        } else if hovered {
            Stroke::new(1.0, ShadcnTheme::with_alpha(theme.foreground, 180))
        } else {
            Stroke::new(1.0, theme.input)
        };

        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, theme.radius, bg);
            ui.painter().rect_stroke(rect, theme.radius, border, StrokeKind::Inside);
        }

        let text_color = if self.disabled { theme.muted_foreground } else { theme.foreground };
        let inner = rect.shrink2(Vec2::new(12.0, 8.0));

        // Scrollable text area clipped to the fixed rect
        ui.scope_builder(egui::UiBuilder::new().max_rect(inner), |ui| {
            egui::ScrollArea::vertical()
                .id_salt(response.id)
                .max_height(inner.height())
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.add(
                        egui::TextEdit::multiline(self.value)
                            .hint_text(self.placeholder)
                            .frame(egui::Frame::new())
                            .text_color(text_color)
                            .interactive(!self.disabled)
                            .desired_width(f32::INFINITY),
                    );
                });
        });

        response
    }
}
