use super::spacing::Spacing;
use crate::ShadcnTheme;
use egui::{Color32, CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Tabs<'t> {
    id: egui::Id,
    labels: &'t [&'t str],
    current: &'t mut usize,
}

impl<'t> Tabs<'t> {
    pub fn new(id: impl std::hash::Hash, labels: &'t [&'t str], current: &'t mut usize) -> Self {
        Self {
            id: egui::Id::new(id),
            labels,
            current,
        }
    }

    pub fn show(self, ui: &mut Ui, mut content: impl FnMut(&mut Ui, usize)) {
        let theme = ShadcnTheme::get(ui.ctx());

        // Tab bar
        let bar_h = 36.0;
        let tab_pad_h = 16.0;
        let available_w = ui.available_width();

        let font = egui::FontId::new(14.0, egui::FontFamily::Proportional);

        // Compute each tab width
        let widths: Vec<f32> = self
            .labels
            .iter()
            .map(|lbl| {
                let g =
                    ui.painter()
                        .layout_no_wrap((*lbl).to_owned(), font.clone(), Color32::WHITE);
                g.size().x + tab_pad_h * 2.0
            })
            .collect();

        let bar_rect_alloc = ui
            .allocate_exact_size(Vec2::new(available_w, bar_h), Sense::hover())
            .0;

        let painter = ui.painter();

        // tab bar background
        painter.rect_filled(
            bar_rect_alloc,
            CornerRadius::same(theme.radius as u8),
            theme.secondary,
        );

        let mut x = bar_rect_alloc.left() + 4.0;
        let bar_top = bar_rect_alloc.top() + 4.0;

        for (i, (lbl, w)) in self.labels.iter().zip(widths.iter()).enumerate() {
            let tab_rect =
                egui::Rect::from_min_size(egui::Pos2::new(x, bar_top), Vec2::new(*w, bar_h - 8.0));

            let tab_resp = ui.interact(tab_rect, self.id.with(i), Sense::click());

            let is_active = *self.current == i;

            let (tab_bg, text_color) = if is_active {
                (theme.background, theme.foreground)
            } else if tab_resp.hovered() {
                (
                    ShadcnTheme::with_alpha(theme.background, 80),
                    theme.foreground,
                )
            } else {
                (Color32::TRANSPARENT, theme.muted_foreground)
            };

            if tab_resp.clicked() {
                *self.current = i;
            }

            painter.rect_filled(
                tab_rect,
                CornerRadius::same((theme.radius - 2.0) as u8),
                tab_bg,
            );

            if is_active {
                painter.rect_stroke(
                    tab_rect,
                    CornerRadius::same((theme.radius - 2.0) as u8),
                    Stroke::new(1.0, theme.border),
                    egui::StrokeKind::Inside,
                );
            }

            let galley = painter.layout_no_wrap((*lbl).to_owned(), font.clone(), text_color);
            painter.galley(
                egui::Pos2::new(
                    tab_rect.center().x - galley.size().x / 2.0,
                    tab_rect.center().y - galley.size().y / 2.0,
                ),
                galley,
                text_color,
            );

            x += *w + 2.0;
        }

        Spacing::Sm.show(ui);

        content(ui, *self.current);
    }
}
