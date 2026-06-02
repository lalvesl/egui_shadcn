//! Treemap — slice-and-dice layout. Children alternate between vertical and
//! horizontal splits at successive depths.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::{TreemapNode, TreemapSeries};
use crate::render::ChartPainter;
use crate::render::text::label_font;
use crate::theme::ChartTheme;
use egui::{Align2, Color32, Pos2, Rect, Stroke};

pub fn render(
    p: &ChartPainter,
    s: &TreemapSeries,
    series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    if s.roots.is_empty() {
        return None;
    }

    let font = label_font();
    let mut tip: Option<TooltipDatum> = None;
    let total: f64 = s.roots.iter().map(|n| n.subtree_value()).sum::<f64>().max(1e-12);

    // Top-level slice-and-dice.
    let mut x = rect.min.x;
    let mut y = rect.min.y;
    let vertical = rect.width() >= rect.height();
    for (ri, root) in s.roots.iter().enumerate() {
        let frac = (root.subtree_value() / total) as f32;
        let tile = if vertical {
            let w = rect.width() * frac;
            let r = Rect::from_min_max(
                Pos2::new(x, rect.min.y),
                Pos2::new(x + w, rect.max.y),
            );
            x += w;
            r
        } else {
            let h = rect.height() * frac;
            let r = Rect::from_min_max(
                Pos2::new(rect.min.x, y),
                Pos2::new(rect.max.x, y + h),
            );
            y += h;
            r
        };
        draw_node(
            p,
            root,
            tile,
            0,
            theme,
            palette_offset + ri * 5,
            series_idx,
            hover_pos,
            &font,
            &mut tip,
            !vertical,
        );
    }

    tip
}

#[allow(clippy::too_many_arguments)]
fn draw_node(
    p: &ChartPainter,
    node: &TreemapNode,
    rect: Rect,
    depth: usize,
    theme: &ChartTheme,
    palette_offset: usize,
    series_idx: usize,
    hover_pos: Option<Pos2>,
    font: &egui::FontId,
    tip: &mut Option<TooltipDatum>,
    vertical_split: bool,
) {
    if rect.width() < 2.0 || rect.height() < 2.0 {
        return;
    }
    let color = theme.series_color(palette_offset + depth);
    let hovered = hover_pos.map(|h| rect.contains(h)).unwrap_or(false);

    // Only draw a tile for leaves; parents are just containers (their colored
    // border highlights group boundaries).
    if node.children.is_empty() {
        let fill = if hovered {
            color
        } else if hover_pos.is_some() {
            Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), 220)
        } else {
            color
        };
        p.rect_filled(rect, fill);
        if hovered {
            *tip = Some(TooltipDatum {
                series_index: series_idx,
                series_name: node.name.clone(),
                data_index: depth,
                value: node.value,
                color,
                screen_pos: Some(rect.center()),
            });
        }
        if rect.width() > 28.0 && rect.height() > 14.0 {
            p.text(
                rect.center(),
                Align2::CENTER_CENTER,
                node.name.clone(),
                font.clone(),
                theme.background,
            );
        }
    } else {
        // Layout children into this rect.
        let inner = rect.shrink(1.0);
        let total: f64 = node.children.iter().map(|c| c.subtree_value()).sum::<f64>().max(1e-12);
        let mut cursor_x = inner.min.x;
        let mut cursor_y = inner.min.y;
        for child in &node.children {
            let frac = (child.subtree_value() / total) as f32;
            let tile = if vertical_split {
                let w = inner.width() * frac;
                let r = Rect::from_min_max(
                    Pos2::new(cursor_x, inner.min.y),
                    Pos2::new(cursor_x + w, inner.max.y),
                );
                cursor_x += w;
                r
            } else {
                let h = inner.height() * frac;
                let r = Rect::from_min_max(
                    Pos2::new(inner.min.x, cursor_y),
                    Pos2::new(inner.max.x, cursor_y + h),
                );
                cursor_y += h;
                r
            };
            draw_node(
                p,
                child,
                tile,
                depth + 1,
                theme,
                palette_offset + 1,
                series_idx,
                hover_pos,
                font,
                tip,
                !vertical_split,
            );
        }
        // Parent group border so nested structure is visible.
        p.painter
            .rect_stroke(rect, 0.0, Stroke::new(1.5, theme.background), egui::StrokeKind::Inside);
    }
}
