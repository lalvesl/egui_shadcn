use egui::{CornerRadius, Direction, Layout, Sense, Stroke, Ui, Vec2};
use crate::ShadcnTheme;

// ── CalDate ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CalDate {
    pub year:  i32,
    pub month: u8,
    pub day:   u8,
}

impl CalDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn today() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let secs = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i32;
        Self::from_epoch_days(secs / 86400)
    }

    pub fn next_month(self) -> Self {
        if self.month == 12 {
            Self { year: self.year + 1, month: 1, day: 1 }
        } else {
            Self { year: self.year, month: self.month + 1, day: 1 }
        }
    }

    pub fn prev_month(self) -> Self {
        if self.month == 1 {
            Self { year: self.year - 1, month: 12, day: 1 }
        } else {
            Self { year: self.year, month: self.month - 1, day: 1 }
        }
    }

    fn from_epoch_days(z: i32) -> Self {
        // http://howardhinnant.github.io/date_algorithms.html
        let z = z + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y   = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp  = (5 * doy + 2) / 153;
        let d   = doy - (153 * mp + 2) / 5 + 1;
        let m   = if mp < 10 { mp + 3 } else { mp - 9 };
        let y   = if m <= 2 { y + 1 } else { y };
        Self { year: y, month: m as u8, day: d as u8 }
    }
}

impl PartialOrd for CalDate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for CalDate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        (self.year, self.month, self.day).cmp(&(other.year, other.month, other.day))
    }
}

// ── date helpers ──────────────────────────────────────────────────────────────

fn days_in_month(year: i32, month: u8) -> u8 {
    match month {
        1 | 3 | 5 | 7 | 8 | 10 | 12 => 31,
        4 | 6 | 9 | 11              => 30,
        2 => if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 { 29 } else { 28 },
        _ => 30,
    }
}

fn weekday_of(year: i32, month: u8, day: u8) -> usize {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 { y -= 1; }
    ((y + y / 4 - y / 100 + y / 400 + T[month as usize - 1] + day as i32).rem_euclid(7)) as usize
}

const MONTHS: [&str; 12] = [
    "January","February","March","April","May","June",
    "July","August","September","October","November","December",
];
const DAYS: [&str; 7] = ["Su","Mo","Tu","We","Th","Fr","Sa"];

// ── internal egui state ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct CalState {
    view: CalDate,
}

// ── Calendar ──────────────────────────────────────────────────────────────────

/// Single or range date-picker.
///
/// ```
/// // Single date
/// Calendar::single("my_cal", &mut selected_date).show(ui);
///
/// // Range
/// Calendar::range("rng", &mut start, &mut end).show(ui);
///
/// // With custom cell content
/// Calendar::single("cal", &mut date)
///     .cell_height(52.0)
///     .cell_content(|ui, d| { if has_event(d) { Badge::new("event").show(ui); } })
///     .show(ui);
/// ```
pub struct Calendar<'a> {
    id:       egui::Id,
    selected: &'a mut Option<CalDate>,
    end:      Option<&'a mut Option<CalDate>>,
    cell_fn:  Option<Box<dyn Fn(&mut Ui, CalDate) + 'a>>,
    cell_h:   f32,
}

impl<'a> Calendar<'a> {
    pub fn single(id: impl std::hash::Hash, selected: &'a mut Option<CalDate>) -> Self {
        Self { id: egui::Id::new(id), selected, end: None, cell_fn: None, cell_h: 32.0 }
    }

    pub fn range(
        id:    impl std::hash::Hash,
        start: &'a mut Option<CalDate>,
        end:   &'a mut Option<CalDate>,
    ) -> Self {
        Self { id: egui::Id::new(id), selected: start, end: Some(end), cell_fn: None, cell_h: 32.0 }
    }

    /// Render custom content inside each date cell (below the day number).
    pub fn cell_content(mut self, f: impl Fn(&mut Ui, CalDate) + 'a) -> Self {
        self.cell_fn = Some(Box::new(f));
        self
    }

    /// Height of each date cell. Default 32. Increase when using `cell_content`.
    pub fn cell_height(mut self, h: f32) -> Self { self.cell_h = h; self }

