use crate::i18n as t;
use ::i18n::t as tr;
use egui_sc::egui_components::spacing::Spacing;
use egui_sc::egui_components::{
    card::{Card, card_header},
    checkbox::Checkbox,
    combobox::Combobox,
    date_picker::DatePicker,
    input::Input,
    input_otp::InputOtp,
    label::Label,
    radio::Radio,
    select::Select,
    slider::Slider,
    switch::Switch,
    textarea::Textarea,
    typography::muted_text,
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_checkbox(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(14);
        let subtitle = tr!(t::CheckboxSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::CheckboxSec::HOptions).as_ref(), None);
            Checkbox::new(&mut self.checkbox1)
                .label(tr!(t::CheckboxSec::Terms).as_ref())
                .show(ui);
            Spacing::Sm.show(ui);
            Checkbox::new(&mut self.checkbox2)
                .label(tr!(t::CheckboxSec::Newsletter).as_ref())
                .show(ui);
            Spacing::Sm.show(ui);
            Checkbox::new(&mut self.checkbox3)
                .label(tr!(t::CheckboxSec::Disabled).as_ref())
                .enabled(false)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_combobox(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(16);
        let subtitle = tr!(t::ComboboxSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let frameworks = &[
            "Next.js",
            "SvelteKit",
            "Nuxt.js",
            "Remix",
            "Astro",
            "SolidStart",
        ];
        Card::new().show(ui, |ui| {
            let desc = tr!(t::ComboboxSec::HFrameworkDesc);
            card_header(
                ui,
                tr!(t::ComboboxSec::HFramework).as_ref(),
                Some(desc.as_ref()),
            );
            let placeholder = tr!(t::ComboboxSec::Placeholder);
            Combobox::new("demo_combobox", &mut self.combobox_val, frameworks)
                .placeholder(placeholder.as_ref())
                .show(ui);
            if let Some(i) = self.combobox_val {
                Spacing::Xs.show(ui);
                muted_text(
                    ui,
                    tr!(t::ComboboxSec::Selected, value = frameworks[i])
                        .as_ref(),
                );
            }
        });
    }

    pub(in crate::app) fn section_date_picker(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(20);
        let subtitle = tr!(t::DatePickerSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::DatePickerSec::HPick).as_ref(), None);
            DatePicker::new("demo_date_picker", &mut self.date_picker_val)
                .show(ui);
            if let Some(d) = self.date_picker_val {
                Spacing::Xs.show(ui);
                let dstr = format!("{:04}-{:02}-{:02}", d.year, d.month, d.day);
                muted_text(
                    ui,
                    tr!(t::DatePickerSec::Selected, date = dstr).as_ref(),
                );
            }
        });
    }

    pub(in crate::app) fn section_input(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(25);
        let subtitle = tr!(t::InputSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::InputSec::HText).as_ref(), None);
            let username = tr!(t::InputSec::Username);
            let username_ph = tr!(t::InputSec::UsernamePh);
            Input::new(&mut self.input_text)
                .label(username.as_ref())
                .placeholder(username_ph.as_ref())
                .show(ui);
            Spacing::Md.show(ui);
            let password = tr!(t::InputSec::Password);
            Input::new(&mut self.password_text)
                .label(password.as_ref())
                .placeholder("••••••••")
                .password(true)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_input_otp(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(26);
        let subtitle = tr!(t::InputOtpSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::InputOtpSec::HOtpDesc);
            card_header(
                ui,
                tr!(t::InputOtpSec::HOtp).as_ref(),
                Some(desc.as_ref()),
            );
            InputOtp::new(&mut self.otp_val, 6)
                .separator_after(3)
                .show(ui);
            if self.otp_val.len() == 6 {
                Spacing::Xs.show(ui);
                let code = self.otp_val.clone();
                muted_text(ui, tr!(t::InputOtpSec::Code, code = code).as_ref());
            }
        });
    }

    pub(in crate::app) fn section_label(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(27);
        let subtitle = tr!(t::LabelSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::LabelSec::HBasic).as_ref(), None);
            Label::new(tr!(t::LabelSec::Email).as_ref()).show(ui);
            Spacing::Xs.show(ui);
            Label::new(tr!(t::LabelSec::Required).as_ref())
                .required(true)
                .show(ui);
            Spacing::Xs.show(ui);
            Label::new(tr!(t::LabelSec::Password).as_ref()).show(ui);
        });
    }

    pub(in crate::app) fn section_radio(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(33);
        let subtitle = tr!(t::RadioSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::RadioSec::HMethod).as_ref(), None);
            Radio::new(&mut self.radio_val, 0u32)
                .label(tr!(t::RadioSec::OptA).as_ref())
                .show(ui);
            Spacing::Sm.show(ui);
            Radio::new(&mut self.radio_val, 1u32)
                .label(tr!(t::RadioSec::OptB).as_ref())
                .show(ui);
            Spacing::Sm.show(ui);
            Radio::new(&mut self.radio_val, 2u32)
                .label(tr!(t::RadioSec::OptC).as_ref())
                .show(ui);
        });
    }

    pub(in crate::app) fn section_select(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(35);
        let subtitle = tr!(t::SelectSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SelectSec::HFramework).as_ref(), None);
            Label::new(tr!(t::SelectSec::Framework).as_ref()).show(ui);
            Spacing::Xs.show(ui);
            let placeholder = tr!(t::SelectSec::Placeholder);
            Select::new(
                &mut self.select_val,
                &["React", "Vue", "Svelte", "egui"],
            )
            .placeholder(placeholder.as_ref())
            .show(ui);
            if let Some(i) = self.select_val {
                Spacing::Xs.show(ui);
                muted_text(ui, ["React", "Vue", "Svelte", "egui"][i]);
            }
        });
    }

    pub(in crate::app) fn section_slider(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(39);
        let subtitle = tr!(t::SliderSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SliderSec::HVolume).as_ref(), None);
            let val_str = format!("{:.0}", self.slider_val);
            muted_text(ui, tr!(t::SliderSec::Value, value = val_str).as_ref());
            Spacing::Sm.show(ui);
            Slider::new(&mut self.slider_val, 0.0, 100.0).show(ui);
        });
    }

    pub(in crate::app) fn section_switch(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(42);
        let subtitle = tr!(t::SwitchSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SwitchSec::HSettings).as_ref(), None);
            Switch::new(&mut self.switch1)
                .label(tr!(t::SwitchSec::Enable).as_ref())
                .show(ui);
            Spacing::Sm.show(ui);
            Switch::new(&mut self.switch2)
                .label(tr!(t::SwitchSec::Disabled).as_ref())
                .enabled(false)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_textarea(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(45);
        let subtitle = tr!(t::TextareaSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::TextareaSec::HVariants).as_ref(), None);
            let lbl_default = tr!(t::TextareaSec::LblDefault);
            let ph_default = tr!(t::TextareaSec::PhDefault);
            Textarea::new(&mut self.textarea_text)
                .label(lbl_default.as_ref())
                .placeholder(ph_default.as_ref())
                .rows(4)
                .show(ui);
            Spacing::Md.show(ui);
            let lbl_scroll = tr!(t::TextareaSec::LblScroll);
            let ph_scroll = tr!(t::TextareaSec::PhScroll);
            Textarea::new(&mut self.textarea_scroll)
                .label(lbl_scroll.as_ref())
                .placeholder(ph_scroll.as_ref())
                .rows(3)
                .scroll(true)
                .show(ui);
            Spacing::Md.show(ui);
            let lbl_grow = tr!(t::TextareaSec::LblGrow);
            let ph_grow = tr!(t::TextareaSec::PhGrow);
            Textarea::new(&mut self.textarea_grow_y)
                .label(lbl_grow.as_ref())
                .placeholder(ph_grow.as_ref())
                .rows(2)
                .max_rows(8)
                .show(ui);
        });
    }
}
