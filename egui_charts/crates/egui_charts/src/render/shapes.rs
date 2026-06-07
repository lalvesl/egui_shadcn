//! Reusable shape primitives most ECharts charts reduce to.
//!
//! Phase 0: symbols, smooth-path (Catmull-Rom → cubic bezier sampled), step-path.

use crate::option::SymbolKind;
use crate::render::painter::ChartPainter;
use egui::{Color32, Pos2, Stroke, StrokeKind, Vec2, vec2};

/// Draw a symbol centered at `pos`.
pub fn symbol(
    p: &ChartPainter,
    kind: SymbolKind,
    pos: Pos2,
    size: f32,
    color: Color32,
) {
    let half = size * 0.5;
    match kind {
        SymbolKind::Circle => p.circle_filled(pos, half, color),
        SymbolKind::Square => {
            let rect = egui::Rect::from_center_size(pos, Vec2::splat(size));
            p.rect_filled(rect, color);
        }
        SymbolKind::Diamond => {
            let pts = vec![
                Pos2::new(pos.x, pos.y - half),
                Pos2::new(pos.x + half, pos.y),
                Pos2::new(pos.x, pos.y + half),
                Pos2::new(pos.x - half, pos.y),
            ];
            p.poly(pts, color, Stroke::NONE);
        }
        SymbolKind::Triangle => {
            let pts = vec![
                Pos2::new(pos.x, pos.y - half),
                Pos2::new(pos.x + half, pos.y + half),
                Pos2::new(pos.x - half, pos.y + half),
            ];
            p.poly(pts, color, Stroke::NONE);
        }
        SymbolKind::Cross => {
            let s = Stroke::new(2.0, color);
            p.line(
                Pos2::new(pos.x - half, pos.y - half),
                Pos2::new(pos.x + half, pos.y + half),
                s,
            );
            p.line(
                Pos2::new(pos.x + half, pos.y - half),
                Pos2::new(pos.x - half, pos.y + half),
                s,
            );
        }
    }
}

/// Symbol outline variant — useful for hovered / unfilled markers.
pub fn symbol_outline(
    p: &ChartPainter,
    kind: SymbolKind,
    pos: Pos2,
    size: f32,
    stroke: Stroke,
    fill: Color32,
) {
    let half = size * 0.5;
    match kind {
        SymbolKind::Circle => {
            p.circle_filled(pos, half, fill);
            p.circle_stroke(pos, half, stroke);
        }
        SymbolKind::Square => {
            let rect = egui::Rect::from_center_size(pos, Vec2::splat(size));
            p.rect_filled(rect, fill);
            p.painter.rect_stroke(rect, 0.0, stroke, StrokeKind::Inside);
        }
        SymbolKind::Diamond => {
            let pts = vec![
                Pos2::new(pos.x, pos.y - half),
                Pos2::new(pos.x + half, pos.y),
                Pos2::new(pos.x, pos.y + half),
                Pos2::new(pos.x - half, pos.y),
            ];
            p.poly(pts, fill, stroke);
        }
        SymbolKind::Triangle => {
            let pts = vec![
                Pos2::new(pos.x, pos.y - half),
                Pos2::new(pos.x + half, pos.y + half),
                Pos2::new(pos.x - half, pos.y + half),
            ];
            p.poly(pts, fill, stroke);
        }
        SymbolKind::Cross => {
            p.line(
                Pos2::new(pos.x - half, pos.y - half),
                Pos2::new(pos.x + half, pos.y + half),
                stroke,
            );
            p.line(
                Pos2::new(pos.x + half, pos.y - half),
                Pos2::new(pos.x - half, pos.y + half),
                stroke,
            );
        }
    }
}

