use crate::i18n as t;
use ::i18n::t as tr;
use egui_sc::egui_components::size::Size;
use egui_sc::egui_components::spacing::Spacing;
use egui_sc::egui_components::{
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
        let title = t::section_name(2);
        let subtitle = tr!(t::AlertSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let info_title = tr!(t::AlertSec::InfoTitle);
        let info_desc = tr!(t::AlertSec::InfoDesc);
        Alert::new(info_title.as_ref())
            .description(info_desc.as_ref())
            .show(ui);
        Spacing::Md.show(ui);
        let dest_title = tr!(t::AlertSec::DestructiveTitle);
        let dest_desc = tr!(t::AlertSec::DestructiveDesc);
        Alert::new(dest_title.as_ref())
            .description(dest_desc.as_ref())
            .variant(AlertVariant::Destructive)
            .show(ui);
        Spacing::Md.show(ui);
        let warn_title = tr!(t::AlertSec::WarningTitle);
        let warn_desc = tr!(t::AlertSec::WarningDesc);
        Alert::new(warn_title.as_ref())
            .description(warn_desc.as_ref())
            .variant(AlertVariant::Warning)
            .show(ui);
    }

    pub(in crate::app) fn section_alert_dialog(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(3);
        let subtitle = tr!(t::AlertDialogSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::AlertDialogSec::HDefault).as_ref(), None);
            if self.alert_dialog_confirmed {
                muted_text(ui, tr!(t::AlertDialogSec::Confirmed).as_ref());
                Spacing::Sm.show(ui);
            }
            if Button::new(tr!(t::AlertDialogSec::Open).as_ref())
                .show(ui)
                .clicked()
            {
                self.alert_dialog_open = true;
                self.alert_dialog_confirmed = false;
            }
        });

        Spacing::Lg.show(ui);

        let ctx = ui.ctx().clone();
        let mut confirmed = false;
        let dlg_title = tr!(t::AlertDialogSec::DialogTitle);
        let dlg_body = tr!(t::AlertDialogSec::DialogBody);
        let delete = tr!(t::AlertDialogSec::Delete);
        AlertDialog::new(
            dlg_title.as_ref(),
            dlg_body.as_ref(),
            &mut self.alert_dialog_open,
        )
        .destructive(true)
        .confirm_label(delete.as_ref())
        .show(&ctx, || {
            confirmed = true;
        });
        if confirmed {
            self.alert_dialog_confirmed = true;
        }
    }

    pub(in crate::app) fn section_dialog(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(21);
        let subtitle = tr!(t::DialogSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let confirm_desc = tr!(t::DialogSec::HConfirmDesc);
            card_header(
                ui,
                tr!(t::DialogSec::HConfirm).as_ref(),
                Some(confirm_desc.as_ref()),
            );
            muted_text(ui, tr!(t::DialogSec::Body).as_ref());
            Spacing::Md.show(ui);
            if Button::new(tr!(t::DialogSec::Open).as_ref())
                .show(ui)
                .clicked()
            {
                self.dialog_open = true;
            }
        });
    }

    pub(in crate::app) fn section_progress(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(32);
        let subtitle = tr!(t::ProgressSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::ProgressSec::HBar).as_ref(), None);
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
        let title = t::section_name(38);
        let subtitle = tr!(t::SkeletonSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SkeletonSec::HLoading).as_ref(), None);
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
            Skeleton::new(ui.available_width(), 120.0)
                .radius(8.0)
                .show(ui);
        });
    }

    pub(in crate::app) fn section_spinner(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(41);
        let subtitle = tr!(t::SpinnerSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::SpinnerSec::HSizes).as_ref(), None);
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
        let title = t::section_name(46);
        let subtitle = tr!(t::ToastSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            let desc = tr!(t::ToastSec::HVariantsDesc);
            card_header(
                ui,
                tr!(t::ToastSec::HVariants).as_ref(),
                Some(desc.as_ref()),
            );
            ui.horizontal(|ui| {
                let ctx = ui.ctx().clone();
                if Button::new(tr!(t::ToastSec::DefaultBtn).as_ref())
                    .show(ui)
                    .clicked()
                {
                    let t_title = tr!(t::ToastSec::DefaultTitle);
                    let t_desc = tr!(t::ToastSec::DefaultDesc);
                    Toaster::push_with_desc(
                        &ctx,
                        t_title.as_ref(),
                        t_desc.as_ref(),
                        ToastVariant::Default,
                    );
                }
                if Button::new(tr!(t::ToastSec::SuccessBtn).as_ref())
                    .show(ui)
                    .clicked()
                {
                    let t_title = tr!(t::ToastSec::SuccessTitle);
                    let t_desc = tr!(t::ToastSec::SuccessDesc);
                    Toaster::push_with_desc(
                        &ctx,
                        t_title.as_ref(),
                        t_desc.as_ref(),
                        ToastVariant::Success,
                    );
                }
                if Button::new(tr!(t::ToastSec::WarningBtn).as_ref())
                    .show(ui)
                    .clicked()
                {
                    let t_title = tr!(t::ToastSec::WarningTitle);
                    let t_desc = tr!(t::ToastSec::WarningDesc);
                    Toaster::push_with_desc(
                        &ctx,
                        t_title.as_ref(),
                        t_desc.as_ref(),
                        ToastVariant::Warning,
                    );
                }
                if Button::new(tr!(t::ToastSec::DestructiveBtn).as_ref())
                    .variant(ButtonVariant::Destructive)
                    .show(ui)
                    .clicked()
                {
                    let t_title = tr!(t::ToastSec::DestructiveTitle);
                    let t_desc = tr!(t::ToastSec::DestructiveDesc);
                    Toaster::push_with_desc(
                        &ctx,
                        t_title.as_ref(),
                        t_desc.as_ref(),
                        ToastVariant::Destructive,
                    );
                }
            });
        });
    }
}
