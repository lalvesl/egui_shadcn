use egui::Color32;
use egui_shadcn::{
    ShadcnTheme,
    avatar::{Avatar, AvatarSize},
    boxed::Boxed,
    calendar::Calendar,
    card::{Card, card_header},
    collapsible::Collapsible,
    data_table::{DataTable, DataColumn},
    label::Label,
    separator::Separator,
    spacing::Spacing,
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
        Spacing::Lg.show(ui);
        Card::new().padding(16.0).show(ui, |ui| {
            card_header(ui, "User Card", None);
            ui.horizontal(|ui| {
                Avatar::new("JD").show(ui);
                Spacing::Md.show(ui);
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
                Spacing::Sm.show(ui);
                Avatar::new("Alice").show(ui);
                Spacing::Sm.show(ui);
                Avatar::new("Bob").size(AvatarSize::Lg).show(ui);
                Spacing::Sm.show(ui);
                Avatar::new("XY")
                    .color(Color32::from_rgb(139, 92, 246))
                    .show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "Label", None);
            Label::new("Username").show(ui);
            Spacing::Xs.show(ui);
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
            Spacing::Md.show(ui);

            Calendar::single("demo_cal_single", &mut self.cal_single).show(ui);
        });

        Spacing::Lg.show(ui);

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
            Spacing::Md.show(ui);

            Calendar::range(
                "demo_cal_range",
                &mut self.cal_range_start,
                &mut self.cal_range_end,
            )
            .show(ui);

            if self.cal_range_start.is_some() {
                Spacing::Sm.show(ui);
                if ui.button("Clear").clicked() {
                    self.cal_range_start = None;
                    self.cal_range_end = None;
                }
            }
        });

        Spacing::Lg.show(ui);

        // ── With cell content ─────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Cell content",
                Some("Custom badges inside specific date cells."),
            );
            muted_text(ui, "Cells on the 1st, 10th, and 20th have event badges.");
            Spacing::Md.show(ui);

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
                    Spacing::Xs.show(ui);
                    muted_text(ui, "@radix-ui/primitives");
                    Spacing::Xs.show(ui);
                    muted_text(ui, "@radix-ui/colors");
                    Spacing::Xs.show(ui);
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

    pub(in crate::app) fn section_separator(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Separator", "Visually separates content — horizontal or vertical.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Horizontal", None);
            muted_text(ui, "My Account");
            Spacing::Md.show(ui);
            Separator::horizontal().show(ui);
            Spacing::Md.show(ui);
            muted_text(ui, "Profile");
            Spacing::Xs.show(ui);
            muted_text(ui, "Billing");
            Spacing::Xs.show(ui);
            muted_text(ui, "Settings");
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Vertical", None);
            ui.horizontal(|ui| {
                muted_text(ui, "Blog");
                Spacing::Sm.show(ui);
                Separator::vertical().length(16.0).show(ui);
                Spacing::Sm.show(ui);
                muted_text(ui, "Docs");
                Spacing::Sm.show(ui);
                Separator::vertical().length(16.0).show(ui);
                Spacing::Sm.show(ui);
                muted_text(ui, "Source");
            });
        });
    }

    pub(in crate::app) fn section_spacing(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Spacing", "Standardized spacing scale based on a 4px grid.");

        let theme = egui_shadcn::ShadcnTheme::get(ui.ctx());

        Card::new().show(ui, |ui| {
            card_header(ui, "Scale", Some("Xs=4 · Sm=8 · Md=12 · Lg=16 · Xl=24 · Xl2=32 · Xl3=48"));

            let variants: &[(&str, Spacing)] = &[
                ("Xs",  Spacing::Xs),
                ("Sm",  Spacing::Sm),
                ("Md",  Spacing::Md),
                ("Lg",  Spacing::Lg),
                ("Xl",  Spacing::Xl),
                ("Xl2", Spacing::Xl2),
                ("Xl3", Spacing::Xl3),
            ];

            for (name, sp) in variants {
                ui.horizontal(|ui| {
                    let label = format!("{:<4}  {:>2}px", name, u32::from(*sp));
                    muted_text(ui, &label);
                    Spacing::Md.show(ui);
                    let bar_w = sp.px();
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(bar_w, 8.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, egui::CornerRadius::same(2), theme.primary);
                });
                Spacing::Xs.show(ui);
            }
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Usage", Some("Spacing::Lg.show(ui)  or  f32::from(Spacing::Lg)"));
            muted_text(ui, "Block A");
            Spacing::Lg.show(ui);
            muted_text(ui, "Block B  (Lg = 16px gap above)");
            Spacing::Xl.show(ui);
            muted_text(ui, "Block C  (Xl = 24px gap above)");
        });
    }

    pub(in crate::app) fn section_boxed(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Boxed", "Transparent layout container with standard padding and margin.");

        let theme = egui_shadcn::ShadcnTheme::get(ui.ctx());

        Card::new().show(ui, |ui| {
            card_header(ui, "Padding variants", None);

            for (label, pad) in [
                ("Xs (4px)",  Spacing::Xs),
                ("Sm (8px)",  Spacing::Sm),
                ("Md (12px)", Spacing::Md),
                ("Lg (16px)", Spacing::Lg),
                ("Xl (24px)", Spacing::Xl),
            ] {
                muted_text(ui, label);
                Boxed::new().padding(pad).show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 24.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, egui::CornerRadius::same(4), theme.accent);
                });
            }
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Padding + margin", Some("Padding inside, margin outside."));
            Boxed::new()
                .padding(Spacing::Lg)
                .margin(Spacing::Sm)
                .show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 40.0),
                        egui::Sense::hover(),
                    );
                    ui.painter().rect_filled(rect, egui::CornerRadius::same(6), theme.primary);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Lg padding · Sm margin",
                        egui::FontId::new(12.0, egui::FontFamily::Proportional),
                        theme.primary_foreground,
                    );
                });
        });
    }
}
