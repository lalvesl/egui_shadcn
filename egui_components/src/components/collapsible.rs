use crate::{
    Animations, ICON_KEYBOARD_ARROW_DOWN, ICON_KEYBOARD_ARROW_RIGHT,
    ShadcnTheme,
};
use egui::{Color32, Frame, Margin, Rect, Sense, Ui, Vec2};

pub struct Collapsible<'a> {
    id: egui::Id,
    open: &'a mut bool,
    trigger: &'a str,
}

impl<'a> Collapsible<'a> {
    pub fn new(
        id: impl std::hash::Hash + std::fmt::Debug,
        trigger: &'a str,
        open: &'a mut bool,
    ) -> Self {
        Self {
            id: egui::Id::new(id),
            trigger,
            open,
        }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = ui.available_width();
        let h = 36.0;
        let (rect, resp) =
            ui.allocate_exact_size(Vec2::new(width, h), Sense::click());
        if resp.clicked() {
            *self.open = !*self.open;
        }

        let icon = if *self.open {
            ICON_KEYBOARD_ARROW_DOWN
        } else {
            ICON_KEYBOARD_ARROW_RIGHT
        };
        ui.painter().text(
            egui::Pos2::new(rect.left() + 2.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            icon,
            crate::icon_font_id(18.0),
            theme.muted_foreground,
        );
        let fg = theme.foreground;
        ui.painter().text(
            egui::Pos2::new(rect.left() + 24.0, rect.center().y),
            egui::Align2::LEFT_CENTER,
            self.trigger,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            fg,
        );

        // Slide the body open/closed by clipping it to an animated height; the
        // natural height is measured each frame and reused on the next.
        let body_dur = Animations::duration(ui.ctx(), 0.2);
        let openness = ui
            .ctx()
            .animate_bool_with_time(self.id, *self.open, body_dur);
        if openness > 0.0 {
            let h_id = self.id.with("body_h");
            let full_h =
                ui.ctx().data(|d| d.get_temp::<f32>(h_id).unwrap_or(0.0));
            let visible_h = (full_h * openness).max(0.0);

            let (window, _) = ui.allocate_exact_size(
                Vec2::new(width, visible_h),
                Sense::hover(),
            );

            let content_rect = Rect::from_min_size(
                window.min,
                Vec2::new(width, full_h.max(1.0)),
            );
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            child.set_clip_rect(window.intersect(ui.clip_rect()));

            let inner = Frame::new()
                .fill(Color32::TRANSPARENT)
                .inner_margin(Margin {
                    left: 24,
                    right: 0,
                    top: 0,
                    bottom: 8,
                })
                .show(&mut child, content);

            ui.ctx().data_mut(|d| {
                d.insert_temp(h_id, inner.response.rect.height())
            });
        }
    }
}
