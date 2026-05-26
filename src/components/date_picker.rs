/// Date picker and date range picker components.
use egui::{Color32, Margin, Response, Sense, Stroke, StrokeKind, Ui, Vec2};
use crate::theme::ShadcnTheme;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Date {
    pub year:  i32,
    pub month: u8,  // 1-12
    pub day:   u8,  // 1-31
}

impl Date {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn today() -> Self {
        // Fallback — egui has no system time access; caller should pass today's date.
        Self { year: 2026, month: 5, day: 23 }
    }

    fn days_in_month(year: i32, month: u8) -> u8 {
        match month {
            1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
            4 | 6 | 9 | 11              => 30,
            2 => if is_leap(year) { 29 } else { 28 },
            _ => 30,
        }
    }

    fn first_weekday(year: i32, month: u8) -> u8 {
        // Tomohiko Sakamoto's algorithm → 0=Sun … 6=Sat
        let d = 1u32;
        let m = month as u32;
        let mut y = year;
        let t = [0i32, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
        if m < 3 { y -= 1; }
        let yw = y as u32;
        ((yw + yw / 4 - yw / 100 + yw / 400 + t[(m - 1) as usize] as u32 + d) % 7) as u8
    }

    fn prev_month(&self) -> Self {
        if self.month == 1 {
            Self::new(self.year - 1, 12, 1)
        } else {
            Self::new(self.year, self.month - 1, 1)
        }
    }

    fn next_month(&self) -> Self {
        if self.month == 12 {
            Self::new(self.year + 1, 1, 1)
        } else {
            Self::new(self.year, self.month + 1, 1)
        }
    }
}

fn is_leap(y: i32) -> bool { y % 4 == 0 && (y % 100 != 0 || y % 400 == 0) }

const MONTHS: [&str; 12] = [
    "January","February","March","April","May","June",
    "July","August","September","October","November","December",
];
const SHORT_DAYS: [&str; 7] = ["Su","Mo","Tu","We","Th","Fr","Sa"];

// ─── Single date picker ────────────────────────────────────────────────────

pub struct DatePicker<'a> {
    selected:   &'a mut Option<Date>,
    open:       &'a mut bool,
    view_month: &'a mut Date,
    today:      Date,
}

impl<'a> DatePicker<'a> {
    pub fn new(
        selected:   &'a mut Option<Date>,
        open:       &'a mut bool,
        view_month: &'a mut Date,
    ) -> Self {
        Self { selected, open, view_month, today: Date::today() }
    }

    pub fn today(mut self, d: Date) -> Self { self.today = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());

        // Trigger button
        let label = self.selected
            .map(|d| format!("{} {:02}, {}", MONTHS[(d.month - 1) as usize], d.day, d.year))
            .unwrap_or_else(|| "Pick a date".to_string());

        let btn = trigger_button(ui, &label, &theme);
        if btn.clicked() {
            *self.open = !*self.open;
        }

        if *self.open {
            let current_selected = *self.selected;
            calendar_body(
                ui,
                self.view_month,
                &theme,
                move |date| current_selected == Some(date),
                |_| false,
                |date| {
                    *self.selected = Some(date);
                    *self.open = false;
                },
            );
        }

        btn
    }
}

// ─── Date range picker ─────────────────────────────────────────────────────

pub struct DateRangePicker<'a> {
    start:      &'a mut Option<Date>,
    end:        &'a mut Option<Date>,
    open:       &'a mut bool,
    view_month: &'a mut Date,
    today:      Date,
}

impl<'a> DateRangePicker<'a> {
    pub fn new(
        start:      &'a mut Option<Date>,
        end:        &'a mut Option<Date>,
        open:       &'a mut bool,
        view_month: &'a mut Date,
    ) -> Self {
        Self { start, end, open, view_month, today: Date::today() }
    }

    pub fn today(mut self, d: Date) -> Self { self.today = d; self }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());

        let label = match (*self.start, *self.end) {
            (Some(s), Some(e)) => format!(
                "{}/{:02}/{} – {}/{:02}/{}",
                s.month, s.day, s.year, e.month, e.day, e.year
            ),
            (Some(s), None) => format!("{}/{:02}/{} – …", s.month, s.day, s.year),
            _ => "Pick a date range".to_string(),
        };

        let btn = trigger_button(ui, &label, &theme);
        if btn.clicked() {
            *self.open = !*self.open;
        }

        if *self.open {
            let start = *self.start;
            let end   = *self.end;

            calendar_body(
                ui,
                self.view_month,
                &theme,
                move |date| start == Some(date) || end == Some(date),
                move |date| match (start, end) {
                    (Some(s), Some(e)) => date > s && date < e,
                    _ => false,
                },
                |date| {
                    match (*self.start, *self.end) {
                        (None, _) => { *self.start = Some(date); *self.end = None; }
                        (Some(s), None) => {
                            if date >= s {
                                *self.end = Some(date);
                                *self.open = false;
                            } else {
                                *self.start = Some(date);
                            }
                        }
                        _ => { *self.start = Some(date); *self.end = None; }
                    }
                },
            );
        }

        btn
    }
}

// ─── Shared calendar popup ─────────────────────────────────────────────────

