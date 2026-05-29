use super::size::Size;
use crate::{ICON_EXPAND_MORE, ShadcnTheme};
use egui::{Color32, CornerRadius, Frame, Margin, Sense, Stroke, Ui, Vec2};

pub enum DropdownItem<'a> {
    Item { label: &'a str, disabled: bool },
    Separator,
}

pub struct DropdownMenu<'a> {
    id: &'a str,
    trigger_label: &'a str,
    items: &'a [DropdownItem<'a>],
    width: f32,
    size: Size,
}

impl<'a> DropdownMenu<'a> {
    pub fn new(id: &'a str, trigger_label: &'a str, items: &'a [DropdownItem<'a>]) -> Self {
        Self { id, trigger_label, items, width: 200.0, size: Size::Default }
    }

    pub fn width(mut self, w: f32) -> Self { self.width = w; self }
    pub fn size(mut self, s: Size) -> Self { self.size = s; self }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let popup_id = egui::Id::new("shadcn_dropdown_menu").with(self.id);

        let trigger_height = self.size.height();
        let (trigger_rect, resp) =
            ui.allocate_exact_size(Vec2::new(self.width, trigger_height), Sense::click());

        let is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let cr = CornerRadius::same(theme.radius as u8);
        let bg = if resp.hovered() || is_open {
            theme.accent
        } else {
            theme.secondary
        };
        let fg = if resp.hovered() || is_open {
            theme.accent_foreground
        } else {
            theme.secondary_foreground
        };

        ui.painter().rect_filled(trigger_rect, cr, bg);
        ui.painter().rect_stroke(
            trigger_rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        ui.painter().text(
            egui::Pos2::new(trigger_rect.left() + 12.0, trigger_rect.center().y),
            egui::Align2::LEFT_CENTER,
            self.trigger_label,
            egui::FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
            fg,
        );
        ui.painter().text(
            egui::Pos2::new(trigger_rect.right() - 20.0, trigger_rect.center().y),
            egui::Align2::CENTER_CENTER,
            ICON_EXPAND_MORE,
            crate::icon_font_id(18.0),
            fg,
        );

        if resp.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, !is_open));
        }

        let mut clicked_item: Option<usize> = None;

        if is_open {
            let pos = trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0);
            let popup_width = self.width;

            let mut item_count = 0usize;
            let mut logical_idx = 0usize;

            egui::Area::new(popup_id)
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
                                    DropdownItem::Separator => {
                                        let (sep_rect, _) = ui.allocate_exact_size(
                                            Vec2::new(popup_width - 8.0, 1.0),
                                            Sense::hover(),
                                        );
                                        ui.add_space(3.0);
                                        ui.painter()
                                            .hline(sep_rect.x_range(), sep_rect.top(), Stroke::new(1.0, theme.border));
                                        ui.add_space(3.0);
                                    }
                                    DropdownItem::Item { label, disabled } => {
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
                                            egui::FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
                                            text_col,
                                        );

                                        if item_resp.clicked() {
                                            clicked_item = Some(idx);
                                            ui.ctx()
                                                .data_mut(|d| d.insert_temp(popup_id, false));
                                        }

                                        item_count += 1;
                                    }
                                }
                            }
                        });
                });

            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());
                let popup_approx = egui::Rect::from_min_size(
                    trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0),
                    Vec2::new(self.width + 8.0, item_count as f32 * 32.0 + 16.0),
                );
                if !trigger_rect.contains(hover) && !popup_approx.contains(hover) {
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }

        clicked_item
    }
}
