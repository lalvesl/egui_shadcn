use super::spacing::Spacing;
use crate::i18n;
use crate::{ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ShadcnTheme};
use egui::{CornerRadius, Sense, Stroke, Ui, Vec2};

// ── CalDate ───────────────────────────────────────────────────────────────────

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct CalDate {
    pub year: i32,
    pub month: u8,
    pub day: u8,
}

impl CalDate {
    pub fn new(year: i32, month: u8, day: u8) -> Self {
        Self { year, month, day }
    }

    pub fn today() -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let secs: i64 = {
            use std::time::{SystemTime, UNIX_EPOCH};
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs() as i64
        };
        #[cfg(target_arch = "wasm32")]
        let secs: i64 = (js_sys::Date::now() / 1000.0) as i64;

        Self::from_epoch_days((secs / 86400) as i32)
    }

    pub fn next_month(self) -> Self {
        if self.month == 12 {
            Self {
                year: self.year + 1,
                month: 1,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month + 1,
                day: 1,
            }
        }
    }

    pub fn prev_month(self) -> Self {
        if self.month == 1 {
            Self {
                year: self.year - 1,
                month: 12,
                day: 1,
            }
        } else {
            Self {
                year: self.year,
                month: self.month - 1,
                day: 1,
            }
        }
    }

    fn from_epoch_days(z: i32) -> Self {
        // http://howardhinnant.github.io/date_algorithms.html
        let z = z + 719468;
        let era = (if z >= 0 { z } else { z - 146096 }) / 146097;
        let doe = z - era * 146097;
        let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146096) / 365;
        let y = yoe + era * 400;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        let y = if m <= 2 { y + 1 } else { y };
        Self {
            year: y,
            month: m as u8,
            day: d as u8,
        }
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
        4 | 6 | 9 | 11 => 30,
        2 => {
            if (year % 4 == 0 && year % 100 != 0) || year % 400 == 0 {
                29
            } else {
                28
            }
        }
        _ => 30,
    }
}

fn weekday_of(year: i32, month: u8, day: u8) -> usize {
    const T: [i32; 12] = [0, 3, 2, 5, 0, 3, 5, 1, 4, 6, 2, 4];
    let mut y = year;
    if month < 3 {
        y -= 1;
    }
    ((y + y / 4 - y / 100 + y / 400 + T[month as usize - 1] + day as i32).rem_euclid(7)) as usize
}

// ── internal egui state ───────────────────────────────────────────────────────

#[derive(Clone, Debug)]
struct CalState {
    view: CalDate,
    /// Day the pointer hovered last frame — drives the range hover preview.
    hover: Option<CalDate>,
}

// ── type alias ────────────────────────────────────────────────────────────────

type CellFn<'a> = dyn Fn(&mut Ui, CalDate) + 'a;

// ── Calendar ──────────────────────────────────────────────────────────────────

/// Single or range date-picker.
///
/// ```ignore
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
    id: egui::Id,
    selected: &'a mut Option<CalDate>,
    end: Option<&'a mut Option<CalDate>>,
    cell_fn: Option<Box<CellFn<'a>>>,
    cell_h: f32,
    compact: bool,
}

impl<'a> Calendar<'a> {
    pub fn single(id: impl std::hash::Hash, selected: &'a mut Option<CalDate>) -> Self {
        Self {
            id: egui::Id::new(id),
            selected,
            end: None,
            cell_fn: None,
            cell_h: 32.0,
            compact: false,
        }
    }

    pub fn range(
        id: impl std::hash::Hash,
        start: &'a mut Option<CalDate>,
        end: &'a mut Option<CalDate>,
    ) -> Self {
        Self {
            id: egui::Id::new(id),
            selected: start,
            end: Some(end),
            cell_fn: None,
            cell_h: 32.0,
            compact: false,
        }
    }

    /// Render custom content inside each date cell (below the day number).
    pub fn cell_content(mut self, f: impl Fn(&mut Ui, CalDate) + 'a) -> Self {
        self.cell_fn = Some(Box::new(f));
        self
    }

    /// Height of each date cell. Default 32. Increase when using `cell_content`.
    pub fn cell_height(mut self, h: f32) -> Self {
        self.cell_h = h;
        self
    }

