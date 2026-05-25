use egui::{Color32, Sense, Stroke, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct Avatar<'a> {
    initials: &'a str,
    size:     f32,
    bg:       Option<Color32>,
}

impl<'a> Avatar<'a> {
    pub fn new(initials: &'a str) -> Self {
        Self { initials, size: 40.0, bg: None }
    }

    pub fn size(mut self, s: f32) -> Self    { self.size = s; self }
    pub fn bg(mut self, c: Color32) -> Self  { self.bg = Some(c); self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::hover());

        if ui.is_rect_visible(rect) {
            let bg = self.bg.unwrap_or(theme.muted);
            let r  = self.size * 0.5;

            ui.painter().circle_filled(rect.center(), r, bg);
            ui.painter().circle_stroke(rect.center(), r - 0.5, Stroke::new(1.0, theme.border));

            let text: String = self.initials.split_whitespace()
                .take(2)
                .filter_map(|word| word.chars().next())
                .collect::<String>()
                .to_uppercase();

            let font_id = egui::FontId::proportional(self.size * 0.36);
            let galley  = ui.painter().layout_no_wrap(text, font_id, Color32::WHITE);
            let pos     = rect.center() - galley.size() * 0.5;
            ui.painter().galley(pos, galley, Color32::WHITE);
        }
    }
}
