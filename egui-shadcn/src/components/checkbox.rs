use crate::{ICON_CHECK, ShadcnTheme};
use egui::{Color32, CornerRadius, Response, Sense, Stroke, Ui, Vec2};

pub struct Checkbox<'a> {
    checked: &'a mut bool,
    label: Option<&'a str>,
    enabled: bool,
}

impl<'a> Checkbox<'a> {
    pub fn new(checked: &'a mut bool) -> Self {
        Self {
            checked,
            label: None,
            enabled: true,
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

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());

        let box_size = 16.0;
        let total_h = box_size;

        let text_galley = self.label.map(|lbl| {
            ui.painter().layout_no_wrap(
                lbl.to_owned(),
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
                theme.foreground,
            )
        });

        let gap = 8.0;
        let text_w = text_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);
        let total_w = box_size + text_w;

        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(total_w, total_h),
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
                    egui::FontId::new(12.0, egui::FontFamily::Name("MaterialIcons".into())),
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

        resp
    }
}
