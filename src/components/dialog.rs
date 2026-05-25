use egui::{Context, Margin, Sense, Stroke, Ui};
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

        // Semi-transparent backdrop
        let screen = ctx.screen_rect();
        egui::Area::new(id.with("backdrop"))
            .fixed_pos(screen.min)
            .order(egui::Order::Background)
            .show(ctx, |ui| {
                let (rect, _) = ui.allocate_exact_size(screen.size(), Sense::hover());
                ui.painter().rect_filled(rect, 0u8, egui::Color32::from_rgba_unmultiplied(0, 0, 0, 140));
            });

        let mut should_close = false;

        egui::Window::new(title)
            .id(id.with("win"))
            .collapsible(false)
            .resizable(false)
            .anchor(egui::Align2::CENTER_CENTER, [0.0, 0.0])
            .frame(
                egui::Frame::new()
                    .fill(theme.background)
                    .stroke(Stroke::new(1.0, theme.border))
                    .corner_radius(theme.radius + 2)
                    .inner_margin(Margin::same(24)),
            )
            .show(ctx, |ui| {
                ui.set_min_width(400.0);
                ui.label(
                    egui::RichText::new(title)
                        .size(18.0)
                        .strong()
                        .color(theme.foreground)
                );
                ui.add_space(4.0);
                add_contents(ui, &mut should_close);
            });

        if should_close {
            ctx.data_mut(|d| d.insert_temp(id, false));
        }
    }

    pub fn open(ctx: &Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, true));
    }

    pub fn close(ctx: &Context, id: egui::Id) {
        ctx.data_mut(|d| d.insert_temp(id, false));
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
