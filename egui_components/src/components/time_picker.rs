//! Touch-first time selector: two drag-scrollable wheels (hour `0..=23`,
//! minute `00..=59`) with snap-to-item, inertia and a centered selection band.
//!
//! Usable inline ([`TimePicker::inline`]) or behind a trigger
//! ([`TimePicker::show`]), which opens a [`Drawer`] on narrow viewports and a
//! [`Popover`] on wide ones.

use super::boxed::Boxed;
use super::breakpoint::Breakpoint;
use super::button::Button;
use super::drawer::Drawer;
use super::popover::Popover;
use super::size::Size;
use super::spacing::Spacing;
use crate::i18n;
use crate::{Animations, ICON_ACCESS_TIME, ShadcnTheme};
use ::i18n::t;
use egui::{
    Align, Align2, CornerRadius, FontId, Layout, Rect, Response, Sense, Stroke,
    StrokeKind, Ui, UiBuilder, Vec2,
};

// ── CalTime ───────────────────────────────────────────────────────────────────

/// Wall-clock time of day, 24-hour. Mirrors [`crate::CalDate`]'s plainness —
/// no timezone, no seconds.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default)]
pub struct CalTime {
    pub hour: u8,
    pub minute: u8,
}

impl CalTime {
    /// Clamps out-of-range input rather than panicking.
    pub fn new(hour: u8, minute: u8) -> Self {
        Self {
            hour: hour.min(23),
            minute: minute.min(59),
        }
    }

    /// Minutes since midnight, wrapping over a full day.
    pub fn from_minutes(total: i32) -> Self {
        let m = total.rem_euclid(24 * 60);
        Self {
            hour: (m / 60) as u8,
            minute: (m % 60) as u8,
        }
    }

    pub fn total_minutes(self) -> u16 {
        self.hour as u16 * 60 + self.minute as u16
    }
}

impl std::fmt::Display for CalTime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hour, self.minute)
    }
}

// ── wheel ─────────────────────────────────────────────────────────────────────

/// Gap between the two wheels, where the `:` sits.
const COLON_W: f32 = 26.0;
/// Below this speed (items/second) the flick is over and the wheel snaps.
const FLICK_CUTOFF: f32 = 0.6;
/// Inertia decay per second — higher stops the flick sooner.
const FRICTION: f32 = 6.0;

/// Per-wheel scroll state. `offset` is the *fractional* item index sitting on
/// the center line, normalized to `0..len`; the value is its rounded form.
#[derive(Clone, Copy)]
struct WheelState {
    offset: f32,
    velocity: f32,
    /// Last index this wheel itself produced, so a value changed from the
    /// outside can be told apart from one the drag produced and re-centered.
    last_idx: i32,
}

