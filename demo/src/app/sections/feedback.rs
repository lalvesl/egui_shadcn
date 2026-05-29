use egui_shadcn::size::Size;
use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    alert::{Alert, AlertVariant},
    alert_dialog::AlertDialog,
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, card_header},
    progress::Progress,
    skeleton::Skeleton,
    spinner::Spinner,
    toast::{ToastVariant, Toaster},
    typography::muted_text,
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_alert(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Alert", "Displays a callout for user attention.");

        Alert::new("Heads up!")
            .description("This is an informational alert with a description.")
            .show(ui);
        Spacing::Md.show(ui);
        Alert::new("Destructive")
            .description("Something went wrong. Please check your inputs.")
            .variant(AlertVariant::Destructive)
            .show(ui);
        Spacing::Md.show(ui);
        Alert::new("Warning")
            .description("This action cannot be undone. Proceed with caution.")
            .variant(AlertVariant::Warning)
            .show(ui);
    }

    pub(in crate::app) fn section_alert_dialog(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Alert Dialog",
            "A modal dialog that interrupts the user with important content and expects a response.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Default", None);
            if self.alert_dialog_confirmed {
                muted_text(ui, "Action confirmed!");
                Spacing::Sm.show(ui);
            }
            if Button::new("Open Alert Dialog").show(ui).clicked() {
                self.alert_dialog_open = true;
                self.alert_dialog_confirmed = false;
            }
        });

        Spacing::Lg.show(ui);

        let ctx = ui.ctx().clone();
        let mut confirmed = false;
        AlertDialog::new(
            "Are you absolutely sure?",
            "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
            &mut self.alert_dialog_open,
        )
        .destructive(true)
        .confirm_label("Delete Account")
        .show(&ctx, || { confirmed = true; });
        if confirmed { self.alert_dialog_confirmed = true; }
    }

    pub(in crate::app) fn section_dialog(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Dialog",
            "A window overlaid on either the primary window or another dialog window, rendering the content underneath inert.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Confirm Action", Some("Type CONFIRM to proceed with this destructive action."));
            muted_text(ui, "This dialog requires explicit confirmation before proceeding.");
            Spacing::Md.show(ui);
            if Button::new("Open Dialog").show(ui).clicked() {
                self.dialog_open = true;
            }
        });
    }

    pub(in crate::app) fn section_progress(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Progress",
            "Displays an indicator showing the completion progress of a task, typically displayed as a progress bar.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Progress bar", None);
            muted_text(ui, &format!("{:.0}%", self.progress_val * 100.0));
            Spacing::Sm.show(ui);
            Progress::new(self.progress_val).show(ui);
            Spacing::Md.show(ui);
            ui.horizontal(|ui| {
                if Button::new("−10%")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    self.progress_val = (self.progress_val - 0.1).max(0.0);
                }
                Spacing::Xs.show(ui);
                if Button::new("+10%").size(ButtonSize::Sm).show(ui).clicked() {
                    self.progress_val = (self.progress_val + 0.1).min(1.0);
                }
            });
        });
    }

    pub(in crate::app) fn section_skeleton(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Skeleton", "Use to show a placeholder while content is loading.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Loading state", None);
            ui.horizontal(|ui| {
                Skeleton::circle(40.0).show(ui);
                Spacing::Md.show(ui);
                ui.vertical(|ui| {
                    Skeleton::new(180.0, 14.0).radius(4.0).show(ui);
                    ui.add_space(6.0);
                    Skeleton::new(120.0, 12.0).radius(4.0).show(ui);
                });
            });
            Spacing::Md.show(ui);
            Skeleton::new(ui.available_width(), 120.0).radius(8.0).show(ui);
        });
    }

    pub(in crate::app) fn section_spinner(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Spinner", "Displays an animated spinner to indicate a loading state.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Sizes", None);
            ui.horizontal(|ui| {
                Spinner::new().size(Size::Sm).show(ui);
                Spacing::Xs.show(ui);
                Spinner::new().size(Size::Default).show(ui);
                Spacing::Xs.show(ui);
                Spinner::new().size(Size::Lg).show(ui);
            });
        });
    }

    pub(in crate::app) fn section_toast(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Toast", "A succinct message that is displayed temporarily.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Variants", Some("Click to show a toast notification."));
            ui.horizontal(|ui| {
                let ctx = ui.ctx().clone();
                if Button::new("Default").show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Scheduled", "Monday, January 3rd at 6:00pm", ToastVariant::Default);
                }
                if Button::new("Success").show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Success", "Your changes have been saved.", ToastVariant::Success);
                }
                if Button::new("Warning").show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Warning", "This action may have consequences.", ToastVariant::Warning);
                }
                if Button::new("Destructive").variant(ButtonVariant::Destructive).show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Destructive", "Your session has expired.", ToastVariant::Destructive);
                }
            });
        });
    }
}
