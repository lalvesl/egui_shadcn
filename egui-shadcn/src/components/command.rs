use super::button::{Button, ButtonSize, ButtonVariant};
use super::card::Card;
use super::input::Input;
use super::separator::Separator;
use super::spacing::Spacing;
use super::typography::{muted_text, small_text};
use crate::i18n;
use crate::{ICON_SEARCH, ShadcnTheme};
use ::i18n::t;
use egui::{Color32, CornerRadius, Frame, Key, Margin, Pos2, Sense, Vec2};

pub struct CommandItem<'a> {
    pub label: &'a str,
    pub description: Option<&'a str>,
    pub shortcut: Option<&'a str>,
    pub icon: Option<&'a str>,
}

pub struct CommandGroup<'a> {
    pub heading: Option<&'a str>,
    pub items: &'a [CommandItem<'a>],
}

pub struct Command<'a> {
    id: &'a str,
    groups: &'a [CommandGroup<'a>],
    open: &'a mut bool,
    placeholder: Option<&'a str>,
    width: f32,
}

impl<'a> Command<'a> {
    pub fn new(id: &'a str, groups: &'a [CommandGroup<'a>], open: &'a mut bool) -> Self {
        Self {
            id,
            groups,
            open,
            placeholder: None,
            width: 480.0,
        }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = Some(p);
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(self, ctx: &egui::Context) -> Option<(usize, usize)> {
        if !*self.open {
            return None;
        }

        let theme = ShadcnTheme::get(ctx);
        let base_id = egui::Id::new(self.id);
        let query_id = base_id.with("query");
        let sel_id = base_id.with("selected");

        let mut query: String = ctx
            .data(|d| d.get_temp::<String>(query_id).clone())
            .unwrap_or_default();
        let mut selected_idx: usize = ctx.data(|d| d.get_temp::<usize>(sel_id).unwrap_or(0));

        let filtered: Vec<(usize, usize)> = self
            .groups
            .iter()
            .enumerate()
            .flat_map(|(gi, g)| {
                g.items
                    .iter()
                    .enumerate()
                    .filter(|(_, item)| {
                        query.is_empty()
                            || item.label.to_lowercase().contains(&query.to_lowercase())
                    })
                    .map(move |(ii, _)| (gi, ii))
            })
            .collect();

        let total = filtered.len();
        if selected_idx >= total && total > 0 {
            selected_idx = total - 1;
        }

        let mut result: Option<(usize, usize)> = None;
        let mut close = false;

        ctx.input(|i| {
            if i.key_pressed(Key::Escape) {
                close = true;
            }
            if i.key_pressed(Key::ArrowDown) && total > 0 {
                selected_idx = (selected_idx + 1) % total;
            }
            if i.key_pressed(Key::ArrowUp) && total > 0 {
                selected_idx = if selected_idx == 0 {
                    total - 1
                } else {
                    selected_idx - 1
                };
            }
            if i.key_pressed(Key::Enter) && total > 0 {
                let (gi, ii) = filtered[selected_idx];
                result = Some((gi, ii));
                close = true;
            }
        });

        if close {
            *self.open = false;
            ctx.data_mut(|d| {
                d.insert_temp(query_id, String::new());
                d.insert_temp(sel_id, 0usize);
            });
            return result;
        }

        // Dim overlay
        let screen = ctx.content_rect();
        egui::Area::new(base_id.with("overlay"))
            .fixed_pos(screen.min)
            .order(egui::Order::Background)
            .interactable(false)
            .show(ctx, |ui| {
                let (r, _) = ui.allocate_exact_size(screen.size(), Sense::hover());
                ui.painter().rect_filled(
                    r,
                    egui::CornerRadius::ZERO,
                    Color32::from_black_alpha(160),
                );
            });

        let dialog_max_h = 400.0;
        let search_h = 44.0;

        egui::Area::new(base_id.with("dialog"))
            .fixed_pos(Pos2::new(
                screen.center().x - self.width / 2.0,
                screen.center().y - dialog_max_h / 2.0,
            ))
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                Card::new()
                    .padding(0.0)
                    .shadow(egui::Shadow {
                        offset: [0, 8],
                        blur: 24,
                        spread: 0,
                        color: Color32::from_black_alpha(80),
                    })
                    .show(ui, |ui| {
                        ui.set_min_width(self.width);
                        ui.set_max_width(self.width);

                        // Search row
                        Spacing::Xs.show(ui);
                        let ph_owned;
                        let ph: &str = match self.placeholder {
                            Some(p) => p,
                            None => {
                                ph_owned = t!(i18n::Command::Placeholder);
                                ph_owned.as_ref()
                            }
                        };
                        ui.horizontal(|ui| {
                            Input::new(&mut query)
                                .icon_left(ICON_SEARCH)
                                .placeholder(ph)
                                .bordered(false)
                                .width(self.width - 44.0)
                                .show(ui)
                                .request_focus();

                            let close_btn = Button::new("")
                                .variant(ButtonVariant::Ghost)
                                .size(ButtonSize::Icon)
                                .show(ui);
                            if close_btn.clicked() {
                                close = true;
                            }
                        });
                        Spacing::Xs.show(ui);

                        Separator::horizontal().show(ui);

                        // Results
                        egui::ScrollArea::vertical()
                            .max_height(dialog_max_h - search_h - 16.0)
                            .show(ui, |ui| {
                                Spacing::Xs.show(ui);

                                Frame::new()
                                    .inner_margin(Margin::symmetric(4, 0))
                                    .show(ui, |ui| {
                                        let mut flat_idx = 0usize;

                                        for (gi, group) in self.groups.iter().enumerate() {
                                            let group_items: Vec<(usize, &CommandItem)> = group
                                                .items
                                                .iter()
                                                .enumerate()
                                                .filter(|(_, item)| {
                                                    query.is_empty()
                                                        || item
                                                            .label
                                                            .to_lowercase()
                                                            .contains(&query.to_lowercase())
                                                })
                                                .collect();

                                            if group_items.is_empty() {
                                                continue;
                                            }

                                            if let Some(heading) = group.heading {
                                                Spacing::Xs.show(ui);
                                                ui.horizontal(|ui| {
                                                    ui.add_space(8.0);
                                                    small_text(ui, heading);
                                                });
                                                ui.add_space(2.0);
                                            }

                                            for (ii, item) in group_items {
                                                let is_selected = flat_idx == selected_idx;
                                                let (item_rect, item_resp) = ui
                                                    .allocate_exact_size(
                                                        Vec2::new(ui.available_width(), 36.0),
                                                        Sense::click(),
                                                    );
                                                let bg = if is_selected || item_resp.hovered() {
                                                    theme.accent
                                                } else {
                                                    Color32::TRANSPARENT
                                                };
                                                ui.painter().rect_filled(
                                                    item_rect,
                                                    CornerRadius::same(4),
                                                    bg,
                                                );

                                                let mut tx = item_rect.left() + 10.0;

                                                if let Some(icon) = item.icon {
                                                    ui.painter().text(
                                                        Pos2::new(tx, item_rect.center().y),
                                                        egui::Align2::LEFT_CENTER,
                                                        icon,
                                                        ShadcnTheme::icon_font(16.0),
                                                        theme.foreground,
                                                    );
                                                    tx += 22.0;
                                                }

                                                ui.painter().text(
                                                    Pos2::new(tx, item_rect.center().y),
                                                    egui::Align2::LEFT_CENTER,
                                                    item.label,
                                                    ShadcnTheme::body_font(),
                                                    theme.foreground,
                                                );

                                                if let Some(desc) = item.description {
                                                    ui.painter().text(
                                                        Pos2::new(
                                                            item_rect.right() - 8.0,
                                                            item_rect.center().y,
                                                        ),
                                                        egui::Align2::RIGHT_CENTER,
                                                        desc,
                                                        ShadcnTheme::small_font(),
                                                        theme.muted_foreground,
                                                    );
                                                }

                                                if let Some(sc) = item.shortcut {
                                                    let sc_x = if item.description.is_some() {
                                                        item_rect.right() - 80.0
                                                    } else {
                                                        item_rect.right() - 8.0
                                                    };
                                                    ui.painter().text(
                                                        Pos2::new(sc_x, item_rect.center().y),
                                                        egui::Align2::RIGHT_CENTER,
                                                        sc,
                                                        ShadcnTheme::small_font(),
                                                        theme.muted_foreground,
                                                    );
                                                }

                                                if item_resp.clicked() {
                                                    result = Some((gi, ii));
                                                    close = true;
                                                }
                                                if item_resp.hovered() {
                                                    selected_idx = flat_idx;
                                                }
                                                flat_idx += 1;
                                            }
                                        }

                                        if flat_idx == 0 {
                                            let no_results = t!(i18n::Command::NoResults);
                                            ui.add_space(20.0);
                                            ui.centered_and_justified(|ui| {
                                                muted_text(ui, no_results.as_ref());
                                            });
                                            ui.add_space(20.0);
                                        }
                                    });

                                Spacing::Xs.show(ui);
                            });
                    });
            });

        ctx.data_mut(|d| {
            d.insert_temp(query_id, query);
            d.insert_temp(sel_id, selected_idx);
        });

        if close {
            *self.open = false;
            ctx.data_mut(|d| {
                d.insert_temp(query_id, String::new());
                d.insert_temp(sel_id, 0usize);
            });
        }

        result
    }
}
