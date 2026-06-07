use crate::ShadcnTheme;
use egui::{Pos2, Rect, Stroke, Ui, Vec2};

pub struct TableColumn<'a> {
    pub header: &'a str,
    pub width: Option<f32>,
}

pub struct Table<'a> {
    columns: &'a [TableColumn<'a>],
    striped: bool,
}

impl<'a> Table<'a> {
    pub fn new(columns: &'a [TableColumn<'a>]) -> Self {
        Self {
            columns,
            striped: true,
        }
    }

    pub fn striped(mut self, s: bool) -> Self {
        self.striped = s;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        rows: usize,
        mut body_fn: impl FnMut(usize, &mut TableRow<'_>),
    ) {
        let theme = ShadcnTheme::get(ui.ctx());

        let available_width = ui.available_width();

        let fixed_total: f32 =
            self.columns.iter().filter_map(|c| c.width).sum();
        let flex_count =
            self.columns.iter().filter(|c| c.width.is_none()).count();
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
        let total_height = header_height + rows as f32 * row_height;

        let (table_rect, _) = ui.allocate_exact_size(
            Vec2::new(total_width, total_height),
            egui::Sense::hover(),
        );

        ui.painter().rect_filled(
            table_rect,
            egui::CornerRadius::ZERO,
            theme.card,
        );

        let header_rect = Rect::from_min_size(
            table_rect.min,
            Vec2::new(total_width, header_height),
        );
        ui.painter().rect_filled(
            header_rect,
            egui::CornerRadius::ZERO,
            theme.muted,
        );

        let mut col_x = table_rect.left();
        for (ci, col) in self.columns.iter().enumerate() {
            let cw = col_widths[ci];
            let cell_rect = Rect::from_min_size(
                Pos2::new(col_x, header_rect.top()),
                Vec2::new(cw, header_height),
            );

            ui.painter().text(
                Pos2::new(cell_rect.left() + 12.0, cell_rect.center().y),
                egui::Align2::LEFT_CENTER,
                col.header,
                egui::FontId::new(13.0, egui::FontFamily::Proportional),
                theme.muted_foreground,
            );

            if ci < self.columns.len() - 1 {
                ui.painter().vline(
                    cell_rect.right(),
                    cell_rect.y_range(),
                    Stroke::new(1.0, theme.border),
                );
            }

            col_x += cw;
        }

        ui.painter().hline(
            table_rect.x_range(),
            header_rect.bottom(),
            Stroke::new(1.0, theme.border),
        );

        for row_idx in 0..rows {
            let row_top =
                table_rect.top() + header_height + row_idx as f32 * row_height;
            let row_rect = Rect::from_min_size(
                Pos2::new(table_rect.left(), row_top),
                Vec2::new(total_width, row_height),
            );

            let row_bg = if self.striped && row_idx % 2 == 1 {
                theme.muted
            } else {
                theme.card
            };
            ui.painter().rect_filled(
                row_rect,
                egui::CornerRadius::ZERO,
                row_bg,
            );

            let mut table_row = TableRow {
                ui,
                col_widths: col_widths.clone(),
                col_idx: 0,
                row_height,
                x: table_rect.left(),
                row_rect,
            };

            body_fn(row_idx, &mut table_row);

            ui.painter().hline(
                row_rect.x_range(),
                row_rect.bottom(),
                Stroke::new(1.0, theme.border),
            );
        }

        ui.painter().rect_stroke(
            table_rect,
            egui::CornerRadius::ZERO,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
    }
}

pub struct TableRow<'a> {
    pub ui: &'a mut egui::Ui,
    pub col_widths: Vec<f32>,
    pub col_idx: usize,
    pub row_height: f32,
    pub x: f32,
    pub row_rect: egui::Rect,
}

impl<'a> TableRow<'a> {
    pub fn cell(&mut self, content: impl FnOnce(&mut egui::Ui)) {
        if self.col_idx >= self.col_widths.len() {
            return;
        }
        let cw = self.col_widths[self.col_idx];
        let cell_rect = egui::Rect::from_min_size(
            egui::Pos2::new(self.x, self.row_rect.top()),
            Vec2::new(cw, self.row_height),
        );

        let inner_rect = egui::Rect::from_min_max(
            egui::Pos2::new(cell_rect.left() + 12.0, cell_rect.top() + 2.0),
            egui::Pos2::new(cell_rect.right() - 4.0, cell_rect.bottom() - 2.0),
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
                    let theme = ShadcnTheme::get(self.ui.ctx());
                    theme.border
                }),
            );
        }

        self.x += cw;
        self.col_idx += 1;
    }
}
