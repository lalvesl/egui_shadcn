use crate::i18n as t;
use ::i18n::t as tr;
use egui::Color32;
use egui_shadcn::{
    ShadcnTheme,
    avatar::Avatar,
    boxed::Boxed,
    calendar::Calendar,
    card::{Card, card_header},
    collapsible::Collapsible,
    data_table::{DataColumn, DataTable},
    separator::Separator,
    size::Size,
    spacing::Spacing,
    table::{Table, TableColumn},
    typography::{body_text, muted_text},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_avatar(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(4);
        let subtitle = tr!(t::AvatarSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::AvatarSec::HSizes).as_ref(), None);
            ui.horizontal(|ui| {
                Avatar::new("JD").size(Size::Sm).show(ui);
                Spacing::Sm.show(ui);
                Avatar::new("Alice").show(ui);
                Spacing::Sm.show(ui);
                Avatar::new("Bob").size(Size::Lg).show(ui);
                Spacing::Sm.show(ui);
                Avatar::new("XY")
                    .color(Color32::from_rgb(139, 92, 246))
                    .show(ui);
            });
        });
    }

    pub(in crate::app) fn section_boxed(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(6);
        let subtitle = tr!(t::BoxedSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let theme = ShadcnTheme::get(ui.ctx());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BoxedSec::HPadding).as_ref(), None);
            let entries: &[(t::BoxedSec, Spacing)] = &[
                (t::BoxedSec::LblXs, Spacing::Xs),
                (t::BoxedSec::LblSm, Spacing::Sm),
                (t::BoxedSec::LblMd, Spacing::Md),
                (t::BoxedSec::LblLg, Spacing::Lg),
                (t::BoxedSec::LblXl, Spacing::Xl),
            ];
            for (label_v, pad) in entries {
                let label = tr!(*label_v);
                muted_text(ui, label.as_ref());
                Boxed::new().padding(*pad).show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 24.0),
                        egui::Sense::hover(),
                    );
                    ui.painter()
                        .rect_filled(rect, egui::CornerRadius::same(4), theme.accent);
                });
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            let pad_margin_desc = tr!(t::BoxedSec::HPadMarginDesc);
            card_header(
                ui,
                tr!(t::BoxedSec::HPadMargin).as_ref(),
                Some(pad_margin_desc.as_ref()),
            );
            let inner_label = tr!(t::BoxedSec::InnerLabel);
            Boxed::new()
                .padding(Spacing::Lg)
                .margin(Spacing::Sm)
                .show(ui, |ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::Vec2::new(ui.available_width(), 40.0),
                        egui::Sense::hover(),
                    );
                    ui.painter()
                        .rect_filled(rect, egui::CornerRadius::same(6), theme.primary);
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        inner_label.as_ref(),
                        egui::FontId::new(12.0, egui::FontFamily::Proportional),
                        theme.primary_foreground,
                    );
                });
        });
    }

    pub(in crate::app) fn section_calendar(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(10);
        let subtitle = tr!(t::CalendarSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::CalendarSec::HSingle).as_ref(), None);
            let label: String = match self.cal_single {
                Some(d) => {
                    let dstr = format!("{:04}-{:02}-{:02}", d.year, d.month, d.day);
                    tr!(t::CalendarSec::SelectedDate, date = dstr).into_owned()
                }
                None => tr!(t::CalendarSec::NoDate).into_owned(),
            };
            muted_text(ui, &label);
            Spacing::Md.show(ui);
            Calendar::single("demo_cal_single", &mut self.cal_single).show(ui);
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            let range_desc = tr!(t::CalendarSec::HRangeDesc);
            card_header(
                ui,
                tr!(t::CalendarSec::HRange).as_ref(),
                Some(range_desc.as_ref()),
            );
            let label: String = match (self.cal_range_start, self.cal_range_end) {
                (Some(s), Some(e)) => format!(
                    "{:04}-{:02}-{:02}  →  {:04}-{:02}-{:02}",
                    s.year, s.month, s.day, e.year, e.month, e.day
                ),
                (Some(s), None) => {
                    let sstr = format!("{:04}-{:02}-{:02}", s.year, s.month, s.day);
                    tr!(t::CalendarSec::RangePending, start = sstr).into_owned()
                }
                _ => tr!(t::CalendarSec::RangeNone).into_owned(),
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
                if ui.button(tr!(t::CalendarSec::Clear).as_ref()).clicked() {
                    self.cal_range_start = None;
                    self.cal_range_end = None;
                }
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            let compact_desc = tr!(t::CalendarSec::HRangeCompactDesc);
            card_header(
                ui,
                tr!(t::CalendarSec::HRangeCompact).as_ref(),
                Some(compact_desc.as_ref()),
            );
            let label: String = match (self.cal_range_compact_start, self.cal_range_compact_end) {
                (Some(s), Some(e)) => format!(
                    "{:04}-{:02}-{:02}  →  {:04}-{:02}-{:02}",
                    s.year, s.month, s.day, e.year, e.month, e.day
                ),
                (Some(s), None) => {
                    let sstr = format!("{:04}-{:02}-{:02}", s.year, s.month, s.day);
                    tr!(t::CalendarSec::RangePending, start = sstr).into_owned()
                }
                _ => tr!(t::CalendarSec::RangeNone).into_owned(),
            };
            muted_text(ui, &label);
            Spacing::Md.show(ui);
            Calendar::range(
                "demo_cal_range_compact",
                &mut self.cal_range_compact_start,
                &mut self.cal_range_compact_end,
            )
            .compact()
            .show(ui);
            if self.cal_range_compact_start.is_some() {
                Spacing::Sm.show(ui);
                if ui.button(tr!(t::CalendarSec::Clear).as_ref()).clicked() {
                    self.cal_range_compact_start = None;
                    self.cal_range_compact_end = None;
                }
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            let prices_desc = tr!(t::CalendarSec::HPricesDesc);
            card_header(
                ui,
                tr!(t::CalendarSec::HPrices).as_ref(),
                Some(prices_desc.as_ref()),
            );
            let label: String = match self.cal_prices_selected {
                Some(d) => {
                    let dstr = format!("{:04}-{:02}-{:02}", d.year, d.month, d.day);
                    tr!(t::CalendarSec::SelectedDate, date = dstr).into_owned()
                }
                None => tr!(t::CalendarSec::NoDate).into_owned(),
            };
            muted_text(ui, &label);
            Spacing::Md.show(ui);
            Calendar::single("demo_cal_prices", &mut self.cal_prices_selected)
                .cell_height(52.0)
                .cell_content(|ui, date| {
                    let hash = (date.day as u32)
                        .wrapping_mul(7919)
                        .wrapping_add((date.month as u32).wrapping_mul(6271));
                    let price = hash % 150 + 49;
                    let cents = (hash / 200) % 10;
                    let price_str = format!("${price}.{cents}0");
                    let theme = ShadcnTheme::get(ui.ctx());
                    let color = if price < 99 {
                        theme.primary
                    } else {
                        theme.muted_foreground
                    };
                    ui.painter().text(
                        ui.max_rect().center(),
                        egui::Align2::CENTER_CENTER,
                        &price_str,
                        egui::FontId::new(9.0, egui::FontFamily::Proportional),
                        color,
                    );
                })
                .show(ui);
        });
    }

    pub(in crate::app) fn section_card(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(11);
        let subtitle = tr!(t::CardSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let card_desc = tr!(t::CardSec::HTitleDesc);
            card_header(
                ui,
                tr!(t::CardSec::HTitle).as_ref(),
                Some(card_desc.as_ref()),
            );
            body_text(ui, tr!(t::CardSec::Body).as_ref());
        });
        Spacing::Lg.show(ui);
        Card::new().padding(16.0).show(ui, |ui| {
            card_header(ui, tr!(t::CardSec::HUser).as_ref(), None);
            ui.horizontal(|ui| {
                Avatar::new("JD").show(ui);
                Spacing::Md.show(ui);
                ui.vertical(|ui| {
                    body_text(ui, tr!(t::CardSec::UserName).as_ref());
                    muted_text(ui, tr!(t::CardSec::UserEmail).as_ref());
                });
            });
        });
    }

    pub(in crate::app) fn section_collapsible(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(15);
        let subtitle = tr!(t::CollapsibleSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let heading = tr!(t::CollapsibleSec::Heading);
            card_header(ui, heading.as_ref(), None);
            Collapsible::new(
                "demo_collapsible",
                heading.as_ref(),
                &mut self.collapsible_open,
            )
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

    pub(in crate::app) fn section_data_table(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(19);
        let subtitle = tr!(t::DataTableSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let col_status = tr!(t::DataTableSec::ColStatus);
        let col_email = tr!(t::DataTableSec::ColEmail);
        let col_amount = tr!(t::DataTableSec::ColAmount);
        let columns = &[
            DataColumn {
                header: col_status.as_ref(),
                width: Some(100.0),
                sortable: true,
            },
            DataColumn {
                header: col_email.as_ref(),
                width: None,
                sortable: true,
            },
            DataColumn {
                header: col_amount.as_ref(),
                width: Some(120.0),
                sortable: true,
            },
        ];

        let s_success = tr!(t::DataTableSec::StatusSuccess);
        let s_processing = tr!(t::DataTableSec::StatusProcessing);
        let s_failed = tr!(t::DataTableSec::StatusFailed);

        let data: &[(&str, &str, &str)] = &[
            (s_success.as_ref(), "ken99@yahoo.com", "$316.00"),
            (s_success.as_ref(), "abe45@gmail.com", "$242.00"),
            (s_processing.as_ref(), "monserrat44@gmail.com", "$837.00"),
            (s_success.as_ref(), "silas22@gmail.com", "$874.00"),
            (s_failed.as_ref(), "carmella@hotmail.com", "$721.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::DataTableSec::HPayments).as_ref(), None);
            DataTable::new("demo_data_table", columns, &mut self.data_table_filter)
                .page_size(3)
                .show(ui, data.len(), |row_idx, row| {
                    let (status, email, amount) = data[row_idx];
                    row.cell(|ui| {
                        muted_text(ui, status);
                    });
                    row.cell(|ui| {
                        muted_text(ui, email);
                    });
                    row.cell(|ui| {
                        muted_text(ui, amount);
                    });
                });
        });
    }

    pub(in crate::app) fn section_separator(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(36);
        let subtitle = tr!(t::SeparatorSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SeparatorSec::HHoriz).as_ref(), None);
            muted_text(ui, tr!(t::SeparatorSec::MyAccount).as_ref());
            Spacing::Md.show(ui);
            Separator::horizontal().show(ui);
            Spacing::Md.show(ui);
            muted_text(ui, tr!(t::SeparatorSec::Profile).as_ref());
            Spacing::Xs.show(ui);
            muted_text(ui, tr!(t::SeparatorSec::Billing).as_ref());
            Spacing::Xs.show(ui);
            muted_text(ui, tr!(t::SeparatorSec::Settings).as_ref());
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SeparatorSec::HVert).as_ref(), None);
            ui.horizontal(|ui| {
                muted_text(ui, tr!(t::SeparatorSec::Blog).as_ref());
                Spacing::Sm.show(ui);
                Separator::vertical().length(16.0).show(ui);
                Spacing::Sm.show(ui);
                muted_text(ui, tr!(t::SeparatorSec::Docs).as_ref());
                Spacing::Sm.show(ui);
                Separator::vertical().length(16.0).show(ui);
                Spacing::Sm.show(ui);
                muted_text(ui, tr!(t::SeparatorSec::Source).as_ref());
            });
        });
    }

    pub(in crate::app) fn section_spacing(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(40);
        let subtitle = tr!(t::SpacingSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let theme = ShadcnTheme::get(ui.ctx());

        Card::new().show(ui, |ui| {
            let scale_desc = tr!(t::SpacingSec::HScaleDesc);
            card_header(
                ui,
                tr!(t::SpacingSec::HScale).as_ref(),
                Some(scale_desc.as_ref()),
            );
            for (name, sp) in [
                ("Xs", Spacing::Xs),
                ("Sm", Spacing::Sm),
                ("Md", Spacing::Md),
                ("Lg", Spacing::Lg),
                ("Xl", Spacing::Xl),
                ("Xl2", Spacing::Xl2),
                ("Xl3", Spacing::Xl3),
            ] {
                ui.horizontal(|ui| {
                    muted_text(ui, &format!("{:<4}  {:>2}px", name, u32::from(sp)));
                    Spacing::Md.show(ui);
                    let (rect, _) =
                        ui.allocate_exact_size(egui::Vec2::new(sp.px(), 8.0), egui::Sense::hover());
                    ui.painter()
                        .rect_filled(rect, egui::CornerRadius::same(2), theme.primary);
                });
                Spacing::Xs.show(ui);
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            let usage_desc = tr!(t::SpacingSec::HUsageDesc);
            card_header(
                ui,
                tr!(t::SpacingSec::HUsage).as_ref(),
                Some(usage_desc.as_ref()),
            );
            muted_text(ui, tr!(t::SpacingSec::BlockA).as_ref());
            Spacing::Lg.show(ui);
            muted_text(ui, tr!(t::SpacingSec::BlockB).as_ref());
            Spacing::Xl.show(ui);
            muted_text(ui, tr!(t::SpacingSec::BlockC).as_ref());
        });
    }

    pub(in crate::app) fn section_table(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(43);
        let subtitle = tr!(t::TableSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let col_inv = tr!(t::TableSec::ColInvoice);
        let col_status = tr!(t::TableSec::ColStatus);
        let col_method = tr!(t::TableSec::ColMethod);
        let col_amount = tr!(t::TableSec::ColAmount);
        let columns = &[
            TableColumn {
                header: col_inv.as_ref(),
                width: Some(120.0),
            },
            TableColumn {
                header: col_status.as_ref(),
                width: Some(100.0),
            },
            TableColumn {
                header: col_method.as_ref(),
                width: Some(120.0),
            },
            TableColumn {
                header: col_amount.as_ref(),
                width: None,
            },
        ];

        let s_paid = tr!(t::TableSec::StatusPaid);
        let s_pending = tr!(t::TableSec::StatusPending);
        let s_unpaid = tr!(t::TableSec::StatusUnpaid);
        let m_cc = tr!(t::TableSec::MethodCreditCard);
        let m_paypal = tr!(t::TableSec::MethodPayPal);
        let m_bank = tr!(t::TableSec::MethodBank);

        let data: &[(&str, &str, &str, &str)] = &[
            ("INV001", s_paid.as_ref(), m_cc.as_ref(), "$250.00"),
            ("INV002", s_pending.as_ref(), m_paypal.as_ref(), "$150.00"),
            ("INV003", s_unpaid.as_ref(), m_bank.as_ref(), "$350.00"),
            ("INV004", s_paid.as_ref(), m_cc.as_ref(), "$450.00"),
            ("INV005", s_paid.as_ref(), m_paypal.as_ref(), "$550.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::TableSec::HInvoices).as_ref(), None);
            Table::new(columns).show(ui, data.len(), |row_idx, row| {
                let (inv, status, method, amount) = data[row_idx];
                row.cell(|ui| {
                    muted_text(ui, inv);
                });
                row.cell(|ui| {
                    muted_text(ui, status);
                });
                row.cell(|ui| {
                    muted_text(ui, method);
                });
                row.cell(|ui| {
                    muted_text(ui, amount);
                });
            });
        });
    }
}
