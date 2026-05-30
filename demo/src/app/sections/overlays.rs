use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ShadcnTheme,
    avatar::Avatar, size::Size,
    button::{Button, ButtonVariant},
    card::{Card, card_header},
    context_menu::{ContextMenu, ContextItem},
    drawer::Drawer,
    dropdown_menu::{DropdownMenu, DropdownItem},
    hover_card::HoverCard,
    input::Input,
    popover::Popover,
    sheet::Sheet,
    tooltip::Tooltip,
    typography::{body_text, heading4, muted_text},
};
use crate::i18n as t;
use ::i18n::t as tr;

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_context_menu(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(18);
        let subtitle = tr!(t::ContextMenuSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let back = tr!(t::ContextMenuSec::Back);
        let fwd = tr!(t::ContextMenuSec::Forward);
        let reload = tr!(t::ContextMenuSec::Reload);
        let save_as = tr!(t::ContextMenuSec::SaveAs);

        let items = [
            ContextItem::Item { label: back.as_ref(),    shortcut: Some("⌘["), disabled: false },
            ContextItem::Item { label: fwd.as_ref(),     shortcut: Some("⌘]"), disabled: true },
            ContextItem::Item { label: reload.as_ref(),  shortcut: Some("⌘R"), disabled: false },
            ContextItem::Separator,
            ContextItem::Item { label: save_as.as_ref(), shortcut: Some("⌘S"), disabled: false },
        ];

        Card::new().show(ui, |ui| {
            let desc = tr!(t::ContextMenuSec::HAreaDesc);
            card_header(ui, tr!(t::ContextMenuSec::HArea).as_ref(), Some(desc.as_ref()));

            let area_hint = tr!(t::ContextMenuSec::AreaHint);
            let clicked = ContextMenu::new("demo_ctx", &items).show(ui, |ui| {
                let (rect, resp) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 80.0),
                    egui::Sense::click(),
                );
                let theme = ShadcnTheme::get(ui.ctx());
                ui.painter().rect_filled(rect, egui::CornerRadius::same(6), theme.muted);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    area_hint.as_ref(),
                    egui::FontId::new(13.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
                resp
            });

            if let Some(i) = clicked {
                let label = match i {
                    0 => back.as_ref(),
                    1 => fwd.as_ref(),
                    2 => reload.as_ref(),
                    _ => save_as.as_ref(),
                };
                self.context_last = Some(label.to_owned());
            }
            if let Some(ref last) = self.context_last {
                Spacing::Xs.show(ui);
                muted_text(ui, tr!(t::ContextMenuSec::Clicked, item = last).as_ref());
            }
        });
    }

    pub(in crate::app) fn section_drawer(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(22);
        let subtitle = tr!(t::DrawerSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::DrawerSec::HBottom).as_ref(), None);
            if Button::new(tr!(t::DrawerSec::Open).as_ref()).show(ui).clicked() {
                self.drawer_open = true;
            }
        });

        let ctx = ui.ctx().clone();
        let drawer_title = tr!(t::DrawerSec::Title);
        Drawer::new(drawer_title.as_ref(), &mut self.drawer_open).show(&ctx, |ui| {
            muted_text(ui, tr!(t::DrawerSec::Body).as_ref());
            Spacing::Lg.show(ui);
            let mut goal = String::from("350");
            Input::new(&mut goal).show(ui);
            Spacing::Lg.show(ui);
            if Button::new(tr!(t::DrawerSec::Submit).as_ref()).show(ui).clicked() {}
        });
    }

    pub(in crate::app) fn section_dropdown_menu(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(23);
        let subtitle = tr!(t::DropdownSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let prof = tr!(t::DropdownSec::Profile);
        let bill = tr!(t::DropdownSec::Billing);
        let set = tr!(t::DropdownSec::Settings);
        let logout = tr!(t::DropdownSec::Logout);

        let items = [
            DropdownItem::Item { label: prof.as_ref(),   disabled: false },
            DropdownItem::Item { label: bill.as_ref(),   disabled: false },
            DropdownItem::Item { label: set.as_ref(),    disabled: false },
            DropdownItem::Separator,
            DropdownItem::Item { label: logout.as_ref(), disabled: false },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::DropdownSec::HAccount).as_ref(), None);
            let my_account = tr!(t::DropdownSec::MyAccount);
            if let Some(i) = DropdownMenu::new("demo_dropdown", my_account.as_ref(), &items).show(ui) {
                let label = match i {
                    0 => prof.as_ref(),
                    1 => bill.as_ref(),
                    2 => set.as_ref(),
                    _ => logout.as_ref(),
                };
                self.dropdown_last = Some(label.to_owned());
            }
            if let Some(ref last) = self.dropdown_last {
                Spacing::Xs.show(ui);
                muted_text(ui, tr!(t::DropdownSec::Clicked, item = last).as_ref());
            }
        });
    }

    pub(in crate::app) fn section_hover_card(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(24);
        let subtitle = tr!(t::HoverCardSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::HoverCardSec::HProfileDesc);
            card_header(ui, tr!(t::HoverCardSec::HProfile).as_ref(), Some(desc.as_ref()));
            Spacing::Sm.show(ui);
            HoverCard::new("demo_hover_card").show(
                ui,
                |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.add(
                        egui::Label::new(
                            egui::RichText::new("@egui_shadcn")
                                .color(theme.primary)
                                .underline(),
                        )
                        .sense(egui::Sense::hover()),
                    )
                },
                |ui| {
                    ui.horizontal(|ui| {
                        Avatar::new("ES").size(Size::Sm).show(ui);
                        Spacing::Sm.show(ui);
                        ui.vertical(|ui| {
                            body_text(ui, "egui-shadcn");
                            muted_text(ui, "@egui_shadcn");
                        });
                    });
                    Spacing::Sm.show(ui);
                    muted_text(ui, tr!(t::HoverCardSec::Body).as_ref());
                },
            );
        });
    }

    pub(in crate::app) fn section_popover(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(31);
        let subtitle = tr!(t::PopoverSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::PopoverSec::HDimensions).as_ref(), None);
            Popover::new("demo_popover").show(
                ui,
                |ui| Button::new(tr!(t::PopoverSec::Open).as_ref()).show(ui),
                |ui| {
                    heading4(ui, tr!(t::PopoverSec::Title).as_ref());
                    Spacing::Sm.show(ui);
                    muted_text(ui, tr!(t::PopoverSec::Desc).as_ref());
                    Spacing::Md.show(ui);
                    body_text(ui, tr!(t::PopoverSec::Width).as_ref());
                    Spacing::Xs.show(ui);
                    let mut w = String::from("100%");
                    Input::new(&mut w).show(ui);
                    Spacing::Sm.show(ui);
                    body_text(ui, tr!(t::PopoverSec::MaxWidth).as_ref());
                    Spacing::Xs.show(ui);
                    let mut mw = String::from("300px");
                    Input::new(&mut mw).show(ui);
                },
            );
        });
    }

    pub(in crate::app) fn section_sheet(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(37);
        let subtitle = tr!(t::SheetSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SheetSec::HRight).as_ref(), None);
            if Button::new(tr!(t::SheetSec::Open).as_ref()).show(ui).clicked() {
                self.sheet_open = true;
            }
        });

        let ctx = ui.ctx().clone();
        let sheet_title = tr!(t::SheetSec::Title);
        Sheet::new(sheet_title.as_ref(), &mut self.sheet_open).show(&ctx, |ui| {
            muted_text(ui, tr!(t::SheetSec::Body).as_ref());
            Spacing::Lg.show(ui);
            let mut name = String::from("Pedro Duarte");
            Input::new(&mut name).show(ui);
            Spacing::Sm.show(ui);
            let mut username = String::from("@peduarte");
            Input::new(&mut username).show(ui);
            Spacing::Lg.show(ui);
            if Button::new(tr!(t::SheetSec::Save).as_ref()).show(ui).clicked() {}
        });
    }

    pub(in crate::app) fn section_tooltip(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(49);
        let subtitle = tr!(t::TooltipSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::TooltipSec::HBasic).as_ref(), None);
            let helpful = tr!(t::TooltipSec::Helpful);
            Tooltip::new(helpful.as_ref()).wrap(ui, |ui| {
                Button::new(tr!(t::TooltipSec::HoverMe).as_ref()).variant(ButtonVariant::Outline).show(ui)
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::TooltipSec::HDescriptive).as_ref(), None);
            let opens = tr!(t::TooltipSec::OpensPalette);
            Tooltip::new(opens.as_ref()).wrap(ui, |ui| {
                Button::new(tr!(t::TooltipSec::CmdPalette).as_ref()).variant(ButtonVariant::Secondary).show(ui)
            });
        });
    }
}
