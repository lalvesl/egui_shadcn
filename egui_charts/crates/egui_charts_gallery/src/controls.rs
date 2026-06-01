//! Right-panel theme + harmony controls.

use egui::{Color32, Ui, vec2};
use egui_charts::{ChartTheme, Distribution, Harmony, ThemeMode};

pub struct ControlsState {
    pub primary: Color32,
    pub mode: ThemeMode,
    pub harmony: Harmony,
    pub distribution: Distribution,
    pub series_count: usize,
}

impl Default for ControlsState {
    fn default() -> Self {
        Self {
            primary: Color32::from_rgb(0x4f, 0x8c, 0xff),
            mode: ThemeMode::Dark,
            harmony: Harmony::Square,
            distribution: Distribution::Even,
            series_count: 6,
        }
    }
}

impl ControlsState {
    pub fn build_theme(&self) -> ChartTheme {
        ChartTheme::from_primary(
            self.primary,
            self.mode,
            self.harmony,
            self.series_count,
            self.distribution,
        )
    }
}

pub fn show(ui: &mut Ui, state: &mut ControlsState) -> bool {
    let mut changed = false;
    ui.heading("Theme");
    ui.add_space(6.0);

    ui.horizontal(|ui| {
        let prev = state.mode;
        if ui
            .selectable_label(state.mode == ThemeMode::Light, "Light")
            .clicked()
        {
            state.mode = ThemeMode::Light;
        }
        if ui
            .selectable_label(state.mode == ThemeMode::Dark, "Dark")
            .clicked()
        {
            state.mode = ThemeMode::Dark;
        }
        if state.mode != prev {
            changed = true;
            let visuals = if state.mode == ThemeMode::Dark {
                egui::Visuals::dark()
            } else {
                egui::Visuals::light()
            };
            ui.ctx().set_visuals(visuals);
        }
    });

    ui.add_space(8.0);
    ui.label("Primary color");
    let mut rgba = [
        state.primary.r() as f32 / 255.0,
        state.primary.g() as f32 / 255.0,
        state.primary.b() as f32 / 255.0,
    ];
    if ui.color_edit_button_rgb(&mut rgba).changed() {
        state.primary = Color32::from_rgb(
            (rgba[0] * 255.0) as u8,
            (rgba[1] * 255.0) as u8,
            (rgba[2] * 255.0) as u8,
        );
        changed = true;
    }

    ui.add_space(8.0);
    ui.label("Harmony scheme");
    egui::ComboBox::from_id_salt("harmony")
        .selected_text(state.harmony.label())
        .show_ui(ui, |ui| {
            for h in Harmony::ALL {
                if ui
                    .selectable_label(state.harmony == *h, h.label())
                    .clicked()
                {
                    state.harmony = *h;
                    changed = true;
                }
            }
        });

    ui.add_space(8.0);
    ui.label("Distribution");
    egui::ComboBox::from_id_salt("dist")
        .selected_text(match state.distribution {
            Distribution::Even => "Even",
            Distribution::GoldenAngle => "Golden angle",
        })
        .show_ui(ui, |ui| {
            for (d, label) in [
                (Distribution::Even, "Even"),
                (Distribution::GoldenAngle, "Golden angle"),
            ] {
                if ui
                    .selectable_label(state.distribution == d, label)
                    .clicked()
                {
                    state.distribution = d;
                    changed = true;
                }
            }
        });

    ui.add_space(8.0);
    ui.label(format!("Series in palette: {}", state.series_count));
    if ui
        .add(egui::Slider::new(&mut state.series_count, 1..=24))
        .changed()
    {
        changed = true;
    }

    ui.add_space(12.0);
    ui.label("Palette");
    let theme = state.build_theme();
    let swatch_size = vec2(18.0, 18.0);
    ui.horizontal_wrapped(|ui| {
        for color in theme.palette.iter() {
            let (rect, _) = ui.allocate_exact_size(swatch_size, egui::Sense::hover());
            ui.painter().rect_filled(rect, 3.0, *color);
        }
    });

    ui.add_space(12.0);
    ui.label("Sequential");
    ui.horizontal(|ui| {
        for color in theme.sequential.iter() {
            let (rect, _) = ui.allocate_exact_size(vec2(14.0, 18.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 0.0, *color);
        }
    });

    ui.add_space(8.0);
    ui.label("Diverging");
    ui.horizontal(|ui| {
        for color in theme.diverging.iter() {
            let (rect, _) = ui.allocate_exact_size(vec2(14.0, 18.0), egui::Sense::hover());
            ui.painter().rect_filled(rect, 0.0, *color);
        }
    });

    changed
}
