use super::spacing::Spacing;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Frame, InnerResponse, Margin, Shadow, Stroke, Ui};

/// Standard box surface. Draws a rounded border in the default border color and
/// overlays the flat bottom run (between the corner arcs) in the primary color —
/// the shared "box" signature reused by Card, Popover, Dialog, Sheet, etc.
pub struct Boxed {
    padding: f32,
    margin: f32,
    fill: Option<Color32>,
    corner_radius: Option<CornerRadius>,
    shadow: Option<Shadow>,
    accent: bool,
}

impl Default for Boxed {
    fn default() -> Self {
        Self {
            padding: Spacing::Md.px(),
            margin: 0.0,
            fill: None,
            corner_radius: None,
            shadow: None,
            accent: true,
        }
    }
}

impl Boxed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, s: Spacing) -> Self {
        self.padding = s.px();
        self
    }

    /// Inner padding in raw pixels (use when no `Spacing` step fits, e.g. `0`).
    pub fn padding_px(mut self, p: f32) -> Self {
        self.padding = p;
        self
    }

    pub fn margin(mut self, s: Spacing) -> Self {
        self.margin = s.px();
        self
    }

    /// Background fill. Defaults to transparent.
    pub fn fill(mut self, c: Color32) -> Self {
        self.fill = Some(c);
        self
    }

    /// Override the corner radius (defaults to the theme radius on all corners).
    /// Pass a partial `CornerRadius` for sheets/drawers that round only some edges.
    pub fn corner_radius(mut self, cr: CornerRadius) -> Self {
        self.corner_radius = Some(cr);
        self
    }

    pub fn shadow(mut self, s: Shadow) -> Self {
        self.shadow = Some(s);
        self
    }

    /// Toggle the primary-colored bottom accent line. Default `true`.
    pub fn accent(mut self, on: bool) -> Self {
        self.accent = on;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let theme = ShadcnTheme::get(ui.ctx());
        let cr = self
            .corner_radius
            .unwrap_or(CornerRadius::same(theme.radius as u8));

        let mut frame = Frame::new()
            .fill(self.fill.unwrap_or(Color32::TRANSPARENT))
            .corner_radius(cr)
            .inner_margin(Margin::same(self.padding as i8))
            .outer_margin(Margin::same(self.margin as i8));
        if let Some(shadow) = self.shadow {
            frame = frame.shadow(shadow);
        }
        let resp = frame.show(ui, content);

        // Rounded border in the default color, then overlay the straight bottom
        // run (between the corner arcs) in the primary (main) color. The rounded
        // corners stay default; the flat bottom edge is the accent.
        // `response.rect` is the box bounds (content + inner padding, excluding
        // outer margin), so its edges are the border.
        let rect = resp.response.rect;
        let painter = ui.painter();
        painter.rect_stroke(rect, cr, Stroke::new(1.0, theme.border), egui::StrokeKind::Middle);
        if self.accent {
            // Inset each end by its own bottom-corner radius so the line spans
            // exactly the flat run, whether the corners are rounded or square.
            painter.line_segment(
                [
                    egui::pos2(rect.left() + cr.sw as f32, rect.bottom()),
                    egui::pos2(rect.right() - cr.se as f32, rect.bottom()),
                ],
                Stroke::new(1.0, theme.primary),
            );
        }

        resp
    }
}
