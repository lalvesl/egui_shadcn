use super::spacing::Spacing;
use crate::{ShadcnTheme, ICON_ARROW_DOWNWARD, ICON_ARROW_UPWARD, ICON_SEARCH};
use egui::{Color32, CornerRadius, Frame, Margin, Pos2, Rect, Sense, Stroke, Ui, Vec2};

pub struct DataColumn<'a> {
    pub header: &'a str,
    pub width: Option<f32>,
    pub sortable: bool,
}

#[derive(Clone, Copy, PartialEq)]
pub enum SortDir {
    Asc,
    Desc,
}

pub struct DataTable<'a> {
    id: &'a str,
    columns: &'a [DataColumn<'a>],
    striped: bool,
    filter: &'a mut String,
    page_size: usize,
}

impl<'a> DataTable<'a> {
    pub fn new(
        id: &'a str,
        columns: &'a [DataColumn<'a>],
        filter: &'a mut String,
    ) -> Self {
        Self {
            id,
            columns,
            striped: true,
            filter,
            page_size: 0,
        }
    }

    pub fn striped(mut self, s: bool) -> Self {
        self.striped = s;
        self
    }

    pub fn page_size(mut self, n: usize) -> Self {
        self.page_size = n;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        rows: usize,
        mut body_fn: impl FnMut(usize, &mut DataTableRow<'_>),
    ) -> Option<(usize, SortDir)> {
        let theme = ShadcnTheme::get(ui.ctx());
        let base_id = egui::Id::new(self.id);
        let sort_id = base_id.with("sort");
        let page_id = base_id.with("page");

        // State
        let mut sort_state: Option<(usize, SortDir)> =
            ui.ctx().data(|d| d.get_temp(sort_id));
        let mut current_page: usize = ui
            .ctx()
            .data(|d| d.get_temp::<usize>(page_id).unwrap_or(0));

        let mut sort_changed: Option<(usize, SortDir)> = None;

        // Filter bar
        ui.horizontal(|ui| {
            let icon_w = 20.0;
            ui.painter().text(
                ui.cursor().left_top() + Vec2::new(0.0, 16.0),
                egui::Align2::LEFT_CENTER,
                ICON_SEARCH,
                ShadcnTheme::icon_font(16.0),
                theme.muted_foreground,
            );
            ui.add_space(icon_w);
            let te = egui::TextEdit::singleline(self.filter)
                .hint_text("Filter…")
                .font(ShadcnTheme::body_font())
                .text_color(theme.foreground)
                .desired_width(200.0);
            ui.add(te);
        });
        ui.add_space(6.0);

        // Column widths
        let available_width = ui.available_width();
        let fixed_total: f32 = self.columns.iter().filter_map(|c| c.width).sum();
        let flex_count = self.columns.iter().filter(|c| c.width.is_none()).count();
        let flex_width = if flex_count > 0 {
            ((available_width - fixed_total) / flex_count as f32).max(40.0)
        } else {
            0.0
        };
        let col_widths: Vec<f32> = self
            .columns
            .iter()
            .map(|c| c.width.unwrap_or(flex_width))
            .collect();
        let total_width: f32 = col_widths.iter().sum();

        let header_height = 40.0f32;
        let row_height = 44.0f32;

        // Pagination calculations
        let (start_row, visible_rows) = if self.page_size > 0 && rows > 0 {
            let total_pages = rows.div_ceil(self.page_size);
            current_page = current_page.min(total_pages.saturating_sub(1));
            let start = current_page * self.page_size;
            let count = self.page_size.min(rows.saturating_sub(start));
            (start, count)
        } else {
            (0, rows)
        };

        let table_height = header_height + visible_rows as f32 * row_height;
        let (table_rect, _) =
            ui.allocate_exact_size(Vec2::new(total_width, table_height), Sense::hover());

        // Background
        ui.painter()
            .rect_filled(table_rect, CornerRadius::ZERO, theme.card);

        // Header row
        let header_rect =
            Rect::from_min_size(table_rect.min, Vec2::new(total_width, header_height));
        ui.painter()
            .rect_filled(header_rect, CornerRadius::ZERO, theme.muted);

        let mut col_x = table_rect.left();
        for (ci, col) in self.columns.iter().enumerate() {
            let cw = col_widths[ci];
            let cell_rect =
                Rect::from_min_size(Pos2::new(col_x, header_rect.top()), Vec2::new(cw, header_height));

            let text_x = cell_rect.left() + 12.0;
            ui.painter().text(
                Pos2::new(text_x, cell_rect.center().y),
                egui::Align2::LEFT_CENTER,
                col.header,
                egui::FontId::new(13.0, egui::FontFamily::Proportional),
                theme.muted_foreground,
            );

            // Sort icon
            if col.sortable {
                let is_sorted = sort_state.map(|(sc, _)| sc == ci).unwrap_or(false);
                let sort_icon = match sort_state {
                    Some((sc, SortDir::Asc)) if sc == ci => ICON_ARROW_UPWARD,
                    Some((sc, SortDir::Desc)) if sc == ci => ICON_ARROW_DOWNWARD,
                    _ => ICON_ARROW_DOWNWARD,
                };
                let icon_color = if is_sorted {
                    theme.foreground
                } else {
                    ShadcnTheme::with_alpha(theme.muted_foreground, 80)
                };
                let icon_x = cell_rect.right() - 20.0;
                let icon_rect =
                    Rect::from_center_size(Pos2::new(icon_x, cell_rect.center().y), Vec2::splat(20.0));
                let icon_resp = ui.interact(icon_rect, base_id.with(("sort_btn", ci)), Sense::click());
                if icon_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }
                ui.painter().text(
                    Pos2::new(icon_x, cell_rect.center().y),
                    egui::Align2::CENTER_CENTER,
                    sort_icon,
                    ShadcnTheme::icon_font(14.0),
                    icon_color,
                );
                if icon_resp.clicked() {
                    let new_dir = match sort_state {
                        Some((sc, SortDir::Asc)) if sc == ci => SortDir::Desc,
                        _ => SortDir::Asc,
                    };
                    sort_state = Some((ci, new_dir));
                    sort_changed = Some((ci, new_dir));
                    ui.ctx().data_mut(|d| d.insert_temp(sort_id, sort_state));
                }

                // Also allow clicking on full header cell
                let header_resp = ui.interact(cell_rect, base_id.with(("hdr_click", ci)), Sense::click());
                if header_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    ui.painter().rect_filled(
                        cell_rect,
                        CornerRadius::ZERO,
                        ShadcnTheme::with_alpha(theme.accent, 60),
                    );
                }
                if header_resp.clicked() {
                    let new_dir = match sort_state {
                        Some((sc, SortDir::Asc)) if sc == ci => SortDir::Desc,
                        _ => SortDir::Asc,
                    };
                    sort_state = Some((ci, new_dir));
                    sort_changed = Some((ci, new_dir));
                    ui.ctx().data_mut(|d| d.insert_temp(sort_id, sort_state));
                }
            }

            // Column divider
            if ci < self.columns.len() - 1 {
                ui.painter().vline(
                    cell_rect.right(),
                    cell_rect.y_range(),
                    Stroke::new(1.0, theme.border),
                );
            }

            col_x += cw;
        }

        // Header bottom border
        ui.painter().hline(
            table_rect.x_range(),
            header_rect.bottom(),
            Stroke::new(1.0, theme.border),
        );

        // Body rows
        for display_idx in 0..visible_rows {
            let logical_idx = start_row + display_idx;
            let row_top = table_rect.top() + header_height + display_idx as f32 * row_height;
            let row_rect = Rect::from_min_size(
                Pos2::new(table_rect.left(), row_top),
                Vec2::new(total_width, row_height),
            );

            let row_bg = if self.striped && display_idx % 2 == 1 {
                theme.muted
            } else {
                theme.card
            };
            ui.painter()
                .rect_filled(row_rect, CornerRadius::ZERO, row_bg);

            let mut data_row = DataTableRow {
                ui,
                col_widths: col_widths.clone(),
                col_idx: 0,
                row_height,
                x: table_rect.left(),
                row_rect,
            };

            body_fn(logical_idx, &mut data_row);

            ui.painter().hline(
                row_rect.x_range(),
                row_rect.bottom(),
                Stroke::new(1.0, theme.border),
            );
        }

        // Table border
        ui.painter().rect_stroke(
            table_rect,
            CornerRadius::ZERO,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        // Pagination bar
        if self.page_size > 0 && rows > 0 {
            let total_pages = rows.div_ceil(self.page_size);
            ui.add_space(6.0);

            Frame::new()
                .inner_margin(Margin::symmetric(8, 4))
                .show(ui, |ui| {
                    ui.horizontal(|ui| {
                        // Prev button
                        let can_prev = current_page > 0;
                        let prev_resp = ui.add_enabled(
                            can_prev,
                            egui::Button::new(
                                egui::RichText::new("← Prev")
                                    .font(ShadcnTheme::small_font()),
                            )
                            .fill(if can_prev {
                                theme.secondary
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(Stroke::new(1.0, theme.border)),
                        );
                        if prev_resp.clicked() && can_prev {
                            current_page -= 1;
                            ui.ctx().data_mut(|d| d.insert_temp(page_id, current_page));
                        }

                        Spacing::Sm.show(ui);
                        ui.label(
                            egui::RichText::new(format!(
                                "Page {} of {}",
                                current_page + 1,
                                total_pages
                            ))
                            .font(ShadcnTheme::small_font())
                            .color(theme.muted_foreground),
                        );
                        Spacing::Sm.show(ui);

                        // Next button
                        let can_next = current_page + 1 < total_pages;
                        let next_resp = ui.add_enabled(
                            can_next,
                            egui::Button::new(
                                egui::RichText::new("Next →")
                                    .font(ShadcnTheme::small_font()),
                            )
                            .fill(if can_next {
                                theme.secondary
                            } else {
                                Color32::TRANSPARENT
                            })
                            .stroke(Stroke::new(1.0, theme.border)),
                        );
                        if next_resp.clicked() && can_next {
                            current_page += 1;
                            ui.ctx().data_mut(|d| d.insert_temp(page_id, current_page));
                        }
                    });
                });
        }

        sort_changed
    }
}

pub struct DataTableRow<'a> {
    pub ui: &'a mut egui::Ui,
    pub col_widths: Vec<f32>,
    pub col_idx: usize,
    pub row_height: f32,
    pub x: f32,
    pub row_rect: egui::Rect,
}

impl<'a> DataTableRow<'a> {
    pub fn cell(&mut self, content: impl FnOnce(&mut Ui)) {
        if self.col_idx >= self.col_widths.len() {
            return;
        }
        let cw = self.col_widths[self.col_idx];
        let cell_rect = Rect::from_min_size(
            Pos2::new(self.x, self.row_rect.top()),
            Vec2::new(cw, self.row_height),
        );

        let inner_rect = Rect::from_min_max(
            Pos2::new(cell_rect.left() + 12.0, cell_rect.top() + 2.0),
            Pos2::new(cell_rect.right() - 4.0, cell_rect.bottom() - 2.0),
        );

        let mut child = self.ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
        );
        content(&mut child);

        if self.col_idx < self.col_widths.len() - 1 {
            self.ui.painter().vline(
                cell_rect.right(),
                cell_rect.y_range(),
                Stroke::new(1.0, {
                    let t = ShadcnTheme::get(self.ui.ctx());
                    t.border
                }),
            );
        }

        self.x += cw;
        self.col_idx += 1;
    }
}
