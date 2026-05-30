use crate::i18n;
use crate::{CalDate, ShadcnTheme, ICON_CALENDAR_TODAY};
use egui::{CornerRadius, Frame, Margin, Sense, Stroke, Ui, Vec2};
use ::i18n::t;

pub struct DatePicker<'a> {
    id: &'a str,
    value: &'a mut Option<CalDate>,
    placeholder: Option<&'a str>,
    width: f32,
}

impl<'a> DatePicker<'a> {
    pub fn new(id: &'a str, value: &'a mut Option<CalDate>) -> Self {
        Self {
            id,
            value,
            placeholder: None,
            width: 240.0,
        }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = Some(p);
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = ShadcnTheme::get(ui.ctx());
        let popup_id = egui::Id::new("shadcn_date_picker").with(self.id);
        let height = 36.0;
        let width = self.width;

        let (trigger_rect, trigger_resp) =
            ui.allocate_exact_size(Vec2::new(width, height), Sense::click());

        let is_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(popup_id).unwrap_or(false));

        let border_color = if trigger_resp.hovered() || is_open {
            theme.ring
        } else {
            theme.border
        };

        let cr = CornerRadius::same(theme.radius as u8);
        ui.painter().rect_filled(trigger_rect, cr, theme.background);
        ui.painter().rect_stroke(
            trigger_rect,
            cr,
            Stroke::new(1.0, border_color),
            egui::StrokeKind::Inside,
        );

        ui.painter().text(
            egui::Pos2::new(trigger_rect.left() + 12.0, trigger_rect.center().y),
            egui::Align2::LEFT_CENTER,
            ICON_CALENDAR_TODAY,
            crate::icon_font_id(16.0),
            theme.muted_foreground,
        );

        let display = match self.value {
            Some(d) => {
                let month_name = i18n::month_short(d.month);
                format!("{} {:02}, {}", month_name, d.day, d.year)
            }
            None => match self.placeholder {
                Some(p) => p.to_string(),
                None => t!(i18n::DatePicker::Placeholder).into_owned(),
            },
        };

        let text_color = if self.value.is_some() {
            theme.foreground
        } else {
            theme.muted_foreground
        };

        ui.painter().text(
            egui::Pos2::new(trigger_rect.left() + 34.0, trigger_rect.center().y),
            egui::Align2::LEFT_CENTER,
            display,
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            text_color,
        );

        if trigger_resp.clicked() {
            ui.ctx().data_mut(|d| d.insert_temp(popup_id, !is_open));
        }

        if trigger_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        let mut changed = false;

        if is_open {
            let popup_pos = trigger_rect.left_bottom() + egui::Vec2::new(0.0, 4.0);

            let prev_value = *self.value;

            let area_resp = egui::Area::new(popup_id.with("area"))
                .fixed_pos(popup_pos)
                .order(egui::Order::Foreground)
                .show(ui.ctx(), |ui| {
                    Frame::new()
                        .fill(theme.card)
                        .corner_radius(CornerRadius::same(theme.radius as u8))
                        .stroke(Stroke::new(1.0, theme.border))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            crate::Calendar::single(
                                egui::Id::new("shadcn_date_picker_cal").with(self.id),
                                self.value,
                            )
                            .show(ui);
                        });
                });

            if *self.value != prev_value {
                changed = true;
                ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
            }

            let popup_rect = area_resp.response.rect;

            if ui.ctx().input(|i| i.pointer.any_click()) {
                let hover_pos = ui
                    .ctx()
                    .input(|i| i.pointer.hover_pos().unwrap_or_default());
                let in_trigger = trigger_rect.contains(hover_pos);
                let in_popup = popup_rect.contains(hover_pos);
                if !in_trigger && !in_popup {
                    ui.ctx().data_mut(|d| d.insert_temp(popup_id, false));
                }
            }
        }

        changed
    }
}
