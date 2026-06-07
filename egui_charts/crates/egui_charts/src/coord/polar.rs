//! Polar coordinate system — origin + max radius. Used by pie, doughnut,
//! rose, gauge.
//!
//! Angles are radians. By chart convention, `angle = 0` points up
//! (12 o'clock) and increases clockwise. This matches ECharts (and is
//! flipped from the math convention) so callers don't have to think about
//! the conversion.

use egui::{Pos2, Rect, Vec2, vec2};

/// Result of laying out a polar plot inside a rect.
#[derive(Clone, Debug)]
pub struct PolarLayout {
    pub center: Pos2,
    pub outer_radius: f32,
    pub bounds: Rect,
}

impl PolarLayout {
    /// Square plot inscribed in `rect`, sized so a `outer_ratio` fraction
    /// of the half-extent is the outer radius. Centered on `rect`.
    pub fn fit(rect: Rect, outer_ratio: f32) -> Self {
        let center = rect.center();
        let half = rect.size().min_elem() * 0.5;
        let outer = half * outer_ratio.clamp(0.1, 1.0);
        Self {
            center,
            outer_radius: outer,
            bounds: rect,
        }
    }

    /// Chart-convention polar→screen: `angle = 0` is up, increases CW.
    pub fn point(&self, angle_rad: f32, radius: f32) -> Pos2 {
        let (s, c) = (angle_rad.sin(), angle_rad.cos());
        // (sin, -cos) gives 0→up, +→clockwise.
        self.center + vec2(s, -c) * radius
    }

    /// Direction unit vector at `angle_rad`.
    pub fn dir(&self, angle_rad: f32) -> Vec2 {
        let (s, c) = (angle_rad.sin(), angle_rad.cos());
        vec2(s, -c)
    }
}

/// Indicators for a radar chart already laid out as evenly-spaced spokes.
#[derive(Clone, Debug)]
pub struct RadarLayout {
    pub polar: PolarLayout,
    pub spokes: Vec<RadarSpoke>,
}

#[derive(Clone, Debug)]
pub struct RadarSpoke {
    pub angle_rad: f32,
    pub max: f64,
    pub label: String,
}

impl RadarLayout {
    /// Build a radar layout: `n` evenly-spaced spokes starting at 12 o'clock.
    pub fn fit(
        rect: Rect,
        indicators: &[(String, f64)],
        outer_ratio: f32,
    ) -> Self {
        let polar = PolarLayout::fit(rect, outer_ratio);
        let n = indicators.len();
        let spokes = indicators
            .iter()
            .enumerate()
            .map(|(i, (name, max))| {
                let angle = if n == 0 {
                    0.0
                } else {
                    std::f32::consts::TAU * (i as f32 / n as f32)
                };
                RadarSpoke {
                    angle_rad: angle,
                    max: *max,
                    label: name.clone(),
                }
            })
            .collect();
        Self { polar, spokes }
    }

    /// Project a value on a spoke to a screen point.
    pub fn point(&self, spoke_idx: usize, normalized: f32) -> Pos2 {
        let spoke = &self.spokes[spoke_idx];
        self.polar
            .point(spoke.angle_rad, self.polar.outer_radius * normalized)
    }
}
