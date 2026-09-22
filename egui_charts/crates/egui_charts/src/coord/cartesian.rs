//! Cartesian (rectangular) coordinate system — scales, nice-number ticks,
//! axis/grid render, data↔screen transforms.

use super::{AxisLayout, AxisTick, CoordKind, CoordLayout, DataPoint};
use crate::option::{Axis, AxisKind, Chart, Extent, Series, XyLineSeries};
use crate::theme::ChartTheme;
use egui::{Pos2, Rect, vec2};

// ── Scales ───────────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum Scale {
    Linear { min: f64, max: f64 },
    Log { min: f64, max: f64 }, // base 10
    Category { count: usize },
}

impl Scale {
    /// Map a data value in this scale to [0..1] (or NaN if outside log domain).
    pub fn normalize(&self, v: f64) -> f64 {
        match *self {
            Scale::Linear { min, max } => {
                if (max - min).abs() < f64::EPSILON {
                    0.5
                } else {
                    (v - min) / (max - min)
                }
            }
            Scale::Log { min, max } => {
                if v <= 0.0 || min <= 0.0 || max <= 0.0 {
                    f64::NAN
                } else {
                    (v.log10() - min.log10()) / (max.log10() - min.log10())
                }
            }
            Scale::Category { count } => {
                if count == 0 {
                    0.5
                } else {
                    (v + 0.5) / count as f64
                }
            }
        }
    }

    /// Inverse of `normalize`.
    pub fn unnormalize(&self, t: f64) -> f64 {
        match *self {
            Scale::Linear { min, max } => min + t * (max - min),
            Scale::Log { min, max } => {
                let lo = min.log10();
                let hi = max.log10();
                10f64.powf(lo + t * (hi - lo))
            }
            Scale::Category { count } => {
                let n = count as f64;
                (t * n).floor().clamp(0.0, (count.max(1) - 1) as f64)
            }
        }
    }
}

impl Scale {
    /// `(min, max, is_log)` of a continuous scale; `None` for categories.
    pub fn range(&self) -> Option<(f64, f64, bool)> {
        match *self {
            Scale::Linear { min, max } => Some((min, max, false)),
            Scale::Log { min, max } => Some((min, max, true)),
            Scale::Category { .. } => None,
        }
    }

    /// Data-space extent covered by the scale (categories span
    /// `-0.5 ..= count - 0.5`), ordered low → high.
    pub fn data_extent(&self) -> (f64, f64) {
        let (a, b) = match *self {
            Scale::Linear { min, max } | Scale::Log { min, max } => (min, max),
            Scale::Category { count } => (-0.5, count as f64 - 0.5),
        };
        (a.min(b), a.max(b))
    }
}

/// Precomputed affine map data → screen pixel along one axis (in log10 space
/// for log scales). One multiply-add per point, no boxed closure — used by
/// series that walk a million points per frame.
#[derive(Clone, Copy, Debug)]
pub struct AxisMap {
    log: bool,
    t0: f64,
    p0: f64,
    k: f64,
    span_px: f64,
}

impl AxisMap {
    /// `p_at_min` / `p_at_max`: pixel coordinate of the scale's min / max.
    pub fn new(scale: &Scale, p_at_min: f32, p_at_max: f32) -> Self {
        let (lo, hi, log) = match *scale {
            Scale::Linear { min, max } => (min, max, false),
            Scale::Log { min, max } => (min.log10(), max.log10(), true),
            Scale::Category { count } => (-0.5, count as f64 - 0.5, false),
        };
        let span = hi - lo;
        if !span.is_finite() || span.abs() < f64::EPSILON {
            // Degenerate range: everything maps to the middle, like
            // `Scale::normalize` does.
            return Self {
                log,
                t0: lo,
                p0: (p_at_min as f64 + p_at_max as f64) * 0.5,
                k: 0.0,
                span_px: 0.0,
            };
        }
        Self {
            log,
            t0: lo,
            p0: p_at_min as f64,
            k: (p_at_max as f64 - p_at_min as f64) / span,
            span_px: p_at_max as f64 - p_at_min as f64,
        }
    }

    /// Signed pixel distance from the scale's min to its max (negative for a
    /// y axis growing upwards or an inverted x axis; 0 when degenerate).
    pub fn map_span_px(&self) -> f64 {
        self.span_px
    }

    /// Data → pixel. NaN when `v` is outside a log scale's domain.
    #[inline]
    pub fn map(&self, v: f64) -> f64 {
        let t = if self.log {
            if v > 0.0 { v.log10() } else { f64::NAN }
        } else {
            v
        };
        self.p0 + (t - self.t0) * self.k
    }

