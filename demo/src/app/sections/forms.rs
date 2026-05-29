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
    pub(in crate::app) fn section_form(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Form Controls", "Checkbox, radio, and switch.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Checkbox", None);
            Checkbox::new(&mut self.checkbox1)
                .label("Accept terms and conditions")
                .show(ui);
            ui.add_space(8.0);
            Checkbox::new(&mut self.checkbox2)
                .label("Subscribe to newsletter")
                .show(ui);
            ui.add_space(8.0);
            Checkbox::new(&mut self.checkbox3)
                .label("Disabled option")
                .enabled(false)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Radio Group", None);
            Radio::new(&mut self.radio_val, 0u32)
                .label("Option A")
                .show(ui);
            ui.add_space(8.0);
            Radio::new(&mut self.radio_val, 1u32)
                .label("Option B")
                .show(ui);
            ui.add_space(8.0);
            Radio::new(&mut self.radio_val, 2u32)
                .label("Option C")
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Switch", None);
            Switch::new(&mut self.switch1)
                .label("Enable notifications")
                .show(ui);
            ui.add_space(8.0);
            Switch::new(&mut self.switch2)
                .label("Disabled switch")
                .enabled(false)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_inputs(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Inputs", "Text fields and selects.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Text Input", None);
            Input::new(&mut self.input_text)
                .label("Username")
                .placeholder("Enter your username…")
                .show(ui);
            ui.add_space(12.0);
            Input::new(&mut self.password_text)
                .label("Password")
                .placeholder("••••••••")
                .password(true)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Textarea", None);

            // Default — fixed rows
            Textarea::new(&mut self.textarea_text)
                .label("Default (fixed 4 rows)")
                .placeholder("Tell us about yourself…")
                .rows(4)
                .show(ui);

            ui.add_space(12.0);

            // Scroll — fixed height, vertical scrollbar
            Textarea::new(&mut self.textarea_scroll)
                .label("Scroll (fixed 3 rows)")
                .placeholder("Keep typing — content scrolls inside…")
                .rows(3)
                .scroll(true)
                .show(ui);

            ui.add_space(12.0);

            // Grow Y — expands up to 8 rows, then scrolls
            Textarea::new(&mut self.textarea_grow_y)
                .label("Grow Y (2 → 8 rows, then scrolls)")
                .placeholder("Start typing — textarea grows…")
                .rows(2)
                .max_rows(8)
                .show(ui);

            ui.add_space(12.0);

            // Grow X — capped width (doesn't fill panel)
            Textarea::new(&mut self.textarea_grow_x)
                .label("Grow X (max width 280 px)")
                .placeholder("Width capped at 280 px…")
                .rows(3)
                .max_width(280.0)
                .show(ui);

            ui.add_space(12.0);

            // Grow both — capped width + grow Y
            Textarea::new(&mut self.textarea_grow_both)
                .label("Grow both (2 → 6 rows, max width 320 px)")
                .placeholder("Grows in height, capped in width…")
                .rows(2)
                .max_rows(6)
                .max_width(320.0)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Select", None);
            Label::new("Framework").show(ui);
            ui.add_space(4.0);
            Select::new(&mut self.select_val, &["React", "Vue", "Svelte", "egui"])
                .placeholder("Pick a framework…")
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Slider", None);
            muted_text(ui, &format!("Value: {:.0}", self.slider_val));
            ui.add_space(8.0);
            Slider::new(&mut self.slider_val, 0.0, 100.0).show(ui);
        });
    }

    pub(in crate::app) fn section_combobox(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Combobox", "An input with autocomplete and filtering.");

        let frameworks = &["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro", "SolidStart"];
        Card::new().show(ui, |ui| {
            card_header(ui, "Framework", Some("Select a framework to use."));
            Combobox::new("demo_combobox", &mut self.combobox_val, frameworks)
                .placeholder("Select framework…")
                .show(ui);
            if let Some(i) = self.combobox_val {
                ui.add_space(4.0);
                muted_text(ui, &format!("Selected: {}", frameworks[i]));
            }
        });
    }

    pub(in crate::app) fn section_input_otp(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Input OTP", "Accessible one-time password component.");

        Card::new().show(ui, |ui| {
            card_header(ui, "One-Time Password", Some("Enter your one-time password."));
            InputOtp::new(&mut self.otp_val, 6).separator_after(3).show(ui);
            if self.otp_val.len() == 6 {
                ui.add_space(4.0);
                muted_text(ui, &format!("Code: {}", self.otp_val));
            }
        });
    }

    pub(in crate::app) fn section_date_picker(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Date Picker", "A date picker with popover calendar.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Pick a date", None);
            DatePicker::new("demo_date_picker", &mut self.date_picker_val).show(ui);
            if let Some(d) = self.date_picker_val {
                ui.add_space(4.0);
                muted_text(ui, &format!("Selected: {:04}-{:02}-{:02}", d.year, d.month, d.day));
            }
        });
    }
}
