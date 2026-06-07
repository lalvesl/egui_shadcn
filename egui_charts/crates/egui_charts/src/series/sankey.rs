//! Sankey diagram. Computes a layer index per node by longest path from
//! sources, lays nodes top-to-bottom proportional to bandwidth, then draws
//! cubic-bezier flow ribbons between source/target strips.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::{SankeyLink, SankeySeries};
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &SankeySeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    let n = s.nodes.len();
    if n == 0 || s.links.is_empty() {
        return None;
    }

    // Per-node layer = longest path from a source.
    let mut layer = vec![0_usize; n];
    let mut changed = true;
    let mut guard = 0;
    while changed && guard < n + 2 {
        changed = false;
        for l in &s.links {
            let want = layer[l.source] + 1;
            if want > layer[l.target] {
                layer[l.target] = want;
                changed = true;
            }
        }
        guard += 1;
    }
    let max_layer = *layer.iter().max().unwrap_or(&0);

    // Node bandwidth = max(in_sum, out_sum).
    let mut in_sum = vec![0.0_f64; n];
    let mut out_sum = vec![0.0_f64; n];
    for l in &s.links {
        out_sum[l.source] += l.value;
        in_sum[l.target] += l.value;
    }
    let node_val: Vec<f64> = (0..n)
        .map(|i| in_sum[i].max(out_sum[i]).max(1e-12))
        .collect();

    // Group nodes by layer.
    let mut layers: Vec<Vec<usize>> = vec![Vec::new(); max_layer + 1];
    for (i, &l) in layer.iter().enumerate() {
        layers[l].push(i);
    }

    // Vertical scale: max layer total drives px per unit.
    let max_layer_total = layers
        .iter()
        .map(|nodes| {
            let total: f64 = nodes.iter().map(|i| node_val[*i]).sum();
            total
        })
        .fold(0.0_f64, f64::max)
        .max(1e-12);
    let inner = rect.shrink(8.0);
    let layer_count = max_layer + 1;
    let layer_x: Vec<f32> = (0..layer_count)
        .map(|i| {
            inner.min.x
                + s.node_width * 0.5
                + (inner.width() - s.node_width)
                    * (i as f32 / (layer_count - 1).max(1) as f32)
        })
        .collect();

    let usable_h = inner.height();
    let scale: f32 = ((usable_h
        - layers
            .iter()
            .map(|l| (l.len().saturating_sub(1)) as f32)
            .fold(0.0_f32, f32::max)
            * s.node_gap)
        / max_layer_total as f32)
        .max(1.0);

    // Compute each node's (y_top, y_bottom).
    let mut node_rect = vec![Rect::NOTHING; n];
    for (li, nodes) in layers.iter().enumerate() {
        let total: f32 = nodes
            .iter()
            .map(|i| node_val[*i] as f32 * scale)
            .sum::<f32>()
            + nodes.len().saturating_sub(1) as f32 * s.node_gap;
        let mut y = inner.min.y + (usable_h - total) * 0.5;
        for &i in nodes {
            let h = node_val[i] as f32 * scale;
            let x = layer_x[li];
            let r = Rect::from_min_max(
                Pos2::new(x - s.node_width * 0.5, y),
                Pos2::new(x + s.node_width * 0.5, y + h),
            );
            node_rect[i] = r;
            y += h + s.node_gap;
        }
    }

    // Per-node strip usage trackers.
    let mut source_used = vec![0.0_f32; n];
    let mut target_used = vec![0.0_f32; n];
    let mut tip: Option<TooltipDatum> = None;

    // Draw flows first so they sit under the node bars.
    let mut links_sorted: Vec<&SankeyLink> = s.links.iter().collect();
    links_sorted.sort_by(|a, b| {
        b.value
            .partial_cmp(&a.value)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    for (li, link) in links_sorted.iter().enumerate() {
        let src = node_rect[link.source];
        let tgt = node_rect[link.target];
        let h = link.value as f32 * scale;
        let src_y0 = src.min.y + source_used[link.source];
        let src_y1 = src_y0 + h;
        let tgt_y0 = tgt.min.y + target_used[link.target];
        let tgt_y1 = tgt_y0 + h;
        source_used[link.source] += h;
        target_used[link.target] += h;

        let color = theme.series_color(palette_offset + li);
        let fill = Color32::from_rgba_unmultiplied(
            color.r(),
            color.g(),
            color.b(),
            110,
        );

        draw_ribbon(
            p,
            Pos2::new(src.max.x, src_y0),
            Pos2::new(src.max.x, src_y1),
            Pos2::new(tgt.min.x, tgt_y0),
            Pos2::new(tgt.min.x, tgt_y1),
            fill,
        );

        if let Some(h_pos) = hover_pos
            && ribbon_contains(
                h_pos,
                Pos2::new(src.max.x, src_y0),
                Pos2::new(src.max.x, src_y1),
                Pos2::new(tgt.min.x, tgt_y0),
                Pos2::new(tgt.min.x, tgt_y1),
            )
        {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: format!(
                    "{} → {}",
                    s.nodes[link.source].name, s.nodes[link.target].name
                ),
                data_index: li,
                value: link.value,
                color,
                screen_pos: None,
            });
        }
    }

    // Draw node bars.
    let font = label_font();
    for (i, r) in node_rect.iter().enumerate() {
        let color = theme.series_color(palette_offset + i);
        let hovered = hover_pos.map(|h| r.contains(h)).unwrap_or(false);
        let fill = if hovered {
            color
        } else {
            Color32::from_rgba_unmultiplied(
                color.r(),
                color.g(),
                color.b(),
                230,
            )
        };
        p.rect_filled(*r, fill);

        // Label outside the bar — right for nodes in the first half, left for back half.
        let on_right = layer[i] < layer_count / 2 || layer_count == 1;
        let (lp, anchor) = if on_right {
            (Pos2::new(r.max.x + 6.0, r.center().y), Align2::LEFT_CENTER)
        } else {
            (Pos2::new(r.min.x - 6.0, r.center().y), Align2::RIGHT_CENTER)
        };
        p.text(
            lp,
            anchor,
            s.nodes[i].name.clone(),
            font.clone(),
            theme.text,
        );

        if hovered {
            tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: s.nodes[i].name.clone(),
                data_index: i,
                value: node_val[i],
                color,
                screen_pos: Some(r.center()),
            });
        }
    }

    tip
}

