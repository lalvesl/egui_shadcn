use crate::ShadcnTheme;
use egui::{Sense, Ui};

pub struct Breadcrumb<'a> {
    items: &'a [&'a str],
    separator: &'a str,
}

impl<'a> Breadcrumb<'a> {
    pub fn new(items: &'a [&'a str]) -> Self {
        Self { items, separator: "/" }
    }
    pub fn separator(mut self, s: &'a str) -> Self { self.separator = s; self }

    /// Returns Some(index) if a non-last item was clicked.
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let mut clicked = None;
        let fs = 14.0;
        let last = self.items.len().saturating_sub(1);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 4.0;
            for (i, item) in self.items.iter().enumerate() {
                let is_last = i == last;
                let color = if is_last { theme.foreground } else { theme.muted_foreground };

                let galley = ui.painter().layout_no_wrap(
                    item.to_string(),
                    egui::FontId::new(fs, egui::FontFamily::Proportional),
                    color,
                );
                let size = galley.size();
                let sense = if is_last { Sense::hover() } else { Sense::click() };
                let (rect, resp) = ui.allocate_exact_size(size, sense);

                if !is_last && resp.hovered() {
                    ui.painter().galley(rect.min, galley, theme.foreground);
                } else {
                    ui.painter().galley(rect.min, galley, color);
                }

                if !is_last && resp.clicked() {
                    clicked = Some(i);
                }

                if i < last {
                    let sep_g = ui.painter().layout_no_wrap(
                        self.separator.to_owned(),
                        egui::FontId::new(fs, egui::FontFamily::Proportional),
                        theme.muted_foreground,
                    );
                    let sz = sep_g.size();
                    let (r2, _) = ui.allocate_exact_size(sz, Sense::hover());
                    ui.painter().galley(r2.min, sep_g, theme.muted_foreground);
                }
            }
        });
        clicked
    }
}