    /// Pixel → data.
    #[inline]
    pub fn unmap(&self, p: f64) -> f64 {
        if self.k == 0.0 {
            return if self.log {
                10f64.powf(self.t0)
            } else {
                self.t0
            };
        }
        let t = self.t0 + (p - self.p0) / self.k;
        if self.log { 10f64.powf(t) } else { t }
    }
}

/// Resolved scales of a cartesian layout, attached to its
/// [`CoordLayout`](super::CoordLayout).
#[derive(Clone, Debug)]
pub struct CartesianScales {
    pub x: Scale,
    pub y: Scale,
    pub x_map: AxisMap,
    pub y_map: AxisMap,
    pub plot_rect: Rect,
    /// The rect the layout was computed in (plot + axis gutters).
    pub outer_rect: Rect,
}

impl CartesianScales {
    /// Data → screen, `None` when either coordinate is off a log domain or
    /// non-finite.
    #[inline]
    pub fn to_screen(&self, x: f64, y: f64) -> Option<Pos2> {
        let sx = self.x_map.map(x);
        let sy = self.y_map.map(y);
        (sx.is_finite() && sy.is_finite())
            .then(|| Pos2::new(sx as f32, sy as f32))
    }
}

// ── Nice-number tick generation ──────────────────────────────────────────────

/// Round `x` to a "nice" 1/2/5 × 10^k step.
fn nice_step(x: f64, round: bool) -> f64 {
    if x <= 0.0 {
        return 1.0;
    }
    let exp = x.log10().floor();
    let f = x / 10f64.powf(exp);
    let nf = if round {
        if f < 1.5 {
            1.0
        } else if f < 3.0 {
            2.0
        } else if f < 7.0 {
            5.0
        } else {
            10.0
        }
    } else if f <= 1.0 {
        1.0
    } else if f <= 2.0 {
        2.0
    } else if f <= 5.0 {
        5.0
    } else {
        10.0
    };
    nf * 10f64.powf(exp)
}

/// Wilkinson-style 1-2-5 tick generator: returns (nice_min, nice_max, step).
pub fn nice_range(min: f64, max: f64, target_ticks: usize) -> (f64, f64, f64) {
    if min == max {
        let pad = if min == 0.0 { 1.0 } else { min.abs() * 0.1 };
        return nice_range(min - pad, max + pad, target_ticks);
    }
    let range = nice_step(max - min, false);
    let step = nice_step(range / (target_ticks.max(1) as f64), true);
    let nice_min = (min / step).floor() * step;
    let nice_max = (max / step).ceil() * step;
    (nice_min, nice_max, step)
}

/// Linear-scale ticks at nice multiples of `step`.
pub fn ticks_linear(min: f64, max: f64, step: f64) -> Vec<f64> {
    if step <= 0.0 || !min.is_finite() || !max.is_finite() {
        return vec![min, max];
    }
    let mut ticks = Vec::new();
    let start = (min / step).round() as i64;
    let end = (max / step).round() as i64;
    for i in start..=end {
        let v = i as f64 * step;
        if v >= min - step * 1e-6 && v <= max + step * 1e-6 {
            ticks.push(v);
        }
    }
    ticks
}

/// Log-scale ticks at each decade boundary in [min, max].
pub fn ticks_log(min: f64, max: f64) -> Vec<f64> {
    if min <= 0.0 || max <= 0.0 {
        return Vec::new();
    }
    let lo = min.log10().floor() as i32;
    let hi = max.log10().ceil() as i32;
    (lo..=hi)
        .map(|e| 10f64.powi(e))
        .filter(|v| *v >= min && *v <= max)
        .collect()
}

/// Log-scale ticks readable at any zoom: decades when the range spans
/// several, 1-2-5 (then 1..9) multiples inside fewer decades, and plain
/// linear nice numbers when zoomed inside a single mantissa step. At most
/// `max_ticks` are returned (decades are thinned to every n-th).
pub fn ticks_log_nice(min: f64, max: f64, max_ticks: usize) -> Vec<f64> {
    let (lo, hi) = (min.min(max), min.max(max));
    if lo.is_nan() || lo <= 0.0 || !hi.is_finite() {
        return Vec::new();
    }
    let max_ticks = max_ticks.max(2);
    let e0 = lo.log10().floor() as i32;
    let e1 = hi.log10().ceil() as i32;
    let in_range = |v: f64| v >= lo * (1.0 - 1e-9) && v <= hi * (1.0 + 1e-9);

    let decades: Vec<i32> =
        (e0..=e1).filter(|e| in_range(10f64.powi(*e))).collect();
    if decades.len() >= 2 {
        let stride = decades.len().div_ceil(max_ticks).max(1) as i32;
        let picked: Vec<f64> = decades
            .iter()
            .filter(|e| e.rem_euclid(stride) == 0)
            .map(|e| 10f64.powi(*e))
            .collect();
        if picked.len() >= 2 {
            return picked;
        }
        return decades
            .iter()
            .map(|e| 10f64.powi(*e))
            .take(max_ticks)
            .collect();
    }

    for mults in [
        &[1.0, 2.0, 5.0][..],
        &[1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0][..],
    ] {
        let v: Vec<f64> = (e0 - 1..=e1)
            .flat_map(|e| mults.iter().map(move |m| m * 10f64.powi(e)))
            .filter(|v| in_range(*v))
            .collect();
        if v.len() >= 3 && v.len() <= max_ticks {
            return v;
        }
    }

    // Zoomed inside one mantissa step: linear nice numbers.
    let (nlo, nhi, step) = nice_range(lo, hi, max_ticks.min(8));
    ticks_linear(nlo, nhi, step)
        .into_iter()
        .filter(|v| in_range(*v))
        .collect()
}

