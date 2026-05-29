use super::size::Size;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct ToggleGroupItem<'a> {
    pub label: &'a str,
    pub icon: Option<&'a str>,
}

pub struct ToggleGroup<'a, T: PartialEq + Clone> {
    items: &'a [(T, ToggleGroupItem<'a>)],
    selected: &'a mut T,
    enabled: bool,
    size: Size,
}

impl<'a, T: PartialEq + Clone> ToggleGroup<'a, T> {
    pub fn new(items: &'a [(T, ToggleGroupItem<'a>)], selected: &'a mut T) -> Self {
        Self { items, selected, enabled: true, size: Size::Default }
    }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }
    pub fn size(mut self, s: Size) -> Self { self.size = s; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());
        let h_pad = self.size.h_pad();
        let height = self.size.height();
        let fs = self.size.font_size();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for (i, (value, item)) in self.items.iter().enumerate() {
                let is_selected = *self.selected == *value;
                let is_first = i == 0;
                let is_last = i == self.items.len() - 1;

                let text_g = ui.painter().layout_no_wrap(
                    item.label.to_owned(),
                    egui::FontId::new(fs, egui::FontFamily::Proportional),
                    Color32::PLACEHOLDER,
                );
                let icon_g = item.icon.map(|ic| {
                    ui.painter().layout_no_wrap(ic.to_owned(), crate::icon_font_id(fs + 2.0), Color32::PLACEHOLDER)
                });

                let icon_w = icon_g.as_ref().map(|g| g.size().x + 6.0).unwrap_or(0.0);
                let total_content_w = text_g.size().x + icon_w;
                let w = total_content_w + h_pad * 2.0;
                let size = Vec2::new(w.max(height), height);

                let (rect, resp) = ui.allocate_exact_size(
                    size,
                    if self.enabled { Sense::click() } else { Sense::hover() },
                );

                if resp.clicked() && self.enabled {
                    *self.selected = value.clone();
                }

                if ui.is_rect_visible(rect) {
                    let r = theme.radius as u8;
                    let cr = CornerRadius {
                        nw: if is_first { r } else { 0 },
                        ne: if is_last  { r } else { 0 },
                        sw: if is_first { r } else { 0 },
                        se: if is_last  { r } else { 0 },
                    };
                    let (bg, fg) = if !self.enabled {
                        let base_bg = if is_selected { theme.primary } else { theme.secondary };
                        let base_fg = if is_selected { theme.primary_foreground } else { theme.secondary_foreground };
                        (ShadcnTheme::with_alpha(base_bg, 100), ShadcnTheme::with_alpha(base_fg, 128))
                    } else if is_selected {
                        (theme.primary, theme.primary_foreground)
                    } else if resp.hovered() {
                        (theme.muted, theme.muted_foreground)
                    } else {
                        (theme.secondary, theme.secondary_foreground)
                    };
                    ui.painter().rect_filled(rect, cr, bg);
                    ui.painter().rect_stroke(rect, cr, Stroke::new(1.0, theme.border), egui::StrokeKind::Inside);

                    let start_x = rect.center().x - total_content_w / 2.0;
                    let mut cx = start_x;
                    if let Some(g) = icon_g {
                        let pos = egui::Pos2::new(cx, rect.center().y - g.size().y / 2.0);
                        ui.painter().galley(pos, g, fg);
                        cx += icon_w;
                    }
                    let ty = rect.center().y - text_g.size().y / 2.0;
                    ui.painter().galley(egui::Pos2::new(cx, ty), text_g, fg);

                    if resp.hovered() && self.enabled {
                        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                    }
                }
            }
        });
    }
}