    /// Render a range picker as a single month (with both nav arrows) instead of
    /// the default side-by-side two-month layout. No-op for `single`.
    pub fn compact(mut self) -> Self {
        self.compact = true;
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let id = self.id;
        let today = CalDate::today();

        let mut state = ui.ctx().data_mut(|d| {
            d.get_temp::<CalState>(id).unwrap_or_else(|| CalState {
                view: CalDate::new(today.year, today.month, 1),
                hover: None,
            })
        });

        let theme = ShadcnTheme::get(ui.ctx());
        let cell_h = self.cell_h;
        let cell_fn: Option<&CellFn<'_>> = self.cell_fn.as_deref();

        if let Some(end_ref) = self.end {
            let mut s_start: Option<CalDate> = *self.selected;
            let mut s_end: Option<CalDate> = *end_ref;
            // Only preview while picking the second endpoint (start set, end not).
            let preview = match (s_start, s_end) {
                (Some(_), None) => state.hover,
                _ => None,
            };

            if self.compact {
                let (nav, clicked, hov) = draw_month(
                    ui,
                    &theme,
                    DrawConfig {
                        id,
                        view: state.view,
                        sel_start: s_start,
                        sel_end: s_end,
                        preview,
                        show_prev: true,
                        show_next: true,
                        today,
                        cell_h,
                        cell_fn,
                    },
                );

                match nav {
                    -1 => state.view = state.view.prev_month(),
                    1 => state.view = state.view.next_month(),
                    _ => {}
                }

                state.hover = hov;
                if let Some(d) = clicked {
                    range_click(d, &mut s_start, &mut s_end);
                }
            } else {
                let right_view = state.view.next_month();
                let (nav, c0, c1, h0, h1) = ui
                    .horizontal(|ui| {
                        let (n, c0, h0) = draw_month(
                            ui,
                            &theme,
                            DrawConfig {
                                id,
                                view: state.view,
                                sel_start: s_start,
                                sel_end: s_end,
                                preview,
                                show_prev: true,
                                show_next: false,
                                today,
                                cell_h,
                                cell_fn,
                            },
                        );
                        Spacing::Xl.show(ui);
                        let (_, c1, h1) = draw_month(
                            ui,
                            &theme,
                            DrawConfig {
                                id: id.with("right"),
                                view: right_view,
                                sel_start: s_start,
                                sel_end: s_end,
                                preview,
                                show_prev: false,
                                show_next: true,
                                today,
                                cell_h,
                                cell_fn,
                            },
                        );
                        (n, c0, c1, h0, h1)
                    })
                    .inner;

                match nav {
                    -1 => state.view = state.view.prev_month(),
                    1 => state.view = state.view.next_month(),
                    _ => {}
                }

                state.hover = h0.or(h1);
                for d in c0.into_iter().chain(c1) {
                    range_click(d, &mut s_start, &mut s_end);
                }
            }
            *self.selected = s_start;
            *end_ref = s_end;
        } else {
            let mut sel: Option<CalDate> = *self.selected;

            let (nav, clicked, hov) = draw_month(
                ui,
                &theme,
                DrawConfig {
                    id,
                    view: state.view,
                    sel_start: sel,
                    sel_end: None,
                    preview: None,
                    show_prev: true,
                    show_next: true,
                    today,
                    cell_h,
                    cell_fn,
                },
            );

            match nav {
                -1 => state.view = state.view.prev_month(),
                1 => state.view = state.view.next_month(),
                _ => {}
            }

            state.hover = hov;
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
        (None, _) => {
            *start = Some(d);
            *end = None;
        }
        (Some(s), None) => {
            if d == s {
                *start = None;
            } else if d < s {
                *end = Some(s);
                *start = Some(d);
            } else {
                *end = Some(d);
            }
        }
        (Some(_), Some(_)) => {
            *start = Some(d);
            *end = None;
        }
    }
}

// ── month grid ────────────────────────────────────────────────────────────────

struct DrawConfig<'f> {
    /// The calendar's own unique id — used to namespace nav-button interactions
    /// so multiple calendars on screen don't share the same egui widget id.
    id: egui::Id,
    view: CalDate,
    sel_start: Option<CalDate>,
    sel_end: Option<CalDate>,
    /// Hovered day (from last frame) used to preview an in-progress range.
    preview: Option<CalDate>,
    show_prev: bool,
    show_next: bool,
    today: CalDate,
    cell_h: f32,
    cell_fn: Option<&'f CellFn<'f>>,
}

