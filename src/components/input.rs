use egui::{Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct TextInput<'a> {
    value:       &'a mut String,
    placeholder: &'a str,
    label:       Option<&'a str>,
    disabled:    bool,
    password:    bool,
}

impl<'a> TextInput<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self { value, placeholder: "", label: None, disabled: false, password: false }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self { self.placeholder = p; self }
    pub fn label(mut self, l: &'a str) -> Self       { self.label = Some(l); self }
    pub fn disabled(mut self, d: bool) -> Self        { self.disabled = d; self }
    pub fn password(mut self, p: bool) -> Self        { self.password = p; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());

        if let Some(lbl) = self.label {
            ui.label(egui::RichText::new(lbl).size(14.0).strong().color(theme.foreground));
            ui.add_space(4.0);
        }

        let width  = ui.available_width();
        let height = 36.0;

        let (outer_rect, response) = ui.allocate_exact_size(
            Vec2::new(width, height),
            Sense::click_and_drag(),
        );

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

        if ui.is_rect_visible(outer_rect) {
            ui.painter().rect_filled(outer_rect, theme.radius, bg);
            ui.painter().rect_stroke(outer_rect, theme.radius, border, StrokeKind::Inside);
        }

        if !self.disabled && response.clicked() {
            response.request_focus();
        }

        let inner = outer_rect.shrink2(Vec2::new(12.0, 0.0));
        let te = ui.put(
            inner,
            egui::TextEdit::singleline(self.value)
                .hint_text(self.placeholder)
                .frame(false)
                .text_color(if self.disabled { theme.muted_foreground } else { theme.foreground })
                .password(self.password)
                .interactive(!self.disabled),
        );

        response | te
    }
}
