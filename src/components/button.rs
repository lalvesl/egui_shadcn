use egui::{Color32, Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum ButtonSize {
    #[default]
    Default,
    Sm,
    Lg,
}

pub struct Button<'a> {
    label: &'a str,
    pub variant: ButtonVariant,
    pub size: ButtonSize,
    pub enabled: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, variant: ButtonVariant::Default, size: ButtonSize::Default, enabled: true }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self { self.variant = v; self }
    pub fn size(mut self, s: ButtonSize) -> Self { self.size = s; self }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());

        let (bg, fg, border) = self.colors(&theme);
        let (pad_x, pad_y, font_size) = self.sizing();

        let font_id = egui::FontId::proportional(font_size);
        let galley = ui.painter().layout_no_wrap(self.label.to_owned(), font_id, fg);
        let text_sz = galley.size();

        let desired = Vec2::new(text_sz.x + pad_x * 2.0, text_sz.y + pad_y * 2.0);
        let (rect, response) = ui.allocate_exact_size(desired, Sense::click());

        if ui.is_rect_visible(rect) {
            let bg = self.hover_bg(bg, &response, &theme);
            let cr = theme.radius;

            ui.painter().rect_filled(rect, cr, bg);

            if border != Stroke::NONE {
                ui.painter().rect_stroke(rect, cr, border, StrokeKind::Inside);
            }

            if self.variant == ButtonVariant::Link && response.hovered() {
                let y = rect.bottom() - pad_y * 0.5;
                ui.painter().line_segment(
                    [egui::pos2(rect.left() + pad_x, y), egui::pos2(rect.right() - pad_x, y)],
                    Stroke::new(1.0, fg),
                );
            }

            let text_pos = rect.center() - text_sz * 0.5;
            ui.painter().galley(text_pos, galley, fg);
        }

        response
    }

    fn colors(&self, t: &ShadcnTheme) -> (Color32, Color32, Stroke) {
        let alpha = if self.enabled { 255 } else { 100 };
        match self.variant {
            ButtonVariant::Default => (
                ShadcnTheme::with_alpha(t.primary, alpha),
                t.primary_foreground,
                Stroke::NONE,
            ),
            ButtonVariant::Destructive => (
                ShadcnTheme::with_alpha(t.destructive, alpha),
                t.destructive_foreground,
                Stroke::NONE,
            ),
            ButtonVariant::Outline => (
                Color32::TRANSPARENT,
                ShadcnTheme::with_alpha(t.foreground, alpha),
                Stroke::new(1.0, ShadcnTheme::with_alpha(t.border, alpha)),
            ),
            ButtonVariant::Secondary => (
                ShadcnTheme::with_alpha(t.secondary, alpha),
                ShadcnTheme::with_alpha(t.secondary_foreground, alpha),
                Stroke::NONE,
            ),
            ButtonVariant::Ghost | ButtonVariant::Link => (
                Color32::TRANSPARENT,
                ShadcnTheme::with_alpha(t.foreground, alpha),
                Stroke::NONE,
            ),
        }
    }

    fn hover_bg(&self, base: Color32, resp: &Response, t: &ShadcnTheme) -> Color32 {
        if !self.enabled || base == Color32::TRANSPARENT {
            if resp.hovered() && self.enabled {
                return ShadcnTheme::with_alpha(t.accent, 180);
            }
            return base;
        }

        let factor = if resp.is_pointer_button_down_on() { 0.85 } else if resp.hovered() { 0.90 } else { 1.0 };
        Color32::from_rgb(
            (base.r() as f32 * factor) as u8,
            (base.g() as f32 * factor) as u8,
            (base.b() as f32 * factor) as u8,
        )
    }

    fn sizing(&self) -> (f32, f32, f32) {
        match self.size {
            ButtonSize::Sm => (12.0, 5.0, 13.0),
            ButtonSize::Default => (16.0, 8.0, 14.0),
            ButtonSize::Lg => (24.0, 12.0, 16.0),
        }
    }
}
