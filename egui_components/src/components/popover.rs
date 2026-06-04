use super::boxed::Boxed;
use crate::ShadcnTheme;
use egui::{Response, Ui};

pub struct Popover {
    id: egui::Id,
    width: f32,
}

impl Popover {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            width: 256.0,
        }
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
        let popup_id = self.id.with("popover_open");
        let width = self.width;

        let trigger_resp = trigger_fn(ui);
        let trigger_rect = trigger_resp.rect;

        let is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        if trigger_resp.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, !is_open));
        }

        if is_open {
            let popup_pos = trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0);

            let area_resp = egui::Area::new(self.id.with("popover_area"))
                .fixed_pos(popup_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Boxed::new()
                        .fill(theme.card)
                        .padding_px(16.0)
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.set_max_width(width);
                            content_fn(ui);
                        });
                });

            let popup_rect = area_resp.response.rect;

            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover_pos = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());
                let in_trigger = trigger_rect.contains(hover_pos);
                let in_popup = popup_rect.contains(hover_pos);
                if !in_trigger && !in_popup {
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }
    }
}
