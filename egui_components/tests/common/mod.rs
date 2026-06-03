//! Shared headless-egui test harness.
//!
//! Components paint directly via `Painter` (no AccessKit nodes), so semantic
//! queries don't apply. Instead we drive a real `egui::Context` frame-by-frame
//! with synthetic `RawInput`, exactly how egui itself is exercised in headless
//! tests — no window, no GPU, no extra dependency.

#![allow(dead_code)]

use egui::{Event, Modifiers, PointerButton, Pos2, RawInput, Rect, Vec2};
use egui_components::ShadcnTheme;

/// Virtual screen size for the root UI.
pub const SCREEN: Vec2 = Vec2::new(800.0, 600.0);

/// A context with fonts (incl. the MaterialIcons family) and a theme installed.
pub fn test_ctx(dark: bool) -> egui::Context {
    let ctx = egui::Context::default();
    ctx.set_fonts(egui_components::font_definitions());
    let theme = if dark {
        ShadcnTheme::dark()
    } else {
        ShadcnTheme::light()
    };
    ShadcnTheme::set(&ctx, theme);
    // Deterministic language so i18n-backed components render stable strings.
    egui_components::i18n::set_language(egui_components::i18n::Languages::EnUs);
    ctx
}

/// Default dark context.
pub fn ctx() -> egui::Context {
    test_ctx(true)
}

pub fn base_input() -> RawInput {
    RawInput {
        screen_rect: Some(Rect::from_min_size(Pos2::ZERO, SCREEN)),
        ..Default::default()
    }
}

/// Run one frame with the given input, building UI into the root `Ui`.
pub fn frame(ctx: &egui::Context, input: RawInput, mut build: impl FnMut(&mut egui::Ui)) {
    let _ = ctx.run_ui(input, |ui| build(ui));
}

/// Render one default frame, returning whatever the closure extracts (e.g. a
/// `Response` or a widget `Rect`).
pub fn render<R>(ctx: &egui::Context, mut build: impl FnMut(&mut egui::Ui) -> R) -> R {
    let mut out: Option<R> = None;
    let _ = ctx.run_ui(base_input(), |ui| out = Some(build(ui)));
    out.expect("run_ui ran the closure at least once")
}

/// Input that performs a full primary-button click at `pos` within one frame
/// (move → press → release). Proven sufficient for the library's click widgets.
pub fn click_input(pos: Pos2) -> RawInput {
    let mut input = base_input();
    input.events = vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::default(),
        },
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: false,
            modifiers: Modifiers::default(),
        },
    ];
    input
}

/// Input that holds the primary button down at `pos` (press, no release) — used
/// to start a drag on sliders.
pub fn press_at(pos: Pos2) -> RawInput {
    let mut input = base_input();
    input.events = vec![
        Event::PointerMoved(pos),
        Event::PointerButton {
            pos,
            button: PointerButton::Primary,
            pressed: true,
            modifiers: Modifiers::default(),
        },
    ];
    input
}

/// Assert a widget rect is finite and non-degenerate-negative.
pub fn assert_rect_sane(r: Rect) {
    assert!(
        r.width().is_finite() && r.height().is_finite(),
        "rect has non-finite size: {r:?}"
    );
    assert!(
        r.width() >= 0.0 && r.height() >= 0.0,
        "rect has negative size: {r:?}"
    );
}

/// Two-phase click: press at `pos` on one frame, release on the next, rebuilding
/// the same UI both times. More faithful to real input than a single-frame click.
pub fn click_two_phase(ctx: &egui::Context, pos: Pos2, mut build: impl FnMut(&mut egui::Ui)) {
    frame(ctx, press_at(pos), &mut build);
    let mut release = base_input();
    release.events = vec![Event::PointerButton {
        pos,
        button: PointerButton::Primary,
        pressed: false,
        modifiers: Modifiers::default(),
    }];
    frame(ctx, release, &mut build);
}
