use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

pub struct Switch<'a> {
    checked: &'a mut bool,
    label: Option<&'a str>,
    enabled: bool,
}

impl<'a> Switch<'a> {
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

        let track_w = 44.0;
        let track_h = 24.0;
        let thumb_r = 10.0;

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
        let total_w = track_w + text_w;

        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(total_w, track_h),
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

            let track_rect = egui::Rect::from_min_size(
                egui::Pos2::new(rect.left(), rect.center().y - track_h / 2.0),
                Vec2::new(track_w, track_h),
            );

            let cr = CornerRadius::same(255);

            let track_color = if *self.checked {
                theme.primary
            } else {
                theme.muted_foreground
            };

            let track_alpha = if self.enabled { 255 } else { 128 };
            let track_color = Color32::from_rgba_unmultiplied(
                track_color.r(),
                track_color.g(),
                track_color.b(),
                track_alpha,
            );

            painter.rect_filled(track_rect, cr, track_color);

            // thumb position: 0..=(track_w - track_h)
            let t = if *self.checked { 1.0 } else { 0.0 };

            // smooth animation using egui's animate_bool
            let anim_t = ui
                .ctx()
                .animate_bool_with_time(resp.id, *self.checked, 0.12);

            let thumb_x = track_rect.left() + track_h / 2.0 + anim_t * (track_w - track_h);
            let thumb_center = egui::Pos2::new(thumb_x, track_rect.center().y);
            let _ = t;

            painter.circle_filled(
                thumb_center,
                thumb_r,
                if self.enabled {
                    Color32::WHITE
                } else {
                    ShadcnTheme::with_alpha(Color32::WHITE, 180)
                },
            );

            if let Some(g) = text_galley {
                let text_pos = egui::Pos2::new(
                    rect.left() + track_w + gap,
                    rect.center().y - g.size().y / 2.0,
                );
                painter.galley(text_pos, g, theme.foreground);
            }
        }

        resp
    }
}
