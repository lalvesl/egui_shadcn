//! XY line series — numeric `(x, y)` pairs on value / log axes, with NaN gaps,
//! visible-range culling and min/max-per-pixel-column decimation.

use crate::coord::cartesian::CartesianScales;
use crate::coord::{CoordLayout, DataPoint};
use crate::interaction::tooltip::TooltipDatum;
use crate::option::{DashStyle, XyLineSeries};
use crate::render::ChartPainter;
use crate::render::shapes::{symbol, symbol_outline};
use crate::theme::ChartTheme;
use egui::{Color32, Painter, Pos2, Shape, Stroke};
use std::ops::Range;

/// Decimate only when the visible points outnumber pixel columns by this.
const DECIMATE_RATIO: usize = 2;
/// Never draw per-point markers for more visible points than this.
const MAX_MARKERS: usize = 4_000;
/// Screen coordinates are clamped to this far outside the plot, so a point
/// far off-screen after a deep zoom stays a finite `f32`.
const CLAMP_PX: f64 = 1.0e5;

/// Index range of `data` (sorted by x) worth drawing for the visible x-range
/// `[lo, hi]`: every point inside plus one neighbour on each side, so the line
/// runs to the plot edge instead of stopping at the last inside point.
pub fn visible_range(data: &[[f64; 2]], lo: f64, hi: f64) -> Range<usize> {
    let a = data.partition_point(|p| p[0] < lo);
    let b = data.partition_point(|p| p[0] <= hi);
    a.saturating_sub(1)..(b + 1).min(data.len())
}

/// Min/max-per-column decimation over `data[range]`.
///
/// `column_of(x)` maps an x value to its pixel column (`None` when the x is
/// not drawable, e.g. `<= 0` on a log axis). `data` must be sorted by x, so
/// every column is one contiguous run. For every column the first, last,
/// minimum-y and maximum-y points are kept, in index order — a one-sample
/// spike therefore always survives. Points with a non-finite y (or an
/// undrawable x) are gaps: one index per run of them is kept so the caller
/// still breaks the line there.
///
/// Returns ascending indices into `data`.
pub fn decimate_minmax(
    data: &[[f64; 2]],
    range: Range<usize>,
    column_of: impl Fn(f64) -> Option<i64>,
) -> Vec<usize> {
    let end = range.end.min(data.len());
    let mut out = Vec::new();
    let mut in_gap = false;
    let mut i = range.start.min(end);
    while i < end {
        let [x, y] = data[i];
        let col = if y.is_finite() { column_of(x) } else { None };
        let Some(col) = col else {
            if !in_gap {
                out.push(i);
                in_gap = true;
            }
            i += 1;
            continue;
        };
        in_gap = false;

        // End of this column's run: gallop, then bisect. Columns are
        // contiguous in sorted data, so this costs O(log run) mappings
        // instead of one per point.
        let same = |k: usize| column_of(data[k][0]) == Some(col);
        let (mut lo, mut hi) = (i, end);
        let mut stride = 1;
        while lo + stride < end {
            if same(lo + stride) {
                lo += stride;
                stride *= 2;
            } else {
                hi = lo + stride;
                break;
            }
        }
        while hi - lo > 1 {
            let mid = lo + (hi - lo) / 2;
            if same(mid) {
                lo = mid;
            } else {
                hi = mid;
            }
        }
        let run_end = hi;

        // Tight y-only scan; a NaN ends this bucket (the gap is handled by
        // the next outer iteration, and the column resumes after it).
        let (mut min_i, mut max_i) = (i, i);
        let (mut mn, mut mx) = (y, y);
        let mut j = i + 1;
        while j < run_end {
            let yj = data[j][1];
            if !yj.is_finite() {
                break;
            }
            if yj < mn {
                mn = yj;
                min_i = j;
            } else if yj > mx {
                mx = yj;
                max_i = j;
            }
            j += 1;
        }
        let mut k = [i, min_i, max_i, j - 1];
        k.sort_unstable();
        let mut prev = usize::MAX;
        for idx in k {
            if idx != prev {
                out.push(idx);
                prev = idx;
            }
        }
        i = j;
    }
    out
}

