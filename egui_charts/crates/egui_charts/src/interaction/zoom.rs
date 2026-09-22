//! Mouse zoom / pan for cartesian charts (opt-in via
//! [`ChartWidget::interactive`](crate::ChartWidget::interactive)).
//!
//! | Input                     | Effect                                   |
//! | ------------------------- | ---------------------------------------- |
//! | wheel                     | zoom both axes around the cursor         |
//! | Shift + wheel             | zoom x only                              |
//! | Ctrl / Cmd + wheel        | zoom y only                              |
//! | pinch (trackpad / touch)  | zoom both axes                           |
//! | primary drag              | pan                                      |
//! | double-click              | reset to axis bounds / auto-fit          |
//!
//! Log axes zoom and pan in log10 space. Category axes never zoom.

use crate::coord::CoordLayout;
use crate::view::{ChartView, pan_range, zoom_range};
use egui::{Event, MouseWheelUnit, PointerButton, Response, Ui, Vec2};

/// Wheel points per e-fold of zoom: one 40-point notch ≈ 10 %.
const WHEEL_ZOOM_SPEED: f64 = 1.0 / 400.0;

/// Apply this frame's pointer input to `view`. Returns `true` when the view
/// changed (the caller then lays the chart out again).
pub fn handle(
    ui: &Ui,
    response: &Response,
    layout: &CoordLayout,
    view: &mut ChartView,
) -> bool {
    let Some(sc) = layout.cartesian() else {
        return false;
    };
    let plot = layout.plot_rect;
    let x_range = sc.x.range();
    let y_range = sc.y.range();
    if x_range.is_none() && y_range.is_none() {
        return false;
    }
    let before = *view;

    let hover = response.hover_pos().filter(|p| plot.contains(*p));

    // ── Double-click: reset ────────────────────────────────────────────────
    if response.double_clicked() && hover.is_some() {
        view.x = None;
        view.y = None;
        return *view != before;
    }

    // ── Wheel / pinch zoom ─────────────────────────────────────────────────
    if let Some(cursor) = hover {
        let (mut zx, mut zy) = (0.0_f64, 0.0_f64); // log-factors
        let mut pinch = 1.0_f64;
        ui.input(|i| {
            for ev in &i.events {
                match ev {
                    Event::MouseWheel {
                        unit,
                        delta,
                        modifiers,
                        ..
                    } => {
                        let px: Vec2 = match unit {
                            MouseWheelUnit::Point => *delta,
                            MouseWheelUnit::Line => *delta * 40.0,
                            MouseWheelUnit::Page => *delta * plot.height(),
                        };
                        let d =
                            if px.y.abs() >= px.x.abs() { px.y } else { px.x };
                        let f = -(d as f64) * WHEEL_ZOOM_SPEED;
                        if modifiers.shift {
                            zx += f;
                        } else if modifiers.command || modifiers.ctrl {
                            zy += f;
                        } else {
                            zx += f;
                            zy += f;
                        }
                    }
                    Event::Zoom(z) => pinch *= *z as f64,
                    _ => {}
                }
            }
        });
        if pinch != 1.0 && pinch > 0.0 {
            zx -= pinch.ln();
            zy -= pinch.ln();
        }
        if zx != 0.0 || zy != 0.0 {
            if let Some((a, b, log)) = x_range
                && zx != 0.0
            {
                let pivot = sc.x_map.unmap(cursor.x as f64);
                view.x = Some(zoom_range((a, b), pivot, zx.exp(), log));
            }
            if let Some((a, b, log)) = y_range
                && zy != 0.0
            {
                let pivot = sc.y_map.unmap(cursor.y as f64);
                view.y = Some(zoom_range((a, b), pivot, zy.exp(), log));
            }
            // The wheel belonged to the chart: keep an enclosing ScrollArea
            // from scrolling the page too.
            ui.ctx().input_mut(|i| i.smooth_scroll_delta = Vec2::ZERO);
        }
    }

    // ── Drag pan ───────────────────────────────────────────────────────────
    if response.dragged_by(PointerButton::Primary) {
        let started_inside = ui
            .input(|i| i.pointer.press_origin())
            .is_some_and(|p| plot.contains(p));
        if started_inside {
            ui.ctx().set_cursor_icon(egui::CursorIcon::Grabbing);
            let d = response.drag_delta();
            if d.x != 0.0
                && let Some((a, b, log)) = x_range
            {
                // Data moves with the pointer, so the range moves against it.
                // `a`/`b` are the scale's min/max; the map's sign handles
                // inverted axes.
                let px_per_span = sc.x_map.map_span_px();
                if px_per_span != 0.0 {
                    view.x = Some(pan_range(
                        (a, b),
                        -(d.x as f64) / px_per_span,
                        log,
                    ));
                }
            }
            if d.y != 0.0
                && let Some((a, b, log)) = y_range
            {
                let px_per_span = sc.y_map.map_span_px();
                if px_per_span != 0.0 {
                    view.y = Some(pan_range(
                        (a, b),
                        -(d.y as f64) / px_per_span,
                        log,
                    ));
                }
            }
        }
    }

    *view != before
}