/// Returns `(nav, clicked_day, hovered_day)`.
fn draw_month<'f>(
    ui: &mut Ui,
    theme: &ShadcnTheme,
    cfg: DrawConfig<'f>,
) -> (i8, Option<CalDate>, Option<CalDate>) {
    let cell_w: f32 = 36.0;
    let mut nav: i8 = 0;
    let mut clicked: Option<CalDate> = None;
    let mut hovered_day: Option<CalDate> = None;

    ui.vertical(|ui| {
        // Zero horizontal spacing so day cells sit flush — keeps the day grid
        // width equal to `cell_w * 7`, which is what the header was allocated
        // for. Without this, the default 8 px item_spacing pushes the grid 48 px
        // past the header and the next-month arrow lands left of the real edge.
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.set_width(cell_w * 7.0);

        // header — absolute positioning within a single allocated row
        let (hdr, _) = ui.allocate_exact_size(Vec2::new(cell_w * 7.0, 36.0), Sense::hover());

        if cfg.show_prev {
            let btn = egui::Rect::from_min_size(hdr.min, Vec2::splat(36.0));
            let r = ui.interact(btn, cfg.id.with("cprev"), Sense::click());
            if r.hovered() || r.is_pointer_button_down_on() {
                ui.painter().rect_filled(btn, egui::CornerRadius::same(theme.radius as u8), theme.accent);
            }
            ui.painter().text(btn.center(), egui::Align2::CENTER_CENTER,
                ICON_CHEVRON_LEFT, ShadcnTheme::icon_font(16.0), theme.foreground);
            if r.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
            if r.clicked() { nav = -1; }
        }

        ui.painter().text(
            hdr.center(),
            egui::Align2::CENTER_CENTER,
            format!("{} {}", i18n::month_long(cfg.view.month), cfg.view.year),
            egui::FontId::new(14.0, egui::FontFamily::Proportional),
            theme.foreground,
        );

        if cfg.show_next {
            let btn = egui::Rect::from_min_max(
                egui::Pos2::new(hdr.max.x - 36.0, hdr.min.y),
                hdr.max,
            );
            let r = ui.interact(btn, cfg.id.with("cnext"), Sense::click());
            if r.hovered() || r.is_pointer_button_down_on() {
                ui.painter().rect_filled(btn, egui::CornerRadius::same(theme.radius as u8), theme.accent);
            }
            ui.painter().text(btn.center(), egui::Align2::CENTER_CENTER,
                ICON_CHEVRON_RIGHT, ShadcnTheme::icon_font(16.0), theme.foreground);
            if r.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }
            if r.clicked() { nav = 1; }
        }

        ui.add_space(6.0);

        // weekday labels
        ui.horizontal(|ui| {
            for day_idx in 0..7usize {
                let d = i18n::weekday_short(day_idx);
                let (rect, _) = ui.allocate_exact_size(Vec2::new(cell_w, 20.0), Sense::hover());
                ui.painter().text(
                    rect.center(), egui::Align2::CENTER_CENTER, d.as_ref(),
                    egui::FontId::new(11.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
            }
        });

        ui.add_space(2.0);

        // day cells
        let first_col = weekday_of(cfg.view.year, cfg.view.month, 1);
        let n_days = days_in_month(cfg.view.year, cfg.view.month) as usize;
        let n_rows = (first_col + n_days).div_ceil(7);

        for row in 0..n_rows {
            ui.horizontal(|ui| {
                for col in 0..7usize {
                    let day = (row * 7 + col) as i32 - first_col as i32 + 1;
                    let (rect, resp) = ui.allocate_exact_size(Vec2::new(cell_w, cfg.cell_h), Sense::click());

                    if day < 1 || day > n_days as i32 { continue; }

                    let date     = CalDate::new(cfg.view.year, cfg.view.month, day as u8);
                    let is_start = cfg.sel_start == Some(date);
                    let is_end   = cfg.sel_end == Some(date);
                    let real_in_range = matches!((cfg.sel_start, cfg.sel_end), (Some(s), Some(e)) if date > s && date < e);

                    // Range hover preview: once the first endpoint is picked
                    // (start set, end still empty) shade start→hovered-day as if
                    // it were already the selected range.
                    let (preview_end, preview_in_range) =
                        match (cfg.sel_start, cfg.sel_end, cfg.preview) {
                            (Some(s), None, Some(p)) => {
                                let (lo, hi) = if p < s { (p, s) } else { (s, p) };
                                (date == p && p != s, date > lo && date < hi)
                            }
                            _ => (false, false),
                        };

                    let in_range = real_in_range || preview_in_range;
                    let is_endpoint = is_start || is_end || preview_end;
                    let is_today = date == cfg.today;
                    let hovered  = resp.hovered();
                    if hovered {
                        hovered_day = Some(date);
                    }
                    // When cell has extra content, pin the number near the top;
                    // all highlight geometry is anchored to the same point.
                    let num_center = if cfg.cell_fn.is_some() {
                        egui::Pos2::new(rect.center().x, rect.top() + 10.0)
                    } else {
                        rect.center()
                    };
                    let r = (cell_w / 2.0 - 2.0).max(1.0);

                    if is_endpoint {
                        ui.painter().circle_filled(num_center, r, theme.primary);
                    } else if in_range {
                        ui.painter().rect_filled(rect, CornerRadius::ZERO, theme.accent);
                    } else if hovered {
                        ui.painter().circle_filled(num_center, r, theme.accent);
                    }

                    if is_today && !is_endpoint {
                        ui.painter().circle_stroke(
                            num_center, r,
                            Stroke::new(1.5, theme.primary),
                        );
                    }

                    let num_color = if is_endpoint        { theme.primary_foreground }
                                    else if is_today      { theme.primary }
                                    else                  { theme.foreground };

                    ui.painter().text(
                        num_center,
                        egui::Align2::CENTER_CENTER,
                        format!("{day}"),
                        egui::FontId::new(13.0, egui::FontFamily::Proportional),
                        num_color,
                    );

                    if let Some(f) = cfg.cell_fn {
                        let inner = egui::Rect::from_min_max(
                            egui::Pos2::new(rect.left() + 1.0, num_center.y + 12.0),
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

    (nav, clicked, hovered_day)
}
