/// Breadcrumb navigation component.
use egui::{Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

pub struct BreadcrumbItem<'a> {
    pub label:     &'a str,
    pub clickable: bool,
}

impl<'a> BreadcrumbItem<'a> {
    pub fn link(label: &'a str) -> Self    { Self { label, clickable: true } }
    pub fn current(label: &'a str) -> Self { Self { label, clickable: false } }
}

pub struct Breadcrumb<'a> {
    items:     &'a [BreadcrumbItem<'a>],
    separator: &'a str,
}

impl<'a> Breadcrumb<'a> {
    pub fn new(items: &'a [BreadcrumbItem<'a>]) -> Self {
        Self { items, separator: "/" }
    }

    pub fn separator(mut self, s: &'a str) -> Self { self.separator = s; self }

    /// Returns the index of the clicked link item (if any).
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let mut clicked = None;
        let last_idx = self.items.len().saturating_sub(1);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            for (i, item) in self.items.iter().enumerate() {
                let is_last = i == last_idx;

                if item.clickable && !is_last {
                    // Link item — underlines on hover
                    let galley = ui.painter().layout_no_wrap(
                        item.label.to_owned(),
                        egui::FontId::proportional(14.0),
                        theme.foreground,
                    );
                    let size = galley.size();
                    let (rect, response) = ui.allocate_exact_size(
                        Vec2::new(size.x, size.y + 2.0),
                        Sense::click(),
                    );

                    if ui.is_rect_visible(rect) {
                        let color = if response.hovered() { theme.foreground } else { theme.muted_foreground };
                        let galley = ui.painter().layout_no_wrap(
                            item.label.to_owned(),
                            egui::FontId::proportional(14.0),
                            color,
                        );
                        ui.painter().galley(rect.left_top(), galley, color);

                        if response.hovered() {
                            ui.painter().hline(
                                rect.left()..=rect.right(),
                                rect.bottom() - 1.0,
                                Stroke::new(1.0, color),
                            );
                        }
                    }

                    if response.clicked() { clicked = Some(i); }
                } else {
                    // Current page or non-clickable — always muted foreground
                    let color = if is_last { theme.foreground } else { theme.muted_foreground };
                    ui.label(egui::RichText::new(item.label).size(14.0).color(color));
                }

                // Separator (skip after last item)
                if !is_last {
                    ui.label(
                        egui::RichText::new(self.separator)
                            .size(14.0)
                            .color(theme.muted_foreground),
                    );
                }
            }
        });

        clicked
    }
}

// ─── Ellipsis / collapsed breadcrumb ──────────────────────────────────────

pub struct CollapsedBreadcrumb<'a> {
    first:   &'a str,
    last:    &'a str,
    open:    &'a mut bool,
    hidden:  &'a [&'a str],
}

impl<'a> CollapsedBreadcrumb<'a> {
    pub fn new(
        first:  &'a str,
        last:   &'a str,
        open:   &'a mut bool,
        hidden: &'a [&'a str],
    ) -> Self {
        Self { first, last, open, hidden }
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let mut clicked = None;
        let sep = egui::RichText::new("/").size(14.0).color(theme.muted_foreground);

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;

            // First item as link
            if ui.add(
                egui::Button::new(egui::RichText::new(self.first).size(14.0).color(theme.muted_foreground))
                    .frame(false)
            ).clicked() {
                clicked = Some(0);
            }

            ui.label(sep.clone());

            if *self.open {
                // Expanded: show hidden items
                for (i, &item) in self.hidden.iter().enumerate() {
                    if ui.add(
                        egui::Button::new(egui::RichText::new(item).size(14.0).color(theme.muted_foreground))
                            .frame(false)
                    ).clicked() {
                        clicked = Some(i + 1);
                    }
                    ui.label(sep.clone());
                }
            } else {
                // Collapsed: ellipsis button
                let ellipsis_btn = ui.add(
                    egui::Button::new(egui::RichText::new("…").size(14.0).color(theme.muted_foreground))
                        .frame(false)
                        .min_size(Vec2::new(24.0, 22.0))
                        .stroke(Stroke::new(1.0, theme.border))
                        .fill(theme.background)
                );

                // Draw a small bordered pill around the ellipsis
                if ui.is_rect_visible(ellipsis_btn.rect) {
                    ui.painter().rect_stroke(
                        ellipsis_btn.rect,
                        theme.radius,
                        Stroke::new(1.0, theme.border),
                        StrokeKind::Inside,
                    );
                }

                if ellipsis_btn.clicked() { *self.open = true; }
                ui.label(sep);
            }

            // Last item (current page)
            ui.label(egui::RichText::new(self.last).size(14.0).color(theme.foreground));
        });

        clicked
    }
}
