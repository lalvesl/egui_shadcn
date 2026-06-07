//! Word cloud — words sized by weight, placed along an Archimedean spiral
//! with axis-aligned bounding-box collision detection.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::WordCloudSeries;
use crate::render::ChartPainter;
use egui::{Align2, FontId, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &WordCloudSeries,
    series_idx: usize,
    rect: Rect,
    theme: &crate::theme::ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.words.is_empty() {
        return None;
    }

    let (mut wmin, mut wmax) = (f64::INFINITY, f64::NEG_INFINITY);
    for (_, w) in &s.words {
        wmin = wmin.min(*w);
        wmax = wmax.max(*w);
    }
    let range = (wmax - wmin).max(1e-12);
    let mut sorted: Vec<(usize, &(String, f64))> =
        s.words.iter().enumerate().collect();
    sorted.sort_by(|a, b| {
        b.1.1
            .partial_cmp(&a.1.1)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    let center = rect.center();
    let mut placed: Vec<Rect> = Vec::with_capacity(sorted.len());
    let mut positions: Vec<(Rect, usize)> = Vec::with_capacity(sorted.len());

    for (rank, (orig_idx, (text, w))) in sorted.iter().enumerate() {
        let t = ((*w - wmin) / range).clamp(0.0, 1.0) as f32;
        let size = s.min_size + (s.max_size - s.min_size) * t;
        let font = FontId::proportional(size);
        let layout = p.painter.ctx().fonts_mut(|f| {
            f.layout_no_wrap(text.clone(), font.clone(), theme.text)
        });
        let half = layout.size() * 0.5;

        // Spiral search.
        let mut placed_rect = None;
        let max_steps = 600;
        for step in 0..max_steps {
            let angle = step as f32 * 0.35;
            let radius = step as f32 * 0.55;
            let cx = center.x + angle.cos() * radius;
            let cy = center.y + angle.sin() * radius;
            let candidate =
                Rect::from_center_size(Pos2::new(cx, cy), layout.size());
            if !rect.contains(candidate.min) || !rect.contains(candidate.max) {
                continue;
            }
            let overlap = placed.iter().any(|r| r.intersects(candidate));
            if !overlap {
                placed_rect = Some(candidate);
                break;
            }
        }
        let Some(r) = placed_rect else {
            continue;
        };

        let color = theme.series_color(palette_offset + rank);
        p.painter.text(
            r.center(),
            Align2::CENTER_CENTER,
            text.clone(),
            font,
            color,
        );
        placed.push(r);
        positions.push((r, *orig_idx));

        let _ = half;
    }

    if let Some(h) = hover_pos {
        for (r, orig_idx) in &positions {
            if r.contains(h) {
                let color = theme.series_color(palette_offset + *orig_idx);
                let _ = Stroke::NONE;
                return Some(TooltipDatum {
                    series_index: series_idx,
                    series_name: s.words[*orig_idx].0.clone(),
                    data_index: *orig_idx,
                    value: s.words[*orig_idx].1,
                    color,
                    screen_pos: Some(r.center()),
                });
            }
        }
    }
    None
}