/// Trim a fixed-point rendering: `"1.500"` → `"1.5"`, `"2.000"` → `"2"`.
fn trim_fixed(s: String) -> String {
    if s.contains('.') && !s.contains('e') {
        s.trim_end_matches('0').trim_end_matches('.').to_string()
    } else {
        s
    }
}

/// Label for a log-axis tick: `0.001 0.01 0.1 1 10 100 1k 10k 100k`, and
/// exponent form (`1e6`, `2e-5`) outside that span.
pub fn format_log_tick(v: f64) -> String {
    if v == 0.0 {
        return "0".into();
    }
    let abs = v.abs();
    if !(1e-4..1e6).contains(&abs) {
        let s = format!("{:.2e}", v);
        // "1.00e6" → "1e6", "2.50e-5" → "2.5e-5"
        return match s.split_once('e') {
            Some((m, e)) => format!("{}e{}", trim_fixed(m.to_string()), e),
            None => s,
        };
    }
    if abs >= 1e3 {
        format!("{}k", trim_fixed(format!("{:.3}", v / 1e3)))
    } else {
        trim_fixed(format!("{:.6}", v))
    }
}

/// Decimals needed to tell ticks `step` apart.
fn step_decimals(step: f64) -> usize {
    if step > 0.0 && step.is_finite() {
        (-step.log10() - 1e-9).ceil().clamp(0.0, 15.0) as usize
    } else {
        0
    }
}

/// Format a tick value compactly. Picks fixed/short/exp form by magnitude,
/// with as many decimals as the tick `step` needs — so a zoomed axis
/// (`1000.125, 1000.250, …`) never collapses to identical labels.
pub fn format_tick(v: f64, step: f64) -> String {
    if v == 0.0 || (step > 0.0 && v.abs() < step * 1e-6) {
        return "0".to_string();
    }
    let abs = v.abs();
    if abs >= 1e6 && step >= 1e5 {
        let d = step_decimals(step / 1e6);
        format!("{:.*}M", d.max(1), v / 1e6)
    } else if abs >= 1e3 && step >= 100.0 {
        let s = v / 1e3;
        if (s - s.round()).abs() < 1e-6 {
            format!("{:.0}k", s)
        } else {
            format!("{:.*}k", step_decimals(step / 1e3).max(1), s)
        }
    } else if abs < 0.01 && step < 0.001 && step_decimals(step) > 5 {
        format!("{:.2e}", v)
    } else {
        format!("{:.*}", step_decimals(step), v)
    }
}

// ── Auto-fit data range from series ──────────────────────────────────────────

