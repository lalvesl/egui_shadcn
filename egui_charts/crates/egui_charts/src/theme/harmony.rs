//! Color-wheel harmony schemes — hue offsets in degrees relative to primary.

use super::color::{Oklch, from_oklch, to_oklch};
use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Harmony {
    Monochromatic,
    Complementary,
    Analogous,
    Triadic,
    SplitComplementary,
    Tetradic,
    Square,
}

impl Harmony {
    pub const ALL: &'static [Harmony] = &[
        Harmony::Monochromatic,
        Harmony::Complementary,
        Harmony::Analogous,
        Harmony::Triadic,
        Harmony::SplitComplementary,
        Harmony::Tetradic,
        Harmony::Square,
    ];

    pub fn label(self) -> &'static str {
        match self {
            Harmony::Monochromatic => "Monochromatic",
            Harmony::Complementary => "Complementary",
            Harmony::Analogous => "Analogous",
            Harmony::Triadic => "Triadic",
            Harmony::SplitComplementary => "Split-complementary",
            Harmony::Tetradic => "Tetradic",
            Harmony::Square => "Square",
        }
    }

    pub fn hue_offsets(self) -> &'static [f32] {
        match self {
            Harmony::Monochromatic => &[0.0],
            Harmony::Complementary => &[0.0, 180.0],
            Harmony::Analogous => &[-30.0, 0.0, 30.0],
            Harmony::Triadic => &[0.0, 120.0, 240.0],
            Harmony::SplitComplementary => &[0.0, 150.0, 210.0],
            Harmony::Tetradic => &[0.0, 60.0, 180.0, 240.0],
            Harmony::Square => &[0.0, 90.0, 180.0, 270.0],
        }
    }
}

/// Anchor colors for a scheme (constant L & C, rotated hue).
pub fn harmony_palette(primary: Color32, scheme: Harmony) -> Vec<Color32> {
    let base = to_oklch(primary);
    scheme
        .hue_offsets()
        .iter()
        .map(|d| from_oklch(Oklch::new(base.l, base.c, base.h + d)))
        .collect()
}
