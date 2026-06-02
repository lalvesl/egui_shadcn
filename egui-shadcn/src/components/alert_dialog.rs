use super::button::{Button, ButtonVariant};
use super::spacing::Spacing;
use super::typography::{heading4, muted_text};
use crate::ShadcnTheme;
use crate::i18n;
use ::i18n::t;
use egui::{Color32, CornerRadius, Frame, Margin, Stroke};

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
        if !*self.open {
            return false;
        }

        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
            return false;
        }

        let theme = ShadcnTheme::get(ctx);

        let overlay_id = egui::Id::new("alert_dialog_overlay");
        let overlay_layer = egui::LayerId::new(egui::Order::Background, overlay_id);
        let overlay_painter = ctx.layer_painter(overlay_layer);
        overlay_painter.rect_filled(egui::Rect::EVERYTHING, 0.0, Color32::from_black_alpha(128));

        let pointer_down = ctx.input(|i| i.pointer.primary_pressed());
        let dialog_id = egui::Id::new("shadcn_alert_dialog").with(self.title);

        let mut confirmed = false;

        let win_resp = egui::Window::new(self.title)
            .id(dialog_id)
            .collapsible(false)
            .resizable(false)
            .default_width(self.width)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                Frame::new()
                    .fill(theme.card)
                    .corner_radius(CornerRadius::same(theme.radius as u8))
                    .stroke(Stroke::new(1.0, theme.border))
                    .inner_margin(Margin::same(24)),
            )
            .title_bar(false)
            .show(ctx, |ui| {
                heading4(ui, self.title);
                Spacing::Sm.show(ui);
                muted_text(ui, self.description);
                Spacing::Xl.show(ui);

                let cancel_owned;
                let cancel_text: &str = match self.cancel_label {
                    Some(l) => l,
                    None => {
                        cancel_owned = t!(i18n::AlertDialog::Cancel);
                        cancel_owned.as_ref()
                    }
                };
                let confirm_owned;
                let confirm_text: &str = match self.confirm_label {
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
                        *self.open = false;
                    }

                    Spacing::Sm.show(ui);

                    let confirm_variant = if self.destructive {
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
                        *self.open = false;
                    }
                });
            });

        if pointer_down {
            let pointer_pos = ctx.input(|i| i.pointer.interact_pos());
            let clicked_inside = match (win_resp.as_ref(), pointer_pos) {
                (Some(r), Some(pos)) => r.response.rect.contains(pos),
                _ => true,
            };
            if !clicked_inside {
                *self.open = false;
            }
        }

        if confirmed {
            on_confirm();
        }

        confirmed
    }
}