fn series_value_extent_opt(chart: &Chart) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut any = false;

    // Stack-aware: sum positives + negatives per stack/index.
    use std::collections::HashMap;
    let mut stacked_pos: HashMap<(String, usize), f64> = HashMap::new();
    let mut stacked_neg: HashMap<(String, usize), f64> = HashMap::new();

    for s in &chart.series {
        match s {
            Series::Line(l) => {
                for (i, &v) in l.data.iter().enumerate() {
                    if !v.is_finite() {
                        continue;
                    }
                    any = true;
                    if let Some(stack) = &l.stack {
                        let key = (stack.clone(), i);
                        if v >= 0.0 {
                            let e = stacked_pos.entry(key).or_insert(0.0);
                            *e += v;
                            max = max.max(*e);
                            min = min.min(0.0);
                        } else {
                            let e = stacked_neg.entry(key).or_insert(0.0);
                            *e += v;
                            min = min.min(*e);
                            max = max.max(0.0);
                        }
                    } else {
                        min = min.min(v);
                        max = max.max(v);
                    }
                }
            }
            Series::Bar(b) => {
                for (i, &v) in b.data.iter().enumerate() {
                    if !v.is_finite() {
                        continue;
                    }
                    any = true;
                    if let Some(stack) = &b.stack {
                        let key = (stack.clone(), i);
                        if v >= 0.0 {
                            let e = stacked_pos.entry(key).or_insert(0.0);
                            *e += v;
                            max = max.max(*e);
                            min = min.min(0.0);
                        } else {
                            let e = stacked_neg.entry(key).or_insert(0.0);
                            *e += v;
                            min = min.min(*e);
                            max = max.max(0.0);
                        }
                    } else {
                        min = min.min(v);
                        max = max.max(v);
                        if v >= 0.0 {
                            min = min.min(0.0); // bars baseline at 0
                        } else {
                            max = max.max(0.0);
                        }
                    }
                }
            }
            Series::Scatter(s) => {
                for &(_, y, _) in &s.data {
                    if !y.is_finite() {
                        continue;
                    }
                    any = true;
                    min = min.min(y);
                    max = max.max(y);
                }
            }
            Series::EffectScatter(s) => {
                for &(_, y, _) in &s.data {
                    if !y.is_finite() {
                        continue;
                    }
                    any = true;
                    min = min.min(y);
                    max = max.max(y);
                }
            }
            Series::Candlestick(c) => {
                for cd in &c.data {
                    if cd.high.is_finite() && cd.low.is_finite() {
                        any = true;
                        min = min.min(cd.low);
                        max = max.max(cd.high);
                    }
                }
            }
            Series::BoxPlot(b) => {
                for bd in &b.data {
                    if bd.min.is_finite() && bd.max.is_finite() {
                        any = true;
                        min = min.min(bd.min);
                        max = max.max(bd.max);
                    }
                }
            }
            Series::LinesCartesian(lc) => {
                for seg in &lc.segments {
                    if seg.from.1.is_finite() && seg.to.1.is_finite() {
                        any = true;
                        min = min.min(seg.from.1).min(seg.to.1);
                        max = max.max(seg.from.1).max(seg.to.1);
                    }
                }
            }
            Series::PictorialBar(pb) => {
                for &v in &pb.data {
                    if v.is_finite() {
                        any = true;
                        min = min.min(v).min(0.0);
                        max = max.max(v).max(0.0);
                    }
                }
            }
            Series::ThemeRiver(tr) => {
                let n_slots =
                    tr.bands.iter().map(|b| b.data.len()).max().unwrap_or(0);
                for slot in 0..n_slots {
                    let total: f64 = tr
                        .bands
                        .iter()
                        .map(|b| {
                            b.data.get(slot).copied().unwrap_or(0.0).max(0.0)
                        })
                        .sum();
                    any = true;
                    min = min.min(-total * 0.5);
                    max = max.max(total * 0.5);
                }
            }
            // Heatmap is purely categorical on both axes; non-cartesian series
            // don't contribute to a cartesian Y extent.
            _ => {}
        }
    }

    any.then_some((min, max))
}

fn series_x_extent_opt(chart: &Chart) -> Option<(f64, f64)> {
    let mut min = f64::INFINITY;
    let mut max = f64::NEG_INFINITY;
    let mut any = false;
    for s in &chart.series {
        match s {
            Series::Scatter(sc) => {
                for &(x, _, _) in &sc.data {
                    if x.is_finite() {
                        any = true;
                        min = min.min(x);
                        max = max.max(x);
                    }
                }
            }
            Series::EffectScatter(sc) => {
                for &(x, _, _) in &sc.data {
                    if x.is_finite() {
                        any = true;
                        min = min.min(x);
                        max = max.max(x);
                    }
                }
            }
            Series::LinesCartesian(lc) => {
                for seg in &lc.segments {
                    if seg.from.0.is_finite() && seg.to.0.is_finite() {
                        any = true;
                        min = min.min(seg.from.0).min(seg.to.0);
                        max = max.max(seg.from.0).max(seg.to.0);
                    }
                }
            }
            _ => {}
        }
    }
    any.then_some((min, max))
}

/// Merge two optional extents.
fn merge(a: Option<(f64, f64)>, b: Option<(f64, f64)>) -> Option<(f64, f64)> {
    match (a, b) {
        (Some(a), Some(b)) => Some((a.0.min(b.0), a.1.max(b.1))),
        (a, None) => a,
        (None, b) => b,
    }
}

/// Default an empty extent and pad a degenerate one. Log axes pad by a decade
/// each way so the result stays positive.
fn finish_extent(e: Option<(f64, f64)>, log: bool) -> (f64, f64) {
    match e {
        None if log => (1.0, 10.0),
        None => (0.0, 1.0),
        Some((min, max)) if (max - min).abs() < f64::EPSILON => {
            if log && min > 0.0 {
                (min / 10.0, max * 10.0)
            } else {
                (min - 1.0, max + 1.0)
            }
        }
        Some(e) => e,
    }
}

