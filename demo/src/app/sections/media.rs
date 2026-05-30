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
use crate::i18n as t;
use ::i18n::t as tr;

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_carousel(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(12);
        let subtitle = tr!(t::CarouselSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::CarouselSec::HItems).as_ref(), None);
            Carousel::new("demo_carousel", 5).height(180.0).show(ui, |idx, ui| {
                let theme = ShadcnTheme::get(ui.ctx());
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 160.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(rect, egui::CornerRadius::same(8), theme.muted);
                let n = idx + 1;
                let slide_label = tr!(t::CarouselSec::Slide, n = n);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    slide_label.as_ref(),
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
            });
        });
    }

    pub(in crate::app) fn section_chart(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(13);
        let subtitle = tr!(t::ChartSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let m1 = egui_shadcn::i18n::month_short(1);
        let m2 = egui_shadcn::i18n::month_short(2);
        let m3 = egui_shadcn::i18n::month_short(3);
        let m4 = egui_shadcn::i18n::month_short(4);
        let m5 = egui_shadcn::i18n::month_short(5);
        let m6 = egui_shadcn::i18n::month_short(6);
        let months = &[m1.as_ref(), m2.as_ref(), m3.as_ref(), m4.as_ref(), m5.as_ref(), m6.as_ref()];
        let desktop = &[186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
        let mobile = &[80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ChartSec::HBar).as_ref(), None);
            let desktop_lbl = tr!(t::ChartSec::Desktop);
            let mobile_lbl = tr!(t::ChartSec::Mobile);
            let datasets = &[
                ChartDataset { label: desktop_lbl.as_ref(), values: desktop, color: None },
                ChartDataset { label: mobile_lbl.as_ref(),  values: mobile, color: Some(egui::Color32::from_rgb(100, 160, 240)) },
            ];
            Chart::new(datasets, months).height(200.0).show_legend(true).show(ui);
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ChartSec::HLine).as_ref(), None);
            let desktop_lbl = tr!(t::ChartSec::Desktop);
            let datasets = &[
                ChartDataset { label: desktop_lbl.as_ref(), values: desktop, color: None },
            ];
            Chart::new(datasets, months).kind(ChartKind::Line).height(200.0).show_grid(true).show(ui);
        });
    }

    pub(in crate::app) fn section_command(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(17);
        let subtitle = tr!(t::CommandSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::CommandSec::HPaletteDesc);
            card_header(ui, tr!(t::CommandSec::HPalette).as_ref(), Some(desc.as_ref()));
            if Button::new(tr!(t::CommandSec::Open).as_ref()).variant(ButtonVariant::Outline).show(ui).clicked() {
                self.command_open = true;
            }
        });

        let grp_sugg = tr!(t::CommandSec::GrpSuggestions);
        let grp_set = tr!(t::CommandSec::GrpSettings);
        let i_cal = tr!(t::CommandSec::ItemCalendar);
        let i_emoji = tr!(t::CommandSec::ItemEmoji);
        let i_calc = tr!(t::CommandSec::ItemCalculator);
        let i_prof = tr!(t::CommandSec::ItemProfile);
        let i_bill = tr!(t::CommandSec::ItemBilling);
        let i_set = tr!(t::CommandSec::ItemSettings);
        let placeholder = tr!(t::CommandSec::Placeholder);

        let suggestion_items = [
            CommandItem { label: i_cal.as_ref(),   description: None, shortcut: None, icon: None },
            CommandItem { label: i_emoji.as_ref(), description: None, shortcut: None, icon: None },
            CommandItem { label: i_calc.as_ref(),  description: None, shortcut: None, icon: None },
        ];
        let settings_items = [
            CommandItem { label: i_prof.as_ref(), description: Some("⌘P"), shortcut: None, icon: None },
            CommandItem { label: i_bill.as_ref(), description: None,       shortcut: None, icon: None },
            CommandItem { label: i_set.as_ref(),  description: Some("⌘S"), shortcut: None, icon: None },
        ];
        let groups = [
            CommandGroup { heading: Some(grp_sugg.as_ref()), items: &suggestion_items },
            CommandGroup { heading: Some(grp_set.as_ref()),  items: &settings_items },
        ];

        let ctx = ui.ctx().clone();
        Command::new("demo_command", &groups, &mut self.command_open)
            .placeholder(placeholder.as_ref())
            .show(&ctx);
    }

    pub(in crate::app) fn section_resizable(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(34);
        let subtitle = tr!(t::ResizableSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().padding(0.0).show(ui, |ui| {
            Resizable::new("demo_resizable_h")
                .dir(ResizeDir::Horizontal)
                .initial_split(0.33)
                .height(220.0)
                .show(ui,
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let rect = ui.available_rect_before_wrap();
                        let one = tr!(t::ResizableSec::One);
                        ui.painter().text(
                            rect.center(), egui::Align2::CENTER_CENTER, one.as_ref(),
                            egui::FontId::new(13.0, egui::FontFamily::Proportional),
                            theme.muted_foreground,
                        );
                        let _ = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
                    },
                    |ui| {
                        Resizable::new("demo_resizable_v")
                            .dir(ResizeDir::Vertical)
                            .initial_split(0.5)
                            .show(ui,
                                |ui| {
                                    let theme = ShadcnTheme::get(ui.ctx());
                                    let rect = ui.available_rect_before_wrap();
                                    let two = tr!(t::ResizableSec::Two);
                                    ui.painter().text(
                                        rect.center(), egui::Align2::CENTER_CENTER, two.as_ref(),
                                        egui::FontId::new(13.0, egui::FontFamily::Proportional),
                                        theme.muted_foreground,
                                    );
                                    let _ = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
                                },
                                |ui| {
                                    let theme = ShadcnTheme::get(ui.ctx());
                                    let rect = ui.available_rect_before_wrap();
                                    let three = tr!(t::ResizableSec::Three);
                                    ui.painter().text(
                                        rect.center(), egui::Align2::CENTER_CENTER, three.as_ref(),
                                        egui::FontId::new(13.0, egui::FontFamily::Proportional),
                                        theme.muted_foreground,
                                    );
                                    let _ = ui.allocate_exact_size(rect.size(), egui::Sense::hover());
                                },
                            );
                    },
                );
        });
    }
}