    pub fn show(self, ui: &mut Ui) {
        let id    = self.id;
        let today = CalDate::today();

        let mut state = ui.ctx().data_mut(|d| {
            d.get_temp::<CalState>(id).unwrap_or_else(|| CalState {
                view: CalDate::new(today.year, today.month, 1),
            })
        });

        let theme   = ShadcnTheme::get(ui.ctx());
        let cell_h  = self.cell_h;
        // Borrow cell_fn as a trait-object reference with the same lifetime 'a,
        // then immediately coerce to a shorter lifetime via subtyping.
        let cell_fn: Option<&(dyn Fn(&mut Ui, CalDate) + '_)> =
            self.cell_fn.as_deref();

        if let Some(end_ref) = self.end {
            let right_view  = state.view.next_month();
            let mut s_start: Option<CalDate> = *self.selected;
            let mut s_end:   Option<CalDate> = *end_ref;

            let (nav, c0, c1) = ui.horizontal(|ui| {
                let (n, c0) = draw_month(
                    ui, &theme, state.view,
                    &s_start, &s_end,
                    true, false, today, cell_h, cell_fn,
                );
                ui.add_space(24.0);
                let (_, c1) = draw_month(
                    ui, &theme, right_view,
                    &s_start, &s_end,
                    false, true, today, cell_h, cell_fn,
                );
                (n, c0, c1)
            }).inner;

            match nav { -1 => state.view = state.view.prev_month(),
                         1 => state.view = state.view.next_month(), _ => {} }

            for d in c0.into_iter().chain(c1) {
                range_click(d, &mut s_start, &mut s_end);
            }
            *self.selected = s_start;
            *end_ref       = s_end;
        } else {
            let mut sel: Option<CalDate> = *self.selected;

            let (nav, clicked) = draw_month(
                ui, &theme, state.view,
                &sel, &None,
                true, true, today, cell_h, cell_fn,
            );

            match nav { -1 => state.view = state.view.prev_month(),
                         1 => state.view = state.view.next_month(), _ => {} }

            if let Some(d) = clicked {
                sel = if sel == Some(d) { None } else { Some(d) };
            }
            *self.selected = sel;
        }

        ui.ctx().data_mut(|d| d.insert_temp(id, state));
    }
}

fn range_click(d: CalDate, start: &mut Option<CalDate>, end: &mut Option<CalDate>) {
    match (*start, *end) {
        (None, _) => { *start = Some(d); *end = None; }
        (Some(s), None) => {
            if d == s { *start = None; }
            else if d < s { *end = Some(s); *start = Some(d); }
            else          { *end = Some(d); }
        }
        (Some(_), Some(_)) => { *start = Some(d); *end = None; }
    }
}

// ── month grid ────────────────────────────────────────────────────────────────

fn draw_month<'f>(
    ui:        &mut Ui,
    theme:     &ShadcnTheme,
    view:      CalDate,
    sel_start: &Option<CalDate>,
    sel_end:   &Option<CalDate>,
    show_prev: bool,
    show_next: bool,
    today:     CalDate,
    cell_h:    f32,
    cell_fn:   Option<&'f (dyn Fn(&mut Ui, CalDate) + 'f)>,
) -> (i8, Option<CalDate>) {
    let cell_w: f32 = 36.0;
    let mut nav: i8 = 0;
    let mut clicked: Option<CalDate> = None;

    ui.vertical(|ui| {
        ui.set_width(cell_w * 7.0);

        // header
        ui.horizontal(|ui| {
            let sz = Vec2::new(24.0, 24.0);
            if show_prev {
                if ui.add_sized(sz, egui::Button::new("‹").frame(false)).clicked() { nav = -1; }
            } else {
                ui.add_space(sz.x);
            }
            ui.with_layout(Layout::centered_and_justified(Direction::LeftToRight), |ui| {
                ui.label(
                    egui::RichText::new(format!(
                        "{} {}", MONTHS[(view.month - 1) as usize], view.year
                    ))
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground)
                    .strong(),
                );
            });
            if show_next {
                if ui.add_sized(sz, egui::Button::new("›").frame(false)).clicked() { nav = 1; }
            } else {
                ui.add_space(sz.x);
            }
        });

        ui.add_space(6.0);

        // weekday labels
        ui.horizontal(|ui| {
            for &d in &DAYS {
                let (rect, _) = ui.allocate_exact_size(Vec2::new(cell_w, 20.0), Sense::hover());
                ui.painter().text(
                    rect.center(), egui::Align2::CENTER_CENTER, d,
                    egui::FontId::new(11.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
            }
        });

        ui.add_space(2.0);

        // day cells
        let first_col = weekday_of(view.year, view.month, 1);
        let n_days    = days_in_month(view.year, view.month) as usize;
        let n_rows    = (first_col + n_days + 6) / 7;

        for row in 0..n_rows {
            ui.horizontal(|ui| {
                for col in 0..7usize {
                    let day = (row * 7 + col) as i32 - first_col as i32 + 1;
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(cell_w, cell_h), Sense::click());

                    if day < 1 || day > n_days as i32 { continue; }

                    let date     = CalDate::new(view.year, view.month, day as u8);
                    let is_start = *sel_start == Some(date);
                    let is_end   = *sel_end   == Some(date);
                    let in_range = matches!((sel_start, sel_end), (Some(s), Some(e)) if date > *s && date < *e);
                    let is_today = date == today;
                    let hovered  = resp.hovered();
                    let r        = (cell_w.min(cell_h) / 2.0 - 2.0).max(1.0);

                    if is_start || is_end {
                        ui.painter().circle_filled(rect.center(), r, theme.primary);
                    } else if in_range {
                        ui.painter().rect_filled(rect, CornerRadius::ZERO, theme.accent);
                    } else if hovered {
                        ui.painter().circle_filled(rect.center(), r, theme.accent);
                    }

                    if is_today && !is_start && !is_end {
                        ui.painter().circle_stroke(
                            rect.center(), r,
                            Stroke::new(1.5, theme.primary),
                        );
                    }

                    let num_color = if is_start || is_end { theme.primary_foreground }
                                    else if is_today      { theme.primary }
                                    else                  { theme.foreground };

                    let num_y = if cell_fn.is_some() { rect.top() + 10.0 } else { rect.center().y };

                    ui.painter().text(
                        egui::Pos2::new(rect.center().x, num_y),
                        egui::Align2::CENTER_CENTER,
                        format!("{day}"),
                        egui::FontId::new(13.0, egui::FontFamily::Proportional),
                        num_color,
                    );

                    if let Some(f) = cell_fn {
                        let inner = egui::Rect::from_min_max(
                            egui::Pos2::new(rect.left() + 1.0, num_y + 12.0),
                            rect.max - egui::Vec2::splat(1.0),
                        );
                        if inner.height() > 4.0 {
                            let mut child = ui.new_child(egui::UiBuilder::new().max_rect(inner));
                            f(&mut child, date);
                        }
                    }

                    if resp.clicked() { clicked = Some(date); }
                    if hovered { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
                }
            });
        }
    });

    (nav, clicked)
}