fn trigger_button(ui: &mut Ui, label: &str, theme: &ShadcnTheme) -> Response {
    let width  = ui.available_width();
    let height = 36.0;

    let (rect, response) = ui.allocate_exact_size(Vec2::new(width, height), Sense::click());
    let hovered = response.hovered();

    if ui.is_rect_visible(rect) {
        let border = if hovered {
            Stroke::new(1.0, ShadcnTheme::with_alpha(theme.foreground, 180))
        } else {
            Stroke::new(1.0, theme.input)
        };
        ui.painter().rect_filled(rect, theme.radius, theme.background);
        ui.painter().rect_stroke(rect, theme.radius, border, StrokeKind::Inside);

        // Calendar icon
        let icon_x = rect.left() + 12.0;
        let icon_y = rect.center().y;
        draw_calendar_icon(ui, egui::pos2(icon_x + 6.0, icon_y), theme.muted_foreground);

        // Label text
        let galley = ui.painter().layout_no_wrap(
            label.to_owned(),
            egui::FontId::proportional(14.0),
            theme.muted_foreground,
        );
        let text_y = rect.center().y - galley.size().y * 0.5;
        ui.painter().galley(egui::pos2(rect.left() + 32.0, text_y), galley, theme.muted_foreground);
    }

    response
}

fn draw_calendar_icon(ui: &mut Ui, center: egui::Pos2, color: Color32) {
    let w = 12.0;
    let h = 11.0;
    let l = center.x - w * 0.5;
    let t = center.y - h * 0.5;
    let stroke = Stroke::new(1.2, color);

    // Outer rect
    let r = egui::Rect::from_min_size(egui::pos2(l, t + 1.5), Vec2::new(w, h));
    ui.painter().rect_stroke(r, 1u8, stroke, StrokeKind::Middle);

    // Header line
    ui.painter().hline(l..=(l + w), t + 4.5, stroke);

    // Tick marks (top binding posts)
    for dx in [w * 0.25, w * 0.75] {
        ui.painter().vline(l + dx, (t - 1.0)..=t + 2.0, stroke);
    }
}

fn calendar_body(
    ui:          &mut Ui,
    view:        &mut Date,
    theme:       &ShadcnTheme,
    is_selected: impl Fn(Date) -> bool,
    is_in_range: impl Fn(Date) -> bool,
    mut on_select: impl FnMut(Date),
) {
    egui::Frame::new()
        .fill(theme.card)
        .stroke(Stroke::new(1.0, theme.border))
        .corner_radius(theme.radius)
        .inner_margin(Margin::same(16))
        .show(ui, |ui| {
            ui.set_width(256.0);

            // Month navigation header
            ui.horizontal(|ui| {
                let prev_btn = ui.add(
                    egui::Button::new(egui::RichText::new("‹").size(18.0).color(theme.foreground))
                        .frame(false)
                        .min_size(Vec2::splat(28.0)),
                );
                if prev_btn.clicked() { *view = view.prev_month(); }

                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let next_btn = ui.add(
                        egui::Button::new(egui::RichText::new("›").size(18.0).color(theme.foreground))
                            .frame(false)
                            .min_size(Vec2::splat(28.0)),
                    );
                    if next_btn.clicked() { *view = view.next_month(); }

                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label(egui::RichText::new(
                            format!("{} {}", MONTHS[(view.month - 1) as usize], view.year)
                        ).size(14.0).strong().color(theme.foreground));
                    });
                });
            });

            ui.add_space(8.0);

            // Day-of-week header
            egui::Grid::new("cal_header")
                .num_columns(7)
                .spacing([0.0, 0.0])
                .min_col_width(32.0)
                .max_col_width(32.0)
                .show(ui, |ui| {
                    for &d in &SHORT_DAYS {
                        ui.centered_and_justified(|ui| {
                            ui.label(egui::RichText::new(d).size(12.0).color(theme.muted_foreground));
                        });
                    }
                    ui.end_row();
                });

            ui.add_space(4.0);

            // Day cells
            let days_in_month = Date::days_in_month(view.year, view.month);
            let first_weekday = Date::first_weekday(view.year, view.month);

            egui::Grid::new("cal_days")
                .num_columns(7)
                .spacing([2.0, 2.0])
                .min_col_width(32.0)
                .max_col_width(32.0)
                .show(ui, |ui| {
                    let mut col = 0u8;

                    // Empty cells before first day
                    for _ in 0..first_weekday {
                        ui.add_space(32.0);
                        col += 1;
                    }

                    for day in 1..=days_in_month {
                        let date = Date::new(view.year, view.month, day);
                        let selected = is_selected(date);
                        let in_range = is_in_range(date);

                        let (rect, response) = ui.allocate_exact_size(Vec2::splat(32.0), Sense::click());
                        if response.clicked() {
                            on_select(date);
                        }

                        if ui.is_rect_visible(rect) {
                            let hovered = response.hovered();

                            let bg = if selected {
                                theme.primary
                            } else if in_range {
                                ShadcnTheme::with_alpha(theme.primary, 30)
                            } else if hovered {
                                theme.accent
                            } else {
                                Color32::TRANSPARENT
                            };

                            ui.painter().rect_filled(rect, theme.radius, bg);

                            let text_color = if selected {
                                theme.primary_foreground
                            } else {
                                theme.foreground
                            };

                            ui.painter().text(
                                rect.center(),
                                egui::Align2::CENTER_CENTER,
                                day.to_string(),
                                egui::FontId::proportional(13.0),
                                text_color,
                            );
                        }

                        col += 1;
                        if col == 7 {
                            ui.end_row();
                            col = 0;
                        }
                    }
                });
        });
}
