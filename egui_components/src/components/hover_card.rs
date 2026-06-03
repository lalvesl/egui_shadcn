use crate::ShadcnTheme;
use egui::{CornerRadius, Frame, Margin, Response, Stroke, Ui};

pub struct HoverCard {
    id: egui::Id,
    delay_frames: u32,
    width: f32,
}

impl HoverCard {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            delay_frames: 20,
            width: 320.0,
        }
    }

    pub fn delay_frames(mut self, f: u32) -> Self {
        self.delay_frames = f;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        trigger_fn: impl FnOnce(&mut Ui) -> Response,
        content_fn: impl FnOnce(&mut Ui),
    ) {
        let theme = ShadcnTheme::get(ui.ctx());
        let hover_count_id = self.id.with("hover_count");
        let width = self.width;
        let delay = self.delay_frames;

        let trigger_resp = trigger_fn(ui);
        let trigger_rect = trigger_resp.rect;

        let hover_count: u32 = ui
            .ctx()
            .data(|d| d.get_temp::<u32>(hover_count_id).unwrap_or(0));

        if trigger_resp.hovered() {
            let new_count = hover_count.saturating_add(1);
            ui.ctx()
                .data_mut(|d| d.insert_temp(hover_count_id, new_count));
            ui.ctx().request_repaint();
        } else {
            if hover_count > 0 {
                ui.ctx().data_mut(|d| d.insert_temp(hover_count_id, 0u32));
            }
        }

        let should_show = hover_count >= delay;

        if should_show {
            let popup_pos = trigger_rect.right_bottom() + egui::Vec2::new(4.0, 4.0);

            egui::Area::new(self.id.with("hover_card_area"))
                .fixed_pos(popup_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Frame::new()
                        .fill(theme.card)
                        .corner_radius(CornerRadius::same(theme.radius as u8))
                        .stroke(Stroke::new(1.0, theme.border))
                        .inner_margin(Margin::same(16))
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.set_max_width(width);
                            content_fn(ui);
                        });
                });
        }
    }
}
