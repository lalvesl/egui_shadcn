use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonGroupVariant {
    #[default]
    Default,
    Outline,
}

pub struct ButtonGroup<'a> {
    buttons: &'a [&'a str],
    selected: Option<usize>,
    variant: ButtonGroupVariant,
    size: f32,
}

impl<'a> ButtonGroup<'a> {
    pub fn new(buttons: &'a [&'a str]) -> Self {
        Self {
            buttons,
            selected: None,
            variant: ButtonGroupVariant::Default,
            size: 36.0,
        }
    }

    pub fn selected(mut self, s: Option<usize>) -> Self {
        self.selected = s;
        self
    }

    pub fn variant(mut self, v: ButtonGroupVariant) -> Self {
        self.variant = v;
        self
    }

    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let height = self.size;
        let h_pad = 16.0;
        let r = theme.radius as u8;
        let n = self.buttons.len();
        let mut clicked_idx: Option<usize> = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for (i, &label) in self.buttons.iter().enumerate() {
                let is_first = i == 0;
                let is_last = i == n - 1;
                let is_selected = self.selected == Some(i);

                let text_g = ui.painter().layout_no_wrap(
                    label.to_owned(),
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                    Color32::TRANSPARENT,
                );

                let btn_w = (text_g.size().x + h_pad * 2.0).max(height);
                let size = Vec2::new(btn_w, height);
                let (rect, resp) = ui.allocate_exact_size(size, Sense::click());

                if resp.clicked() {
                    clicked_idx = Some(i);
                }

                if ui.is_rect_visible(rect) {
                    let cr = CornerRadius {
                        nw: if is_first { r } else { 0 },
                        ne: if is_last { r } else { 0 },
                        sw: if is_first { r } else { 0 },
                        se: if is_last { r } else { 0 },
                    };

                    let (bg, fg) = if is_selected {
                        (theme.primary, theme.primary_foreground)
                    } else if resp.hovered() {
                        (theme.accent, theme.accent_foreground)
                    } else {
                        match self.variant {
                            ButtonGroupVariant::Default => (theme.background, theme.foreground),
                            ButtonGroupVariant::Outline => {
                                (Color32::TRANSPARENT, theme.foreground)
                            }
                        }
                    };

                    ui.painter().rect_filled(rect, cr, bg);
                    ui.painter().rect_stroke(
                        rect,
                        cr,
                        Stroke::new(1.0, theme.border),
                        egui::StrokeKind::Inside,
                    );

                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        label,
                        egui::FontId::new(14.0, egui::FontFamily::Proportional),
                        fg,
                    );

                    if resp.hovered() {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                }
            }
        });

        clicked_idx
    }
}
