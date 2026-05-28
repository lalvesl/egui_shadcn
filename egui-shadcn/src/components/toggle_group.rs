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
}

impl<'a, T: PartialEq + Clone> ToggleGroup<'a, T> {
    pub fn new(items: &'a [(T, ToggleGroupItem<'a>)], selected: &'a mut T) -> Self {
        Self { items, selected, enabled: true }
    }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }

    pub fn show(self, ui: &mut Ui) {
        let theme = ShadcnTheme::get(ui.ctx());
        let h_pad = 12.0;
        let height = 36.0;
        let fs = 13.0;
        let gap = 1.0;

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = gap;
            for (i, (value, item)) in self.items.iter().enumerate() {
                let is_selected = *self.selected == *value;
                let is_first = i == 0;
                let is_last = i == self.items.len() - 1;

                let text_g = ui.painter().layout_no_wrap(
                    item.label.to_owned(),
                    egui::FontId::new(fs, egui::FontFamily::Proportional),
                    Color32::TRANSPARENT,
                );
                let icon_g = item.icon.map(|ic| {
                    ui.painter().layout_no_wrap(
                        ic.to_owned(),
                        egui::FontId::new(fs + 2.0, egui::FontFamily::Name("MaterialIcons".into())),
                        Color32::TRANSPARENT,
                    )
                });
                let icon_w = icon_g.as_ref().map(|g| g.size().x + 6.0).unwrap_or(0.0);
                let w = text_g.size().x + icon_w + h_pad * 2.0;
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
                        ne: if is_last { r } else { 0 },
                        sw: if is_first { r } else { 0 },
                        se: if is_last { r } else { 0 },
                    };
                    let (bg, fg) = if is_selected {
                        (theme.accent, theme.accent_foreground)
                    } else if resp.hovered() && self.enabled {
                        (ShadcnTheme::with_alpha(theme.accent, 80), theme.foreground)
                    } else {
                        (Color32::TRANSPARENT, theme.muted_foreground)
                    };
                    ui.painter().rect_filled(rect, cr, bg);
                    ui.painter().rect_stroke(rect, cr, Stroke::new(1.0, theme.border), egui::StrokeKind::Inside);

                    let mut cx = rect.left() + h_pad;
                    if let Some(g) = icon_g {
                        let pos = egui::Pos2::new(cx, rect.center().y - g.size().y / 2.0);
                        ui.painter().galley(pos, g, fg);
                        cx += icon_w;
                    }
                    let ty = rect.center().y - text_g.size().y / 2.0;
                    ui.painter().galley(egui::Pos2::new(cx, ty), text_g, fg);
                }
            }
        });
    }
}
