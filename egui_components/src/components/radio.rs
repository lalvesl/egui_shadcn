use super::size::Size;
use crate::ShadcnTheme;
use egui::{Response, Sense, Stroke, Ui, Vec2};

pub struct Radio<'a, T: PartialEq> {
    current: &'a mut T,
    value: T,
    label: Option<&'a str>,
    enabled: bool,
    size: Size,
}

impl<'a, T: PartialEq + Clone> Radio<'a, T> {
    pub fn new(current: &'a mut T, value: T) -> Self {
        Self {
            current,
            value,
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
        let box_r = self.size.box_size() / 2.0;
        let box_d = box_r * 2.0;

        let text_g = self.label.map(|lbl| {
            ui.painter().layout_no_wrap(
                lbl.to_owned(),
                egui::FontId::new(
                    self.size.font_size(),
                    egui::FontFamily::Proportional,
                ),
                theme.foreground,
            )
        });

        let gap = self.size.h_pad() * 0.67;
        let text_w = text_g.as_ref().map(|g| g.size().x + gap).unwrap_or(0.0);
        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(box_d + text_w, box_d),
            if self.enabled {
                Sense::click()
            } else {
                Sense::hover()
            },
        );

        let checked = *self.current == self.value;
        if resp.clicked() && self.enabled {
            *self.current = self.value.clone();
        }

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let center = egui::Pos2::new(rect.left() + box_r, rect.center().y);
            let border_color = if checked || resp.hovered() {
                theme.primary
            } else {
                theme.border
            };
            painter.circle_stroke(
                center,
                box_r - 1.0,
                Stroke::new(1.5, border_color),
            );
            if checked {
                painter.circle_filled(center, box_r * 0.52, theme.primary);
            }
            if let Some(g) = text_g {
                let pos = egui::Pos2::new(
                    rect.left() + box_d + gap,
                    rect.center().y - g.size().y / 2.0,
                );
                painter.galley(pos, g, theme.foreground);
            }
        }
        ui.set_opacity(prev_opacity);
        resp
    }
}
