use super::boxed::Boxed;
use super::spacing::Spacing;
use crate::{ShadcnTheme, ICON_CLOSE};
use egui::{Color32, CornerRadius, Frame, Stroke, Vec2};

pub struct Drawer<'a> {
    title: &'a str,
    open: &'a mut bool,
    height: f32,
    handle: bool,
}

impl<'a> Drawer<'a> {
    pub fn new(title: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            open,
            height: 300.0,
            handle: true,
        }
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn handle(mut self, h: bool) -> Self {
        self.handle = h;
        self
    }

    pub fn show(self, ctx: &egui::Context, content: impl FnOnce(&mut egui::Ui)) {
        if !*self.open {
            return;
        }

        // ESC closes
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            *self.open = false;
            return;
        }

        let theme = ShadcnTheme::get(ctx);

        // Dim overlay
        let overlay_id = egui::Id::new("drawer_overlay");
        let overlay_layer = egui::LayerId::new(egui::Order::Background, overlay_id);
        let overlay_painter = ctx.layer_painter(overlay_layer);
        overlay_painter.rect_filled(egui::Rect::EVERYTHING, 0.0, Color32::from_black_alpha(100));

        let pointer_down = ctx.input(|i| i.pointer.primary_pressed());

        let screen = ctx.viewport_rect();
        let panel_width = screen.width();

        // Rounded top corners only
        let r = theme.radius as u8;
        let cr = CornerRadius {
            nw: r,
            ne: r,
            sw: 0,
            se: 0,
        };

        let win_id = egui::Id::new("shadcn_drawer").with(self.title);

        let show_handle = self.handle;
        let win_resp = egui::Window::new(self.title)
            .id(win_id)
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(panel_width, self.height))
            .anchor(egui::Align2::CENTER_BOTTOM, [0.0, 0.0])
            .frame(
                Frame::new()
                    .fill(theme.card)
                    .corner_radius(cr)
                    .stroke(Stroke::new(1.0, theme.border)),
            )
            .title_bar(false)
            .show(ctx, |ui| {
                Boxed::new().padding(Spacing::Lg).show(ui, |ui| {
                    // Drag handle
                    if show_handle {
                        ui.vertical_centered(|ui| {
                            let (handle_rect, _) = ui.allocate_exact_size(
                                Vec2::new(40.0, 4.0),
                                egui::Sense::hover(),
                            );
                            ui.painter().rect_filled(
                                handle_rect,
                                CornerRadius::same(2),
                                theme.muted_foreground,
                            );
                        });
                        Spacing::Md.show(ui);
                    }

                    // Title row with close button
                    ui.horizontal(|ui| {
                        ui.label(
                            egui::RichText::new(self.title)
                                .font(egui::FontId::new(16.0, egui::FontFamily::Proportional))
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
                                ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                            }
                        });
                    });

                    Spacing::Sm.show(ui);
                    ui.separator();
                    Spacing::Sm.show(ui);

                    content(ui);
                });
            });

        // Close if pointer pressed outside the drawer rect
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
