use super::spacing::Spacing;
use crate::{Animations, ICON_CLOSE, ShadcnTheme};
use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Ui};

pub struct Dialog<'a> {
    title: &'a str,
    open: &'a mut bool,
    width: f32,
}

impl<'a> Dialog<'a> {
    pub fn new(title: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            open,
            width: 448.0,
        }
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(self, ctx: &egui::Context, content: impl FnOnce(&mut Ui)) {
        // Animate 0→1 toward the open state. Driving rendering off `t` (not
        // `*open`) lets the dialog play a closing transition after `*open`
        // flips to false. `animate_bool_with_time` repaints while in motion.
        let anim_id = egui::Id::new("shadcn_dialog_anim").with(self.title);
        let dur = Animations::duration(ctx, 0.18);
        let t = ctx.animate_bool_with_time(anim_id, *self.open, dur);
        if t <= 0.001 {
            return; // fully closed
        }

        // Escape key closes (animates out over the next frames).
        if *self.open && ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
        }

        let theme = ShadcnTheme::get(ctx);

        // Dim overlay — fades with the dialog; also the click-outside detector.
        let overlay_id = egui::Id::new("dialog_overlay");
        let overlay_layer = egui::LayerId::new(egui::Order::Background, overlay_id);
        let overlay_painter = ctx.layer_painter(overlay_layer);
        overlay_painter.rect_filled(
            egui::Rect::EVERYTHING,
            0.0,
            Color32::from_black_alpha((t * 128.0) as u8),
        );

        // Only react to fresh clicks while genuinely open (not mid-close).
        let pointer_down = *self.open && ctx.input(|i| i.pointer.primary_pressed());
        let dialog_window_id = egui::Id::new("shadcn_dialog").with(self.title);

        // Slide up into place on open; slide back down while closing. Fade the
        // card fill/stroke with `t` so the whole panel eases in and out.
        let y_offset = (1.0 - t) * 16.0;
        let alpha = (t * 255.0) as u8;
        let closing = !*self.open;

        let win_resp = egui::Window::new(self.title)
            .id(dialog_window_id)
            .collapsible(false)
            .resizable(false)
            .default_width(self.width)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, y_offset])
            .frame(
                Frame::new()
                    .fill(ShadcnTheme::with_alpha(theme.card, alpha))
                    .corner_radius(CornerRadius::same(theme.radius as u8))
                    .stroke(Stroke::new(1.0, ShadcnTheme::with_alpha(theme.border, alpha)))
                    .inner_margin(Margin::same(24)),
            )
            .title_bar(false)
            .show(ctx, |ui| {
                ui.multiply_opacity(t);
                if closing {
                    ui.disable();
                }
                ui.horizontal(|ui| {
                    ui.label(
                        egui::RichText::new(self.title)
                            .font(egui::FontId::new(18.0, egui::FontFamily::Proportional))
                            .color(theme.foreground)
                            .strong(),
                    );
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let close_resp = ui.add(
                            egui::Label::new(
                                egui::RichText::new(ICON_CLOSE)
                                    .font(crate::icon_font_id(18.0))
                                    .color(theme.muted_foreground),
                            )
                            .sense(egui::Sense::click()),
                        );
                        if close_resp.clicked() {
                            *self.open = false;
                        }
                        if close_resp.hovered() {
                            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                    });
                });

                Spacing::Sm.show(ui);
                ui.separator();
                Spacing::Sm.show(ui);

                content(ui);
            });

        // Close if pointer pressed outside the dialog rect
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
    }
}
