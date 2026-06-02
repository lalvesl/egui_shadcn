//! Custom series — escape hatch. Calls the user-supplied closure with the
//! chart painter, cell rect, theme, and hover position.

use crate::interaction::tooltip::TooltipDatum;
use crate::option::CustomSeries;
use crate::render::ChartPainter;
use crate::theme::ChartTheme;
use egui::{Pos2, Rect};

pub fn render(
    p: &ChartPainter,
    s: &CustomSeries,
    _series_idx: usize,
    rect: Rect,
    theme: &ChartTheme,
    _palette_offset: usize,
    hover_pos: Option<Pos2>,
) -> Option<TooltipDatum> {
    (s.render)(p, rect, theme, hover_pos);
    None
}
