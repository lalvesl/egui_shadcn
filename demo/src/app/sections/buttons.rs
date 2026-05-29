use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ICON_ADD, ICON_DOWNLOAD, ICON_FORMAT_BOLD, ICON_FORMAT_ITALIC, ICON_FORMAT_STRIKETHROUGH,
    ICON_FORMAT_UNDERLINE, ICON_MAIL, ICON_SEARCH, ICON_SEND,
    alert::Alert,
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupVariant},
    card::{Card, card_header},
    toggle::Toggle,
    toggle_group::{ToggleGroup, ToggleGroupItem},
    typography::muted_text,
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_buttons(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Button", "Trigger actions and events.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Variants", None);
            ui.horizontal_wrapped(|ui| {
                if Button::new("Default").show(ui).clicked() {
                    self.btn_clicked = true;
                }
                Spacing::Sm.show(ui);
                Button::new("Destructive")
                    .variant(ButtonVariant::Destructive)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Outline")
                    .variant(ButtonVariant::Outline)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Secondary")
                    .variant(ButtonVariant::Secondary)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui);
                Spacing::Sm.show(ui);
                Button::new("Link").variant(ButtonVariant::Link).show(ui);
            });
            if self.btn_clicked {
                Spacing::Sm.show(ui);
                Alert::new("Button clicked!").show(ui);
                if ui.small_button("Dismiss").clicked() {
                    self.btn_clicked = false;
                }
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "Sizes", None);
            ui.horizontal(|ui| {
                Button::new("Small").size(ButtonSize::Sm).show(ui);
                Spacing::Sm.show(ui);
                Button::new("Default").show(ui);
                Spacing::Sm.show(ui);
                Button::new("Large").size(ButtonSize::Lg).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "States", None);
            ui.horizontal(|ui| {
                Button::new("Enabled").show(ui);
                Spacing::Sm.show(ui);
                Button::new("Disabled").enabled(false).show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "With Icon", None);
            ui.horizontal_wrapped(|ui| {
                Button::new("Send").icon(ICON_SEND).show(ui);
                Spacing::Sm.show(ui);
                Button::new("Mail")
                    .icon(ICON_MAIL)
                    .variant(ButtonVariant::Outline)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Download")
                    .icon(ICON_DOWNLOAD)
                    .variant(ButtonVariant::Secondary)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Search")
                    .icon(ICON_SEARCH)
                    .variant(ButtonVariant::Ghost)
                    .show(ui);
                Spacing::Sm.show(ui);
                Button::new("Add")
                    .icon(ICON_ADD)
                    .variant(ButtonVariant::Destructive)
                    .show(ui);
            });
        });
    }

    pub(in crate::app) fn section_badges(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Badge", "Labels and status indicators.");
        Card::new().show(ui, |ui| {
            ui.horizontal(|ui| {
                Badge::new("Default").show(ui);
                Spacing::Sm.show(ui);
                Badge::new("Secondary")
                    .variant(BadgeVariant::Secondary)
                    .show(ui);
                Spacing::Sm.show(ui);
                Badge::new("Destructive")
                    .variant(BadgeVariant::Destructive)
                    .show(ui);
                Spacing::Sm.show(ui);
                Badge::new("Outline")
                    .variant(BadgeVariant::Outline)
                    .show(ui);
            });
        });
    }

    pub(in crate::app) fn section_button_group(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Button Group",
            "Multiple actions in a connected button strip.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Single select", None);
            let labels = &["Months", "Weeks", "Days"];
            if let Some(i) = ButtonGroup::new(labels)
                .selected(self.button_group_sel)
                .show(ui)
            {
                self.button_group_sel = Some(i);
            }
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Outline variant", None);
            if let Some(i) = ButtonGroup::new(&["Left", "Center", "Right"])
                .variant(ButtonGroupVariant::Outline)
                .selected(self.button_group_outline_sel)
                .show(ui)
            {
                self.button_group_outline_sel = Some(i);
            }
        });
    }

    pub(in crate::app) fn section_toggle(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Toggle",
            "A two-state button that can be turned on or off.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Toggle", None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle1, "Bold").show(ui);
                Toggle::new(&mut self.toggle2, "Italic").show(ui);
            });
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "With Icon", None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle_bold, "Bold")
                    .icon(ICON_FORMAT_BOLD)
                    .show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_italic, "Italic")
                    .icon(ICON_FORMAT_ITALIC)
                    .show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_underline, "Underline")
                    .icon(ICON_FORMAT_UNDERLINE)
                    .show(ui);
                Spacing::Xs.show(ui);
                Toggle::new(&mut self.toggle_strikethrough, "Strikethrough")
                    .icon(ICON_FORMAT_STRIKETHROUGH)
                    .show(ui);
            });
        });

        Spacing::Lg.show(ui);

        Card::new().show(ui, |ui| {
            card_header(ui, "Disabled", None);
            let mut dummy = true;
            ui.horizontal(|ui| {
                Toggle::new(&mut dummy, "Disabled On")
                    .enabled(false)
                    .show(ui);
                let mut dummy2 = false;
                Toggle::new(&mut dummy2, "Disabled Off")
                    .enabled(false)
                    .show(ui);
            });
        });
    }

    pub(in crate::app) fn section_toggle_group(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Toggle Group",
            "A set of two-state buttons — only one active at a time.",
        );

        let items: &[(u8, ToggleGroupItem<'_>)] = &[
            (
                0,
                ToggleGroupItem {
                    label: "Day",
                    icon: None,
                },
            ),
            (
                1,
                ToggleGroupItem {
                    label: "Week",
                    icon: None,
                },
            ),
            (
                2,
                ToggleGroupItem {
                    label: "Month",
                    icon: None,
                },
            ),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "View", None);
            ToggleGroup::new(items, &mut self.toggle_group_val).show(ui);
            Spacing::Xs.show(ui);
            muted_text(
                ui,
                &format!(
                    "Selected: {}",
                    ["Day", "Week", "Month"][self.toggle_group_val as usize]
                ),
            );
        });
    }
}
