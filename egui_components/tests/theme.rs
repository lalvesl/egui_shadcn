//! Pure-logic tests for the theme / design-token layer — no UI required.

use egui::Color32;
use egui_components::theme::hsl;
use egui_components::ShadcnTheme;

fn luminance(c: Color32) -> f32 {
    let f = |v: u8| {
        let v = v as f32 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
}

#[test]
fn hsl_primary_hues_are_exact() {
    assert_eq!(hsl(0.0, 1.0, 0.5), Color32::from_rgb(255, 0, 0), "red");
    assert_eq!(hsl(120.0, 1.0, 0.5), Color32::from_rgb(0, 255, 0), "green");
    assert_eq!(hsl(240.0, 1.0, 0.5), Color32::from_rgb(0, 0, 255), "blue");
}

#[test]
fn hsl_achromatic_is_gray() {
    assert_eq!(hsl(0.0, 0.0, 0.0), Color32::from_rgb(0, 0, 0), "black");
    assert_eq!(hsl(0.0, 0.0, 1.0), Color32::from_rgb(255, 255, 255), "white");
    // Saturation 0 → r==g==b regardless of hue.
    let gray = hsl(200.0, 0.0, 0.5);
    assert_eq!(gray.r(), gray.g());
    assert_eq!(gray.g(), gray.b());
}

#[test]
fn hsl_hue_wraps_360() {
    assert_eq!(hsl(0.0, 0.8, 0.5), hsl(360.0, 0.8, 0.5));
}

#[test]
fn dark_and_light_have_correct_flag_and_radius() {
    let dark = ShadcnTheme::dark();
    let light = ShadcnTheme::light();
    assert!(dark.dark);
    assert!(!light.dark);
    assert_eq!(dark.radius, 6.0);
    assert_eq!(light.radius, 6.0);
    assert!(dark.primary_hue.is_none());
    assert!(light.primary_hue.is_none());
}

#[test]
fn dark_background_is_darker_than_light() {
    let dark = ShadcnTheme::dark();
    let light = ShadcnTheme::light();
    assert!(
        luminance(dark.background) < luminance(light.background),
        "dark bg lum {} should be < light bg lum {}",
        luminance(dark.background),
        luminance(light.background)
    );
    // Foreground must contrast with background in both modes.
    assert!(luminance(dark.foreground) > luminance(dark.background));
    assert!(luminance(light.foreground) < luminance(light.background));
}

#[test]
fn custom_hue_changes_primary_and_is_recorded() {
    let zinc = ShadcnTheme::dark();
    let red = ShadcnTheme::build(true, Some(0.0));
    assert_eq!(red.primary_hue, Some(0.0));
    assert_ne!(zinc.primary, red.primary, "hue should change the primary color");
    // ring tracks primary.
    assert_eq!(red.ring, red.primary);
}

#[test]
fn primary_foreground_is_high_contrast() {
    // foreground_for() picks near-black or near-white; either way it must be a
    // strong contrast against the primary it pairs with.
    for hue in [0.0, 90.0, 200.0, 300.0] {
        let t = ShadcnTheme::build(true, Some(hue));
        let contrast = (luminance(t.primary) - luminance(t.primary_foreground)).abs();
        assert!(
            contrast > 0.2,
            "hue {hue}: primary/fg contrast too low ({contrast})"
        );
    }
}

#[test]
fn with_alpha_sets_alpha_preserving_unmultiplied_rgb() {
    // Color32 stores *premultiplied* RGBA, so the raw r/g/b are scaled by alpha.
    // The original color round-trips through the unmultiplied accessor.
    let c = Color32::from_rgb(200, 100, 50);

    let opaque = ShadcnTheme::with_alpha(c, 255);
    assert_eq!(
        (opaque.r(), opaque.g(), opaque.b(), opaque.a()),
        (200, 100, 50, 255)
    );

    let semi = ShadcnTheme::with_alpha(c, 128);
    assert_eq!(semi.a(), 128, "alpha must be set exactly");
    let u = semi.to_srgba_unmultiplied();
    assert!((u[0] as i32 - 200).abs() <= 2, "r round-trip: {u:?}");
    assert!((u[1] as i32 - 100).abs() <= 2, "g round-trip: {u:?}");
    assert!((u[2] as i32 - 50).abs() <= 2, "b round-trip: {u:?}");
    assert_eq!(u[3], 128);
}

#[test]
fn get_set_roundtrip_via_context() {
    let ctx = egui::Context::default();
    // Default (nothing stored) falls back to dark.
    assert!(ShadcnTheme::get(&ctx).dark);

    ShadcnTheme::set(&ctx, ShadcnTheme::build(false, Some(142.0)));
    let got = ShadcnTheme::get(&ctx);
    assert!(!got.dark);
    assert_eq!(got.primary_hue, Some(142.0));
}

#[test]
fn apply_sets_visuals_without_panicking() {
    let ctx = egui::Context::default();
    ctx.set_fonts(egui_components::font_definitions());
    ShadcnTheme::dark().apply(&ctx);
    let _ = ctx.run_ui(
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(200.0, 200.0),
            )),
            ..Default::default()
        },
        |ui| {
            ui.label("themed");
        },
    );
    // Visuals were applied: panel fill matches the theme background.
    assert_eq!(
        ctx.global_style().visuals.panel_fill,
        ShadcnTheme::dark().background
    );
}

#[test]
fn font_helpers_have_expected_sizes() {
    assert_eq!(ShadcnTheme::body_font().size, 14.0);
    assert_eq!(ShadcnTheme::small_font().size, 12.0);
    assert_eq!(ShadcnTheme::heading_font(28.0).size, 28.0);
    assert_eq!(ShadcnTheme::icon_font(18.0).size, 18.0);
}
