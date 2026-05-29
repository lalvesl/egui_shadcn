use crate::{ICON_KEYBOARD_ARROW_DOWN, ICON_KEYBOARD_ARROW_RIGHT, ShadcnTheme};
use egui::{Color32, Frame, Margin, Sense, Ui, Vec2};

pub struct Collapsible<'a> {
    id: egui::Id,
    open: &'a mut bool,
    trigger: &'a str,
}

impl<'a> Collapsible<'a> {
    pub fn new(id: impl std::hash::Hash, trigger: &'a str, open: &'a mut bool) -> Self {
        Self { id: egui::Id::new(id), trigger, open }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = ui.available_width();
        let h = 36.0;
        let (rect, resp) = ui.allocate_exact_size(Vec2::new(width, h), Sense::click());
        if resp.clicked() { *self.open = !*self.open; }

        let icon = if *self.open { ICON_KEYBOARD_ARROW_DOWN } else { ICON_KEYBOARD_ARROW_RIGHT };
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

        let anim = ui.ctx().animate_bool_with_time(self.id, *self.open, 0.15);
        if anim > 0.01 {
            Frame::new()
                .fill(Color32::TRANSPARENT)
                .inner_margin(Margin { left: 24, right: 0, top: 0, bottom: 8 })
                .show(ui, content);
        }
    }
}
