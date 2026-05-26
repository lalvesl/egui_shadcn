use egui::{Color32, Frame, Margin, Sense, Stroke, Ui, Vec2};
use crate::{ShadcnTheme, ICON_EXPAND_LESS, ICON_EXPAND_MORE};

pub struct Accordion<'a> {
    id:    egui::Id,
    title: &'a str,
    open:  &'a mut bool,
}

impl<'a> Accordion<'a> {
    pub fn new(id: impl std::hash::Hash, title: &'a str, open: &'a mut bool) -> Self {
        Self { id: egui::Id::new(id), title, open }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme    = ShadcnTheme::get(ui.ctx());
        let width    = ui.available_width();
        let header_h = 44.0;

        let (header_rect, resp) = ui.allocate_exact_size(Vec2::new(width, header_h), Sense::click());

        if resp.clicked() {
            *self.open = !*self.open;
        }

        ui.painter().line_segment(
            [header_rect.left_bottom(), header_rect.right_bottom()],
            Stroke::new(1.0, theme.border),
        );

        let font = egui::FontId::new(14.0, egui::FontFamily::Proportional);
        let title_g = ui.painter().layout_no_wrap(self.title.to_owned(), font, theme.foreground);
        ui.painter().galley(
            egui::Pos2::new(header_rect.left(), header_rect.center().y - title_g.size().y / 2.0),
            title_g,
            theme.foreground,
        );

        let icon = if *self.open { ICON_EXPAND_LESS } else { ICON_EXPAND_MORE };
        let _ = ui.ctx().animate_bool_with_time(self.id, *self.open, 0.15);
        ui.painter().text(
            egui::Pos2::new(header_rect.right() - 20.0, header_rect.center().y),
            egui::Align2::CENTER_CENTER,
            icon,
            egui::FontId::new(18.0, egui::FontFamily::Name("MaterialIcons".into())),
            theme.muted_foreground,
        );

        let anim = ui.ctx().animate_bool_with_time(self.id.with("body"), *self.open, 0.15);
        if anim > 0.01 {
            Frame::new()
                .fill(Color32::TRANSPARENT)
                .inner_margin(Margin { left: 0, right: 0, top: 4, bottom: 16 })
                .show(ui, content);
        }
    }
}
