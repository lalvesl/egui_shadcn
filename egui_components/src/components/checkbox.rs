use super::size::Size;
use crate::{ICON_CHECK, ShadcnTheme};
use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<&'a str>,
    enabled: bool,
    size: Size,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
            size: Size::Default,
        }
    }

    pub fn label(mut self, l: &'a str) -> Self {
        self.label = Some(l);
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let prev_opacity = ui.opacity();
        if !self.enabled {
            ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY);
        }
        let box_size = self.size.box_size();
        let icon_size = (box_size * 0.75).floor();
        let gap = self.size.h_pad() * 0.67;

        let text_galley = self.label.map(|lbl| {
            ui.painter().layout_no_wrap(
                lbl.to_owned(),
                egui::FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
                theme.foreground,
            )
        });

        let text_w = text_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);
        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(box_size + text_w, box_size),
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        if resp.clicked() && self.enabled {
            *self.checked = !*self.checked;
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let box_rect = egui::Rect::from_min_size(
                egui::Pos2::new(rect.left(), rect.center().y - box_size / 2.0),
                Vec2::splat(box_size),
            );
            let cr = CornerRadius::same(4);
            let (bg, border) = if *self.checked {
                (theme.primary, theme.primary)
            } else if resp.hovered() {
                (Color32::TRANSPARENT, theme.foreground)
            } else {
                (Color32::TRANSPARENT, theme.border)
            };
            painter.rect_filled(box_rect, cr, bg);
            painter.rect_stroke(
                box_rect,
                cr,
                Stroke::new(1.5, border),
                egui::StrokeKind::Inside,
            );
            if *self.checked {
                painter.text(
                    box_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    ICON_CHECK,
                    crate::icon_font_id(icon_size),
                    theme.primary_foreground,
                );
            }
            if let Some(g) = text_galley {
                let text_pos = egui::Pos2::new(
                    rect.left() + box_size + gap,
                    rect.center().y - g.size().y / 2.0,
                );
                painter.galley(text_pos, g, theme.foreground);
            }
        }
        ui.set_opacity(prev_opacity);
        resp
    }
}
