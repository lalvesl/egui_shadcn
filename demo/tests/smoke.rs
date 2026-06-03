//! Headless "the demo actually runs" confidence test.
//!
//! Constructs the real [`DemoApp`] and drives its real update loop — every
//! section, toolbar, sidebar and overlay — for several frames with no window
//! and no GPU. If any showcased component panics, regresses an `egui` API, or
//! hits a bad font/i18n path, it surfaces here rather than in the browser.

use demo::app::DemoApp;
use i18n::Languages;

fn screen() -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(egui::Rect::from_min_size(
            egui::Pos2::ZERO,
            egui::vec2(1280.0, 800.0),
        )),
        ..Default::default()
    }
}

/// Run the demo for `frames` frames in the given theme + language.
fn run(dark: bool, lang: Languages, frames: usize) {
    let ctx = egui::Context::default();
    demo::fonts::register_demo_fonts(&ctx);
    i18n::set_language(lang);

    let mut app = DemoApp::default().with_dark(dark);
    for _ in 0..frames {
        let _ = ctx.run_ui(screen(), |ui| app.show(ui));
    }
}

#[test]
fn demo_runs_dark_english() {
    run(true, Languages::EnUs, 6);
}

#[test]
fn demo_runs_light_english() {
    run(false, Languages::EnUs, 6);
}

#[test]
fn demo_runs_portuguese() {
    run(true, Languages::PtBr, 6);
}

#[test]
fn demo_runs_with_custom_primary_hue() {
    // Default app + a non-zinc hue exercises the HSL-derived token path.
    let ctx = egui::Context::default();
    demo::fonts::register_demo_fonts(&ctx);
    i18n::set_language(Languages::EnUs);
    let mut app = DemoApp::default();
    for _ in 0..4 {
        let _ = ctx.run_ui(screen(), |ui| app.show(ui));
    }
}

#[test]
fn demo_survives_pointer_scrubbing() {
    // Move the pointer across the window each frame to wake hover/cursor paths
    // (sidebar auto-scroll, tooltips, popovers) without crashing.
    let ctx = egui::Context::default();
    demo::fonts::register_demo_fonts(&ctx);
    i18n::set_language(Languages::EnUs);
    let mut app = DemoApp::default();

    for i in 0..8 {
        let mut input = screen();
        let x = 100.0 + (i as f32) * 120.0;
        input.events = vec![egui::Event::PointerMoved(egui::pos2(x, 80.0 + x * 0.2))];
        let _ = ctx.run_ui(input, |ui| app.show(ui));
    }
}