/// One scrollable column. Returns the new index when the wheel moved onto a
/// different item this frame.
#[allow(clippy::too_many_arguments)]
fn wheel_column(
    ui: &mut Ui,
    id: egui::Id,
    size: Vec2,
    len: usize,
    current: usize,
    row_h: f32,
    font: f32,
    label: impl Fn(usize) -> String,
) -> Option<usize> {
    let theme = ShadcnTheme::get(ui.ctx());
    let len_i = len as i32;
    let (rect, resp) = ui.allocate_exact_size(size, Sense::click_and_drag());

    let mut st =
        ui.ctx()
            .data(|d| d.get_temp::<WheelState>(id))
            .unwrap_or(WheelState {
                offset: current as f32,
                velocity: 0.0,
                last_idx: current as i32,
            });

    // Value moved under us (set by the app, or by a different widget): re-center.
    if st.last_idx != current as i32 {
        st.offset = current as f32;
        st.velocity = 0.0;
        st.last_idx = current as i32;
    }

    // Clamped so a stalled frame cannot fling the wheel across the list.
    let dt = ui.input(|i| i.stable_dt).clamp(1.0 / 240.0, 1.0 / 15.0);
    let mut animating = false;

    if resp.dragged() {
        let dy = resp.drag_delta().y;
        if dy != 0.0 {
            st.offset -= dy / row_h;
            st.velocity = -dy / row_h / dt;
        }
    } else if resp.clicked() {
        // Tapping a row brings it to the center — one tap for a small hop,
        // which is faster than dragging on a touch screen.
        if let Some(pos) = resp.interact_pointer_pos() {
            st.offset += ((pos.y - rect.center().y) / row_h).round();
            st.velocity = 0.0;
        }
    }

    // Motion budget: `0.0` when animations are off, which turns both the flick
    // and the snap into an immediate jump.
    let anim = Animations::duration(ui.ctx(), 0.20);
    if !resp.dragged() {
        if anim > 0.0 && st.velocity.abs() > FLICK_CUTOFF {
            st.offset += st.velocity * dt;
            st.velocity *= (-dt * FRICTION).exp();
            animating = true;
        } else {
            st.velocity = 0.0;
            let target = st.offset.round();
            let delta = target - st.offset;
            if anim <= 0.0 || delta.abs() < 0.001 {
                st.offset = target;
            } else {
                // Exponential approach: framerate-independent, unlike a fixed
                // per-frame step.
                st.offset += delta * (1.0 - (-dt / (anim / 3.0)).exp());
                animating = true;
            }
        }
    }

    st.offset = st.offset.rem_euclid(len as f32);
    let idx = (st.offset.round() as i32).rem_euclid(len_i) as usize;
    st.last_idx = idx as i32;
    ui.ctx().data_mut(|d| d.insert_temp(id, st));
    if animating || resp.dragged() {
        ui.ctx().request_repaint();
    }
    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::ResizeVertical);
    }

    // Rows are drawn relative to the fractional offset, so the whole column
    // moves continuously with the finger instead of jumping item by item.
    let painter = ui.painter().with_clip_rect(rect);
    let half = (size.y / row_h / 2.0).ceil() as i32;
    let base = st.offset.floor() as i32;
    for k in -half - 1..=half + 1 {
        let i = base + k;
        let dist = i as f32 - st.offset;
        let ad = dist.abs();
        let fade = (1.0 - ad / (half as f32 + 0.8)).clamp(0.0, 1.0);
        let color = if ad < 0.5 {
            theme.accent_foreground
        } else {
            theme.muted_foreground
        }
        .gamma_multiply(0.25 + 0.75 * fade);
        painter.text(
            egui::pos2(rect.center().x, rect.center().y + dist * row_h),
            Align2::CENTER_CENTER,
            label(i.rem_euclid(len_i) as usize),
            FontId::proportional(font * (1.0 - 0.10 * ad.min(2.0))),
            color,
        );
    }

    (idx != current).then_some(idx)
}