/// X extent of every XY series (positive-only on a log axis).
fn xy_x_extent(chart: &Chart, log: bool) -> Option<(f64, f64)> {
    let mut out = None;
    for s in &chart.series {
        if let Series::XyLine(l) = s {
            out = merge(out, l.data.x_extent(log));
        }
    }
    out
}

/// Y extent of one XY series restricted to points whose x is inside
/// `x_range` — so auto-fit y follows what is on screen when x is zoomed or
/// pinned (the "follow latest N seconds" case). Positive-only on a log axis.
pub fn xy_y_extent(
    s: &XyLineSeries,
    x_range: Option<(f64, f64)>,
    log: bool,
) -> Option<(f64, f64)> {
    let full = |s: &XyLineSeries| {
        let st = s.data.stats();
        if log { st.y_pos } else { st.y }
    };
    let Some((lo, hi)) = x_range else {
        return full(s);
    };
    let data = s.data.as_slice();
    let slice = if s.data.is_sorted() {
        let a = data.partition_point(|p| p[0] < lo);
        let b = data.partition_point(|p| p[0] <= hi);
        if a == 0 && b == data.len() {
            return full(s);
        }
        &data[a..b]
    } else {
        if let Some((x0, x1)) = s.data.stats().x
            && x0 >= lo
            && x1 <= hi
        {
            return full(s);
        }
        data
    };
    let mut e = Extent::default();
    for &[x, y] in slice {
        if x >= lo && x <= hi && y.is_finite() && (!log || y > 0.0) {
            e.add(y);
        }
    }
    e.get()
}

fn xy_y_extent_all(
    chart: &Chart,
    x_range: Option<(f64, f64)>,
    log: bool,
) -> Option<(f64, f64)> {
    let mut out = None;
    for s in &chart.series {
        if let Series::XyLine(l) = s {
            out = merge(out, xy_y_extent(l, x_range, log));
        }
    }
    out
}

fn has_xy(chart: &Chart) -> bool {
    chart.series.iter().any(|s| matches!(s, Series::XyLine(_)))
}

/// A view override turned into an exact scale (no nice-rounding). Category
/// axes are never zoomed.
fn override_scale(axis: &Axis, range: Option<(f64, f64)>) -> Option<Scale> {
    let (a, b) = range?;
    if axis.kind == AxisKind::Category || !a.is_finite() || !b.is_finite() {
        return None;
    }
    let (lo, hi) = (a.min(b), a.max(b));
    Some(match axis.kind {
        AxisKind::Log => {
            let lo = lo.max(1e-300);
            let hi = hi.max(lo * 10.0_f64.powf(1e-9));
            Scale::Log { min: lo, max: hi }
        }
        _ => {
            let (lo, hi) = if hi - lo <= 0.0 {
                finish_extent(Some((lo, hi)), false)
            } else {
                (lo, hi)
            };
            Scale::Linear { min: lo, max: hi }
        }
    })
}

fn max_series_len(chart: &Chart) -> usize {
    chart
        .series
        .iter()
        .map(|s| match s {
            Series::Line(l) => l.data.len(),
            Series::Bar(b) => b.data.len(),
            Series::Scatter(sc) => sc.data.len(),
            Series::Candlestick(c) => c.data.len(),
            Series::BoxPlot(b) => b.data.len(),
            Series::PictorialBar(pb) => pb.data.len(),
            Series::Heatmap(h) => h
                .data
                .iter()
                .map(|(x, _, _)| *x)
                .max()
                .map(|m| m + 1)
                .unwrap_or(0),
            Series::ThemeRiver(tr) => {
                tr.bands.iter().map(|b| b.data.len()).max().unwrap_or(0)
            }
            _ => 0,
        })
        .max()
        .unwrap_or(0)
}

// ── Build a Scale from an Axis spec + data ──────────────────────────────────

pub fn build_scale(
    axis: &Axis,
    data_min: f64,
    data_max: f64,
    default_categories: usize,
) -> Scale {
    match axis.kind {
        AxisKind::Category => Scale::Category {
            count: if axis.categories.is_empty() {
                default_categories
            } else {
                axis.categories.len()
            },
        },
        AxisKind::Log => Scale::Log {
            min: axis.min.unwrap_or(data_min).max(1e-12),
            max: axis.max.unwrap_or(data_max).max(1e-12),
        },
        AxisKind::Value => {
            let raw_min = axis.min.unwrap_or(data_min);
            let raw_max = axis.max.unwrap_or(data_max);
            let (lo, hi, _step) = nice_range(raw_min, raw_max, 6);
            // Explicit user bounds are exact; nice-rounding only widens the
            // auto-fit side (a flat-zero series otherwise pads to [-1, 1]
            // even when the caller pinned min to 0).
            Scale::Linear {
                min: axis.min.unwrap_or(lo),
                max: axis.max.unwrap_or(hi),
            }
        }
    }
}

