use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Pos2, Sense, Stroke, Ui, Vec2};

pub enum ContextItem<'a> {
    Item {
        label: &'a str,
        shortcut: Option<&'a str>,
        disabled: bool,
    },
    Separator,
}

pub struct ContextMenu<'a> {
    id: egui::Id,
    items: &'a [ContextItem<'a>],
    width: f32,
}

impl<'a> ContextMenu<'a> {
    pub fn new(id: impl std::hash::Hash, items: &'a [ContextItem<'a>]) -> Self {
        Self {
            id: egui::Id::new(id),
            items,
            width: 200.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        area_fn: impl FnOnce(&mut Ui) -> egui::Response,
    ) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let popup_id = egui::Id::new("shadcn_ctx_menu").with(self.id);
        let pos_id = popup_id.with("pos");

        let target_resp = area_fn(ui);

        if target_resp.secondary_clicked() {
            let click_pos = ui
                .ctx()
                .input(|i| i.pointer.hover_pos().unwrap_or_default());
            ui.ctx().data_mut(|d| {
                d.insert_temp(pos_id, click_pos);
                d.insert_temp(popup_id, true);
            });
        }

        let is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let mut clicked_item: Option<usize> = None;

        if is_open {
            let pos = ui
                .ctx()
                .data(|d| d.get_temp::<Pos2>(pos_id).unwrap_or_default());

            let popup_width = self.width;
            let mut logical_idx = 0usize;

            let area_resp = egui::Area::new(popup_id)
                .fixed_pos(pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Frame::new()
                        .fill(theme.card)
                        .corner_radius(CornerRadius::same(theme.radius as u8))
                        .stroke(Stroke::new(1.0, theme.border))
                        .inner_margin(Margin::same(4))
                        .show(ui, |ui| {
                            ui.set_min_width(popup_width);
                            ui.set_max_width(popup_width);

                            for item in self.items.iter() {
                                match item {
                                    ContextItem::Separator => {
                                        let (sep_rect, _) = ui.allocate_exact_size(
                                            Vec2::new(popup_width - 8.0, 1.0),
                                            Sense::hover(),
                                        );
                                        ui.add_space(3.0);
                                        ui.painter().hline(
                                            sep_rect.x_range(),
                                            sep_rect.top(),
                                            Stroke::new(1.0, theme.border),
                                        );
                                        ui.add_space(3.0);
                                    }
                                    ContextItem::Item {
                                        label,
                                        shortcut,
                                        disabled,
                                    } => {
                                        let idx = logical_idx;
                                        logical_idx += 1;

                                        let sense = if *disabled {
                                            Sense::hover()
                                        } else {
                                            Sense::click()
                                        };

                                        let (item_rect, item_resp) = ui.allocate_exact_size(
                                            Vec2::new(popup_width - 8.0, 32.0),
                                            sense,
                                        );

                                        let bg = if !disabled && item_resp.hovered() {
                                            theme.accent
                                        } else {
                                            Color32::TRANSPARENT
                                        };
                                        let text_col = if *disabled {
                                            theme.muted_foreground
                                        } else if item_resp.hovered() {
                                            theme.accent_foreground
                                        } else {
                                            theme.foreground
                                        };

                                        ui.painter().rect_filled(
                                            item_rect,
                                            CornerRadius::same(4),
                                            bg,
                                        );
                                        ui.painter().text(
                                            egui::Pos2::new(
                                                item_rect.left() + 8.0,
                                                item_rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            *label,
                                            egui::FontId::new(14.0, egui::FontFamily::Proportional),
                                            text_col,
                                        );

                                        if let Some(sc) = shortcut {
                                            ui.painter().text(
                                                egui::Pos2::new(
                                                    item_rect.right() - 8.0,
                                                    item_rect.center().y,
                                                ),
                                                egui::Align2::RIGHT_CENTER,
                                                *sc,
                                                egui::FontId::new(
                                                    12.0,
                                                    egui::FontFamily::Proportional,
                                                ),
                                                theme.muted_foreground,
                                            );
                                        }

                                        if item_resp.clicked() {
                                            clicked_item = Some(idx);
                                            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                                        }
                                    }
                                }
                            }
                        });
                });

            let popup_rect = area_resp.response.rect;

            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());
                if !popup_rect.contains(hover) {
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }

        clicked_item
    }
}
