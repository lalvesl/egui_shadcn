use egui::Color32;
use egui_shadcn::{
    ShadcnTheme,
    avatar::{Avatar, AvatarSize},
    calendar::Calendar,
    card::{Card, card_header},
    collapsible::Collapsible,
    data_table::{DataTable, DataColumn},
    label::Label,
    table::{Table, TableColumn},
    typography::{body_text, muted_text},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_cards(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Card", "Container with optional header and footer.");
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Card Title",
                Some("A brief description of the card content."),
            );
            body_text(ui, "Card body content goes here.");
        });
        ui.add_space(16.0);
        Card::new().padding(16.0).show(ui, |ui| {
            card_header(ui, "User Card", None);
            ui.horizontal(|ui| {
                Avatar::new("JD").show(ui);
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    body_text(ui, "John Doe");
                    muted_text(ui, "john@example.com");
                });
            });
        });
    }

    pub(in crate::app) fn section_data_display(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Data Display", "Avatars and labels.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Avatar", None);
            ui.horizontal(|ui| {
                Avatar::new("JD").size(AvatarSize::Sm).show(ui);
                ui.add_space(8.0);
                Avatar::new("Alice").show(ui);
                ui.add_space(8.0);
                Avatar::new("Bob").size(AvatarSize::Lg).show(ui);
                ui.add_space(8.0);
                Avatar::new("XY")
                    .color(Color32::from_rgb(139, 92, 246))
                    .show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Label", None);
            Label::new("Username").show(ui);
            ui.add_space(4.0);
            Label::new("Email").required(true).show(ui);
        });
    }

    pub(in crate::app) fn section_calendar(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Calendar",
            "Date and range pickers with optional cell content.",
        );

        // ── Single date ──────────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(ui, "Single date", None);

            let label = match self.cal_single {
                Some(d) => format!("Selected: {:04}-{:02}-{:02}", d.year, d.month, d.day),
                None => "No date selected".to_owned(),
            };
            muted_text(ui, &label);
            ui.add_space(12.0);

            Calendar::single("demo_cal_single", &mut self.cal_single).show(ui);
        });

        ui.add_space(16.0);

        // ── Range date ───────────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Date range",
                Some("Click a start date then an end date."),
            );

            let label = match (self.cal_range_start, self.cal_range_end) {
                (Some(s), Some(e)) => format!(
                    "{:04}-{:02}-{:02}  →  {:04}-{:02}-{:02}",
                    s.year, s.month, s.day, e.year, e.month, e.day
                ),
                (Some(s), None) => {
                    format!("{:04}-{:02}-{:02}  →  (pick end)", s.year, s.month, s.day)
                }
                _ => "No range selected".to_owned(),
            };
            muted_text(ui, &label);
            ui.add_space(12.0);

            Calendar::range(
                "demo_cal_range",
                &mut self.cal_range_start,
                &mut self.cal_range_end,
            )
            .show(ui);

            if self.cal_range_start.is_some() {
                ui.add_space(8.0);
                if ui.button("Clear").clicked() {
                    self.cal_range_start = None;
                    self.cal_range_end = None;
                }
            }
        });

        ui.add_space(16.0);

        // ── With cell content ─────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Cell content",
                Some("Custom badges inside specific date cells."),
            );
            muted_text(ui, "Cells on the 1st, 10th, and 20th have event badges.");
            ui.add_space(12.0);

            let theme = ShadcnTheme::get(ui.ctx());
            let accent = theme.primary;

            Calendar::single("demo_cal_events", &mut self.cal_single)
                .cell_height(52.0)
                .cell_content(move |ui, date| {
                    if matches!(date.day, 1 | 10 | 20) {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 12.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(
                            rect.shrink2(egui::Vec2::new(2.0, 0.0)),
                            egui::CornerRadius::same(3),
                            accent,
                        );
                    }
                })
                .show(ui);
        });
    }

    pub(in crate::app) fn section_collapsible(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Collapsible", "An interactive component that expands/collapses a section.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Collapsible", None);
            Collapsible::new("demo_collapsible", "Starred repositories", &mut self.collapsible_open)
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    muted_text(ui, "@radix-ui/primitives");
                    ui.add_space(4.0);
                    muted_text(ui, "@radix-ui/colors");
                    ui.add_space(4.0);
                    muted_text(ui, "@stitches/react");
                });
        });
    }

    pub(in crate::app) fn section_table(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Table", "A simple table with header and striped rows.");

        let columns = &[
            TableColumn { header: "Invoice", width: Some(120.0) },
            TableColumn { header: "Status", width: Some(100.0) },
            TableColumn { header: "Method", width: Some(120.0) },
            TableColumn { header: "Amount", width: None },
        ];

        let data = &[
            ("INV001", "Paid", "Credit Card", "$250.00"),
            ("INV002", "Pending", "PayPal", "$150.00"),
            ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
            ("INV004", "Paid", "Credit Card", "$450.00"),
            ("INV005", "Paid", "PayPal", "$550.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Recent Invoices", None);
            Table::new(columns).show(ui, data.len(), |row_idx, row| {
                let (inv, status, method, amount) = data[row_idx];
                row.cell(|ui| { muted_text(ui, inv); });
                row.cell(|ui| { muted_text(ui, status); });
                row.cell(|ui| { muted_text(ui, method); });
                row.cell(|ui| { muted_text(ui, amount); });
            });
        });
    }

    pub(in crate::app) fn section_data_table(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Data Table", "A powerful table with sorting and filtering.");

        let columns = &[
            DataColumn { header: "Status", width: Some(100.0), sortable: true },
            DataColumn { header: "Email", width: None, sortable: true },
            DataColumn { header: "Amount", width: Some(120.0), sortable: true },
        ];

        let data = &[
            ("Success", "ken99@yahoo.com", "$316.00"),
            ("Success", "abe45@gmail.com", "$242.00"),
            ("Processing", "monserrat44@gmail.com", "$837.00"),
            ("Success", "silas22@gmail.com", "$874.00"),
            ("Failed", "carmella@hotmail.com", "$721.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Payments", None);
            DataTable::new("demo_data_table", columns, &mut self.data_table_filter)
                .page_size(3)
                .show(ui, data.len(), |row_idx, row| {
                    let (status, email, amount) = data[row_idx];
                    row.cell(|ui| { muted_text(ui, status); });
                    row.cell(|ui| { muted_text(ui, email); });
                    row.cell(|ui| { muted_text(ui, amount); });
                });
        });
    }
}
