use super::boxed::Boxed;
use super::size::Size;
use super::spacing::Spacing;
use crate::i18n;
use crate::{ICON_EXPAND_MORE, ShadcnTheme};
use ::i18n::t;
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Combobox<'a> {
    id: &'a str,
    current: &'a mut Option<usize>,
    options: &'a [&'a str],
    placeholder: Option<&'a str>,
    width: Option<f32>,
    size: Size,
}

impl<'a> Combobox<'a> {
    pub fn new(id: &'a str, current: &'a mut Option<usize>, options: &'a [&'a str]) -> Self {
        Self {
            id,
            current,
            options,
            placeholder: None,
            width: None,
            size: Size::Default,
        }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = Some(p);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = self.width.unwrap_or_else(|| ui.available_width());
        let height = self.size.height();

        let popup_id = egui::Id::new("shadcn_combobox").with(self.id);
        let query_id = popup_id.with("query");

        let (trigger_rect, resp) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        let cr = CornerRadius::same(theme.radius as u8);
        let is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let border_color = if resp.hovered() || is_open {
            theme.ring
        } else {
            theme.border
        };

        ui.painter().rect_filled(trigger_rect, cr, theme.background);
        ui.painter().rect_stroke(
            trigger_rect,
            cr,
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        let ph_owned;
        let placeholder: &str = match self.placeholder {
            Some(p) => p,
            None => {
                ph_owned = t!(i18n::Combobox::Placeholder);
                ph_owned.as_ref()
            }
        };
        let display = self.current.map(|i| self.options[i]).unwrap_or(placeholder);
        let text_color = if self.current.is_some() {
            theme.foreground
        } else {
            theme.muted_foreground
        };

        ui.painter().text(
            egui::Pos2::new(trigger_rect.left() + 12.0, trigger_rect.center().y),
            egui::Align2::LEFT_CENTER,
            display,
            egui::FontId::new(self.size.font_size(), egui::FontFamily::Proportional),
            text_color,
        );
        ui.painter().text(
            egui::Pos2::new(trigger_rect.right() - 20.0, trigger_rect.center().y),
            egui::Align2::CENTER_CENTER,
            ICON_EXPAND_MORE,
            crate::icon_font_id(18.0),
            theme.muted_foreground,
        );

        if resp.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, !is_open));
            if !is_open {
                ui.ctx()
                    .data_mut(|d| d.insert_temp::<String>(query_id, String::new()));
            }
        }

        let mut changed = false;

        if is_open {
            let pos = trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0);

            let mut query = ui
                .ctx()
                .data(|d| d.get_temp::<String>(query_id).unwrap_or_default());

            egui::Area::new(popup_id)
                .fixed_pos(pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Boxed::new()
                        .fill(theme.card)
                        .padding(Spacing::Xs)
                        .show(ui, |ui| {
                            ui.set_min_width(width);
                            ui.set_max_width(width);

                            let search_rect_alloc = ui
                                .allocate_exact_size(Vec2::new(width - 8.0, 32.0), Sense::hover());
                            let search_rect = search_rect_alloc.0;

                            ui.painter().rect_filled(
                                search_rect,
                                CornerRadius::same(4),
                                theme.background,
                            );
                            ui.painter().rect_stroke(
                                search_rect,
                                CornerRadius::same(4),
                                Stroke::new(1.0, theme.border),
                                egui::StrokeKind::Inside,
                            );

                            let inner = egui::Rect::from_min_max(
                                egui::Pos2::new(search_rect.left() + 8.0, search_rect.top()),
                                egui::Pos2::new(search_rect.right() - 8.0, search_rect.bottom()),
                            );
                            let search_hint = t!(i18n::Combobox::SearchPlaceholder);
                            let te = egui::TextEdit::singleline(&mut query)
                                .desired_width(inner.width())
                                .hint_text(search_hint.as_ref())
                                .font(egui::FontId::new(
                                    self.size.font_size(),
                                    egui::FontFamily::Proportional,
                                ))
                                .text_color(theme.foreground)
                                .frame(egui::Frame::new());

                            let mut child = ui.new_child(
                                egui::UiBuilder::new()
                                    .max_rect(inner)
                                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
                            );
                            child.add(te);

                            ui.ctx()
                                .data_mut(|d| d.insert_temp(query_id, query.clone()));

                            let q_lower = query.to_lowercase();

                            egui::ScrollArea::vertical()
                                .max_height(200.0)
                                .show(ui, |ui| {
                                    for (i, opt) in self.options.iter().enumerate() {
                                        if !q_lower.is_empty()
                                            && !opt.to_lowercase().contains(&q_lower)
                                        {
                                            continue;
                                        }

                                        let is_sel = *self.current == Some(i);

                                        let (item_rect, item_resp) = ui.allocate_exact_size(
                                            Vec2::new(width - 8.0, 32.0),
                                            Sense::click(),
                                        );

                                        let bg = if is_sel || item_resp.hovered() {
                                            theme.accent
                                        } else {
                                            Color32::TRANSPARENT
                                        };
                                        let text_col = if is_sel {
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
                                            *opt,
                                            egui::FontId::new(
                                                self.size.font_size(),
                                                egui::FontFamily::Proportional,
                                            ),
                                            text_col,
                                        );

                                        if item_resp.clicked() {
                                            *self.current = Some(i);
                                            ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                                            changed = true;
                                        }
                                    }
                                });
                        });
                });

            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());
                let popup_approx = egui::Rect::from_min_size(
                    trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0),
                    Vec2::new(width + 8.0, 260.0),
                );
                if !trigger_rect.contains(hover) && !popup_approx.contains(hover) {
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }

        changed
    }
}