// ── CartesianCoord ───────────────────────────────────────────────────────────

pub struct CartesianCoord<'a> {
    pub chart: &'a Chart,
}

impl<'a> CartesianCoord<'a> {
    pub fn new(chart: &'a Chart) -> Self {
        Self { chart }
    }

    pub fn layout(&self, rect: Rect, theme: &ChartTheme) -> CoordLayout {
        self.layout_view(rect, theme, None, None)
    }

    /// Layout with optional zoom/pan overrides for the x and y ranges (data
    /// units). An override beats `Axis::min`/`Axis::max`, which beat auto-fit.
    /// Overrides on category axes are ignored.
    pub fn layout_view(
        &self,
        rect: Rect,
        theme: &ChartTheme,
        view_x: Option<(f64, f64)>,
        view_y: Option<(f64, f64)>,
    ) -> CoordLayout {
        let default_x_axis;
        let default_y_axis;
        let x_axis = if let Some(a) = self.chart.x_axis.as_ref() {
            a
        } else if has_xy(self.chart) {
            // XY series carry numeric x — a category default makes no sense.
            default_x_axis = Axis::value();
            &default_x_axis
        } else {
            default_x_axis = Axis::category(
                (0..max_series_len(self.chart)).map(|i| i.to_string()),
            );
            &default_x_axis
        };
        let y_axis = if let Some(a) = self.chart.y_axis.as_ref() {
            a
        } else {
            default_y_axis = Axis::value();
            &default_y_axis
        };
        let x_log = x_axis.kind == AxisKind::Log;
        let y_log = y_axis.kind == AxisKind::Log;

        let cat_count = max_series_len(self.chart);
        let x_scale = override_scale(x_axis, view_x).unwrap_or_else(|| {
            let (lo, hi) = finish_extent(
                merge(
                    series_x_extent_opt(self.chart),
                    xy_x_extent(self.chart, x_log),
                ),
                x_log,
            );
            build_scale(x_axis, lo, hi, cat_count)
        });
        let y_scale = override_scale(y_axis, view_y).unwrap_or_else(|| {
            // XY series fit y to the points inside the resolved x range.
            let x_range = x_scale.range().map(|(a, b, _)| (a.min(b), a.max(b)));
            let (lo, hi) = finish_extent(
                merge(
                    series_value_extent_opt(self.chart),
                    xy_y_extent_all(self.chart, x_range, y_log),
                ),
                y_log,
            );
            build_scale(y_axis, lo, hi, cat_count)
        });

        // Reserve gutter for tick labels + axis names. A named axis gets a
        // dedicated line so the title never overlaps tick labels.
        let _ = theme;
        let gutter_bottom: f32 =
            if x_axis.name.is_some() { 44.0 } else { 32.0 };
        let gutter_right: f32 = 16.0;
        let gutter_top: f32 = if y_axis.name.is_some() { 22.0 } else { 8.0 };

        // Y ticks first: their label width sizes the left gutter.
        let plot_h = (rect.height() - gutter_top - gutter_bottom).max(1.0);
        let y_ticks = build_ticks(&y_scale, y_axis, plot_h, false);
        let widest = y_ticks
            .iter()
            .map(|t| estimate_label_width(&t.label))
            .fold(0.0_f32, f32::max);
        let gutter_left: f32 = (widest + 14.0)
            .max(56.0)
            .min((rect.width() * 0.35).max(56.0));

        let plot_rect = Rect::from_min_max(
            rect.min + vec2(gutter_left, gutter_top),
            rect.max - vec2(gutter_right, gutter_bottom),
        );

        let x_ticks = build_ticks(&x_scale, x_axis, plot_rect.width(), true);

        let plot_min = plot_rect.min;
        let plot_max = plot_rect.max;
        let x_scale_c = x_scale.clone();
        let y_scale_c = y_scale.clone();
        let x_inv = x_axis.inverse;
        let y_inv = y_axis.inverse;

        let to_screen = Box::new(move |p: DataPoint| {
            let tx = x_scale_c.normalize(p.x) as f32;
            let ty = y_scale_c.normalize(p.y) as f32;
            let tx = if x_inv { 1.0 - tx } else { tx };
            let ty = if y_inv { ty } else { 1.0 - ty };
            Pos2::new(
                plot_min.x + tx * (plot_max.x - plot_min.x),
                plot_min.y + ty * (plot_max.y - plot_min.y),
            )
        });

        let x_scale_c2 = x_scale.clone();
        let y_scale_c2 = y_scale.clone();
        let to_data = Box::new(move |pos: Pos2| {
            let mut tx = ((pos.x - plot_min.x)
                / (plot_max.x - plot_min.x).max(1e-6))
                as f64;
            let mut ty = ((pos.y - plot_min.y)
                / (plot_max.y - plot_min.y).max(1e-6))
                as f64;
            if x_inv {
                tx = 1.0 - tx;
            }
            if !y_inv {
                ty = 1.0 - ty;
            }
            DataPoint {
                x: x_scale_c2.unnormalize(tx),
                y: y_scale_c2.unnormalize(ty),
            }
        });

        let mut axes = Vec::new();

        // X axis layout (along bottom of plot_rect).
        let mut x_tick_layout = Vec::new();
        for t in &x_ticks {
            let nx = x_scale.normalize(t.value) as f32;
            let nx = if x_inv { 1.0 - nx } else { nx };
            let px = plot_min.x + nx * (plot_max.x - plot_min.x);
            x_tick_layout.push(AxisTick {
                pixel: px,
                value: t.value,
                label: t.label.clone(),
            });
        }
        axes.push(AxisLayout {
            is_x: true,
            line_start: Pos2::new(plot_min.x, plot_max.y),
            line_end: Pos2::new(plot_max.x, plot_max.y),
            ticks: x_tick_layout,
            name: x_axis.name.clone(),
        });

        // Y axis layout (along left of plot_rect).
        let mut y_tick_layout = Vec::new();
        for t in &y_ticks {
            let ny = y_scale.normalize(t.value) as f32;
            let ny = if y_inv { ny } else { 1.0 - ny };
            let py = plot_min.y + ny * (plot_max.y - plot_min.y);
            y_tick_layout.push(AxisTick {
                pixel: py,
                value: t.value,
                label: t.label.clone(),
            });
        }
        axes.push(AxisLayout {
            is_x: false,
            line_start: Pos2::new(plot_min.x, plot_min.y),
            line_end: Pos2::new(plot_min.x, plot_max.y),
            ticks: y_tick_layout,
            name: y_axis.name.clone(),
        });

        let (x_lo_px, x_hi_px) = if x_inv {
            (plot_max.x, plot_min.x)
        } else {
            (plot_min.x, plot_max.x)
        };
        let (y_lo_px, y_hi_px) = if y_inv {
            (plot_min.y, plot_max.y)
        } else {
            (plot_max.y, plot_min.y)
        };
        let scales = CartesianScales {
            x_map: AxisMap::new(&x_scale, x_lo_px, x_hi_px),
            y_map: AxisMap::new(&y_scale, y_lo_px, y_hi_px),
            x: x_scale,
            y: y_scale,
            plot_rect,
            outer_rect: rect,
        };

        CoordLayout::new(
            CoordKind::Cartesian2D,
            plot_rect,
            axes,
            to_screen,
            to_data,
        )
        .with_cartesian(scales)
    }
}

