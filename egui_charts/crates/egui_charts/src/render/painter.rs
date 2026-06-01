//! Thin wrapper over `egui::Painter` providing chart-aware text + clip helpers.

use egui::{Align2, Color32, FontId, Painter, Pos2, Rect, Stroke};

pub struct ChartPainter<'a> {
    pub painter: &'a Painter,
    pub clip: Rect,
}

impl<'a> ChartPainter<'a> {
    pub fn new(painter: &'a Painter, clip: Rect) -> Self {
        Self { painter, clip }
    }

    pub fn line(&self, a: Pos2, b: Pos2, stroke: Stroke) {
        self.painter.line_segment([a, b], stroke);
    }

    pub fn rect_filled(&self, rect: Rect, fill: Color32) {
        self.painter.rect_filled(rect, 0.0, fill);
    }

    pub fn rect_stroke(&self, rect: Rect, stroke: Stroke) {
        self.painter
            .rect_stroke(rect, 0.0, stroke, egui::StrokeKind::Inside);
    }

    pub fn rounded_rect_filled(&self, rect: Rect, radius: f32, fill: Color32) {
        self.painter.rect_filled(rect, radius, fill);
    }

    pub fn text(
        &self,
        pos: Pos2,
        anchor: Align2,
        text: impl ToString,
        font: FontId,
        color: Color32,
    ) -> Rect {
        self.painter.text(pos, anchor, text, font, color)
    }

    pub fn poly(&self, pts: Vec<Pos2>, fill: Color32, stroke: Stroke) {
        self.painter.add(egui::Shape::convex_polygon(pts, fill, stroke));
    }

    pub fn path(&self, pts: Vec<Pos2>, stroke: Stroke) {
        if pts.len() < 2 {
            return;
        }
        self.painter
            .add(egui::Shape::line(pts, stroke));
    }

    pub fn circle_filled(&self, pos: Pos2, radius: f32, fill: Color32) {
        self.painter.circle_filled(pos, radius, fill);
    }

    pub fn circle_stroke(&self, pos: Pos2, radius: f32, stroke: Stroke) {
        self.painter.circle_stroke(pos, radius, stroke);
    }
}
