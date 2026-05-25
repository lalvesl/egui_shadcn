use egui::{Color32, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

const PILL: u8 = 255;

#[derive(Clone, Copy, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

pub struct Badge<'a> {
    label:   &'a str,
    variant: BadgeVariant,
}

impl<'a> Badge<'a> {
    pub fn new(label: &'a str) -> Self {
        Self { label, variant: BadgeVariant::Default }
    }

    pub fn variant(mut self, v: BadgeVariant) -> Self { self.variant = v; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        let (bg, fg, border) = match self.variant {
            BadgeVariant::Default     => (theme.primary, theme.primary_foreground, Stroke::NONE),
            BadgeVariant::Secondary   => (theme.secondary, theme.secondary_foreground, Stroke::NONE),
            BadgeVariant::Destructive => (theme.destructive, theme.destructive_foreground, Stroke::NONE),
            BadgeVariant::Outline     => (Color32::TRANSPARENT, theme.foreground, Stroke::new(1.0, theme.border)),
        };

        let galley  = ui.painter().layout_no_wrap(
            self.label.to_owned(),
            egui::FontId::proportional(12.0),
            fg,
        );
        let text_sz = galley.size();
        let desired = Vec2::new(text_sz.x + 16.0, text_sz.y + 4.0);
        let (rect, _) = ui.allocate_exact_size(desired, egui::Sense::hover());

        if ui.is_rect_visible(rect) {
            ui.painter().rect_filled(rect, PILL, bg);

            if border != Stroke::NONE {
                ui.painter().rect_stroke(rect, PILL, border, StrokeKind::Inside);
            }

            ui.painter().galley(rect.center() - text_sz * 0.5, galley, fg);
        }
    }
}
