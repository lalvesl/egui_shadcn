use egui::Ui;

/// Responsive column grid.
///
/// Column count = floor((available_width + gap) / (min_col_width + gap)), min 1.
///
/// ```ignore
/// Grid::new().min_col_width(240.0).show(ui, 3, |ui, idx| {
///     Card::new().show(ui, |ui| { ... });
/// });
/// ```
pub struct Grid {
    min_col_width: f32,
    gap: f32,
}

impl Default for Grid {
    fn default() -> Self {
        Self::new()
    }
}

impl Grid {
    pub fn new() -> Self {
        Self {
            min_col_width: 240.0,
            gap: 16.0,
        }
    }

    pub fn min_col_width(mut self, w: f32) -> Self {
        self.min_col_width = w;
        self
    }
    pub fn gap(mut self, g: f32) -> Self {
        self.gap = g;
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        count: usize,
        mut item_fn: impl FnMut(&mut Ui, usize),
    ) {
        if count == 0 {
            return;
        }

        let available = ui.available_width();
        let cols = ((available + self.gap) / (self.min_col_width + self.gap))
            .floor()
            .max(1.0) as usize;
        let cols = cols.min(count);
        let rows = count.div_ceil(cols);

        for row in 0..rows {
            if row > 0 {
                ui.add_space(self.gap);
            }
            ui.columns(cols, |col_uis| {
                for (col, col_ui) in col_uis.iter_mut().enumerate().take(cols) {
                    let idx = row * cols + col;
                    if idx < count {
                        item_fn(col_ui, idx);
                    }
                }
            });
        }
    }
}
