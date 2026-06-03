use crate::{Animations, ICON_EXPAND_LESS, ICON_EXPAND_MORE, ShadcnTheme};
use egui::{Color32, Frame, Margin, Rect, Sense, Stroke, Ui, Vec2};

pub struct Accordion<'a> {
    id: egui::Id,
    title: &'a str,
    open: &'a mut bool,
}

impl<'a> Accordion<'a> {
    pub fn new(id: impl std::hash::Hash, title: &'a str, open: &'a mut bool) -> Self {
        Self {
            id: egui::Id::new(id),
            title,
            open,
        }
    }

    pub fn show(self, ui: &mut Ui, content: impl FnOnce(&mut Ui)) {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = ui.available_width();
        let header_h = 44.0;

        let (header_rect, resp) =
            ui.allocate_exact_size(Vec2::new(width, header_h), Sense::click());

        if resp.clicked() {
            *self.open = !*self.open;
        }

        ui.painter().line_segment(
            [header_rect.left_bottom(), header_rect.right_bottom()],
            Stroke::new(1.0, theme.border),
        );

        let font = egui::FontId::new(14.0, egui::FontFamily::Proportional);
        let title_g = ui
            .painter()
            .layout_no_wrap(self.title.to_owned(), font, theme.foreground);
        ui.painter().galley(
            egui::Pos2::new(
                header_rect.left(),
                header_rect.center().y - title_g.size().y / 2.0,
            ),
            title_g,
            theme.foreground,
        );

        let icon = if *self.open {
            ICON_EXPAND_LESS
        } else {
            ICON_EXPAND_MORE
        };
        let icon_dur = Animations::duration(ui.ctx(), 0.15);
        let _ = ui.ctx().animate_bool_with_time(self.id, *self.open, icon_dur);
        ui.painter().text(
            egui::Pos2::new(header_rect.right() - 20.0, header_rect.center().y),
            egui::Align2::CENTER_CENTER,
            icon,
            crate::icon_font_id(18.0),
            theme.muted_foreground,
        );

        // Slide the body open/closed by clipping it to an animated height.
        // The natural content height is measured each frame and reused next
        // frame to drive the reveal — a one-frame lag that is imperceptible.
        let body_dur = Animations::duration(ui.ctx(), 0.2);
        let openness = ui
            .ctx()
            .animate_bool_with_time(self.id.with("body"), *self.open, body_dur);
        if openness > 0.0 {
            let h_id = self.id.with("body_h");
            let full_h = ui.ctx().data(|d| d.get_temp::<f32>(h_id).unwrap_or(0.0));
            let width = ui.available_width();
            let visible_h = (full_h * openness).max(0.0);

            // Reserve only the currently-visible (clipped) height in the layout.
            let (window, _) = ui.allocate_exact_size(Vec2::new(width, visible_h), Sense::hover());

            // Render the full body into a child clipped to the visible window so
            // it slides into view instead of popping in at full height.
            let content_rect = Rect::from_min_size(window.min, Vec2::new(width, full_h.max(1.0)));
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(content_rect)
                    .layout(egui::Layout::top_down(egui::Align::Min)),
            );
            child.set_clip_rect(window.intersect(ui.clip_rect()));

            let inner = Frame::new()
                .fill(Color32::TRANSPARENT)
                .inner_margin(Margin {
                    left: 0,
                    right: 0,
                    top: 4,
                    bottom: 16,
                })
                .show(&mut child, content);

            ui.ctx()
                .data_mut(|d| d.insert_temp(h_id, inner.response.rect.height()));
        }
    }
}
