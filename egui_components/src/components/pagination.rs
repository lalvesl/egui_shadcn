use super::button::{Button, ButtonSize, ButtonVariant};
use crate::{ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ShadcnTheme};
use egui::{Sense, Ui, Vec2};

pub struct Pagination {
    current: usize,
    total: usize,
    siblings: usize,
}

impl Pagination {
    pub fn new(current: usize, total: usize) -> Self {
        Self {
            current,
            total,
            siblings: 1,
        }
    }
    pub fn siblings(mut self, s: usize) -> Self {
        self.siblings = s;
        self
    }

    /// Returns Some(new_page) if a page was clicked (1-indexed).
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let mut result = None;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            let prev_en = self.current > 1;
            if Button::new("")
                .icon(ICON_CHEVRON_LEFT)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .enabled(prev_en)
                .show(ui)
                .clicked()
            {
                result = Some(self.current - 1);
            }

            let pages = self.visible_pages();
            let mut prev_page: Option<usize> = None;
            for page in &pages {
                if let Some(pp) = prev_page
                    && *page > pp + 1
                {
                    Self::ellipsis(ui);
                }
                let active = self.current == *page;
                let label = page.to_string();
                let variant = if active {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Ghost
                };
                if Button::new(&label).variant(variant).show(ui).clicked()
                    && !active
                {
                    result = Some(*page);
                }
                prev_page = Some(*page);
            }

            let next_en = self.current < self.total;
            if Button::new("")
                .icon(ICON_CHEVRON_RIGHT)
                .variant(ButtonVariant::Ghost)
                .size(ButtonSize::Icon)
                .enabled(next_en)
                .show(ui)
                .clicked()
            {
                result = Some(self.current + 1);
            }
        });
        result
    }

    fn visible_pages(&self) -> Vec<usize> {
        if self.total <= 7 {
            return (1..=self.total).collect();
        }
        let mut pages = std::collections::BTreeSet::new();
        pages.insert(1);
        pages.insert(self.total);
        let start = self.current.saturating_sub(self.siblings).max(1);
        let end = (self.current + self.siblings).min(self.total);
        for p in start..=end {
            pages.insert(p);
        }
        pages.into_iter().collect()
    }

    fn ellipsis(ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());
        let (rect, _) =
            ui.allocate_exact_size(Vec2::splat(36.0), Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "…",
                egui::FontId::new(14.0, egui::FontFamily::Proportional),
                theme.muted_foreground,
            );
        }
    }
}