/// Cubic-bezier ribbon between two vertical strips on left and right anchors.
fn draw_ribbon(
    p: &ChartPainter,
    sa: Pos2,
    sb: Pos2,
    ta: Pos2,
    tb: Pos2,
    fill: Color32,
) {
    // Build a closed polygon along the top edge then back along the bottom edge.
    let mid_x_a = (sa.x + ta.x) * 0.5;
    let mut pts: Vec<Pos2> = Vec::with_capacity(64);
    let n = 24;
    // Top edge (sa → ta).
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let x = cubic(sa.x, mid_x_a, mid_x_a, ta.x, t);
        let y = cubic(sa.y, sa.y, ta.y, ta.y, t);
        pts.push(Pos2::new(x, y));
    }
    // Bottom edge in reverse (tb → sb).
    for i in 0..=n {
        let t = i as f32 / n as f32;
        let x = cubic(tb.x, mid_x_a, mid_x_a, sb.x, t);
        let y = cubic(tb.y, tb.y, sb.y, sb.y, t);
        pts.push(Pos2::new(x, y));
    }
    // Decompose into quads for safety (ribbon is monotone in x).
    let half = n + 1;
    for i in 0..(half - 1) {
        let poly = vec![
            pts[i],
            pts[i + 1],
            pts[2 * half - i - 2],
            pts[2 * half - i - 1],
        ];
        p.poly(poly, fill, Stroke::NONE);
    }
}

fn cubic(a: f32, b: f32, c: f32, d: f32, t: f32) -> f32 {
    let mt = 1.0 - t;
    mt * mt * mt * a
        + 3.0 * mt * mt * t * b
        + 3.0 * mt * t * t * c
        + t * t * t * d
}

fn ribbon_contains(p: Pos2, sa: Pos2, sb: Pos2, ta: Pos2, tb: Pos2) -> bool {
    if p.x < sa.x.min(ta.x) || p.x > sa.x.max(ta.x) {
        return false;
    }
    let dx_total = (ta.x - sa.x).abs().max(1e-3);
    let t = ((p.x - sa.x.min(ta.x)) / dx_total).clamp(0.0, 1.0);
    let top = cubic(sa.y, sa.y, ta.y, ta.y, t);
    let bot = cubic(sb.y, sb.y, tb.y, tb.y, t);
    p.y >= top.min(bot) && p.y <= top.max(bot)
}
