use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

pub struct Toggle<'a> {
    pressed: &'a mut bool,
    label: &'a str,
    icon: Option<&'a str>,
    enabled: bool,
}

impl<'a> Toggle<'a> {
    pub fn new(pressed: &'a mut bool, label: &'a str) -> Self {
        Self { pressed, label, icon: None, enabled: true }
    }
    pub fn icon(mut self, i: &'a str) -> Self { self.icon = Some(i); self }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let h_pad = 12.0;
        let height = 36.0;
        let fs = 13.0;

        let text_g = ui.painter().layout_no_wrap(
            self.label.to_owned(),
            egui::FontId::new(fs, egui::FontFamily::Proportional),
            Color32::TRANSPARENT,
        );
        let icon_g = self.icon.map(|ic| {
            ui.painter().layout_no_wrap(
                ic.to_owned(),
                egui::FontId::new(fs + 2.0, egui::FontFamily::Name("MaterialIcons".into())),
                Color32::TRANSPARENT,
            )
        });

        let icon_w = icon_g.as_ref().map(|g| g.size().x + 6.0).unwrap_or(0.0);
        let total_w = text_g.size().x + icon_w + h_pad * 2.0;
        let size = Vec2::new(total_w.max(height), height);
        let (rect, resp) = ui.allocate_exact_size(
            size,
            if self.enabled { Sense::click() } else { Sense::hover() },
        );

        if resp.clicked() && self.enabled {
            *self.pressed = !*self.pressed;
        }

        if ui.is_rect_visible(rect) {
            let cr = CornerRadius::same(theme.radius as u8);
            let (bg, fg) = if *self.pressed {
                (theme.accent, theme.accent_foreground)
            } else if resp.hovered() && self.enabled {
                (ShadcnTheme::with_alpha(theme.accent, 120), theme.foreground)
            } else {
                (Color32::TRANSPARENT, theme.muted_foreground)
            };
            ui.painter().rect_filled(rect, cr, bg);

            let mut cx = rect.left() + h_pad;
            if let Some(g) = icon_g {
                let pos = egui::Pos2::new(cx, rect.center().y - g.size().y / 2.0);
                ui.painter().galley(pos, g, fg);
                cx += icon_w;
            }
            let ty = rect.center().y - text_g.size().y / 2.0;
            ui.painter().galley(egui::Pos2::new(cx, ty), text_g, fg);
        }
        resp
    }
}
