/// Shadcn theme color picker — select from built-in presets.
use egui::{Color32, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

#[derive(Clone, Debug, PartialEq)]
pub struct ThemePreset {
    pub name:        &'static str,
    pub primary:     Color32,
    pub secondary:   Color32,
    pub accent:      Color32,
    pub destructive: Color32,
    pub radius:      u8,
}

impl ThemePreset {
    pub fn all() -> Vec<ThemePreset> {
        vec![
            ThemePreset { name: "Zinc",    primary: Color32::from_rgb(9,9,11),       secondary: Color32::from_rgb(244,244,245), accent: Color32::from_rgb(244,244,245), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Slate",   primary: Color32::from_rgb(15,23,42),      secondary: Color32::from_rgb(241,245,249), accent: Color32::from_rgb(241,245,249), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Stone",   primary: Color32::from_rgb(28,25,23),      secondary: Color32::from_rgb(245,245,244), accent: Color32::from_rgb(245,245,244), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Gray",    primary: Color32::from_rgb(17,24,39),      secondary: Color32::from_rgb(243,244,246), accent: Color32::from_rgb(243,244,246), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Neutral", primary: Color32::from_rgb(10,10,10),      secondary: Color32::from_rgb(245,245,245), accent: Color32::from_rgb(245,245,245), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Red",     primary: Color32::from_rgb(239,68,68),     secondary: Color32::from_rgb(254,226,226), accent: Color32::from_rgb(254,226,226), destructive: Color32::from_rgb(185,28,28), radius: 6 },
            ThemePreset { name: "Rose",    primary: Color32::from_rgb(244,63,94),     secondary: Color32::from_rgb(255,228,230), accent: Color32::from_rgb(255,228,230), destructive: Color32::from_rgb(190,18,60), radius: 6 },
            ThemePreset { name: "Orange",  primary: Color32::from_rgb(249,115,22),    secondary: Color32::from_rgb(255,237,213), accent: Color32::from_rgb(255,237,213), destructive: Color32::from_rgb(194,65,12), radius: 6 },
            ThemePreset { name: "Green",   primary: Color32::from_rgb(34,197,94),     secondary: Color32::from_rgb(220,252,231), accent: Color32::from_rgb(220,252,231), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Blue",    primary: Color32::from_rgb(59,130,246),    secondary: Color32::from_rgb(219,234,254), accent: Color32::from_rgb(219,234,254), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Violet",  primary: Color32::from_rgb(139,92,246),    secondary: Color32::from_rgb(237,233,254), accent: Color32::from_rgb(237,233,254), destructive: Color32::from_rgb(239,68,68), radius: 6 },
            ThemePreset { name: "Yellow",  primary: Color32::from_rgb(234,179,8),     secondary: Color32::from_rgb(254,249,195), accent: Color32::from_rgb(254,249,195), destructive: Color32::from_rgb(239,68,68), radius: 6 },
        ]
    }

    pub fn apply_to(&self, theme: &mut ShadcnTheme) {
        theme.primary   = self.primary;
        theme.secondary = self.secondary;
        theme.accent    = self.accent;
        theme.primary_foreground = if is_dark(self.primary) {
            Color32::from_rgb(250, 250, 250)
        } else {
            Color32::from_rgb(9, 9, 11)
        };
        theme.secondary_foreground = if is_dark(self.secondary) {
            Color32::from_rgb(250, 250, 250)
        } else {
            Color32::from_rgb(9, 9, 11)
        };
        theme.accent_foreground = theme.secondary_foreground;
        theme.destructive       = self.destructive;
        theme.radius            = self.radius;
    }
}

fn is_dark(c: Color32) -> bool {
    (c.r() as u32 * 299 + c.g() as u32 * 587 + c.b() as u32 * 114) / 1000 < 128
}

pub struct ThemePicker<'a> {
    current: &'a mut usize,
}

impl<'a> ThemePicker<'a> {
    pub fn new(current: &'a mut usize) -> Self {
        Self { current }
    }

    pub fn show(self, ui: &mut Ui) {
        let theme   = ShadcnTheme::get(ui.ctx());
        let presets = ThemePreset::all();

        // Color swatches grid
        ui.horizontal_wrapped(|ui| {
            ui.spacing_mut().item_spacing = Vec2::splat(8.0);

            for (i, preset) in presets.iter().enumerate() {
                let selected = *self.current == i;
                let swatch_size = 32.0;

                let (rect, response) = ui.allocate_exact_size(
                    Vec2::new(swatch_size + 8.0, swatch_size + 20.0),
                    Sense::click(),
                );

                if response.clicked() {
                    *self.current = i;
                }

                if ui.is_rect_visible(rect) {
                    let swatch = egui::Rect::from_min_size(
                        egui::pos2(rect.left(), rect.top()),
                        Vec2::splat(swatch_size),
                    );

                    if selected {
                        ui.painter().rect_stroke(
                            swatch.expand(2.0),
                            theme.radius,
                            Stroke::new(2.0, theme.ring),
                            StrokeKind::Outside,
                        );
                    } else if response.hovered() {
                        ui.painter().rect_stroke(
                            swatch.expand(1.0),
                            theme.radius,
                            Stroke::new(1.0, theme.border),
                            StrokeKind::Outside,
                        );
                    }

                    let top = egui::Rect::from_min_max(swatch.left_top(), egui::pos2(swatch.right(), swatch.center().y));
                    let bot = egui::Rect::from_min_max(egui::pos2(swatch.left(), swatch.center().y), swatch.right_bottom());

                    ui.painter().rect_filled(top, egui::epaint::CornerRadius { nw: theme.radius, ne: theme.radius, sw: 0, se: 0 }, preset.primary);
                    ui.painter().rect_filled(bot, egui::epaint::CornerRadius { nw: 0, ne: 0, sw: theme.radius, se: theme.radius }, preset.secondary);

                    let name_pos = egui::pos2(rect.center().x, swatch.bottom() + 4.0);
                    ui.painter().text(
                        name_pos,
                        egui::Align2::CENTER_TOP,
                        preset.name,
                        egui::FontId::proportional(10.0),
                        if selected { theme.foreground } else { theme.muted_foreground },
                    );
                }
            }
        });

        ui.add_space(16.0);

        // Radius selector
        ui.horizontal(|ui| {
            ui.label(egui::RichText::new("Radius").size(13.0).color(theme.muted_foreground));
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                for (label, r) in [("None", 0u8), ("sm", 4), ("md", 6), ("lg", 8), ("full", 255)] {
                    let active = presets[*self.current].radius == r;
                    let btn = ui.add(
                        egui::Button::new(egui::RichText::new(label).size(12.0).color(
                            if active { theme.primary_foreground } else { theme.foreground }
                        ))
                        .fill(if active { theme.primary } else { Color32::TRANSPARENT })
                        .stroke(Stroke::new(1.0, if active { theme.primary } else { theme.border }))
                        .min_size(Vec2::new(42.0, 26.0)),
                    );
                    if btn.clicked() {
                        ui.ctx().data_mut(|d| d.insert_temp::<u8>(egui::Id::new("picker_radius"), r));
                    }
                }
            });
        });
    }
}