#[inline]
fn project(sc: &CartesianScales, x: f64, y: f64) -> Option<Pos2> {
    if !y.is_finite() {
        return None;
    }
    let sx = sc.x_map.map(x);
    let sy = sc.y_map.map(y);
    if !sx.is_finite() || !sy.is_finite() {
        return None;
    }
    let r = sc.plot_rect;
    let sx = sx.clamp(r.min.x as f64 - CLAMP_PX, r.max.x as f64 + CLAMP_PX);
    let sy = sy.clamp(r.min.y as f64 - CLAMP_PX, r.max.y as f64 + CLAMP_PX);
    Some(Pos2::new(sx as f32, sy as f32))
}

fn stroke_run(
    painter: &Painter,
    run: &[Pos2],
    s: &XyLineSeries,
    color: Color32,
) {
    let stroke = Stroke::new(s.line_width, color);
    match run.len() {
        0 => {}
        // An isolated sample between two gaps would otherwise be invisible.
        1 => {
            painter.circle_filled(
                run[0],
                (s.line_width * 0.75).max(1.0),
                color,
            );
        }
        _ => match s.dash {
            DashStyle::Solid => {
                painter.add(Shape::line(run.to_vec(), stroke));
            }
            DashStyle::Dashed => {
                let w = s.line_width.max(1.0);
                painter.extend(Shape::dashed_line(
                    run,
                    stroke,
                    6.0 * w,
                    4.0 * w,
                ));
            }
            DashStyle::Dotted => {
                let w = s.line_width.max(1.0);
                painter.extend(Shape::dotted_line(
                    run,
                    color,
                    2.5 * w + 1.0,
                    0.6 * w,
                ));
            }
        },
    }
}

