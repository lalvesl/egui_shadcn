use super::button::{Button, ButtonVariant};
use super::dialog::Dialog;
use super::spacing::Spacing;
use super::typography::{heading4, muted_text};
use crate::i18n;
use ::i18n::t;

pub struct AlertDialog<'a> {
    title: &'a str,
    description: &'a str,
    open: &'a mut bool,
    cancel_label: Option<&'a str>,
    confirm_label: Option<&'a str>,
    destructive: bool,
    width: f32,
}

impl<'a> AlertDialog<'a> {
    pub fn new(title: &'a str, description: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            description,
            open,
            cancel_label: None,
            confirm_label: None,
            destructive: false,
            width: 448.0,
        }
    }

    pub fn cancel_label(mut self, l: &'a str) -> Self {
        self.cancel_label = Some(l);
        self
    }

    pub fn confirm_label(mut self, l: &'a str) -> Self {
        self.confirm_label = Some(l);
        self
    }

    pub fn destructive(mut self, d: bool) -> Self {
        self.destructive = d;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(self, ctx: &egui::Context, on_confirm: impl FnOnce()) -> bool {
        let AlertDialog {
            title,
            description,
            open,
            cancel_label,
            confirm_label,
            destructive,
            width,
        } = self;

        // Local intents — `open` is borrowed by the Dialog shell (which owns
        // escape / click-outside / fade-out), so buttons signal here and we
        // apply after the shell returns.
        let mut cancel = false;
        let mut confirmed = false;

        // Reuse Dialog as a headerless animated modal; render the alert body
        // (heading + description + action row) as its content.
        Dialog::new(title, &mut *open)
            .width(width)
            .header(false)
            .show(ctx, |ui| {
                heading4(ui, title);
                Spacing::Sm.show(ui);
                muted_text(ui, description);
                Spacing::Xl.show(ui);

                let cancel_owned;
                let cancel_text: &str = match cancel_label {
                    Some(l) => l,
                    None => {
                        cancel_owned = t!(i18n::AlertDialog::Cancel);
                        cancel_owned.as_ref()
                    }
                };
                let confirm_owned;
                let confirm_text: &str = match confirm_label {
                    Some(l) => l,
                    None => {
                        confirm_owned = t!(i18n::AlertDialog::Confirm);
                        confirm_owned.as_ref()
                    }
                };

                ui.horizontal(|ui| {
                    if Button::new(cancel_text)
                        .variant(ButtonVariant::Outline)
                        .show(ui)
                        .clicked()
                    {
                        cancel = true;
                    }

                    Spacing::Sm.show(ui);

                    let confirm_variant = if destructive {
                        ButtonVariant::Destructive
                    } else {
                        ButtonVariant::Default
                    };

                    if Button::new(confirm_text)
                        .variant(confirm_variant)
                        .show(ui)
                        .clicked()
                    {
                        confirmed = true;
                    }
                });
            });

        if cancel || confirmed {
            *open = false;
        }

        if confirmed {
            on_confirm();
        }

        confirmed
    }
}