/// Sample a Catmull-Rom spline through `pts` into a dense polyline.
/// Resolution is points-per-segment (linear segments per source pair).
pub fn smooth_polyline(pts: &[Pos2], resolution: usize) -> Vec<Pos2> {
    if pts.len() < 2 {
        return pts.to_vec();
    }
    let n = pts.len();
    let mut out = Vec::with_capacity(n * resolution);

    for i in 0..n - 1 {
        let p0 = pts[if i == 0 { 0 } else { i - 1 }];
        let p1 = pts[i];
        let p2 = pts[i + 1];
        let p3 = pts[if i + 2 < n { i + 2 } else { n - 1 }];

        for s in 0..resolution {
            let t = s as f32 / resolution as f32;
            out.push(catmull_rom(p0, p1, p2, p3, t));
        }
    }
    out.push(pts[n - 1]);
    out
}

fn catmull_rom(p0: Pos2, p1: Pos2, p2: Pos2, p3: Pos2, t: f32) -> Pos2 {
    let t2 = t * t;
    let t3 = t2 * t;
    let f = |a: f32, b: f32, c: f32, d: f32| -> f32 {
        0.5 * ((2.0 * b)
            + (-a + c) * t
            + (2.0 * a - 5.0 * b + 4.0 * c - d) * t2
            + (-a + 3.0 * b - 3.0 * c + d) * t3)
    };
    Pos2::new(f(p0.x, p1.x, p2.x, p3.x), f(p0.y, p1.y, p2.y, p3.y))
}

/// Build a step-style polyline (horizontal-then-vertical) through `pts`.
pub fn step_polyline(pts: &[Pos2]) -> Vec<Pos2> {
    if pts.len() < 2 {
        return pts.to_vec();
    }
    let mut out = Vec::with_capacity(pts.len() * 2);
    out.push(pts[0]);
    for w in pts.windows(2) {
        let a = w[0];
        let b = w[1];
        let mid_x = (a.x + b.x) * 0.5;
        out.push(Pos2::new(mid_x, a.y));
        out.push(Pos2::new(mid_x, b.y));
        out.push(b);
    }
    out
}

/// Fill an area under a polyline down to a baseline y.
pub fn fill_under(
    p: &ChartPainter,
    line: &[Pos2],
    baseline_y: f32,
    fill: Color32,
) {
    if line.len() < 2 {
        return;
    }
    let mut shape = Vec::with_capacity(line.len() + 2);
    shape.extend_from_slice(line);
    shape.push(Pos2::new(line[line.len() - 1].x, baseline_y));
    shape.push(Pos2::new(line[0].x, baseline_y));
    // Filled area may be concave when line crosses baseline — split by sign.
    // For Phase 0 we feed it through convex_polygon when monotonic, otherwise
    // decompose into trapezoids per segment.
    if is_monotone_y_above_baseline(line, baseline_y) {
        p.poly(shape, fill, Stroke::NONE);
    } else {
        for w in line.windows(2) {
            let a = w[0];
            let b = w[1];
            let poly = vec![
                a,
                b,
                Pos2::new(b.x, baseline_y),
                Pos2::new(a.x, baseline_y),
            ];
            p.poly(poly, fill, Stroke::NONE);
        }
    }
}

fn is_monotone_y_above_baseline(line: &[Pos2], baseline_y: f32) -> bool {
    line.iter().all(|p| p.y <= baseline_y)
        || line.iter().all(|p| p.y >= baseline_y)
}

/// Annular sector (donut slice). Used by pie/doughnut/sunburst.
#[allow(clippy::too_many_arguments)]
pub fn annular_sector(
    p: &ChartPainter,
    center: Pos2,
    inner_r: f32,
    outer_r: f32,
    start_rad: f32,
    end_rad: f32,
    segments: usize,
    fill: Color32,
    stroke: Stroke,
) {
    let segments = segments.max(4);
    let mut pts = Vec::with_capacity(segments * 2 + 2);
    for i in 0..=segments {
        let t = i as f32 / segments as f32;
        let a = start_rad + (end_rad - start_rad) * t;
        pts.push(center + vec2(a.cos(), a.sin()) * outer_r);
    }
    for i in (0..=segments).rev() {
        let t = i as f32 / segments as f32;
        let a = start_rad + (end_rad - start_rad) * t;
        pts.push(center + vec2(a.cos(), a.sin()) * inner_r);
    }
    p.poly(pts, fill, stroke);
}
