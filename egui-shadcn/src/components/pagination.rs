use crate::{ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ShadcnTheme};
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Pagination {
    current: usize,
    total: usize,
    siblings: usize,
}

impl Pagination {
    pub fn new(current: usize, total: usize) -> Self {
        Self { current, total, siblings: 1 }
    }
    pub fn siblings(mut self, s: usize) -> Self { self.siblings = s; self }

    /// Returns Some(new_page) if a page was clicked (1-indexed).
    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let mut result = None;
        let btn_size = 36.0;
        let r = theme.radius as u8;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 2.0;

            // Prev
            let prev_en = self.current > 1;
            if Self::nav_btn(ui, &theme, ICON_CHEVRON_LEFT, btn_size, prev_en) && prev_en {
                result = Some(self.current - 1);
            }

            // Page buttons
            let pages = self.visible_pages();
            let mut prev_page: Option<usize> = None;
            for page in &pages {
                if let Some(pp) = prev_page
                    && *page > pp + 1
                {
                    // ellipsis
                    Self::ellipsis(ui, &theme, btn_size);
                }
                if Self::page_btn(ui, &theme, *page, self.current == *page, btn_size, r)
                    && self.current != *page
                {
                    result = Some(*page);
                }
                prev_page = Some(*page);
            }

            // Next
            let next_en = self.current < self.total;
            if Self::nav_btn(ui, &theme, ICON_CHEVRON_RIGHT, btn_size, next_en) && next_en {
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
        for p in start..=end { pages.insert(p); }
        pages.into_iter().collect()
    }

    fn page_btn(ui: &mut Ui, theme: &ShadcnTheme, page: usize, active: bool, sz: f32, r: u8) -> bool {
        let label = page.to_string();
        let galley = ui.painter().layout_no_wrap(
            label,
            egui::FontId::new(13.0, egui::FontFamily::Proportional),
            Color32::PLACEHOLDER,
        );
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(sz), Sense::click());
        if ui.is_rect_visible(rect) {
            let cr = CornerRadius::same(r);
            let (bg, fg) = if active {
                (theme.primary, theme.primary_foreground)
            } else if resp.hovered() {
                (theme.accent, theme.accent_foreground)
            } else {
                (Color32::TRANSPARENT, theme.foreground)
            };
            ui.painter().rect_filled(rect, cr, bg);
            if !active {
                ui.painter().rect_stroke(rect, cr, Stroke::new(1.0, Color32::TRANSPARENT), egui::StrokeKind::Inside);
            }
            let pos = egui::Pos2::new(
                rect.center().x - galley.size().x / 2.0,
                rect.center().y - galley.size().y / 2.0,
            );
            ui.painter().galley(pos, galley, fg);
        }
        resp.clicked()
    }

    fn nav_btn(ui: &mut Ui, theme: &ShadcnTheme, icon: &str, sz: f32, enabled: bool) -> bool {
        let (rect, resp) = ui.allocate_exact_size(Vec2::splat(sz), if enabled { Sense::click() } else { Sense::hover() });
        if ui.is_rect_visible(rect) {
            let cr = CornerRadius::same(theme.radius as u8);
            if resp.hovered() && enabled {
                ui.painter().rect_filled(rect, cr, theme.accent);
            }
            let col = if enabled { theme.foreground } else { ShadcnTheme::with_alpha(theme.foreground, 80) };
            ui.painter().text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                icon,
                crate::icon_font_id(18.0),
                col,
            );
        }
        resp.clicked() && enabled
    }

    fn ellipsis(ui: &mut Ui, theme: &ShadcnTheme, sz: f32) {
        let (rect, _) = ui.allocate_exact_size(Vec2::splat(sz), Sense::hover());
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
