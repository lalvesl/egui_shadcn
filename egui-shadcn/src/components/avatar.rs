use egui::{Color32, Sense, Ui, Vec2};
use crate::ShadcnTheme;

#[derive(Clone, Copy, PartialEq)]
pub enum AvatarSize {
    Sm,
    Default,
    Lg,
}

impl AvatarSize {
    fn px(self) -> f32 {
        match self {
            AvatarSize::Sm      => 32.0,
            AvatarSize::Default => 40.0,
            AvatarSize::Lg      => 56.0,
        }
    }
}

pub struct Avatar<'a> {
    initials: &'a str,
    color:    Option<Color32>,
    size:     AvatarSize,
}

impl<'a> Avatar<'a> {
    pub fn new(initials: &'a str) -> Self {
        Self { initials, color: None, size: AvatarSize::Default }
    }

    pub fn color(mut self, c: Color32)  -> Self { self.color = Some(c); self }
    pub fn size(mut self, s: AvatarSize) -> Self { self.size  = s; self }

    pub fn show(self, ui: &mut Ui) {
        let theme  = ShadcnTheme::get(ui.ctx());
        let px     = self.size.px();
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(px), Sense::hover());

        if !ui.is_rect_visible(rect) { return }

        let painter = ui.painter();
        let center  = rect.center();
        let radius  = px / 2.0;

        let bg = self.color.unwrap_or_else(|| {
            // deterministic color from initials
            let hash: u32 = self.initials.bytes().fold(0u32, |a, b| a.wrapping_mul(31).wrapping_add(b as u32));
            let hue = (hash % 360) as f32;
            crate::theme::hsl(hue, 0.6, if theme.dark { 0.4 } else { 0.55 })
        });

        painter.circle_filled(center, radius, bg);

        let font_size = px * 0.38;
        let display = if self.initials.len() > 2 { &self.initials[..2] } else { self.initials };

        painter.text(
            center,
            egui::Align2::CENTER_CENTER,
            display.to_uppercase(),
            egui::FontId::new(font_size, egui::FontFamily::Proportional),
            Color32::WHITE,
        );
    }
}