/// Rough pixel width of a tick label in the 12 px label font (digits and
/// punctuation average ~6.5 px). Only used to size gutters and tick density,
/// where no `Fonts` handle is available.
fn estimate_label_width(label: &str) -> f32 {
    label.chars().count() as f32 * 6.5
}

#[derive(Clone)]
struct RawTick {
    value: f64,
    label: String,
}

fn build_ticks(
    scale: &Scale,
    axis: &Axis,
    axis_pixels: f32,
    is_x: bool,
) -> Vec<RawTick> {
    if !axis.show_tick_labels && !axis.show_grid {
        return Vec::new();
    }
    match scale {
        Scale::Linear { min, max } => {
            let (min, max) = (min.min(*max), min.max(*max));
            let linear = |target: usize| -> Vec<RawTick> {
                let (lo, hi, step) = nice_range(min, max, target);
                // lo/hi are nice-rounded outward; keep only ticks inside the
                // visible range (an explicit or zoomed range is not "nice").
                let eps = step * 1e-6;
                ticks_linear(lo, hi, step)
                    .into_iter()
                    .filter(|v| *v >= min - eps && *v <= max + eps)
                    .map(|v| RawTick {
                        value: v,
                        label: format_tick(v, step),
                    })
                    .collect()
            };
            let target = ((axis_pixels / 64.0).round() as usize).clamp(2, 12);
            let ticks = linear(target);
            if !is_x || ticks.len() < 2 {
                return ticks;
            }
            // Long labels (zoomed time axis: "1000.125") need more room than
            // the 64 px default; re-space once so they cannot overlap.
            let widest = ticks
                .iter()
                .map(|t| estimate_label_width(&t.label))
                .fold(0.0_f32, f32::max);
            let per_tick = axis_pixels / ticks.len() as f32;
            if widest + 12.0 > per_tick {
                let target = ((axis_pixels / (widest + 16.0)).floor() as usize)
                    .clamp(2, 12);
                linear(target)
            } else {
                ticks
            }
        }
        Scale::Log { min, max } => {
            let spacing = if is_x { 56.0 } else { 36.0 };
            let max_ticks =
                ((axis_pixels / spacing).floor() as usize).clamp(2, 16);
            ticks_log_nice(*min, *max, max_ticks)
                .into_iter()
                .map(|v| RawTick {
                    value: v,
                    label: format_log_tick(v),
                })
                .collect()
        }
        Scale::Category { count } => {
            let count = *count;
            if count == 0 {
                return Vec::new();
            }
            (0..count)
                .map(|i| RawTick {
                    value: i as f64,
                    label: axis
                        .categories
                        .get(i)
                        .cloned()
                        .unwrap_or_else(|| i.to_string()),
                })
                .collect()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nice_range_picks_integer_steps() {
        let (lo, hi, step) = nice_range(0.0, 97.0, 5);
        assert!(step == 20.0 || step == 25.0, "got step={step}");
        assert!(lo <= 0.0 && hi >= 97.0);
    }

    #[test]
    fn linear_ticks_inclusive_endpoints() {
        let ticks = ticks_linear(0.0, 100.0, 20.0);
        assert_eq!(ticks, vec![0.0, 20.0, 40.0, 60.0, 80.0, 100.0]);
    }

    #[test]
    fn linear_normalize_inverse() {
        let s = Scale::Linear {
            min: -10.0,
            max: 30.0,
        };
        assert!((s.normalize(10.0) - 0.5).abs() < 1e-9);
        assert!((s.unnormalize(0.5) - 10.0).abs() < 1e-9);
    }

    #[test]
    fn log_tick_labels_are_plain_numbers() {
        let labels: Vec<String> =
            [0.001, 0.01, 0.1, 1.0, 10.0, 100.0, 1e3, 1e4, 1e5, 1e6, 2e-5]
                .iter()
                .map(|v| format_log_tick(*v))
                .collect();
        assert_eq!(
            labels,
            [
                "0.001", "0.01", "0.1", "1", "10", "100", "1k", "10k", "100k",
                "1e6", "2e-5"
            ]
        );
    }

    #[test]
    fn log_ticks_adapt_to_range() {
        // Many decades: decades only.
        assert_eq!(
            ticks_log_nice(1.0, 1e4, 10),
            vec![1.0, 10.0, 100.0, 1e3, 1e4]
        );
        // Too many decades for the space: thinned, still decades.
        let t = ticks_log_nice(1e-6, 1e6, 5);
        assert!(t.len() <= 5 && t.len() >= 2, "{t:?}");
        // Inside one decade: 1-2-5 or 1..9 multiples.
        let t = ticks_log_nice(15.0, 90.0, 10);
        assert!(
            t.len() >= 3 && t.iter().all(|v| (15.0..=90.0).contains(v)),
            "{t:?}"
        );
        // Deep zoom: still at least two ticks, inside the range.
        let t = ticks_log_nice(101.0, 102.0, 8);
        assert!(
            t.len() >= 2 && t.iter().all(|v| (101.0..=102.0).contains(v)),
            "{t:?}"
        );
    }

    #[test]
    fn zoomed_linear_labels_stay_distinct() {
        let (lo, hi, step) = nice_range(1000.1, 1000.2, 5);
        let labels: Vec<String> = ticks_linear(lo, hi, step)
            .into_iter()
            .map(|v| format_tick(v, step))
            .collect();
        let mut dedup = labels.clone();
        dedup.dedup();
        assert_eq!(labels, dedup, "{labels:?}");
        assert_eq!(format_tick(0.5, 0.1), "0.5");
        assert_eq!(format_tick(20.0, 5.0), "20");
        assert_eq!(format_tick(2000.0, 500.0), "2k");
        assert_eq!(format_tick(0.004, 0.002), "0.004");
    }

    #[test]
    fn explicit_bounds_clip_ticks_to_the_visible_range() {
        let axis = Axis::value();
        let ticks = build_ticks(
            &Scale::Linear { min: 0.3, max: 9.7 },
            &axis,
            400.0,
            false,
        );
        assert!(ticks.iter().all(|t| t.value >= 0.3 && t.value <= 9.7));
        assert!(ticks.len() >= 3);
    }

    #[test]
    fn axis_map_matches_normalize() {
        let s = Scale::Log { min: 1.0, max: 1e4 };
        let m = AxisMap::new(&s, 0.0, 400.0);
        assert!((m.map(100.0) - 200.0).abs() < 1e-9);
        assert!(m.map(0.0).is_nan());
        assert!((m.unmap(300.0) - 1000.0).abs() < 1e-6);
    }
}