/// The bare wheels — no surface of its own, so the caller's `Boxed`, popover or
/// drawer provides it. Returns `true` when `value` changed.
fn wheels(
    ui: &mut Ui,
    id: egui::Id,
    value: &mut CalTime,
    minute_step: u8,
    rows: usize,
    size: Size,
) -> bool {
    let theme = ShadcnTheme::get(ui.ctx());
    let row_h = size.height() + 4.0;
    let font = size.font_size() + 6.0;
    let height = rows as f32 * row_h;
    let avail = ui.available_rect_before_wrap();
    let col_w = ((avail.width() - COLON_W) / 2.0).clamp(48.0, 110.0).floor();

    // The wheels are capped at `col_w`, so on anything wider than they need —
    // a drawer, a full-width card — they are centered in the space rather than
    // left over on one edge.
    let total_w = col_w * 2.0 + COLON_W;
    let row_rect = Rect::from_min_size(
        egui::pos2(
            avail.left() + ((avail.width() - total_w) / 2.0).max(0.0),
            avail.top(),
        ),
        Vec2::new(total_w, height),
    );

    // Reserved slot: the selection band has to sit *under* the digits, but its
    // rect is only known once the row has been laid out.
    let band_idx = ui.painter().add(egui::Shape::Noop);

    let step = minute_step.clamp(1, 30) as usize;
    let mut changed = false;
    let row = ui.scope_builder(
        UiBuilder::new()
            .max_rect(row_rect)
            .layout(Layout::left_to_right(Align::Center)),
        |ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            if let Some(h) = wheel_column(
                ui,
                id.with("hour"),
                Vec2::new(col_w, height),
                24,
                value.hour as usize,
                row_h,
                font,
                |i| format!("{i:02}"),
            ) {
                value.hour = h as u8;
                changed = true;
            }

            let (colon, _) = ui.allocate_exact_size(
                Vec2::new(COLON_W, height),
                Sense::hover(),
            );
            ui.painter().text(
                colon.center(),
                Align2::CENTER_CENTER,
                ":",
                FontId::proportional(font),
                theme.accent_foreground,
            );

            // Minutes are shown at `step` granularity but stored as real minutes,
            // so an app-set 07 with `minute_step(5)` still lands on the 05 row.
            let len = 60 / step;
            let cur = (value.minute as usize / step).min(len - 1);
            if let Some(m) = wheel_column(
                ui,
                id.with("minute"),
                Vec2::new(col_w, height),
                len,
                cur,
                row_h,
                font,
                move |i| format!("{:02}", i * step),
            ) {
                value.minute = (m * step) as u8;
                changed = true;
            }
        },
    );

    // Claim the full width so the surface around the wheels (Boxed, popover,
    // drawer) stays symmetric instead of hugging the centered row.
    ui.expand_to_include_rect(Rect::from_min_size(
        avail.min,
        Vec2::new(avail.width(), height),
    ));

    let r = row.response.rect;
    ui.painter().set(
        band_idx,
        egui::epaint::RectShape::new(
            Rect::from_center_size(r.center(), Vec2::new(r.width(), row_h)),
            CornerRadius::same(theme.radius as u8),
            theme.accent,
            Stroke::new(1.0, theme.border),
            StrokeKind::Inside,
        ),
    );

    changed
}

// ── TimePicker ────────────────────────────────────────────────────────────────

pub struct TimePicker<'a> {
    id: &'a str,
    value: &'a mut CalTime,
    title: Option<&'a str>,
    minute_step: u8,
    rows: usize,
    size: Option<Size>,
    width: f32,
}

impl<'a> TimePicker<'a> {
    pub fn new(id: &'a str, value: &'a mut CalTime) -> Self {
        Self {
            id,
            value,
            title: None,
            minute_step: 1,
            rows: 5,
            size: None,
            width: 240.0,
        }
    }

    /// Minute granularity, clamped to `1..=30`. Default `1`.
    pub fn minute_step(mut self, step: u8) -> Self {
        self.minute_step = step;
        self
    }

    /// Visible rows per wheel — odd values keep the selection centered.
    /// Default `5`.
    pub fn rows(mut self, rows: usize) -> Self {
        self.rows = rows.max(3);
        self
    }

    /// Row height / type scale. Defaults to [`Size::Lg`] on narrow viewports
    /// (44 px touch rows) and [`Size::Default`] elsewhere.
    pub fn size(mut self, s: Size) -> Self {
        self.size = Some(s);
        self
    }

    /// Trigger width for [`TimePicker::show`]. Default `240.0`.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    /// Drawer heading on the mobile path. Defaults to a translated "Select time".
    pub fn title(mut self, t: &'a str) -> Self {
        self.title = Some(t);
        self
    }

    fn resolved_size(&self, ui: &Ui) -> Size {
        self.size.unwrap_or(if Breakpoint::current(ui).is_narrow() {
            Size::Lg
        } else {
            Size::Default
        })
    }

    /// The wheels alone, boxed, laid out where they are called. Returns `true`
    /// when the time changed this frame.
    pub fn inline(self, ui: &mut Ui) -> bool {
        let theme = ShadcnTheme::get(ui.ctx());
        let size = self.resolved_size(ui);
        let id = egui::Id::new("shadcn_time_picker").with(self.id);
        let (value, step, rows) = (self.value, self.minute_step, self.rows);
        Boxed::new()
            .fill(theme.card)
            .padding(Spacing::Sm)
            .show(ui, |ui| wheels(ui, id, value, step, rows, size))
            .inner
    }