#[allow(clippy::too_many_arguments)]
pub fn render(
    p: &ChartPainter,
    s: &XyLineSeries,
    series_idx: usize,
    color: Color32,
    layout: &CoordLayout,
    theme: &ChartTheme,
    hover: Option<DataPoint>,
) -> Option<TooltipDatum> {
    let data = s.data.as_slice();
    if data.is_empty() {
        return None;
    }
    let sc = layout.cartesian()?;
    let plot = sc.plot_rect;
    let painter = p
        .painter
        .with_clip_rect(plot.intersect(p.painter.clip_rect()));
    let ppp = p.painter.ctx().pixels_per_point().max(0.1) as f64;

    let sorted = s.data.is_sorted();
    let range = if sorted {
        let (lo, hi) = sc.x.data_extent();
        visible_range(data, lo, hi)
    } else {
        0..data.len()
    };
    let visible = range.len();
    let columns = ((plot.width() as f64 * ppp).ceil() as usize).max(1);
    let decimated = sorted && visible > columns * DECIMATE_RATIO;

    // Collect the screen polyline, split at gaps.
    let mut run: Vec<Pos2> = Vec::with_capacity(visible.min(columns * 4 + 8));
    let mut marker_pts: Vec<Pos2> = Vec::new();
    let want_markers =
        s.marker.is_some() && !decimated && visible <= MAX_MARKERS;
    let mut emit = |i: usize, run: &mut Vec<Pos2>| {
        let [x, y] = data[i];
        match project(sc, x, y) {
            Some(pt) => {
                run.push(pt);
                if want_markers {
                    marker_pts.push(pt);
                }
            }
            None => {
                stroke_run(&painter, run, s, color);
                run.clear();
            }
        }
    };
    if decimated {
        let x_map = sc.x_map;
        let x0 = plot.min.x as f64;
        let idx = decimate_minmax(data, range.clone(), |x| {
            let px = x_map.map(x);
            px.is_finite().then(|| ((px - x0) * ppp).floor() as i64)
        });
        for i in idx {
            emit(i, &mut run);
        }
    } else {
        for i in range.clone() {
            emit(i, &mut run);
        }
    }
    stroke_run(&painter, &run, s, color);

    if let Some(kind) = s.marker {
        for pt in &marker_pts {
            symbol(
                &ChartPainter::new(&painter, plot),
                kind,
                *pt,
                s.marker_size,
                color,
            );
        }
    }

    // Tooltip / crosshair: nearest point by x.
    let h = hover?;
    let hx = sc.x_map.map(h.x);
    if !hx.is_finite() {
        return None;
    }
    let nearest = if sorted {
        let k = data.partition_point(|p| p[0] < h.x);
        [k.checked_sub(1), (k < data.len()).then_some(k)]
            .into_iter()
            .flatten()
            .min_by(|&a, &b| {
                let da = (sc.x_map.map(data[a][0]) - hx).abs();
                let db = (sc.x_map.map(data[b][0]) - hx).abs();
                da.total_cmp(&db)
            })
    } else {
        range
            .clone()
            .filter(|&i| sc.x_map.map(data[i][0]).is_finite())
            .min_by(|&a, &b| {
                let da = (sc.x_map.map(data[a][0]) - hx).abs();
                let db = (sc.x_map.map(data[b][0]) - hx).abs();
                da.total_cmp(&db)
            })
    }?;
    let [nx, ny] = data[nearest];
    // Inside a NaN gap there is no value to report.
    let pt = project(sc, nx, ny)?;
    if !plot.expand(1.0).contains(pt) {
        return None;
    }
    symbol_outline(
        &ChartPainter::new(&painter, plot),
        s.marker.unwrap_or(crate::option::SymbolKind::Circle),
        pt,
        (s.marker_size + 3.0).max(7.0),
        Stroke::new(1.5, color),
        theme.surface,
    );
    Some(TooltipDatum {
        series_index: series_idx,
        series_name: s.name.clone(),
        data_index: nearest,
        value: ny,
        color,
        screen_pos: Some(pt),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn col_unit(x: f64) -> Option<i64> {
        Some(x.floor() as i64)
    }

    #[test]
    fn visible_range_includes_neighbours() {
        let d: Vec<[f64; 2]> = (0..10).map(|i| [i as f64, 0.0]).collect();
        assert_eq!(visible_range(&d, 3.5, 6.5), 3..8);
        assert_eq!(visible_range(&d, -5.0, 100.0), 0..10);
        assert_eq!(visible_range(&d, 20.0, 30.0), 9..10);
    }

    #[test]
    fn decimation_preserves_extrema_per_column() {
        // 1000 points over 10 columns, one spike up and one down.
        let mut d: Vec<[f64; 2]> = (0..1000)
            .map(|i| [i as f64 / 100.0, (i as f64 * 0.37).sin()])
            .collect();
        d[421][1] = 50.0;
        d[777][1] = -80.0;
        let idx = decimate_minmax(&d, 0..d.len(), col_unit);
        assert!(idx.len() <= 10 * 4, "kept {}", idx.len());
        assert!(idx.windows(2).all(|w| w[0] < w[1]), "ascending");
        assert!(idx.contains(&421) && idx.contains(&777));
        assert!(idx.contains(&0) && idx.contains(&999));
        // Every column's true min and max survive.
        for c in 0..10 {
            let in_col: Vec<usize> =
                (0..1000).filter(|&i| d[i][0].floor() as i64 == c).collect();
            let kept: Vec<usize> =
                idx.iter().copied().filter(|i| in_col.contains(i)).collect();
            let true_max =
                in_col.iter().map(|&i| d[i][1]).fold(f64::MIN, f64::max);
            let true_min =
                in_col.iter().map(|&i| d[i][1]).fold(f64::MAX, f64::min);
            let kmax = kept.iter().map(|&i| d[i][1]).fold(f64::MIN, f64::max);
            let kmin = kept.iter().map(|&i| d[i][1]).fold(f64::MAX, f64::min);
            assert_eq!(true_max, kmax);
            assert_eq!(true_min, kmin);
        }
    }

    #[test]
    fn decimation_keeps_gaps() {
        let mut d: Vec<[f64; 2]> =
            (0..100).map(|i| [i as f64 / 10.0, 1.0]).collect();
        for p in d.iter_mut().take(60).skip(40) {
            p[1] = f64::NAN;
        }
        let idx = decimate_minmax(&d, 0..d.len(), col_unit);
        let nan_kept: Vec<usize> =
            idx.iter().copied().filter(|&i| d[i][1].is_nan()).collect();
        assert_eq!(nan_kept, vec![40], "one marker per NaN run");
    }

    #[test]
    fn decimation_treats_undrawable_x_as_gap() {
        let d: Vec<[f64; 2]> = (-5..5).map(|i| [i as f64, 1.0]).collect();
        let idx =
            decimate_minmax(&d, 0..d.len(), |x| (x > 0.0).then_some(x as i64));
        // one gap marker (index 0) + four drawable columns (x = 1..4)
        assert_eq!(idx, vec![0, 6, 7, 8, 9]);
    }
}
