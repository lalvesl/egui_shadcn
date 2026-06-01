//! Gallery + theme playground for egui_charts.

mod controls;
mod samples;

use controls::ControlsState;
use eframe::egui;
use egui_charts::{ChartKind, ChartWidget};
use std::collections::BTreeMap;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 760.0])
            .with_title("egui_charts gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "egui_charts_gallery",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Ok(Box::new(GalleryApp::default()))
        }),
    )
}

struct GalleryApp {
    selected: ChartKind,
    controls: ControlsState,
    expanded: BTreeMap<String, bool>,
}

impl Default for GalleryApp {
    fn default() -> Self {
        let mut expanded = BTreeMap::new();
        for k in ChartKind::all() {
            expanded.entry(k.category().to_string()).or_insert(true);
        }
        Self {
            selected: ChartKind::Line,
            controls: ControlsState::default(),
            expanded,
        }
    }
}

impl eframe::App for GalleryApp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        egui::Panel::left("sidebar")
            .resizable(false)
            .exact_size(240.0)
            .show_inside(ui, |ui| {
                ui.heading("egui_charts");
                ui.label(egui::RichText::new("gallery").small().italics());
                ui.separator();
                self.draw_sidebar(ui);
            });

        egui::Panel::right("controls")
            .resizable(false)
            .exact_size(260.0)
            .show_inside(ui, |ui| {
                controls::show(ui, &mut self.controls);
            });

        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.add_space(4.0);
            ui.heading(self.selected.label());
            ui.label(
                egui::RichText::new(format!(
                    "Category: {}  ·  {}",
                    self.selected.category(),
                    if self.selected.is_implemented() {
                        "implemented"
                    } else {
                        "planned (Phase 1+)"
                    }
                ))
                .small()
                .color(ui.visuals().weak_text_color()),
            );
            ui.separator();
            ui.add_space(8.0);

            if let Some(chart) = samples::build(self.selected) {
                let theme = self.controls.build_theme();
                let available = ui.available_size();
                ChartWidget::new(&chart)
                    .theme(theme)
                    .min_size(egui::vec2(available.x.max(400.0), available.y.max(360.0)))
                    .show(ui);
            } else {
                ui.add_space(60.0);
                ui.vertical_centered(|ui| {
                    ui.label(
                        egui::RichText::new("Not yet implemented")
                            .heading()
                            .color(ui.visuals().weak_text_color()),
                    );
                    ui.add_space(8.0);
                    ui.label(
                        "Phase 0 ships line, bar, scatter. See implementation plan §8 for the roadmap.",
                    );
                });
            }
        });
    }
}

impl GalleryApp {
    fn draw_sidebar(&mut self, ui: &mut egui::Ui) {
        egui::ScrollArea::vertical().show(ui, |ui| {
            // Group kinds by category, preserving category order from ChartKind::all().
            let mut groups: Vec<(&'static str, Vec<ChartKind>)> = Vec::new();
            for k in ChartKind::all() {
                let cat = k.category();
                if let Some(g) = groups.iter_mut().find(|g| g.0 == cat) {
                    g.1.push(*k);
                } else {
                    groups.push((cat, vec![*k]));
                }
            }

            for (cat, kinds) in groups {
                let header = self.expanded.entry(cat.to_string()).or_insert(true);
                let toggle = ui.add(
                    egui::Button::new(format!(
                        "{} {}",
                        if *header { "▾" } else { "▸" },
                        cat
                    ))
                    .frame(false),
                );
                if toggle.clicked() {
                    *header = !*header;
                }
                if *header {
                    ui.indent(cat, |ui| {
                        for k in &kinds {
                            let selected = self.selected == *k;
                            let label = if k.is_implemented() {
                                egui::RichText::new(k.label())
                            } else {
                                egui::RichText::new(k.label()).color(ui.visuals().weak_text_color())
                            };
                            if ui.selectable_label(selected, label).clicked() {
                                self.selected = *k;
                            }
                        }
                    });
                }
            }
        });
    }
}
