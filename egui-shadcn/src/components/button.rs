use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonVariant {
    #[default]
    Default,
    Destructive,
    Outline,
    Secondary,
    Ghost,
    Link,
}

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonSize {
    Sm,
    #[default]
    Default,
    Lg,
    Icon,
}

pub struct Button<'a> {
    label: &'a str,
    variant: ButtonVariant,
    size: ButtonSize,
    icon: Option<&'a str>,
    enabled: bool,
}

impl<'a> Button<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: ButtonVariant::Default,
            size: ButtonSize::Default,
            icon: None,
            enabled: true,
        }
    }

    pub fn variant(mut self, v: ButtonVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn size(mut self, s: ButtonSize) -> Self {
        self.size = s;
        self
    }
    pub fn icon(mut self, i: &'a str) -> Self {
        self.icon = Some(i);
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let (bg, fg, stroke) = self.colors(&theme);
        let (h_pad, _v_pad, height, font_size) = self.metrics();

        let text_galley = ui.painter().layout_no_wrap(
            self.label.to_owned(),
            egui::FontId::new(font_size, egui::FontFamily::Proportional),
            fg,
        );

        let icon_galley = self.icon.map(|i| {
            ui.painter().layout_no_wrap(
                i.to_owned(),
                crate::icon_font_id(font_size + 2.0),
                fg,
            )
        });

        let text_w = text_galley.size().x;
        let icon_w = icon_galley
            .as_ref()
            .map(|g| g.size().x + 6.0)
            .unwrap_or(0.0);
        let total_w = text_w + icon_w + h_pad * 2.0;
        let size = Vec2::new(total_w.max(height), height);

        let (rect, resp) = ui.allocate_exact_size(
            size,
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let is_link = self.variant == ButtonVariant::Link;
            let cr = CornerRadius::same(if is_link { 0 } else { theme.radius as u8 });

            let actual_bg = if !self.enabled {
                ShadcnTheme::with_alpha(bg, 128)
            } else if resp.is_pointer_button_down_on() {
                Color32::from_rgba_unmultiplied(bg.r(), bg.g(), bg.b(), 220)
            } else if resp.hovered() {
                Color32::from_rgba_unmultiplied(bg.r(), bg.g(), bg.b(), 230)
            } else {
                bg
            };

            if !is_link {
                painter.rect_filled(rect, cr, actual_bg);
                if stroke.width > 0.0 {
                    painter.rect_stroke(rect, cr, stroke, egui::StrokeKind::Inside);
                }
            }

            let text_color = if !self.enabled {
                ShadcnTheme::with_alpha(fg, 128)
            } else {
                fg
            };

            let content_x = rect.left() + h_pad + icon_w;
            let text_pos = egui::Pos2::new(
                content_x - icon_w / 2.0,
                rect.center().y - text_galley.size().y / 2.0,
            );

            if let Some(g) = icon_galley {
                let icon_pos =
                    egui::Pos2::new(rect.left() + h_pad, rect.center().y - g.size().y / 2.0);
                painter.galley(icon_pos, g, text_color);
            }

            painter.galley(text_pos, text_galley, text_color);

            if is_link && (resp.hovered() || resp.has_focus()) {
                painter.line_segment(
                    [rect.left_bottom(), rect.right_bottom()],
                    Stroke::new(1.0, text_color),
                );
            }
        }

        resp
    }

    fn colors(&self, t: &ShadcnTheme) -> (Color32, Color32, Stroke) {
        match self.variant {
            ButtonVariant::Default => (t.primary, t.primary_foreground, Stroke::NONE),
            ButtonVariant::Destructive => (t.destructive, t.destructive_foreground, Stroke::NONE),
            ButtonVariant::Outline => (
                Color32::TRANSPARENT,
                t.foreground,
                Stroke::new(1.0, t.border),
            ),
            ButtonVariant::Secondary => (t.secondary, t.secondary_foreground, Stroke::NONE),
            ButtonVariant::Ghost => (Color32::TRANSPARENT, t.foreground, Stroke::NONE),
            ButtonVariant::Link => (Color32::TRANSPARENT, t.primary, Stroke::NONE),
        }
    }

    fn metrics(&self) -> (f32, f32, f32, f32) {
        // (h_pad, v_pad, height, font_size)
        match self.size {
            ButtonSize::Sm => (12.0, 4.0, 32.0, 13.0),
            ButtonSize::Default => (16.0, 6.0, 36.0, 14.0),
            ButtonSize::Lg => (24.0, 8.0, 44.0, 16.0),
            ButtonSize::Icon => (0.0, 0.0, 36.0, 18.0),
        }
    }
}
