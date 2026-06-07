use super::size::Size;
use crate::{Animations, ShadcnTheme};
use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

pub struct Switch<'a> {
    checked: &'a mut bool,
    label: Option<&'a str>,
    enabled: bool,
    size: Size,
}

impl<'a> Switch<'a> {
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
        let (track_w, track_h, thumb_r) = self.size.switch_track();

        let text_galley = self.label.map(|lbl| {
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
        let text_w = text_galley
            .as_ref()
            .map(|g| g.size().x + gap)
            .unwrap_or(0.0);
        let (rect, resp) = ui.allocate_exact_size(
            Vec2::new(track_w + text_w, track_h),
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
            painter.rect_filled(track_rect, cr, track_color);

            let thumb_dur = Animations::duration(ui.ctx(), 0.12);
            let anim_t = ui.ctx().animate_bool_with_time(
                resp.id,
                *self.checked,
                thumb_dur,
            );
            let thumb_x = track_rect.left()
                + track_h / 2.0
                + anim_t * (track_w - track_h);
            let thumb_center = egui::Pos2::new(thumb_x, track_rect.center().y);
            painter.circle_filled(thumb_center, thumb_r, Color32::WHITE);

            if let Some(g) = text_galley {
                let text_pos = egui::Pos2::new(
                    rect.left() + track_w + gap,
                    rect.center().y - g.size().y / 2.0,
                );
                painter.galley(text_pos, g, theme.foreground);
            }
        }
        ui.set_opacity(prev_opacity);
        resp
    }
}
