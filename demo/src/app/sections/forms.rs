use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
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
        self.section_title(ui, "Checkbox", "A control that allows the user to toggle between checked and not checked.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Options", None);
            Checkbox::new(&mut self.checkbox1).label("Accept terms and conditions").show(ui);
            Spacing::Sm.show(ui);
            Checkbox::new(&mut self.checkbox2).label("Subscribe to newsletter").show(ui);
            Spacing::Sm.show(ui);
            Checkbox::new(&mut self.checkbox3).label("Disabled option").enabled(false).show(ui);
        });
    }

    pub(in crate::app) fn section_combobox(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Combobox", "Autocomplete input and command palette with a list of suggestions.");

        let frameworks = &["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro", "SolidStart"];
        Card::new().show(ui, |ui| {
            card_header(ui, "Framework", Some("Select a framework to use."));
            Combobox::new("demo_combobox", &mut self.combobox_val, frameworks)
                .placeholder("Select framework…")
                .show(ui);
            if let Some(i) = self.combobox_val {
                Spacing::Xs.show(ui);
                muted_text(ui, &format!("Selected: {}", frameworks[i]));
            }
        });
    }

    pub(in crate::app) fn section_date_picker(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Date Picker", "A date picker component with calendar popover.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Pick a date", None);
            DatePicker::new("demo_date_picker", &mut self.date_picker_val).show(ui);
            if let Some(d) = self.date_picker_val {
                Spacing::Xs.show(ui);
                muted_text(ui, &format!("Selected: {:04}-{:02}-{:02}", d.year, d.month, d.day));
            }
        });
    }

    pub(in crate::app) fn section_input(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Input", "Displays a form input field or a component that looks like an input field.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Text input", None);
            Input::new(&mut self.input_text)
                .label("Username")
                .placeholder("Enter your username…")
                .show(ui);
            Spacing::Md.show(ui);
            Input::new(&mut self.password_text)
                .label("Password")
                .placeholder("••••••••")
                .password(true)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_input_otp(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Input OTP", "Accessible one-time password component with copy paste functionality.");

        Card::new().show(ui, |ui| {
            card_header(ui, "One-Time Password", Some("Enter your one-time password."));
            InputOtp::new(&mut self.otp_val, 6).separator_after(3).show(ui);
            if self.otp_val.len() == 6 {
                Spacing::Xs.show(ui);
                muted_text(ui, &format!("Code: {}", self.otp_val));
            }
        });
    }

    pub(in crate::app) fn section_label(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Label", "Renders an accessible label associated with controls.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Basic", None);
            Label::new("Email address").show(ui);
            Spacing::Xs.show(ui);
            Label::new("Required field").required(true).show(ui);
            Spacing::Xs.show(ui);
            Label::new("Password").show(ui);
        });
    }

    pub(in crate::app) fn section_radio(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Radio", "A set of checkable buttons where no more than one can be checked at a time.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Notification method", None);
            Radio::new(&mut self.radio_val, 0u32).label("Option A").show(ui);
            Spacing::Sm.show(ui);
            Radio::new(&mut self.radio_val, 1u32).label("Option B").show(ui);
            Spacing::Sm.show(ui);
            Radio::new(&mut self.radio_val, 2u32).label("Option C").show(ui);
        });
    }

    pub(in crate::app) fn section_select(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Select", "Displays a list of options for the user to pick from—triggered by a button.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Framework", None);
            Label::new("Framework").show(ui);
            Spacing::Xs.show(ui);
            Select::new(&mut self.select_val, &["React", "Vue", "Svelte", "egui"])
                .placeholder("Pick a framework…")
                .show(ui);
            if let Some(i) = self.select_val {
                Spacing::Xs.show(ui);
                muted_text(ui, ["React", "Vue", "Svelte", "egui"][i]);
            }
        });
    }

    pub(in crate::app) fn section_slider(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Slider", "An input where the user selects a value from within a given range.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Volume", None);
            muted_text(ui, &format!("Value: {:.0}", self.slider_val));
            Spacing::Sm.show(ui);
            Slider::new(&mut self.slider_val, 0.0, 100.0).show(ui);
        });
    }

    pub(in crate::app) fn section_switch(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Switch", "A control that allows the user to toggle between on and off states.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Settings", None);
            Switch::new(&mut self.switch1).label("Enable notifications").show(ui);
            Spacing::Sm.show(ui);
            Switch::new(&mut self.switch2).label("Disabled switch").enabled(false).show(ui);
        });
    }

    pub(in crate::app) fn section_textarea(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Textarea", "Displays a form textarea or a component that looks like a textarea.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Variants", None);
            Textarea::new(&mut self.textarea_text)
                .label("Default (fixed 4 rows)")
                .placeholder("Tell us about yourself…")
                .rows(4)
                .show(ui);
            Spacing::Md.show(ui);
            Textarea::new(&mut self.textarea_scroll)
                .label("Scroll (fixed 3 rows)")
                .placeholder("Keep typing — content scrolls inside…")
                .rows(3)
                .scroll(true)
                .show(ui);
            Spacing::Md.show(ui);
            Textarea::new(&mut self.textarea_grow_y)
                .label("Grow Y (2 → 8 rows, then scrolls)")
                .placeholder("Start typing — textarea grows…")
                .rows(2)
                .max_rows(8)
                .show(ui);
        });
    }
}
