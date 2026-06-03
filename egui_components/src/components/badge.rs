use super::size::Size;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Ui, Vec2};

#[derive(Clone, Copy, PartialEq, Default)]
pub enum BadgeVariant {
    #[default]
    Default,
    Secondary,
    Destructive,
    Outline,
}

pub struct Badge<'a> {
    label: &'a str,
    variant: BadgeVariant,
    size: Size,
}

impl<'a> Badge<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: BadgeVariant::Default,
            size: Size::Default,
        }
    }

    pub fn variant(mut self, v: BadgeVariant) -> Self {
        self.variant = v;
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());
        let (bg, fg, border) = self.colors(&theme);

        let font_size = match self.size {
            Size::Sm => 11.0,
            Size::Default => 12.0,
            Size::Lg => 13.0,
        };
        let h_pad = match self.size {
            Size::Sm => 6.0,
            Size::Default => 10.0,
            Size::Lg => 14.0,
        };
        let v_pad = match self.size {
            Size::Sm => 1.0,
            Size::Default => 2.0,
            Size::Lg => 3.0,
        };

        let font = egui::FontId::new(font_size, egui::FontFamily::Proportional);
        let galley = ui.painter().layout_no_wrap(self.label.to_owned(), font, fg);

        let size = Vec2::new(galley.size().x + h_pad * 2.0, galley.size().y + v_pad * 2.0);
        let (rect, _) = ui.allocate_exact_size(size, Sense::hover());

        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            let cr = CornerRadius::same(255);
            painter.rect_filled(rect, cr, bg);
            if let Some(border_color) = border {
                painter.rect_stroke(
                    rect,
                    cr,
                    egui::Stroke::new(1.0, border_color),
                    egui::StrokeKind::Inside,
                );
            }
            painter.galley(
                egui::Pos2::new(rect.left() + h_pad, rect.top() + v_pad),
                galley,
                fg,
            );
        }
    }

    fn colors(&self, t: &ShadcnTheme) -> (Color32, Color32, Option<Color32>) {
        match self.variant {
            BadgeVariant::Default => (t.primary, t.primary_foreground, None),
            BadgeVariant::Secondary => (t.secondary, t.secondary_foreground, None),
            BadgeVariant::Destructive => (t.destructive, t.destructive_foreground, None),
            BadgeVariant::Outline => (Color32::TRANSPARENT, t.foreground, Some(t.border)),
        }
    }
}
