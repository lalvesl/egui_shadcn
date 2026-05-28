use crate::{ShadcnTheme, ICON_CLOSE};
use egui::{Color32, CornerRadius, Frame, Margin, Stroke, Vec2};

pub enum SheetSide {
    Left,
    Right,
}

pub struct Sheet<'a> {
    title: &'a str,
    open: &'a mut bool,
    side: SheetSide,
    width: f32,
}

impl<'a> Sheet<'a> {
    pub fn new(title: &'a str, open: &'a mut bool) -> Self {
        Self {
            title,
            open,
            side: SheetSide::Right,
            width: 320.0,
        }
    }

    pub fn side(mut self, s: SheetSide) -> Self {
        self.side = s;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
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

        // Dim overlay — also acts as click-outside detector
        let overlay_id = egui::Id::new("sheet_overlay");
        let overlay_layer = egui::LayerId::new(egui::Order::Background, overlay_id);
        let overlay_painter = ctx.layer_painter(overlay_layer);
        overlay_painter.rect_filled(egui::Rect::EVERYTHING, 0.0, Color32::from_black_alpha(100));

        let pointer_down = ctx.input(|i| i.pointer.primary_pressed());

        let screen = ctx.viewport_rect();
        let panel_height = screen.height();

        // Corner radius only on the inner edge
        let r = theme.radius as u8;
        let cr = match self.side {
            SheetSide::Right => CornerRadius {
                nw: r,
                ne: 0,
                sw: r,
                se: 0,
            },
            SheetSide::Left => CornerRadius {
                nw: 0,
                ne: r,
                sw: 0,
                se: r,
            },
        };

        let anchor = match self.side {
            SheetSide::Right => egui::Align2::RIGHT_CENTER,
            SheetSide::Left => egui::Align2::LEFT_CENTER,
        };

        let win_id = egui::Id::new("shadcn_sheet").with(self.title);

        let win_resp = egui::Window::new(self.title)
            .id(win_id)
            .collapsible(false)
            .resizable(false)
            .fixed_size(Vec2::new(self.width, panel_height))
            .anchor(anchor, [0.0, 0.0])
            .frame(
                Frame::new()
                    .fill(theme.card)
                    .corner_radius(cr)
                    .stroke(Stroke::new(1.0, theme.border))
                    .inner_margin(Margin::same(24)),
            )
            .title_bar(false)
            .show(ctx, |ui| {
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

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(12.0);

                content(ui);
            });

        // Close if pointer pressed outside the sheet rect
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
