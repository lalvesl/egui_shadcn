//! Cartesian (rectangular) coordinate system — scales, nice-number ticks,
//! axis/grid render, data↔screen transforms.

use super::{AxisLayout, AxisTick, CoordKind, CoordLayout, DataPoint};
use crate::option::{Axis, AxisKind, Chart, Series};
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

/// Format a tick value compactly. Picks fixed/short/exp form by magnitude.
pub fn format_tick(v: f64, step: f64) -> String {
    if v == 0.0 {
        return "0".to_string();
    }
    let abs = v.abs();
    if abs >= 1e6 {
        format!("{:.1}M", v / 1e6)
    } else if abs >= 1e3 {
        let s = v / 1e3;
        if (s - s.round()).abs() < 1e-6 {
            format!("{:.0}k", s)
        } else {
            format!("{:.1}k", s)
        }
    } else if abs < 0.01 {
        format!("{:.2e}", v)
    } else {
        let decimals = if step >= 1.0 {
            0
        } else if step >= 0.1 {
            1
        } else if step >= 0.01 {
            2
        } else {
            3
        };
        format!("{:.*}", decimals, v)
    }
}

// ── Auto-fit data range from series ──────────────────────────────────────────

fn series_value_extent(chart: &Chart) -> (f64, f64) {
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

    if !any {
        (0.0, 1.0)
    } else if (max - min).abs() < f64::EPSILON {
        (min - 1.0, max + 1.0)
    } else {
        (min, max)
    }
}

fn series_x_extent(chart: &Chart) -> (f64, f64) {
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
    if !any {
        (0.0, 1.0)
    } else if (max - min).abs() < f64::EPSILON {
        (min - 1.0, max + 1.0)
    } else {
        (min, max)
    }
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
        let default_x_axis;
        let default_y_axis;
        let x_axis = if let Some(a) = self.chart.x_axis.as_ref() {
            a
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

        let cat_count = max_series_len(self.chart);
        let (data_min_x, data_max_x) = series_x_extent(self.chart);
        let (data_min_y, data_max_y) = series_value_extent(self.chart);

        let x_scale = build_scale(x_axis, data_min_x, data_max_x, cat_count);
        let y_scale = build_scale(y_axis, data_min_y, data_max_y, cat_count);

        // Reserve gutter for tick labels + axis name.
        let _ = theme;
        let gutter_left: f32 = 56.0;
        let gutter_bottom: f32 = 32.0;
        let gutter_right: f32 = 16.0;
        let gutter_top: f32 = 8.0;

        let plot_rect = Rect::from_min_max(
            rect.min + vec2(gutter_left, gutter_top),
            rect.max - vec2(gutter_right, gutter_bottom),
        );

        // Axis ticks.
        let x_ticks = build_ticks(&x_scale, x_axis, plot_rect.width(), true);
        let y_ticks = build_ticks(&y_scale, y_axis, plot_rect.height(), false);

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

        CoordLayout::new(
            CoordKind::Cartesian2D,
            plot_rect,
            axes,
            to_screen,
            to_data,
        )
    }
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
    _is_x: bool,
) -> Vec<RawTick> {
    if !axis.show_tick_labels && !axis.show_grid {
        return Vec::new();
    }
    match scale {
        Scale::Linear { min, max } => {
            let target = ((axis_pixels / 64.0).round() as usize).clamp(2, 12);
            let (lo, hi, step) = nice_range(*min, *max, target);
            // If the axis was explicitly bounded, keep [min,max] as the visible range
            // but step from the nice grid.
            // lo/hi are already the nice-rounded bounds from nice_range.
            ticks_linear(lo, hi, step)
                .into_iter()
                .map(|v| RawTick {
                    value: v,
                    label: format_tick(v, step),
                })
                .collect()
        }
        Scale::Log { min, max } => ticks_log(*min, *max)
            .into_iter()
            .map(|v| RawTick {
                value: v,
                label: if (1.0..1e6).contains(&v) {
                    format!("{}", v as i64)
                } else {
                    format!("{:.0e}", v)
                },
            })
            .collect(),
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
}