    /// Trigger field that opens the wheels — in a [`Drawer`] on narrow
    /// viewports (thumb-reachable, full width) and a [`Popover`] otherwise.
    /// Returns `true` when the time changed this frame.
    pub fn show(self, ui: &mut Ui) -> bool {
        let size = self.resolved_size(ui);
        let id = egui::Id::new("shadcn_time_picker").with(self.id);
        let (step, rows, width) = (self.minute_step, self.rows, self.width);
        let mobile = Breakpoint::current(ui).is_narrow();
        let mut changed = false;

        if !mobile {
            let value = self.value;
            let shown = *value;
            Popover::new(id.with("popover")).width(width).show(
                ui,
                |ui| trigger(ui, width, shown),
                |ui| {
                    changed = wheels(ui, id, value, step, rows, size);
                },
            );
            return changed;
        }

        let open_id = id.with("open");
        // Read before the trigger is drawn, and open only from the *next* frame:
        // the drawer treats a click outside its rect as dismiss, so a drawer
        // shown on the very frame the trigger was clicked would close instantly.
        let was_open = ui
            .ctx()
            .data(|d| d.get_temp::<bool>(open_id).unwrap_or(false));
        let resp = trigger(ui, width, *self.value);
        let mut open = was_open;
        if resp.clicked() {
            open = !was_open;
        }

        if was_open {
            let default_title = t!(i18n::TimePicker::Title);
            let title = self.title.unwrap_or(default_title.as_ref());
            let done = t!(i18n::TimePicker::Done);
            let value = self.value;
            // Header, wheels, confirm button, plus the drawer's own padding.
            let height = rows as f32 * (size.height() + 4.0) + 140.0;
            let mut still_open = true;
            Drawer::new(title, &mut still_open).height(height).show(
                ui.ctx(),
                |ui| {
                    changed = wheels(ui, id, value, step, rows, size);
                    Spacing::Md.show(ui);
                    ui.vertical_centered(|ui| {
                        if Button::new(done.as_ref()).show(ui).clicked() {
                            ui.ctx()
                                .data_mut(|d| d.insert_temp(open_id, false));
                        }
                    });
                },
            );
            // The drawer clears the flag on ESC, backdrop click or its close
            // button; the confirm button writes the same flag directly.
            open = still_open
                && ui
                    .ctx()
                    .data(|d| d.get_temp::<bool>(open_id).unwrap_or(true));
        }

        ui.ctx().data_mut(|d| d.insert_temp(open_id, open));
        changed
    }
}

/// Read-only field showing the current time — same shape as the `DatePicker`
/// trigger.
fn trigger(ui: &mut Ui, width: f32, value: CalTime) -> Response {
    let theme = ShadcnTheme::get(ui.ctx());
    let (rect, resp) =
        ui.allocate_exact_size(Vec2::new(width, 36.0), Sense::click());

    let cr = CornerRadius::same(theme.radius as u8);
    let border = if resp.hovered() {
        theme.ring
    } else {
        theme.border
    };
    ui.painter().rect_filled(rect, cr, theme.background);
    ui.painter().rect_stroke(
        rect,
        cr,
        Stroke::new(1.0, border),
        StrokeKind::Inside,
    );

    ui.painter().text(
        egui::pos2(rect.left() + 12.0, rect.center().y),
        Align2::LEFT_CENTER,
        ICON_ACCESS_TIME,
        crate::icon_font_id(16.0),
        theme.muted_foreground,
    );
    ui.painter().text(
        egui::pos2(rect.left() + 34.0, rect.center().y),
        Align2::LEFT_CENTER,
        value.to_string(),
        FontId::proportional(14.0),
        theme.foreground,
    );

    if resp.hovered() {
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }
    resp
}
