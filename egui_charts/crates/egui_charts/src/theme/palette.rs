//! Categorical / sequential / diverging palettes from a primary color.

use super::color::{Oklch, from_oklch, mix_oklab, to_oklch};
use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Distribution {
    /// Evenly spaced around the wheel (360 / n).
    Even,
    /// Golden angle (~137.5°) — maximizes consecutive separation for large n.
    GoldenAngle,
}

/// Categorical palette of `n` colors derived from primary.
///
/// Lightness & chroma are tuned per mode so colors stay readable on the active background.
pub fn categorical(primary: Color32, n: usize, dark: bool, dist: Distribution) -> Vec<Color32> {
    if n == 0 {
        return Vec::new();
    }
    let base = to_oklch(primary);
    let l = if dark {
        (base.l + 0.12).clamp(0.55, 0.85)
    } else {
        base.l.clamp(0.45, 0.70)
    };
    let c = if dark { base.c * 0.90 } else { base.c };

    let step = match dist {
        Distribution::Even => 360.0 / n as f32,
        Distribution::GoldenAngle => 137.508,
    };

    (0..n)
        .map(|i| {
            let h = base.h + step * i as f32;
            from_oklch(Oklch::new(l, c, h))
        })
        .collect()
}

/// Light tint → saturated primary (sequential ramp).
pub fn sequential(primary: Color32, steps: usize, dark: bool) -> Vec<Color32> {
    if steps == 0 {
        return Vec::new();
    }
    let base = to_oklch(primary);
    (0..steps)
        .map(|i| {
            let t = if steps == 1 { 0.0 } else { i as f32 / (steps - 1) as f32 };
            let (l_lo, l_hi) = if dark { (0.20, 0.85) } else { (0.95, 0.40) };
            let l = l_lo + (l_hi - l_lo) * t;
            let c = base.c * (0.20 + 0.80 * t);
            from_oklch(Oklch::new(l, c, base.h))
        })
        .collect()
}

/// Primary → neutral → complement (diverging ramp), through perceptual gray.
pub fn diverging(primary: Color32, steps: usize, dark: bool) -> Vec<Color32> {
    if steps == 0 {
        return Vec::new();
    }
    let base = to_oklch(primary);
    let comp = from_oklch(Oklch::new(base.l, base.c, base.h + 180.0));
    let neutral = if dark {
        Color32::from_gray(64)
    } else {
        Color32::from_gray(232)
    };

    (0..steps)
        .map(|i| {
            let t = if steps == 1 { 0.5 } else { i as f32 / (steps - 1) as f32 };
            if t < 0.5 {
                mix_oklab(primary, neutral, t * 2.0)
            } else {
                mix_oklab(neutral, comp, (t - 0.5) * 2.0)
            }
        })
        .collect()
}
