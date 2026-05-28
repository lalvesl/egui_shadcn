use crate::ShadcnTheme;
use egui::{CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct InputOtp<'a> {
    value: &'a mut String,
    digits: usize,
    separator_after: Option<usize>,
}

impl<'a> InputOtp<'a> {
    pub fn new(value: &'a mut String, digits: usize) -> Self {
        Self {
            value,
            digits,
            separator_after: None,
        }
    }

    pub fn separator_after(mut self, n: usize) -> Self {
        self.separator_after = Some(n);
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = ShadcnTheme::get(ui.ctx());

        let box_w = 44.0f32;
        let box_h = 52.0f32;
        let gap = 8.0f32;
        let sep_w = 16.0f32;

        let sep_count = if self.separator_after.is_some() { 1 } else { 0 };
        let total_width =
            self.digits as f32 * box_w + (self.digits - 1) as f32 * gap + sep_count as f32 * (sep_w + gap * 2.0);

        let hidden_id = egui::Id::new("shadcn_otp_hidden").with(ui.id());

        let (group_rect, group_resp) =
            ui.allocate_exact_size(Vec2::new(total_width, box_h), Sense::click());

        if group_resp.clicked() {
            ui.ctx().memory_mut(|m| m.request_focus(hidden_id));
        }

        let has_focus = ui.ctx().memory(|m| m.has_focus(hidden_id));

        let mut hidden_buf = self.value.clone();
        hidden_buf.truncate(self.digits);

        {
            let hidden_rect = egui::Rect::from_min_size(
                egui::Pos2::new(group_rect.left(), group_rect.top()),
                Vec2::new(0.0, 0.0),
            );
            let te = egui::TextEdit::singleline(&mut hidden_buf)
                .desired_width(0.0)
                .frame(egui::Frame::new())
                .id(hidden_id);

            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(hidden_rect)
                    .layout(egui::Layout::left_to_right(egui::Align::Center)),
            );
            child.add(te);
        }

        hidden_buf.truncate(self.digits);
        *self.value = hidden_buf.clone();

        let cursor_pos = self.value.len();
        let cr = CornerRadius::same(theme.radius as u8);

        let mut x = group_rect.left();

        for i in 0..self.digits {
            if let Some(sep_after) = self.separator_after {
                if i == sep_after + 1 {
                    let sep_center_y = group_rect.center().y;
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(x + 2.0, sep_center_y),
                            egui::Pos2::new(x + sep_w - 2.0, sep_center_y),
                        ],
                        Stroke::new(2.0, theme.muted_foreground),
                    );
                    x += sep_w + gap;
                }
            }

            let box_rect = egui::Rect::from_min_size(
                egui::Pos2::new(x, group_rect.top()),
                Vec2::new(box_w, box_h),
            );

            let border_color = if has_focus && i == cursor_pos.min(self.digits - 1) {
                theme.ring
            } else {
                theme.border
            };

            ui.painter().rect_filled(box_rect, cr, theme.background);
            ui.painter().rect_stroke(
                box_rect,
                cr,
                Stroke::new(1.0, border_color),
                egui::StrokeKind::Inside,
            );

            let ch: Option<char> = self.value.chars().nth(i);

            if let Some(c) = ch {
                ui.painter().text(
                    box_rect.center(),
                    egui::Align2::CENTER_CENTER,
                    c.to_string(),
                    egui::FontId::new(20.0, egui::FontFamily::Proportional),
                    theme.foreground,
                );
            } else if has_focus && i == cursor_pos {
                let cursor_x = box_rect.center().x;
                let cursor_top = box_rect.center().y - 12.0;
                let cursor_bot = box_rect.center().y + 12.0;

                let blink = {
                    let t = ui.ctx().input(|i| i.time);
                    (t * 1.2).floor() as i64 % 2 == 0
                };

                if blink {
                    ui.painter().line_segment(
                        [
                            egui::Pos2::new(cursor_x, cursor_top),
                            egui::Pos2::new(cursor_x, cursor_bot),
                        ],
                        Stroke::new(2.0, theme.foreground),
                    );
                }
                ui.ctx().request_repaint();
            }

            x += box_w + gap;
        }

        group_resp
    }
}
