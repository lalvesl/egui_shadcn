use crate::ShadcnTheme;
use egui::{Color32, Pos2, Rect, Sense, Stroke, Ui, Vec2};

pub struct ChartDataset<'a> {
    pub label: &'a str,
    pub values: &'a [f64],
    pub color: Option<Color32>,
}

pub enum ChartKind {
    Bar,
    Line,
}

pub struct Chart<'a> {
    datasets: &'a [ChartDataset<'a>],
    labels: &'a [&'a str],
    kind: ChartKind,
    height: f32,
    width: f32,
    show_grid: bool,
    show_legend: bool,
}

impl<'a> Chart<'a> {
    pub fn new(datasets: &'a [ChartDataset<'a>], labels: &'a [&'a str]) -> Self {
        Self {
            datasets,
            labels,
            kind: ChartKind::Bar,
            height: 200.0,
            width: 0.0,
            show_grid: true,
            show_legend: true,
        }
    }

    pub fn kind(mut self, k: ChartKind) -> Self {
        self.kind = k;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show_grid(mut self, g: bool) -> Self {
        self.show_grid = g;
        self
    }

    pub fn show_legend(mut self, l: bool) -> Self {
        self.show_legend = l;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let avail_width = if self.width > 0.0 {
            self.width
        } else {
            ui.available_width()
        };

        let legend_height = if self.show_legend && !self.datasets.is_empty() {
            24.0
        } else {
            0.0
        };
        let x_label_height = 20.0;
        let y_label_width = 40.0;
        let pad_top = 10.0;
        let pad_right = 10.0;

        let total_height = self.height + legend_height + x_label_height;
        let (outer_rect, _) =
            ui.allocate_exact_size(Vec2::new(avail_width, total_height), Sense::hover());

        // Chart plot area
        let plot_rect = Rect::from_min_max(
            Pos2::new(outer_rect.left() + y_label_width, outer_rect.top() + pad_top),
            Pos2::new(
                outer_rect.right() - pad_right,
                outer_rect.top() + pad_top + self.height - x_label_height,
            ),
        );

        // Background
        ui.painter()
            .rect_filled(plot_rect, egui::CornerRadius::ZERO, theme.background);

        // Auto-scale Y
        let max_val = self
            .datasets
            .iter()
            .flat_map(|ds| ds.values.iter().copied())
            .fold(0.0_f64, f64::max);
        let max_val = if max_val == 0.0 { 1.0 } else { max_val };

        // Grid lines
        let grid_lines = 4usize;
        if self.show_grid {
            for i in 0..=grid_lines {
                let y = plot_rect.bottom()
                    - i as f32 * (plot_rect.height() / grid_lines as f32);
                ui.painter().hline(
                    plot_rect.x_range(),
                    y,
                    Stroke::new(1.0, ShadcnTheme::with_alpha(theme.muted_foreground, 40)),
                );
                // Y label
                let val = max_val as f32 * i as f32 / grid_lines as f32;
                let label = if val >= 1000.0 {
                    format!("{:.0}k", val / 1000.0)
                } else {
                    format!("{:.0}", val)
                };
                ui.painter().text(
                    Pos2::new(outer_rect.left() + y_label_width - 4.0, y),
                    egui::Align2::RIGHT_CENTER,
                    label,
                    ShadcnTheme::small_font(),
                    theme.muted_foreground,
                );
            }
        }

        // Default dataset colors
        let default_colors = [
            theme.primary,
            Color32::from_rgb(99, 179, 237),  // blue-300
            Color32::from_rgb(154, 205, 50),  // yellow-green
            Color32::from_rgb(237, 137, 54),  // orange
            Color32::from_rgb(184, 100, 184), // purple
        ];

        let n_labels = self.labels.len().max(1);
        let slot_w = plot_rect.width() / n_labels as f32;

        match self.kind {
            ChartKind::Bar => {
                let n_datasets = self.datasets.len().max(1);
                let bar_group_w = slot_w * 0.8;
                let bar_w = bar_group_w / n_datasets as f32;
                let gap = slot_w * 0.1;

                for (di, dataset) in self.datasets.iter().enumerate() {
                    let color = dataset
                        .color
                        .unwrap_or_else(|| default_colors[di % default_colors.len()]);

                    for (li, &val) in dataset.values.iter().enumerate() {
                        if li >= n_labels {
                            break;
                        }
                        let bar_h = (val / max_val) as f32 * plot_rect.height();
                        let x =
                            plot_rect.left() + li as f32 * slot_w + gap + di as f32 * bar_w;
                        let bar_rect = Rect::from_min_max(
                            Pos2::new(x, plot_rect.bottom() - bar_h),
                            Pos2::new(x + bar_w - 1.0, plot_rect.bottom()),
                        );
                        ui.painter()
                            .rect_filled(bar_rect, egui::CornerRadius::same(2), color);
                    }
                }
            }
            ChartKind::Line => {
                for (di, dataset) in self.datasets.iter().enumerate() {
                    let color = dataset
                        .color
                        .unwrap_or_else(|| default_colors[di % default_colors.len()]);

                    let points: Vec<Pos2> = dataset
                        .values
                        .iter()
                        .enumerate()
                        .take(n_labels)
                        .map(|(li, &val)| {
                            let x = plot_rect.left() + li as f32 * slot_w + slot_w / 2.0;
                            let y = plot_rect.bottom() - (val / max_val) as f32 * plot_rect.height();
                            Pos2::new(x, y)
                        })
                        .collect();

                    // Lines
                    for w in points.windows(2) {
                        ui.painter()
                            .line_segment([w[0], w[1]], Stroke::new(2.0, color));
                    }
                    // Dots
                    for &pt in &points {
                        ui.painter().circle_filled(pt, 4.0, color);
                        ui.painter()
                            .circle_stroke(pt, 4.0, Stroke::new(1.5, theme.background));
                    }
                }
            }
        }

        // X-axis labels
        for (li, label) in self.labels.iter().enumerate() {
            let x = plot_rect.left() + li as f32 * slot_w + slot_w / 2.0;
            let y = plot_rect.bottom() + 14.0;
            ui.painter().text(
                Pos2::new(x, y),
                egui::Align2::CENTER_CENTER,
                *label,
                ShadcnTheme::small_font(),
                theme.muted_foreground,
            );
        }

        // Border
        ui.painter().rect_stroke(
            plot_rect,
            egui::CornerRadius::ZERO,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        // Legend
        if self.show_legend && !self.datasets.is_empty() {
            let legend_y = outer_rect.top() + pad_top + self.height + 4.0;
            let mut lx = outer_rect.left() + y_label_width;
            for (di, dataset) in self.datasets.iter().enumerate() {
                let color = dataset
                    .color
                    .unwrap_or_else(|| default_colors[di % default_colors.len()]);
                let dot_center = Pos2::new(lx + 6.0, legend_y + 8.0);
                ui.painter().circle_filled(dot_center, 5.0, color);
                let text_rect = ui.painter().text(
                    Pos2::new(lx + 16.0, legend_y + 8.0),
                    egui::Align2::LEFT_CENTER,
                    dataset.label,
                    ShadcnTheme::small_font(),
                    theme.foreground,
                );
                lx += 16.0 + text_rect.width() + 16.0;
            }
        }
    }
}
