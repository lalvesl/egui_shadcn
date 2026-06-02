//! Color32 ↔ OKLCH conversion (Björn Ottosson's OKLab, cylindrical form).
//!
//! Self-contained — no `palette` crate dep. Round-trip tolerance: ±1 u8 channel.

use egui::Color32;

/// OKLCH triple: lightness 0..1, chroma 0..~0.4, hue degrees.
#[derive(Clone, Copy, Debug)]
pub struct Oklch {
    pub l: f32,
    pub c: f32,
    pub h: f32, // degrees
}

impl Oklch {
    pub fn new(l: f32, c: f32, h: f32) -> Self {
        Self { l, c, h }
    }
}

// ── sRGB ⇄ linear ───────────────────────────────────────────────────────────

fn srgb_to_linear(v: f32) -> f32 {
    if v <= 0.04045 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(v: f32) -> f32 {
    if v <= 0.0031308 {
        12.92 * v
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

// ── linear sRGB ⇄ OKLab (Ottosson 2020) ──────────────────────────────────────

fn linear_to_oklab(r: f32, g: f32, b: f32) -> (f32, f32, f32) {
    let l = 0.412_221_46 * r + 0.536_332_55 * g + 0.051_445_995 * b;
    let m = 0.211_903_5 * r + 0.680_699_5 * g + 0.107_396_96 * b;
    let s = 0.088_302_46 * r + 0.281_718_85 * g + 0.629_978_7 * b;

    let l_ = l.cbrt();
    let m_ = m.cbrt();
    let s_ = s.cbrt();

    (
        0.210_454_26 * l_ + 0.793_617_8 * m_ - 0.004_072_047 * s_,
        1.977_998_5 * l_ - 2.428_592_2 * m_ + 0.450_593_7 * s_,
        0.025_904_037 * l_ + 0.782_771_77 * m_ - 0.808_675_77 * s_,
    )
}

fn oklab_to_linear(l: f32, a: f32, b: f32) -> (f32, f32, f32) {
    let l_ = l + 0.396_337_78 * a + 0.215_803_76 * b;
    let m_ = l - 0.105_561_346 * a - 0.063_854_17 * b;
    let s_ = l - 0.089_484_18 * a - 1.291_485_5 * b;

    let l = l_ * l_ * l_;
    let m = m_ * m_ * m_;
    let s = s_ * s_ * s_;

    (
        4.076_741_7 * l - 3.307_711_6 * m + 0.230_969_94 * s,
        -1.268_438 * l + 2.609_757_4 * m - 0.341_319_38 * s,
        -0.0041960863 * l - 0.703_418_6 * m + 1.707_614_7 * s,
    )
}

// ── public API ───────────────────────────────────────────────────────────────

pub fn to_oklch(c: Color32) -> Oklch {
    let r = srgb_to_linear(c.r() as f32 / 255.0);
    let g = srgb_to_linear(c.g() as f32 / 255.0);
    let b = srgb_to_linear(c.b() as f32 / 255.0);
    let (l, a, b2) = linear_to_oklab(r, g, b);
    let chroma = (a * a + b2 * b2).sqrt();
    let hue = b2.atan2(a).to_degrees();
    let hue = if hue < 0.0 { hue + 360.0 } else { hue };
    Oklch {
        l,
        c: chroma,
        h: hue,
    }
}

pub fn from_oklch(c: Oklch) -> Color32 {
    let hue_rad = c.h.to_radians();
    let a = c.c * hue_rad.cos();
    let b = c.c * hue_rad.sin();
    let (lr, lg, lb) = oklab_to_linear(c.l, a, b);
    let r = (linear_to_srgb(lr.clamp(0.0, 1.0)) * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8;
    let g = (linear_to_srgb(lg.clamp(0.0, 1.0)) * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8;
    let b = (linear_to_srgb(lb.clamp(0.0, 1.0)) * 255.0)
        .round()
        .clamp(0.0, 255.0) as u8;
    Color32::from_rgb(r, g, b)
}

/// Mix two colors in OKLab space (perceptually smooth).
pub fn mix_oklab(a: Color32, b: Color32, t: f32) -> Color32 {
    let oa = to_oklch(a);
    let ob = to_oklch(b);
    // Convert hue/chroma → ab for unambiguous interpolation.
    let aa = oa.c * oa.h.to_radians().cos();
    let ab = oa.c * oa.h.to_radians().sin();
    let ba = ob.c * ob.h.to_radians().cos();
    let bb = ob.c * ob.h.to_radians().sin();
    let l = oa.l + (ob.l - oa.l) * t;
    let av = aa + (ba - aa) * t;
    let bv = ab + (bb - ab) * t;
    let chroma = (av * av + bv * bv).sqrt();
    let mut hue = bv.atan2(av).to_degrees();
    if hue < 0.0 {
        hue += 360.0;
    }
    from_oklch(Oklch::new(l, chroma, hue))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_primaries() {
        for c in [
            Color32::RED,
            Color32::GREEN,
            Color32::BLUE,
            Color32::from_rgb(123, 45, 200),
            Color32::WHITE,
            Color32::BLACK,
        ] {
            let back = from_oklch(to_oklch(c));
            let dr = (back.r() as i32 - c.r() as i32).abs();
            let dg = (back.g() as i32 - c.g() as i32).abs();
            let db = (back.b() as i32 - c.b() as i32).abs();
            assert!(
                dr <= 1 && dg <= 1 && db <= 1,
                "round-trip drift {c:?} → {back:?}"
            );
        }
    }
}
