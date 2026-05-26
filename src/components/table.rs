/// Full-featured data table: filter, multi-select checkboxes, sortable columns, pagination.
use egui::{Color32, Margin, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;
use crate::components::input::TextInput;

// ─── Column definition ─────────────────────────────────────────────────────

pub struct TableColumn {
    pub header:   String,
    pub sortable: bool,
}

impl TableColumn {
    pub fn new(header: impl Into<String>) -> Self {
        Self { header: header.into(), sortable: true }
    }
    #[allow(dead_code)]
    pub fn sortable(mut self, v: bool) -> Self { self.sortable = v; self }
}

// ─── Persistent state ──────────────────────────────────────────────────────

#[derive(Clone, Default)]
pub struct DataTableState {
    pub filter:   String,
    pub selected: Vec<bool>,
    pub sort_col: Option<usize>,
    pub sort_asc: bool,
    pub page:     usize,
    col_open:     bool,
    col_vis:      Vec<bool>,
}

impl DataTableState {
    pub fn new(num_rows: usize, num_cols: usize) -> Self {
        Self {
            sort_asc: true,
            selected: vec![false; num_rows],
            col_vis:  vec![true; num_cols],
            ..Default::default()
        }
    }

    #[allow(dead_code)]
    pub fn selected_count(&self) -> usize {
        self.selected.iter().filter(|&&b| b).count()
    }
}

// ─── Widget ────────────────────────────────────────────────────────────────

pub struct DataTable<'a> {
    columns:      &'a [TableColumn],
    rows:         &'a [Vec<String>],
    state:        &'a mut DataTableState,
    page_size:    usize,
    filter_hint:  &'a str,
}

impl<'a> DataTable<'a> {
    pub fn new(
        columns: &'a [TableColumn],
        rows:    &'a [Vec<String>],
        state:   &'a mut DataTableState,
    ) -> Self {
        Self { columns, rows, state, page_size: 5, filter_hint: "Filter..." }
    }

