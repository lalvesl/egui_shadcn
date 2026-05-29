use super::spacing::Spacing;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, Margin, Sense, Stroke, Vec2};

pub struct AlertDialog<'a> {
    title: &'a str,
    description: &'a str,
    open: &'a mut bool,
    cancel_label: &'a str,
    confirm_label: &'a str,
    destructive: bool,
    width: f32,
}

impl<'a> AlertDialog<'a> {
    pub fn new(title: &'a str, description: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            description,
            open,
            cancel_label: "Cancel",
            confirm_label: "Continue",
            destructive: false,
            width: 448.0,
        }
    }

    pub fn cancel_label(mut self, l: &'a str) -> Self {
        self.cancel_label = l;
        self
    }

    pub fn confirm_label(mut self, l: &'a str) -> Self {
        self.confirm_label = l;
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
                ui.label(
                    egui::RichText::new(self.title)
                        .font(egui::FontId::new(18.0, egui::FontFamily::Proportional))
                        .color(theme.foreground)
                        .strong(),
                );

                Spacing::Sm.show(ui);

                ui.label(
                    egui::RichText::new(self.description)
                        .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                        .color(theme.muted_foreground),
                );

                ui.add_space(20.0);

                ui.horizontal(|ui| {
                    let btn_h = 36.0;
                    let btn_w = 96.0;

                    let (cancel_rect, cancel_resp) =
                        ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());
                    let cr = CornerRadius::same(theme.radius as u8);

                    let cancel_bg = if cancel_resp.is_pointer_button_down_on() {
                        theme.secondary
                    } else if cancel_resp.hovered() {
                        ShadcnTheme::with_alpha(theme.secondary, 200)
                    } else {
                        Color32::TRANSPARENT
                    };

                    ui.painter().rect_filled(cancel_rect, cr, cancel_bg);
                    ui.painter().rect_stroke(
                        cancel_rect,
                        cr,
                        Stroke::new(1.0, theme.border),
                        egui::StrokeKind::Inside,
                    );
                    ui.painter().text(
                        cancel_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        self.cancel_label,
                        egui::FontId::new(14.0, egui::FontFamily::Proportional),
                        theme.foreground,
                    );

                    if cancel_resp.clicked() {
                        *self.open = false;
                    }
                    if cancel_resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }

                    Spacing::Sm.show(ui);

                    let (confirm_rect, confirm_resp) =
                        ui.allocate_exact_size(Vec2::new(btn_w, btn_h), Sense::click());

                    let (confirm_bg, confirm_fg) = if self.destructive {
                        (theme.destructive, theme.destructive_foreground)
                    } else {
                        (theme.primary, theme.primary_foreground)
                    };

                    let actual_bg = if confirm_resp.is_pointer_button_down_on() {
                        ShadcnTheme::with_alpha(confirm_bg, 200)
                    } else if confirm_resp.hovered() {
                        ShadcnTheme::with_alpha(confirm_bg, 220)
                    } else {
                        confirm_bg
                    };

                    ui.painter().rect_filled(confirm_rect, cr, actual_bg);
                    ui.painter().text(
                        confirm_rect.center(),
                        egui::Align2::CENTER_CENTER,
                        self.confirm_label,
                        egui::FontId::new(14.0, egui::FontFamily::Proportional),
                        confirm_fg,
                    );

                    if confirm_resp.clicked() {
                        confirmed = true;
                        *self.open = false;
                    }
                    if confirm_resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
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

