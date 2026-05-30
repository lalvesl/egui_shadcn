use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ICON_ADD, ICON_DOWNLOAD, ICON_FORMAT_BOLD, ICON_FORMAT_ITALIC, ICON_FORMAT_STRIKETHROUGH,
    ICON_FORMAT_UNDERLINE, ICON_MAIL, ICON_SEARCH, ICON_SEND,
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupVariant},
    card::{Card, card_header},
    toggle::Toggle,
    toggle_group::{ToggleGroup, ToggleGroupItem},
    typography::muted_text,
};
use crate::i18n as t;
use ::i18n::t as tr;

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_button(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(8);
        let subtitle = tr!(t::BtnSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnSec::HVariants).as_ref(), None);
            ui.horizontal_wrapped(|ui| {
                if Button::new(tr!(t::BtnSec::Default).as_ref()).show(ui).clicked() { self.btn_clicked = true; }
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Destructive).as_ref()).variant(ButtonVariant::Destructive).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Outline).as_ref()).variant(ButtonVariant::Outline).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Secondary).as_ref()).variant(ButtonVariant::Secondary).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Ghost).as_ref()).variant(ButtonVariant::Ghost).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Link).as_ref()).variant(ButtonVariant::Link).show(ui);
            });
            if self.btn_clicked {
                Spacing::Sm.show(ui);
                muted_text(ui, tr!(t::BtnSec::Clicked).as_ref());
                if ui.small_button(tr!(t::BtnSec::Dismiss).as_ref()).clicked() { self.btn_clicked = false; }
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnSec::HSizes).as_ref(), None);
            ui.horizontal(|ui| {
                Button::new(tr!(t::BtnSec::Small).as_ref()).size(ButtonSize::Sm).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Default).as_ref()).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Large).as_ref()).size(ButtonSize::Lg).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnSec::HStates).as_ref(), None);
            ui.horizontal(|ui| {
                Button::new(tr!(t::BtnSec::Enabled).as_ref()).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Disabled).as_ref()).enabled(false).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnSec::HWithIcon).as_ref(), None);
            ui.horizontal_wrapped(|ui| {
                Button::new(tr!(t::BtnSec::Send).as_ref()).icon(ICON_SEND).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Mail).as_ref()).icon(ICON_MAIL).variant(ButtonVariant::Outline).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Download).as_ref()).icon(ICON_DOWNLOAD).variant(ButtonVariant::Secondary).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Search).as_ref()).icon(ICON_SEARCH).variant(ButtonVariant::Ghost).show(ui);
                Spacing::Sm.show(ui);
                Button::new(tr!(t::BtnSec::Add).as_ref()).icon(ICON_ADD).variant(ButtonVariant::Destructive).show(ui);
            });
        });
    }

    pub(in crate::app) fn section_badge(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(5);
        let subtitle = tr!(t::BadgeSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BadgeSec::HVariants).as_ref(), None);
            ui.horizontal(|ui| {
                Badge::new(tr!(t::BadgeSec::Default).as_ref()).show(ui);
                Spacing::Sm.show(ui);
                Badge::new(tr!(t::BadgeSec::Secondary).as_ref()).variant(BadgeVariant::Secondary).show(ui);
                Spacing::Sm.show(ui);
                Badge::new(tr!(t::BadgeSec::Destructive).as_ref()).variant(BadgeVariant::Destructive).show(ui);
                Spacing::Sm.show(ui);
                Badge::new(tr!(t::BadgeSec::Outline).as_ref()).variant(BadgeVariant::Outline).show(ui);
            });
        });
    }

    pub(in crate::app) fn section_button_group(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(9);
        let subtitle = tr!(t::BtnGrpSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnGrpSec::HSingle).as_ref(), None);
            let months = tr!(t::BtnGrpSec::Months);
            let weeks = tr!(t::BtnGrpSec::Weeks);
            let days = tr!(t::BtnGrpSec::Days);
            let labels = &[months.as_ref(), weeks.as_ref(), days.as_ref()];
            if let Some(i) = ButtonGroup::new(labels).selected(self.button_group_sel).show(ui) {
                self.button_group_sel = Some(i);
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BtnGrpSec::HOutline).as_ref(), None);
            let left = tr!(t::BtnGrpSec::Left);
            let center = tr!(t::BtnGrpSec::Center);
            let right = tr!(t::BtnGrpSec::Right);
            let labels = &[left.as_ref(), center.as_ref(), right.as_ref()];
            if let Some(i) = ButtonGroup::new(labels)
                .variant(ButtonGroupVariant::Outline)
                .selected(self.button_group_outline_sel)
                .show(ui)
            {
                self.button_group_outline_sel = Some(i);
            }
        });
    }

    pub(in crate::app) fn section_toggle(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(47);
        let subtitle = tr!(t::ToggleSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ToggleSec::HBasic).as_ref(), None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle1, tr!(t::ToggleSec::Bold).as_ref()).show(ui);
                Toggle::new(&mut self.toggle2, tr!(t::ToggleSec::Italic).as_ref()).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ToggleSec::HWithIcon).as_ref(), None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle_bold, tr!(t::ToggleSec::Bold).as_ref()).icon(ICON_FORMAT_BOLD).show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_italic, tr!(t::ToggleSec::Italic).as_ref()).icon(ICON_FORMAT_ITALIC).show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_underline, tr!(t::ToggleSec::Underline).as_ref()).icon(ICON_FORMAT_UNDERLINE).show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_strikethrough, tr!(t::ToggleSec::Strikethrough).as_ref()).icon(ICON_FORMAT_STRIKETHROUGH).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ToggleSec::HDisabled).as_ref(), None);
            let mut dummy = true;
            ui.horizontal(|ui| {
                Toggle::new(&mut dummy, tr!(t::ToggleSec::DisabledOn).as_ref()).enabled(false).show(ui);
                let mut dummy2 = false;
                Toggle::new(&mut dummy2, tr!(t::ToggleSec::DisabledOff).as_ref()).enabled(false).show(ui);
            });
        });
    }

    pub(in crate::app) fn section_toggle_group(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(48);
        let subtitle = tr!(t::ToggleGrpSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let day = tr!(t::ToggleGrpSec::Day);
        let week = tr!(t::ToggleGrpSec::Week);
        let month = tr!(t::ToggleGrpSec::Month);

        let items: &[(u8, ToggleGroupItem<'_>)] = &[
            (0, ToggleGroupItem { label: day.as_ref(),   icon: None }),
            (1, ToggleGroupItem { label: week.as_ref(),  icon: None }),
            (2, ToggleGroupItem { label: month.as_ref(), icon: None }),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ToggleGrpSec::HView).as_ref(), None);
            ToggleGroup::new(items, &mut self.toggle_group_val).show(ui);
            Spacing::Xs.show(ui);
            let names = [day.as_ref(), week.as_ref(), month.as_ref()];
            let selected_name = names[self.toggle_group_val as usize];
            muted_text(ui, tr!(t::ToggleGrpSec::Selected, value = selected_name).as_ref());
        });
    }
}
