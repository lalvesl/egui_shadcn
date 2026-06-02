//! Coordinate systems. Phase 0 ships Cartesian2D; others stubbed.

pub mod atlas;
pub mod cartesian;
pub mod geo;
pub mod polar;
pub mod three_d;

use crate::theme::ChartTheme;
use egui::{Pos2, Rect};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CoordKind {
    Cartesian2D,
    Polar,
    Radar,
    Single,
    Calendar,
    Parallel,
    Geo,
    None,
    ThreeD,
}

/// A 2D point in *data* space (axis-relative).
#[derive(Clone, Copy, Debug)]
pub struct DataPoint {
    pub x: f64,
    pub y: f64,
}

/// One rendered axis (positions of tick marks + their labels).
#[derive(Clone, Debug)]
pub struct AxisLayout {
    pub is_x: bool,
    pub line_start: Pos2,
    pub line_end: Pos2,
    pub ticks: Vec<AxisTick>,
    pub name: Option<String>,
}

#[derive(Clone, Debug)]
pub struct AxisTick {
    /// Pixel position along the axis.
    pub pixel: f32,
    /// Data value at this tick.
    pub value: f64,
    /// Pre-formatted label.
    pub label: String,
}

/// Result of laying out a coordinate system inside a host rect. Owns the
/// transforms used by series renderers and hit-testing.
pub struct CoordLayout {
    pub plot_rect: Rect,
    pub axes: Vec<AxisLayout>,
    pub kind: CoordKind,
    to_screen: Box<dyn Fn(DataPoint) -> Pos2>,
    to_data: Box<dyn Fn(Pos2) -> DataPoint>,
}

impl CoordLayout {
    pub fn new(
        kind: CoordKind,
        plot_rect: Rect,
        axes: Vec<AxisLayout>,
        to_screen: Box<dyn Fn(DataPoint) -> Pos2>,
        to_data: Box<dyn Fn(Pos2) -> DataPoint>,
    ) -> Self {
        Self {
            kind,
            plot_rect,
            axes,
            to_screen,
            to_data,
        }
    }

    pub fn to_screen(&self, p: DataPoint) -> Pos2 {
        (self.to_screen)(p)
    }

    pub fn to_data(&self, p: Pos2) -> DataPoint {
        (self.to_data)(p)
    }
}

/// Coordinate-system entry trait. Phase 0 only Cartesian uses this; others
/// land in later phases.
pub trait Coordinate {
    fn layout(&self, rect: Rect, theme: &ChartTheme) -> CoordLayout;
}
