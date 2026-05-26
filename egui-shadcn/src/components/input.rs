use crate::ShadcnTheme;
use egui::{CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Input<'a> {
    value: &'a mut String,
    placeholder: &'a str,
    label: Option<&'a str>,
    password: bool,
    enabled: bool,
    icon_left: Option<&'a str>,
    width: Option<f32>,
}

impl<'a> Input<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: "",
            label: None,
            password: false,
            enabled: true,
            icon_left: None,
            width: None,
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
    pub fn password(mut self, p: bool) -> Self {
        self.password = p;
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }
    pub fn icon_left(mut self, i: &'a str) -> Self {
        self.icon_left = Some(i);
        self
    }
    pub fn width(mut self, w: f32) -> Self {
        self.width = Some(w);
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let width = self.width.unwrap_or(ui.available_width());

        if let Some(lbl) = self.label {
            ui.label(
                egui::RichText::new(lbl)
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground),
            );
            ui.add_space(4.0);
        }

        let height = 36.0;
        let cr = CornerRadius::same(theme.radius as u8);
        let bg = if self.enabled {
            theme.background
        } else {
            theme.muted
        };

        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        // Draw background and border using temporary painter borrows
        ui.painter().rect_filled(rect, cr, bg);
        ui.painter().rect_stroke(
            rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        let x_pad = 12.0;
        let icon_w = if self.icon_left.is_some() { 24.0 } else { 0.0 };

        if let Some(icon) = self.icon_left {
            ui.painter().text(
                egui::Pos2::new(rect.left() + x_pad, rect.center().y),
                egui::Align2::LEFT_CENTER,
                icon,
                egui::FontId::new(16.0, egui::FontFamily::Name("MaterialIcons".into())),
                theme.muted_foreground,
            );
        }

        let inner_rect = egui::Rect::from_min_max(
            egui::Pos2::new(rect.left() + x_pad + icon_w, rect.top()),
            egui::Pos2::new(rect.right() - x_pad, rect.bottom()),
        );

        let te = egui::TextEdit::singleline(self.value)
            .desired_width(inner_rect.width())
            .password(self.password)
            .hint_text(self.placeholder)
            .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
            .text_color(if self.enabled {
                theme.foreground
            } else {
                theme.muted_foreground
            })
            .interactive(self.enabled)
            .frame(egui::Frame::new());

        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
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
