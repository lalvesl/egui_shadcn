use crate::ShadcnTheme;
use egui::{CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Textarea<'a> {
    value: &'a mut String,
    placeholder: &'a str,
    label: Option<&'a str>,
    rows: usize,
    enabled: bool,
}

impl<'a> Textarea<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: "",
            label: None,
            rows: 4,
            enabled: true,
        }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = p;
        self
    }
    pub fn label(mut self, l: &'a str) -> Self {
        self.label = Some(l);
        self
    }
    pub fn rows(mut self, r: usize) -> Self {
        self.rows = r;
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = ShadcnTheme::get(ui.ctx());

        if let Some(lbl) = self.label {
            ui.label(
                egui::RichText::new(lbl)
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground),
            );
            ui.add_space(4.0);
        }

        let line_h = 20.0;
        let v_pad = 8.0;
        let height = line_h * self.rows as f32 + v_pad * 2.0;
        let width = ui.available_width();
        let cr = CornerRadius::same(theme.radius as u8);
        let bg = if self.enabled {
            theme.background
        } else {
            theme.muted
        };

        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        ui.painter().rect_filled(rect, cr, bg);
        ui.painter().rect_stroke(
            rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        let inner_rect = rect.shrink2(Vec2::new(12.0, v_pad));

        let te = egui::TextEdit::multiline(self.value)
            .desired_width(inner_rect.width())
            .desired_rows(self.rows)
            .hint_text(self.placeholder)
            .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
            .text_color(theme.foreground)
            .interactive(self.enabled)
            .frame(egui::Frame::new());

        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );
        let resp = child.add(te);

        if resp.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(1.0),
                cr,
                Stroke::new(2.0, theme.ring),
                egui::StrokeKind::Outside,
            );
        }

        resp
    }
}
