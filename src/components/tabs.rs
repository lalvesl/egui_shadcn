use egui::{Margin, Stroke, StrokeKind, Ui};
use crate::theme::ShadcnTheme;

pub struct Tabs<'a> {
    active: &'a mut usize,
    tabs:   &'a [&'a str],
}

impl<'a> Tabs<'a> {
    pub fn new(active: &'a mut usize, tabs: &'a [&'a str]) -> Self {
        Self { active, tabs }
    }

    pub fn show_bar(&mut self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        egui::Frame::new()
            .fill(theme.muted)
            .corner_radius(theme.radius)
            .inner_margin(Margin::same(4))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    for (i, &tab) in self.tabs.iter().enumerate() {
                        let selected = *self.active == i;

                        let text_color = if selected { theme.foreground } else { theme.muted_foreground };
                        let galley     = ui.painter().layout_no_wrap(
                            tab.to_owned(),
                            egui::FontId::proportional(14.0),
                            text_color,
                        );
                        let text_sz = galley.size();
                        let desired = text_sz + egui::Vec2::new(24.0, 12.0);

                        let (rect, response) = ui.allocate_exact_size(desired, egui::Sense::click());
                        if response.clicked() {
                            *self.active = i;
                        }

                        if ui.is_rect_visible(rect) {
                            let cr = theme.radius.saturating_sub(2);

                            if selected {
                                ui.painter().rect_filled(rect, cr, theme.background);
                                ui.painter().rect_stroke(rect, cr, Stroke::new(1.0, theme.border), StrokeKind::Inside);
                            } else if response.hovered() {
                                let hover_bg = ShadcnTheme::with_alpha(theme.background, 100);
                                ui.painter().rect_filled(rect, cr, hover_bg);
                            }

                            let text_pos = rect.center() - text_sz * 0.5;
                            ui.painter().galley(text_pos, galley, text_color);
                        }
                    }
                });
            });
    }

    pub fn content(active: usize, index: usize, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui)) {
        if active == index {
            add_contents(ui);
        }
    }
}
