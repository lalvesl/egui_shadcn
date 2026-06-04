use super::spacing::Spacing;
use crate::ShadcnTheme;
use egui::{Color32, Frame, InnerResponse, Margin, Stroke, Ui};

pub struct Boxed {
    padding: Spacing,
    margin: Option<Spacing>,
}

impl Default for Boxed {
    fn default() -> Self {
        Self {
            padding: Spacing::Md,
            margin: None,
        }
    }
}

impl Boxed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, s: Spacing) -> Self {
        self.padding = s;
        self
    }

    pub fn margin(mut self, s: Spacing) -> Self {
        self.margin = Some(s);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let inner_margin = Margin::same(self.padding.px() as i8);
        let outer_margin = Margin::same(self.margin.map(|s| s.px() as i8).unwrap_or(0));

        let theme = ShadcnTheme::get(ui.ctx());
        let resp = Frame::new()
            .fill(Color32::TRANSPARENT)
            .inner_margin(inner_margin)
            .outer_margin(outer_margin)
            .show(ui, content);

        // Rounded border in the default color, then overlay the straight bottom
        // run (between the corner arcs) in the primary (main) color. The rounded
        // corners stay default; the flat bottom edge is the accent.
        // `response.rect` is the box bounds (content + inner padding, excluding
        // outer margin), so its edges are the border.
        let rect = resp.response.rect;
        let radius = theme.radius;
        let painter = ui.painter();
        painter.rect_stroke(
            rect,
            egui::CornerRadius::same(radius as u8),
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Middle,
        );
        painter.line_segment(
            [
                egui::pos2(rect.left() + radius, rect.bottom()),
                egui::pos2(rect.right() - radius, rect.bottom()),
            ],
            Stroke::new(1.0, theme.primary),
        );

        resp
    }
}