    pub fn page_size(mut self, n: usize)     -> Self { self.page_size = n; self }
    pub fn filter_hint(mut self, h: &'a str) -> Self { self.filter_hint = h; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        // Sync lengths
        self.state.selected.resize(self.rows.len(), false);
        self.state.col_vis.resize(self.columns.len(), true);

        // ── Filter input + Columns button ──────────────────────────────
        let avail_w = ui.available_width();

        ui.horizontal(|ui| {
            // Filter — reuse TextInput for consistent styling and vertical centering
            let inp_w = (avail_w - 120.0).max(100.0);
            ui.allocate_ui(Vec2::new(inp_w, 36.0), |ui| {
                TextInput::new(&mut self.state.filter)
                    .placeholder(self.filter_hint)
                    .show(ui);
            });

            ui.add_space(8.0);

            // Columns toggle button
            let col_btn = ui.add(
                egui::Button::new(
                    egui::RichText::new("Columns ▾").size(13.0).color(theme.foreground),
                )
                .min_size(Vec2::new(96.0, 32.0))
                .fill(theme.background)
                .stroke(Stroke::new(1.0, theme.border)),
            );
            if col_btn.clicked() {
                self.state.col_open = !self.state.col_open;
            }

            // Columns popup
            if self.state.col_open {
                let popup_id = egui::Id::new("__dt_col_popup__");
                egui::Area::new(popup_id)
                    .fixed_pos(col_btn.rect.left_bottom() + egui::vec2(0.0, 4.0))
                    .order(egui::Order::Foreground)
                    .show(ui.ctx(), |ui| {
                        egui::Frame::new()
                            .fill(theme.card)
                            .stroke(Stroke::new(1.0, theme.border))
                            .corner_radius(theme.radius)
                            .inner_margin(Margin::same(8))
                            .show(ui, |ui| {
                                ui.set_width(160.0);
                                for (i, col) in self.columns.iter().enumerate() {
                                    let vis = &mut self.state.col_vis[i];
                                    ui.horizontal(|ui| {
                                        let (r, resp) = ui.allocate_exact_size(Vec2::splat(16.0), Sense::click());
                                        if resp.clicked() { *vis = !*vis; }
                                        draw_checkbox(ui, r, *vis, &theme);
                                        ui.add_space(4.0);
                                        ui.label(egui::RichText::new(&col.header).size(13.0).color(theme.foreground));
                                    });
                                    ui.add_space(2.0);
                                }
                            });
                    });

                // Close on click outside
                if ui.input(|i| i.pointer.any_click()) {
                    let area_rect: Option<egui::Rect> = ui.ctx().memory(|m| {
                        m.area_rect(egui::Id::new("__dt_col_popup__"))
                    });
                    let click_pos = ui.input(|i| i.pointer.interact_pos());
                    let outside = match (area_rect, click_pos) {
                        (Some(r), Some(p)) => !r.contains(p),
                        _ => true,
                    };
                    if outside && !col_btn.clicked() {
                        self.state.col_open = false;
                    }
                }
            }
        });

        ui.add_space(8.0);

        // ── Compute visible data ────────────────────────────────────────
        let filter_lc = self.state.filter.to_lowercase();
        let mut vis_indices: Vec<usize> = (0..self.rows.len())
            .filter(|&i| {
                filter_lc.is_empty()
                    || self.rows[i].iter().any(|c| c.to_lowercase().contains(&filter_lc))
            })
            .collect();

        if let Some(sc) = self.state.sort_col {
            let asc = self.state.sort_asc;
            vis_indices.sort_by(|&a, &b| {
                let va = self.rows[a].get(sc).map(|s| s.as_str()).unwrap_or("");
                let vb = self.rows[b].get(sc).map(|s| s.as_str()).unwrap_or("");
                if asc { va.cmp(vb) } else { vb.cmp(va) }
            });
        }

        let total_filtered = vis_indices.len();
        let total_pages    = (total_filtered.div_ceil(self.page_size)).max(1);
        if self.state.page >= total_pages { self.state.page = 0; }

        let p_start = self.state.page * self.page_size;
        let page_indices = &vis_indices[p_start..(p_start + self.page_size).min(total_filtered)];

        // Visible columns
        let vis_cols: Vec<usize> = (0..self.columns.len())
            .filter(|&i| self.state.col_vis.get(i).copied().unwrap_or(true))
            .collect();

        // Layout widths
        let cb_w     = 40.0;
        let action_w = 44.0;
        let data_w   = (avail_w - cb_w - action_w).max(0.0);
        let col_w    = if vis_cols.is_empty() { data_w } else { data_w / vis_cols.len() as f32 };
        let row_h    = 44.0;
        let header_h = 40.0;

        // ── Table frame ─────────────────────────────────────────────────
        egui::Frame::new()
            .fill(theme.card)
            .stroke(Stroke::new(1.0, theme.border))
            .corner_radius(theme.radius)
            .inner_margin(Margin::same(0))
            .show(ui, |ui| {
                ui.set_width(avail_w);

                // Header row
                let (hdr, _) = ui.allocate_exact_size(Vec2::new(avail_w, header_h), Sense::hover());

                let all_sel = !page_indices.is_empty()
                    && page_indices.iter().all(|&i| self.state.selected.get(i).copied().unwrap_or(false));

                // Select-all checkbox
                let cb_rect = egui::Rect::from_center_size(
                    egui::pos2(hdr.left() + cb_w * 0.5, hdr.center().y),
                    Vec2::splat(16.0),
                );
                let cb_resp = ui.interact(cb_rect, egui::Id::new("__dt_cb_all__"), Sense::click());
                if cb_resp.clicked() {
                    for &i in page_indices {
                        if let Some(s) = self.state.selected.get_mut(i) { *s = !all_sel; }
                    }
                }
                draw_checkbox(ui, cb_rect, all_sel, &theme);

                // Column headers
                for (pos, &ci) in vis_cols.iter().enumerate() {
                    let col  = &self.columns[ci];
                    let x    = hdr.left() + cb_w + col_w * pos as f32;
                    let cell = egui::Rect::from_min_size(egui::pos2(x, hdr.top()), Vec2::new(col_w, header_h));

                    let sort_icon = match (self.state.sort_col == Some(ci), self.state.sort_asc) {
                        (true,  true)  => " ↑",
                        (true,  false) => " ↓",
                        (false, _)     => if col.sortable { " ↑↓" } else { "" },
                    };

                    let hdr_resp = ui.interact(
                        cell,
                        egui::Id::new("__dt_hdr__").with(ci),
                        if col.sortable { Sense::click() } else { Sense::hover() },
                    );

                    if col.sortable && hdr_resp.clicked() {
                        if self.state.sort_col == Some(ci) {
                            self.state.sort_asc = !self.state.sort_asc;
                        } else {
                            self.state.sort_col = Some(ci);
                            self.state.sort_asc = true;
                        }
                    }

                    let lbl_color = if hdr_resp.hovered() && col.sortable {
                        theme.foreground
                    } else {
                        theme.muted_foreground
                    };

                    ui.painter().text(
                        egui::pos2(cell.left() + 8.0, cell.center().y),
                        egui::Align2::LEFT_CENTER,
                        format!("{}{}", col.header, sort_icon),
                        egui::FontId::proportional(13.0),
                        lbl_color,
                    );
                }

                // Header divider
                ui.painter().hline(hdr.left()..=hdr.right(), hdr.bottom(), Stroke::new(1.0, theme.border));

                // Data rows
                for (row_pos, &orig_i) in page_indices.iter().enumerate() {
                    let (row_rect, _) = ui.allocate_exact_size(Vec2::new(avail_w, row_h), Sense::hover());
                    let row_resp = ui.interact(row_rect, egui::Id::new("__dt_row__").with(row_pos), Sense::hover());

                    let is_sel = self.state.selected.get(orig_i).copied().unwrap_or(false);

                    let bg = if is_sel {
                        ShadcnTheme::with_alpha(theme.primary, 18)
                    } else if row_resp.hovered() {
                        ShadcnTheme::with_alpha(theme.foreground, 10)
                    } else {
                        Color32::TRANSPARENT
                    };

                    ui.painter().rect_filled(row_rect, 0u8, bg);
                    ui.painter().hline(
                        row_rect.left()..=row_rect.right(),
                        row_rect.top(),
                        Stroke::new(1.0, ShadcnTheme::with_alpha(theme.border, 80)),
                    );

                    // Row checkbox
                    let cb = egui::Rect::from_center_size(
                        egui::pos2(row_rect.left() + cb_w * 0.5, row_rect.center().y),
                        Vec2::splat(16.0),
                    );
                    let cr = ui.interact(cb, egui::Id::new("__dt_rowcb__").with(row_pos), Sense::click());
                    if cr.clicked()
                        && let Some(s) = self.state.selected.get_mut(orig_i)
                    {
                        *s = !*s;
                    }
                    draw_checkbox(ui, cb, is_sel, &theme);

                    // Cells
                    for (pos, &ci) in vis_cols.iter().enumerate() {
                        let x    = row_rect.left() + cb_w + col_w * pos as f32;
                        let text = self.rows[orig_i].get(ci).map(|s| s.as_str()).unwrap_or("");
                        ui.painter().text(
                            egui::pos2(x + 8.0, row_rect.center().y),
                            egui::Align2::LEFT_CENTER,
                            text,
                            egui::FontId::proportional(13.0),
                            theme.foreground,
                        );
                    }

                    // "···" action button
                    let act_rect = egui::Rect::from_min_size(
                        egui::pos2(row_rect.right() - action_w, row_rect.top()),
                        Vec2::new(action_w, row_h),
                    );
                    let ar = ui.interact(act_rect, egui::Id::new("__dt_act__").with(row_pos), Sense::click());
                    ui.painter().text(
                        act_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "···",
                        egui::FontId::proportional(15.0),
                        if ar.hovered() { theme.foreground } else { theme.muted_foreground },
                    );
                }
            });

        ui.add_space(8.0);

        // ── Bottom bar ─────────────────────────────────────────────────
        ui.horizontal(|ui| {
            let sel = page_indices.iter()
                .filter(|&&i| self.state.selected.get(i).copied().unwrap_or(false))
                .count();
            ui.label(
                egui::RichText::new(format!("{} of {} row(s) selected.", sel, total_filtered))
                    .size(13.0)
                    .color(theme.muted_foreground),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.spacing_mut().item_spacing.x = 8.0;

                let can_next = self.state.page + 1 < total_pages;
                let next = ui.add_enabled(
                    can_next,
                    egui::Button::new(egui::RichText::new("Next").size(13.0))
                        .min_size(Vec2::new(80.0, 32.0))
                        .stroke(Stroke::new(1.0, theme.border))
                        .fill(theme.background),
                );
                if next.clicked() { self.state.page += 1; }

                let can_prev = self.state.page > 0;
                let prev = ui.add_enabled(
                    can_prev,
                    egui::Button::new(egui::RichText::new("Previous").size(13.0))
                        .min_size(Vec2::new(80.0, 32.0))
                        .stroke(Stroke::new(1.0, theme.border))
                        .fill(theme.background),
                );
                if prev.clicked() { self.state.page = self.state.page.saturating_sub(1); }
            });
        });
    }
}

// ─── Shared helpers ────────────────────────────────────────────────────────

fn draw_checkbox(ui: &mut Ui, rect: egui::Rect, checked: bool, theme: &ShadcnTheme) {
    if checked {
        ui.painter().rect_filled(rect, 3u8, theme.primary);
        ui.painter().rect_stroke(rect, 3u8, Stroke::new(1.0, theme.primary), StrokeKind::Inside);
        ui.painter().text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "✓",
            egui::FontId::proportional(11.0),
            theme.primary_foreground,
        );
    } else {
        ui.painter().rect_filled(rect, 3u8, theme.background);
        ui.painter().rect_stroke(rect, 3u8, Stroke::new(1.0, theme.border), StrokeKind::Inside);
    }
}
