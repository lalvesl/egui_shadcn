//! Zoom / pan view state of a cartesian [`ChartWidget`](crate::ChartWidget).
//!
//! The view lives in `egui` memory keyed by the widget id, so it survives the
//! `Chart` being rebuilt every frame. Callers address it with the same id they
//! pass to [`ChartWidget::id`](crate::ChartWidget::id):
//!
//! ```no_run
//! use egui_charts::ChartView;
//! # egui::__run_test_ui(|ui| {
//! let id = egui::Id::new("scope");
//! // "Follow latest 10 s": pin x every frame, keep whatever y the user chose.
//! let now = 42.0;
//! ChartView::update(ui.ctx(), id, |v| v.x = Some((now - 10.0, now)));
//! // "Reset view" button.
//! ChartView::reset(ui.ctx(), id);
//! # });
//! ```
//!
//! Precedence per axis: view override > `Axis::min`/`Axis::max` > auto-fit.

use egui::{Context, Id};

/// Persistent zoom/pan state for one chart.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct ChartView {
    /// X range override `(min, max)` in data units. `None` = axis bounds or
    /// auto-fit.
    pub x: Option<(f64, f64)>,
    /// Y range override `(min, max)` in data units. `None` = axis bounds or
    /// auto-fit.
    pub y: Option<(f64, f64)>,
    /// X range the widget actually drew on its last frame (written by the
    /// widget; `None` before the first frame or for category axes).
    pub drawn_x: Option<(f64, f64)>,
    /// Y range the widget actually drew on its last frame.
    pub drawn_y: Option<(f64, f64)>,
}

impl ChartView {
    fn key(id: Id) -> Id {
        id.with("egui_charts::view")
    }

    /// Read the view stored for the chart with widget id `id`.
    pub fn load(ctx: &Context, id: impl Into<Id>) -> Self {
        let key = Self::key(id.into());
        ctx.data(|d| d.get_temp::<Self>(key)).unwrap_or_default()
    }

    /// Store this view for the chart with widget id `id`. Takes effect on the
    /// chart's next `show` (call it before `show` for the same frame).
    pub fn store(self, ctx: &Context, id: impl Into<Id>) {
        let key = Self::key(id.into());
        ctx.data_mut(|d| d.insert_temp(key, self));
    }

    /// Load, modify, store.
    pub fn update(ctx: &Context, id: impl Into<Id>, f: impl FnOnce(&mut Self)) {
        let id = id.into();
        let mut v = Self::load(ctx, id);
        f(&mut v);
        v.store(ctx, id);
    }

    /// Drop both overrides — back to axis bounds / auto-fit. Same as a
    /// double-click on an interactive chart.
    pub fn reset(ctx: &Context, id: impl Into<Id>) {
        Self::update(ctx, id, |v| {
            v.x = None;
            v.y = None;
        });
    }

    pub fn with_x(mut self, min: f64, max: f64) -> Self {
        self.x = Some((min, max));
        self
    }

    pub fn with_y(mut self, min: f64, max: f64) -> Self {
        self.y = Some((min, max));
        self
    }

    /// `true` when neither axis is overridden.
    pub fn is_auto(&self) -> bool {
        self.x.is_none() && self.y.is_none()
    }
}

// ── Range math shared by zoom / pan (linear or log10 space) ─────────────────

#[inline]
pub(crate) fn to_t(v: f64, log: bool) -> f64 {
    if log { v.log10() } else { v }
}

#[inline]
pub(crate) fn from_t(t: f64, log: bool) -> f64 {
    if log { 10f64.powf(t) } else { t }
}

/// Scale `range` by `factor` around `pivot` (all in data units). `factor < 1`
/// zooms in. Log ranges are scaled in log10 space. Refuses to collapse or
/// blow up the range past what `f64` can represent sensibly.
pub fn zoom_range(
    range: (f64, f64),
    pivot: f64,
    factor: f64,
    log: bool,
) -> (f64, f64) {
    let (a, b) = (to_t(range.0, log), to_t(range.1, log));
    let p = to_t(pivot, log);
    if !a.is_finite() || !b.is_finite() || !p.is_finite() || factor <= 0.0 {
        return range;
    }
    let na = p + (a - p) * factor;
    let nb = p + (b - p) * factor;
    let span = (nb - na).abs();
    let mag = na.abs().max(nb.abs()).max(1e-300);
    let max_span = if log { 600.0 } else { 1e300 };
    if span < mag * 1e-12 || span > max_span || !span.is_finite() {
        return range;
    }
    (from_t(na, log), from_t(nb, log))
}

/// Shift `range` by `frac` of its own span (positive = towards larger values).
pub fn pan_range(range: (f64, f64), frac: f64, log: bool) -> (f64, f64) {
    let (a, b) = (to_t(range.0, log), to_t(range.1, log));
    if !a.is_finite() || !b.is_finite() || !frac.is_finite() {
        return range;
    }
    let d = (b - a) * frac;
    (from_t(a + d, log), from_t(b + d, log))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zoom_linear_keeps_pivot_fixed() {
        let r = zoom_range((0.0, 10.0), 2.0, 0.5, false);
        assert!((r.0 - 1.0).abs() < 1e-12 && (r.1 - 6.0).abs() < 1e-12);
    }

    #[test]
    fn zoom_log_happens_in_decades() {
        let r = zoom_range((1.0, 1e4), 100.0, 0.5, true);
        assert!((r.0 - 10.0).abs() < 1e-9, "{r:?}");
        assert!((r.1 - 1000.0).abs() < 1e-6, "{r:?}");
    }

    #[test]
    fn zoom_refuses_degenerate_range() {
        let r = (5.0, 5.0 + 1e-10);
        assert_eq!(zoom_range(r, 5.0, 1e-9, false), r);
    }

    #[test]
    fn pan_log_shifts_by_decades() {
        let r = pan_range((1.0, 100.0), 0.5, true);
        assert!((r.0 - 10.0).abs() < 1e-9 && (r.1 - 1000.0).abs() < 1e-6);
    }
}
