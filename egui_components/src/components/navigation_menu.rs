use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Ui, Vec2};

pub struct NavItem<'a> {
    pub label: &'a str,
    pub icon: Option<&'a str>,
    pub badge: Option<&'a str>,
}

pub enum NavOrientation {
    Horizontal,
    Vertical,
}

pub struct NavigationMenu<'a> {
    items: &'a [NavItem<'a>],
    active: usize,
    orientation: NavOrientation,
}

impl<'a> NavigationMenu<'a> {
    pub fn new(items: &'a [NavItem<'a>], active: usize) -> Self {
        Self {
            items,
            active,
            orientation: NavOrientation::Horizontal,
        }
    }

    pub fn vertical(mut self) -> Self {
        self.orientation = NavOrientation::Vertical;
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let theme = ShadcnTheme::get(ui.ctx());
        let mut clicked: Option<usize> = None;

        let item_height = 36.0;
        let h_padding = 12.0;
        let icon_font = crate::icon_font_id(16.0);
        let label_font =
            egui::FontId::new(14.0, egui::FontFamily::Proportional);
        let badge_font =
            egui::FontId::new(11.0, egui::FontFamily::Proportional);

        let draw_item = |ui: &mut Ui,
                         idx: usize,
                         item: &NavItem,
                         active: usize,
                         theme: &ShadcnTheme,
                         clicked: &mut Option<usize>| {
            let is_active = idx == active;

            // Measure label width to size the item
            let label_galley = ui.painter().layout_no_wrap(
                item.label.to_string(),
                label_font.clone(),
                Color32::TRANSPARENT,
            );
            let mut item_width = label_galley.rect.width() + h_padding * 2.0;
            if item.icon.is_some() {
                item_width += 20.0; // icon + gap
            }
            if item.badge.is_some() {
                item_width += 32.0; // badge pill
            }
            item_width = item_width.max(60.0);

            let (item_rect, resp) = ui.allocate_exact_size(
                Vec2::new(item_width, item_height),
                Sense::click(),
            );

            let bg = if is_active {
                theme.accent
            } else if resp.hovered() {
                ShadcnTheme::with_alpha(theme.muted, 180)
            } else {
                Color32::TRANSPARENT
            };
            let text_color = if is_active {
                theme.accent_foreground
            } else {
                theme.foreground
            };

            let cr = CornerRadius::same(theme.radius as u8);
            ui.painter().rect_filled(item_rect, cr, bg);

            let mut x = item_rect.left() + h_padding;

            // Optional icon
            if let Some(icon) = item.icon {
                ui.painter().text(
                    egui::Pos2::new(x + 8.0, item_rect.center().y),
                    egui::Align2::CENTER_CENTER,
                    icon,
                    icon_font.clone(),
                    text_color,
                );
                x += 20.0;
            }

            // Label
            ui.painter().text(
                egui::Pos2::new(x, item_rect.center().y),
                egui::Align2::LEFT_CENTER,
                item.label,
                label_font.clone(),
                text_color,
            );

            // Optional badge
            if let Some(badge_text) = item.badge {
                let badge_x = item_rect.right() - h_padding - 10.0;
                let badge_center =
                    egui::Pos2::new(badge_x, item_rect.center().y);
                let badge_rect = egui::Rect::from_center_size(
                    badge_center,
                    Vec2::new(20.0, 16.0),
                );
                ui.painter().rect_filled(
                    badge_rect,
                    CornerRadius::same(8),
                    theme.primary,
                );
                ui.painter().text(
                    badge_center,
                    egui::Align2::CENTER_CENTER,
                    badge_text,
                    badge_font.clone(),
                    theme.primary_foreground,
                );
            }

            if resp.clicked() {
                *clicked = Some(idx);
            }
            if resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }
        };

        match self.orientation {
            NavOrientation::Horizontal => {
                ui.horizontal(|ui| {
                    ui.spacing_mut().item_spacing.x = 4.0;
                    for (idx, item) in self.items.iter().enumerate() {
                        draw_item(
                            ui,
                            idx,
                            item,
                            self.active,
                            &theme,
                            &mut clicked,
                        );
                    }
                });
            }
            NavOrientation::Vertical => {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    for (idx, item) in self.items.iter().enumerate() {
                        draw_item(
                            ui,
                            idx,
                            item,
                            self.active,
                            &theme,
                            &mut clicked,
                        );
                    }
                });
            }
        }

        clicked
    }
}
