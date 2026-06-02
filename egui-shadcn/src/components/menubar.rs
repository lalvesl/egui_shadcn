use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Sense, Stroke, Ui, Vec2};

pub struct MenubarItem<'a> {
    pub label: &'a str,
    pub items: &'a [MenubarMenuItem<'a>],
}

pub enum MenubarMenuItem<'a> {
    Item {
        label: &'a str,
        shortcut: Option<&'a str>,
        disabled: bool,
    },
    Separator,
}

pub struct Menubar<'a> {
    menus: &'a [MenubarItem<'a>],
}

impl<'a> Menubar<'a> {
    pub fn new(menus: &'a [MenubarItem<'a>]) -> Self {
        Self { menus }
    }

    pub fn show(self, ui: &mut Ui) -> Option<(usize, usize)> {
        let theme = ShadcnTheme::get(ui.ctx());

        // Shared open-index: which top-level menu is open (None = all closed)
        let open_key = egui::Id::new("shadcn_menubar_open");
        let open_idx: Option<usize> = ui.ctx().data(|d| d.get_temp(open_key));

        let trigger_height = 32.0;
        let h_padding = 12.0;
        let v_padding = 4.0;
        let label_font = egui::FontId::new(14.0, egui::FontFamily::Proportional);
        let shortcut_font = egui::FontId::new(12.0, egui::FontFamily::Proportional);

        let mut result: Option<(usize, usize)> = None;
        // We'll collect the trigger rects for each menu to position dropdowns
        let mut trigger_rects: Vec<egui::Rect> = Vec::with_capacity(self.menus.len());

        // Draw the horizontal trigger row
        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            for (menu_idx, menu) in self.menus.iter().enumerate() {
                let is_open = open_idx == Some(menu_idx);

                // Measure label to size trigger
                let galley = ui.painter().layout_no_wrap(
                    menu.label.to_string(),
                    label_font.clone(),
                    Color32::TRANSPARENT,
                );
                let trigger_width = galley.rect.width() + h_padding * 2.0;

                let (trig_rect, trig_resp) = ui
                    .allocate_exact_size(Vec2::new(trigger_width, trigger_height), Sense::click());

                trigger_rects.push(trig_rect);

                let bg = if is_open || trig_resp.hovered() {
                    theme.accent
                } else {
                    Color32::TRANSPARENT
                };
                let fg = if is_open || trig_resp.hovered() {
                    theme.accent_foreground
                } else {
                    theme.foreground
                };

                ui.painter()
                    .rect_filled(trig_rect, CornerRadius::same(theme.radius as u8), bg);
                ui.painter().text(
                    egui::Pos2::new(trig_rect.center().x, trig_rect.center().y),
                    egui::Align2::CENTER_CENTER,
                    menu.label,
                    label_font.clone(),
                    fg,
                );

                if trig_resp.clicked() {
                    let new_open = if is_open { None } else { Some(menu_idx) };
                    ui.ctx().data_mut(|d| d.insert_temp(open_key, new_open));
                }
                if trig_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
            }
        });

        // Draw the open dropdown (if any)
        if let Some(open_menu_idx) = open_idx
            && open_menu_idx < self.menus.len()
            && open_menu_idx < trigger_rects.len()
        {
            let menu = &self.menus[open_menu_idx];
            let trig_rect = trigger_rects[open_menu_idx];

            let popup_id = egui::Id::new("shadcn_menubar_popup").with(open_menu_idx);
            let pos = trig_rect.left_bottom() + egui::Vec2::new(0.0, v_padding);

            // Calculate popup width: max of trigger width, item label widths, etc.
            let dropdown_width = 200.0_f32.max(trig_rect.width());
            let item_height = 32.0;

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
                            ui.set_min_width(dropdown_width);
                            ui.set_max_width(dropdown_width);

                            for menu_item in menu.items.iter() {
                                match menu_item {
                                    MenubarMenuItem::Separator => {
                                        let (sep_rect, _) = ui.allocate_exact_size(
                                            Vec2::new(dropdown_width - 8.0, 1.0),
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
                                    MenubarMenuItem::Item {
                                        label,
                                        shortcut,
                                        disabled,
                                    } => {
                                        let item_idx = logical_idx;
                                        logical_idx += 1;

                                        let sense = if *disabled {
                                            Sense::hover()
                                        } else {
                                            Sense::click()
                                        };

                                        let (item_rect, item_resp) = ui.allocate_exact_size(
                                            Vec2::new(dropdown_width - 8.0, item_height),
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

                                        // Label
                                        ui.painter().text(
                                            egui::Pos2::new(
                                                item_rect.left() + 8.0,
                                                item_rect.center().y,
                                            ),
                                            egui::Align2::LEFT_CENTER,
                                            *label,
                                            label_font.clone(),
                                            text_col,
                                        );

                                        // Shortcut hint (right-aligned)
                                        if let Some(sc) = shortcut {
                                            ui.painter().text(
                                                egui::Pos2::new(
                                                    item_rect.right() - 8.0,
                                                    item_rect.center().y,
                                                ),
                                                egui::Align2::RIGHT_CENTER,
                                                *sc,
                                                shortcut_font.clone(),
                                                theme.muted_foreground,
                                            );
                                        }

                                        if item_resp.clicked() {
                                            result = Some((open_menu_idx, item_idx));
                                            ui.ctx().data_mut(|d| {
                                                d.insert_temp::<Option<usize>>(open_key, None)
                                            });
                                        }
                                    }
                                }
                            }
                        });
                });

            // Close on click outside
            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover_pos = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());

                let clicked_on_trigger = trigger_rects.iter().any(|r| r.contains(hover_pos));
                let popup_approx = egui::Rect::from_min_size(
                    pos,
                    Vec2::new(
                        dropdown_width + 8.0,
                        menu.items.len() as f32 * item_height + 16.0,
                    ),
                );
                let clicked_in_popup = popup_approx.contains(hover_pos);

                if !clicked_on_trigger && !clicked_in_popup {
                    ui.ctx()
                        .data_mut(|d| d.insert_temp::<Option<usize>>(open_key, None));
                }
            }
        }

        result
    }
}
