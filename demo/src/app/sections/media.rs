use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ShadcnTheme,
    button::{Button, ButtonVariant},
    card::{Card, card_header},
    carousel::Carousel,
    chart::{Chart, ChartDataset, ChartKind},
    command::{Command, CommandGroup, CommandItem},
    resizable::{Resizable, ResizeDir},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_carousel(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Carousel", "A carousel with motion and swipe capabilities.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Items", None);
            Carousel::new("demo_carousel", 5).height(180.0).show(ui, |idx, ui| {
                let theme = ShadcnTheme::get(ui.ctx());
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 160.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(rect, egui::CornerRadius::same(8), theme.muted);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    format!("Slide {}", idx + 1),
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
            });
        });
    }

    pub(in crate::app) fn section_chart(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Chart", "Bar and line charts for data visualization.");

        let months = &["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
        let desktop = &[186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
        let mobile = &[80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

        Card::new().show(ui, |ui| {
            card_header(ui, "Bar chart – Desktop vs Mobile", None);
            let datasets = &[
                ChartDataset { label: "Desktop", values: desktop, color: None },
                ChartDataset { label: "Mobile", values: mobile, color: Some(egui::Color32::from_rgb(100, 160, 240)) },
            ];
            Chart::new(datasets, months).height(200.0).show_legend(true).show(ui);
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Line chart", None);
            let datasets = &[
                ChartDataset { label: "Desktop", values: desktop, color: None },
            ];
            Chart::new(datasets, months).kind(ChartKind::Line).height(200.0).show_grid(true).show(ui);
        });
    }

    pub(in crate::app) fn section_command(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Command", "Fast, composable, keyboard-first command menu.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Command palette", Some("Press the button or ⌘K to open."));
            if Button::new("Open Command Palette").variant(ButtonVariant::Outline).show(ui).clicked() {
                self.command_open = true;
            }
        });

        let groups = &[
            CommandGroup {
                heading: Some("Suggestions"),
                items: &[
                    CommandItem { label: "Calendar", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Search Emoji", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Calculator", description: None, shortcut: None, icon: None },
                ],
            },
            CommandGroup {
                heading: Some("Settings"),
                items: &[
                    CommandItem { label: "Profile", description: Some("⌘P"), shortcut: None, icon: None },
                    CommandItem { label: "Billing", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Settings", description: Some("⌘S"), shortcut: None, icon: None },
                ],
            },
        ];

        let ctx = ui.ctx().clone();
        Command::new("demo_command", groups, &mut self.command_open)
            .placeholder("Type a command or search…")
            .show(&ctx);
    }

    pub(in crate::app) fn section_resizable(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Resizable", "Accessible resizable panel groups and layouts.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Horizontal panels", None);
            Resizable::new("demo_resizable_h")
                .dir(ResizeDir::Horizontal)
                .initial_split(0.5)
                .height(120.0)
                .show(ui,
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 120.0), egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(4), theme.muted);
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Panel A",
                            egui::FontId::new(13.0, egui::FontFamily::Proportional), theme.muted_foreground);
                    },
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 120.0), egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(4), theme.muted);
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Panel B",
                            egui::FontId::new(13.0, egui::FontFamily::Proportional), theme.muted_foreground);
                    }
                );
        });
    }
}
