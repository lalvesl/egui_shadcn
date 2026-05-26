use egui::{Color32, Context, Key, Margin, Sense, Stroke, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct Dialog;

impl Dialog {
    pub fn show(
        ctx:          &Context,
        id:           egui::Id,
        title:        &str,
        add_contents: impl FnOnce(&mut Ui, &mut bool),
    ) {
        let open = ctx.data(|d| d.get_temp::<bool>(id).unwrap_or(false));
        if !open { return; }

        let theme = ShadcnTheme::get(ctx);
        let mut should_close = false;

        // Escape key
        if ctx.input(|i| i.key_pressed(Key::Escape)) {
            should_close = true;
        }

        // Click-outside: skip on the very first frame (the opening click would
        // match "outside" since no rect is stored yet from this session).
        let first_frame: bool = ctx.data(|d| d.get_temp(id.with("first_frame")).unwrap_or(true));
        if !first_frame {
            let dialog_rect: Option<egui::Rect> = ctx.data(|d| d.get_temp(id.with("rect")));
            if ctx.input(|i| i.pointer.any_click())
                && let Some(pos) = ctx.input(|i| i.pointer.interact_pos())
                && let Some(dr) = dialog_rect
                && !dr.contains(pos)
            {
                should_close = true;
            }
        }
        ctx.data_mut(|d| d.insert_temp(id.with("first_frame"), false));

        // Backdrop
        let screen = ctx.viewport_rect();
        egui::Area::new(id.with("backdrop"))
            .fixed_pos(screen.min)
            .order(egui::Order::Background)
            .interactable(false)
            .show(ctx, |ui| {
                let (r, _) = ui.allocate_exact_size(screen.size(), Sense::hover());
                ui.painter().rect_filled(r, 0u8, Color32::from_rgba_unmultiplied(0, 0, 0, 140));
            });

        // Dialog panel
        let resp = egui::Area::new(id.with("dialog"))
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .order(egui::Order::Foreground)
            .show(ctx, |ui| {
                egui::Frame::new()
                    .fill(theme.background)
                    .stroke(Stroke::new(1.0, theme.border))
                    .corner_radius(theme.radius + 2)
                    .inner_margin(Margin::same(24))
                    .show(ui, |ui| {
                        ui.set_min_width(440.0);
                        ui.set_max_width(520.0);

                        ui.horizontal(|ui| {
                            ui.label(
                                egui::RichText::new(title)
                                    .size(18.0)
                                    .strong()
                                    .color(theme.foreground),
                            );
                            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                let btn = ui.add(
                                    egui::Button::new(
                                        egui::RichText::new("✕").size(14.0).color(theme.muted_foreground)
                                    )
                                    .frame(false)
                                    .min_size(Vec2::splat(24.0)),
                                );
                                if btn.clicked() { should_close = true; }
                            });
                        });

                        ui.add_space(4.0);
                        add_contents(ui, &mut should_close);
                    });
            });

        // Store rect for next-frame click-outside detection
        ctx.data_mut(|d| d.insert_temp(id.with("rect"), resp.response.rect));

        if should_close {
            // Clear session state so re-open starts fresh
            ctx.data_mut(|d| {
                d.insert_temp(id, false);
                d.insert_temp::<bool>(id.with("first_frame"), true);
                d.remove::<egui::Rect>(id.with("rect"));
            });
        }
    }

    pub fn open(ctx: &Context, id: egui::Id) {
        ctx.data_mut(|d| {
            d.insert_temp(id, true);
            d.insert_temp::<bool>(id.with("first_frame"), true);
        });
    }
}

pub fn dialog_description(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(egui::RichText::new(text).size(14.0).color(theme.muted_foreground));
    ui.add_space(16.0);
}

pub fn dialog_footer(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
    ui.add_space(16.0);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        add_contents(ui);
    });
}
