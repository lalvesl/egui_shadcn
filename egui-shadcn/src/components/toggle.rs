use super::size::Size;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Response, Sense, Ui, Vec2};

pub struct Toggle<'a> {
    pressed: &'a mut bool,
    label: &'a str,
    icon: Option<&'a str>,
    enabled: bool,
    size: Size,
}

impl<'a> Toggle<'a> {
    pub fn new(pressed: &'a mut bool, label: &'a str) -> Self {
        Self { pressed, label, icon: None, enabled: true, size: Size::Default }
    }
    pub fn icon(mut self, i: &'a str) -> Self { self.icon = Some(i); self }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }
    pub fn size(mut self, s: Size) -> Self { self.size = s; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let h_pad = self.size.h_pad();
        let height = self.size.height();
        let fs = self.size.font_size();

        let text_g = ui.painter().layout_no_wrap(
            self.label.to_owned(),
            egui::FontId::new(fs, egui::FontFamily::Proportional),
            Color32::PLACEHOLDER,
        );
        let icon_g = self.icon.map(|ic| {
            ui.painter().layout_no_wrap(ic.to_owned(), crate::icon_font_id(fs + 2.0), Color32::PLACEHOLDER)
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
            let (bg, fg) = if !self.enabled {
                let base_bg = if *self.pressed { theme.primary } else { theme.secondary };
                let base_fg = if *self.pressed { theme.primary_foreground } else { theme.secondary_foreground };
                (ShadcnTheme::with_alpha(base_bg, 100), ShadcnTheme::with_alpha(base_fg, 128))
            } else if *self.pressed {
                (theme.primary, theme.primary_foreground)
            } else if resp.hovered() {
                (theme.muted, theme.muted_foreground)
            } else {
                (theme.secondary, theme.secondary_foreground)
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
