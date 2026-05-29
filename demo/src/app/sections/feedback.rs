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
    pub(in crate::app) fn section_alerts(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Alert", "Informational messages.");
        Alert::new("Heads up!")
            .description("This is an informational alert with a description.")
            .show(ui);
        ui.add_space(12.0);
        Alert::new("Destructive")
            .description("Something went wrong. Please check your inputs.")
            .variant(AlertVariant::Destructive)
            .show(ui);
        ui.add_space(12.0);
        Alert::new("Warning")
            .description("This action cannot be undone. Proceed with caution.")
            .variant(AlertVariant::Warning)
            .show(ui);
    }

    pub(in crate::app) fn section_feedback(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Feedback", "Progress, spinners, and skeletons.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Progress", None);
            muted_text(ui, &format!("{:.0}%", self.progress_val * 100.0));
            ui.add_space(8.0);
            Progress::new(self.progress_val).show(ui);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if Button::new("−10%")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    self.progress_val = (self.progress_val - 0.1).max(0.0);
                }
                ui.add_space(4.0);
                if Button::new("+10%").size(ButtonSize::Sm).show(ui).clicked() {
                    self.progress_val = (self.progress_val + 0.1).min(1.0);
                }
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Spinner", None);
            ui.horizontal(|ui| {
                Spinner::new().size(16.0).show(ui);
                ui.add_space(4.0);
                Spinner::new().size(24.0).show(ui);
                ui.add_space(4.0);
                Spinner::new().size(40.0).show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Skeleton", None);
            ui.horizontal(|ui| {
                Skeleton::circle(40.0).show(ui);
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    Skeleton::new(180.0, 14.0).radius(4.0).show(ui);
                    ui.add_space(6.0);
                    Skeleton::new(120.0, 12.0).radius(4.0).show(ui);
                });
            });
            ui.add_space(12.0);
            Skeleton::new(ui.available_width(), 120.0)
                .radius(8.0)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_alert_dialog(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Alert Dialog",
            "A modal dialog that interrupts with important content.",
        );

        Card::new().show(ui, |ui| {
            card_header(ui, "Default", None);
            if self.alert_dialog_confirmed {
                muted_text(ui, "Action confirmed!");
                ui.add_space(8.0);
            }
            if Button::new("Open Alert Dialog").show(ui).clicked() {
                self.alert_dialog_open = true;
                self.alert_dialog_confirmed = false;
            }
        });

        ui.add_space(16.0);

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
        if confirmed {
            self.alert_dialog_confirmed = true;
        }
    }

    pub(in crate::app) fn section_toast(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Toast", "A succinct message that appears temporarily.");

        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Toast variants",
                Some("Click to show a toast notification."),
            );
            ui.horizontal(|ui| {
                let ctx = ui.ctx().clone();
                if Button::new("Default").show(ui).clicked() {
                    Toaster::push_with_desc(
                        &ctx,
                        "Scheduled",
                        "Monday, January 3rd at 6:00pm",
                        ToastVariant::Default,
                    );
                }
                if Button::new("Success")
                    .variant(ButtonVariant::Default)
                    .show(ui)
                    .clicked()
                {
                    Toaster::push_with_desc(
                        &ctx,
                        "Success",
                        "Your changes have been saved.",
                        ToastVariant::Success,
                    );
                }
                if Button::new("Warning").show(ui).clicked() {
                    Toaster::push_with_desc(
                        &ctx,
                        "Warning",
                        "This action may have consequences.",
                        ToastVariant::Warning,
                    );
                }
                if Button::new("Destructive")
                    .variant(ButtonVariant::Destructive)
                    .show(ui)
                    .clicked()
                {
                    Toaster::push_with_desc(
                        &ctx,
                        "Destructive",
                        "Your session has expired.",
                        ToastVariant::Destructive,
                    );
                }
            });
        });
    }
}
