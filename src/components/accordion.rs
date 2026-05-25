use egui::{Margin, Sense, Stroke, Ui};
use crate::theme::ShadcnTheme;

pub struct Accordion;

impl Accordion {
    pub fn item(
        ui:           &mut Ui,
        id:           egui::Id,
        title:        &str,
        add_contents: impl FnOnce(&mut Ui),
    ) {
        let theme = ShadcnTheme::get(ui.ctx());
        let open  = ui.data(|d| d.get_temp::<bool>(id).unwrap_or(false));

        let (sep_rect, _) = ui.allocate_exact_size(
            egui::Vec2::new(ui.available_width(), 1.0),
            Sense::hover(),
        );
        ui.painter().rect_filled(sep_rect, 0u8, theme.border);

        let header = ui.horizontal(|ui| {
            ui.set_min_height(44.0);

            let label = ui.add(
                egui::Label::new(
                    egui::RichText::new(title).size(14.0).strong().color(theme.foreground)
                ).sense(Sense::click()),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                let t     = ui.ctx().animate_bool(id.with("chevron"), open);
                let angle = t * std::f32::consts::PI;
                let cr    = ui.cursor();
                let cx    = cr.left() + 8.0;
                let cy    = cr.top()  + 22.0;
                let size  = 4.0;

                let pts = chevron_points(egui::pos2(cx, cy), angle, size);
                ui.allocate_exact_size(egui::Vec2::new(20.0, 44.0), Sense::hover());
                ui.painter().add(egui::Shape::line(pts, Stroke::new(1.5, theme.muted_foreground)));
            });

            label
        });

        if header.inner.clicked() {
            ui.data_mut(|d| d.insert_temp(id, !open));
        }

        let t = ui.ctx().animate_bool(id.with("open"), open);
        if t > 0.0 {
            egui::Frame::new()
                .inner_margin(Margin { left: 0, right: 0, top: 0, bottom: 16 })
                .show(ui, |ui| {
                    ui.label(
                        egui::RichText::new("").size(0.0)
                    );
                    add_contents(ui);
                });
        }
    }
}

fn chevron_points(center: egui::Pos2, flip_angle: f32, size: f32) -> Vec<egui::Pos2> {
    let base = std::f32::consts::FRAC_PI_2;
    let rot  = flip_angle;
    let la   = base + rot + std::f32::consts::FRAC_PI_4;
    let ra   = base + rot - std::f32::consts::FRAC_PI_4;
    vec![
        egui::pos2(center.x - size * la.cos(), center.y - size * la.sin()),
        center,
        egui::pos2(center.x - size * ra.cos(), center.y - size * ra.sin()),
    ]
}
