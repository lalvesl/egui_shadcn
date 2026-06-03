//! Behavioural tests: synthetic pointer/keyboard input must drive real state
//! changes (toggles flip, buttons report clicks, sliders move, disabled widgets
//! stay inert).

mod common;
use common::*;

use egui::{Event, vec2};

#[test]
fn checkbox_click_toggles_and_disabled_is_inert() {
    use egui_components::checkbox::Checkbox;
    let ctx = ctx();

    // Enabled: click flips false → true.
    let mut checked = false;
    let rect = render(&ctx, |ui| Checkbox::new(&mut checked).show(ui).rect);
    frame(&ctx, click_input(rect.center()), |ui| {
        Checkbox::new(&mut checked).show(ui);
    });
    assert!(checked, "enabled checkbox should toggle on click");

    // Disabled: identical click must NOT flip it.
    let mut checked2 = false;
    let rect2 = render(&ctx, |ui| {
        Checkbox::new(&mut checked2).enabled(false).show(ui).rect
    });
    frame(&ctx, click_input(rect2.center()), |ui| {
        Checkbox::new(&mut checked2).enabled(false).show(ui);
    });
    assert!(!checked2, "disabled checkbox must stay unchecked");
}

#[test]
fn switch_click_toggles() {
    use egui_components::switch::Switch;
    let ctx = ctx();
    let mut on = false;
    let rect = render(&ctx, |ui| Switch::new(&mut on).show(ui).rect);
    frame(&ctx, click_input(rect.center()), |ui| {
        Switch::new(&mut on).show(ui);
    });
    assert!(on, "switch should toggle on click");
}

#[test]
fn toggle_click_toggles() {
    use egui_components::toggle::Toggle;
    let ctx = ctx();
    let mut pressed = false;
    let rect = render(&ctx, |ui| Toggle::new(&mut pressed, "B").show(ui).rect);
    frame(&ctx, click_input(rect.center()), |ui| {
        Toggle::new(&mut pressed, "B").show(ui);
    });
    assert!(pressed, "toggle should flip pressed on click");
}

#[test]
fn button_reports_click_and_disabled_does_not() {
    use egui_components::button::Button;
    let ctx = ctx();

    let rect = render(&ctx, |ui| Button::new("Go").show(ui).rect);
    let mut clicked = false;
    frame(&ctx, click_input(rect.center()), |ui| {
        clicked = Button::new("Go").show(ui).clicked();
    });
    assert!(clicked, "enabled button must report a click");

    let rect_d = render(&ctx, |ui| Button::new("Go").enabled(false).show(ui).rect);
    let mut clicked_d = false;
    frame(&ctx, click_input(rect_d.center()), |ui| {
        clicked_d = Button::new("Go").enabled(false).show(ui).clicked();
    });
    assert!(!clicked_d, "disabled button must not report a click");
}

#[test]
fn radio_click_selects_value() {
    use egui_components::radio::Radio;
    let ctx = ctx();
    let mut current = 0u32;
    let rect = render(&ctx, |ui| Radio::new(&mut current, 2u32).show(ui).rect);
    frame(&ctx, click_input(rect.center()), |ui| {
        Radio::new(&mut current, 2u32).show(ui);
    });
    assert_eq!(current, 2, "clicking a radio selects its value");
}

#[test]
fn accordion_header_click_toggles_open() {
    use egui_components::accordion::Accordion;
    let ctx = ctx();
    let mut open = false;

    // Header is a full-width, 44px-tall band at the component's top-left.
    let header_pt = render(&ctx, |ui| {
        let cur = ui.cursor();
        let mut throwaway = open;
        Accordion::new("acc", "Title", &mut throwaway).show(ui, |ui| {
            ui.label("body");
        });
        egui::pos2(cur.left() + 40.0, cur.top() + 22.0)
    });

    frame(&ctx, click_input(header_pt), |ui| {
        Accordion::new("acc", "Title", &mut open).show(ui, |ui| {
            ui.label("body");
        });
    });
    assert!(open, "clicking the accordion header opens it");
}

#[test]
fn slider_drag_changes_value() {
    use egui_components::slider::Slider;
    let ctx = ctx();
    let mut v = 50.0_f32;

    let rect = render(&ctx, |ui| Slider::new(&mut v, 0.0, 100.0).show(ui).rect);
    let start = rect.center();

    // Press on the slider…
    frame(&ctx, press_at(start), |ui| {
        Slider::new(&mut v, 0.0, 100.0).show(ui);
    });
    // …then move right with the button still held (no release event ⇒ still down).
    let mut moved = base_input();
    moved.events = vec![Event::PointerMoved(start + vec2(120.0, 0.0))];
    frame(&ctx, moved, |ui| {
        Slider::new(&mut v, 0.0, 100.0).show(ui);
    });

    assert!(
        v > 50.0 && v <= 100.0,
        "dragging right should raise the value (got {v})"
    );
}

#[test]
fn input_typing_appends_text() {
    use egui_components::input::Input;
    let ctx = ctx();
    let mut text = String::new();

    // Discover the field, click to focus it, then type.
    let rect = render(&ctx, |ui| Input::new(&mut text).show(ui).rect);
    frame(&ctx, click_input(rect.center()), |ui| {
        Input::new(&mut text).show(ui);
    });

    let mut typed = base_input();
    typed.events = vec![Event::Text("Hi".to_string())];
    frame(&ctx, typed, |ui| {
        Input::new(&mut text).show(ui);
    });

    assert_eq!(text, "Hi", "typing into a focused Input should append text");
}
