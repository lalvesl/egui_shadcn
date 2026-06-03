//! Right-panel theme + harmony controls — built entirely from
//! `egui_components` (shadcn) widgets.

use egui::{Color32, Ui};
use egui_charts::{ChartTheme, Distribution, Harmony, ThemeMode};
use egui_components::{
    ICON_DARK_MODE, ICON_LIGHT_MODE, ShadcnTheme, Size,
    select::Select,
    separator::Separator,
    slider::Slider,
    theme::hsl,
    toggle_group::{ToggleGroup, ToggleGroupItem},
    typography::{heading4, small_text},
};

/// Preset primary hues (label, degrees).
const PRESET_HUES: &[(&str, f32)] = &[
    ("Red", 0.0),
    ("Orange", 25.0),
    ("Amber", 45.0),
    ("Green", 142.0),
    ("Teal", 168.0),
    ("Blue", 217.0),
    ("Violet", 263.0),
    ("Pink", 320.0),
];

const DISTRIBUTIONS: &[(Distribution, &str)] = &[
    (Distribution::Even, "Even"),
    (Distribution::GoldenAngle, "Golden angle"),
];

pub struct ControlsState {
    pub hue: f32,
    pub mode: ThemeMode,
    pub harmony: Harmony,
    pub distribution: Distribution,
    pub series_count: usize,
}

impl Default for ControlsState {
    fn default() -> Self {
        Self {
            hue: 217.0,
            mode: ThemeMode::Dark,
            harmony: Harmony::Square,
            distribution: Distribution::Even,
            series_count: 6,
        }
    }
}

impl ControlsState {
    /// Primary seed color derived from the selected hue.
    pub fn primary(&self) -> Color32 {
        hsl(self.hue, 0.72, 0.55)
    }

    pub fn build_theme(&self) -> ChartTheme {
        ChartTheme::from_primary(
            self.primary(),
            self.mode,
            self.harmony,
            self.series_count,
            self.distribution,
        )
    }
}

pub fn show(ui: &mut Ui, state: &mut ControlsState) {
    let theme = ShadcnTheme::get(ui.ctx());

    egui::ScrollArea::vertical()
        .auto_shrink([false, false])
        .show(ui, |ui| {
            heading4(ui, "Theme");
            ui.add_space(12.0);

            // ── Mode (light / dark) ──────────────────────────────────────────
            small_text(ui, "Mode");
            ui.add_space(4.0);
            let mode_items = [
                (
                    ThemeMode::Light,
                    ToggleGroupItem {
                        label: "Light",
                        icon: Some(ICON_LIGHT_MODE),
                    },
                ),
                (
                    ThemeMode::Dark,
                    ToggleGroupItem {
                        label: "Dark",
                        icon: Some(ICON_DARK_MODE),
                    },
                ),
            ];
            ToggleGroup::new(&mode_items, &mut state.mode)
                .size(Size::Sm)
                .show(ui);

            ui.add_space(14.0);

            // ── Primary color ────────────────────────────────────────────────
            small_text(ui, "Primary color");
            ui.add_space(6.0);
            ui.horizontal_wrapped(|ui| {
                for (name, hue) in PRESET_HUES {
                    let color = hsl(*hue, 0.72, 0.55);
                    let selected = (state.hue - *hue).abs() < 0.5;
                    let (rect, resp) =
                        ui.allocate_exact_size(egui::Vec2::splat(26.0), egui::Sense::click());
                    ui.painter().circle_filled(rect.center(), 11.0, color);
                    if selected {
                        ui.painter().circle_stroke(
                            rect.center(),
                            13.0,
                            egui::Stroke::new(2.0, theme.foreground),
                        );
                    }
                    let resp = resp.on_hover_text(*name);
                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                    if resp.clicked() {
                        state.hue = *hue;
                    }
                }
            });

            ui.add_space(8.0);
            small_text(ui, "Hue");
            ui.add_space(4.0);
            Slider::new(&mut state.hue, 0.0, 360.0).show(ui);

            ui.add_space(14.0);
            Separator::horizontal().show(ui);
            ui.add_space(14.0);

            // ── Harmony scheme ───────────────────────────────────────────────
            small_text(ui, "Harmony scheme");
            ui.add_space(4.0);
            let harmony_labels: Vec<&str> = Harmony::ALL.iter().map(|h| h.label()).collect();
            let mut harmony_idx = Harmony::ALL.iter().position(|h| *h == state.harmony);
            if Select::new(&mut harmony_idx, &harmony_labels).show(ui)
                && let Some(i) = harmony_idx
            {
                state.harmony = Harmony::ALL[i];
            }

            ui.add_space(10.0);

            // ── Distribution ─────────────────────────────────────────────────
            small_text(ui, "Distribution");
            ui.add_space(4.0);
            let dist_labels: Vec<&str> = DISTRIBUTIONS.iter().map(|(_, l)| *l).collect();
            let mut dist_idx = DISTRIBUTIONS
                .iter()
                .position(|(d, _)| *d == state.distribution);
            if Select::new(&mut dist_idx, &dist_labels).show(ui)
                && let Some(i) = dist_idx
            {
                state.distribution = DISTRIBUTIONS[i].0;
            }

            ui.add_space(14.0);

            // ── Series count ─────────────────────────────────────────────────
            small_text(ui, &format!("Series in palette: {}", state.series_count));
            ui.add_space(4.0);
            let mut series = state.series_count as f32;
            Slider::new(&mut series, 1.0, 24.0).step(1.0).show(ui);
            state.series_count = series.round().clamp(1.0, 24.0) as usize;

            ui.add_space(14.0);
            Separator::horizontal().show(ui);
            ui.add_space(14.0);

            // ── Generated palette previews ───────────────────────────────────
            let chart_theme = state.build_theme();

            small_text(ui, "Palette");
            ui.add_space(4.0);
            swatch_row(ui, &chart_theme.palette, egui::vec2(18.0, 18.0), 3.0, true);

            ui.add_space(12.0);
            small_text(ui, "Sequential");
            ui.add_space(4.0);
            swatch_row(
                ui,
                &chart_theme.sequential,
                egui::vec2(14.0, 18.0),
                0.0,
                false,
            );

            ui.add_space(12.0);
            small_text(ui, "Diverging");
            ui.add_space(4.0);
            swatch_row(
                ui,
                &chart_theme.diverging,
                egui::vec2(14.0, 18.0),
                0.0,
                false,
            );
        });
}

/// Draw a strip of color swatches (palette preview).
fn swatch_row(ui: &mut Ui, colors: &[Color32], size: egui::Vec2, radius: f32, wrap: bool) {
    let draw = |ui: &mut Ui| {
        for color in colors {
            let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());
            ui.painter().rect_filled(rect, radius, *color);
        }
    };
    if wrap {
        ui.horizontal_wrapped(draw);
    } else {
        ui.horizontal(draw);
    }
}
