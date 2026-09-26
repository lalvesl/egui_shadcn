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

/// Center x of each wheel inside an inline `TimePicker` whose boxed rect is
/// `r`: the two 110 px-capped columns and the 26 px colon gap sit centered in
/// the box, inside its `Spacing::Sm` padding.
fn wheel_centers(r: egui::Rect) -> (f32, f32) {
    let (pad, colon_w) = (8.0, 26.0);
    let inner_w = r.width() - pad * 2.0;
    let col_w = ((inner_w - colon_w) / 2.0).clamp(48.0, 110.0).floor();
    let left =
        r.left() + pad + ((inner_w - (col_w * 2.0 + colon_w)) / 2.0).max(0.0);
    (left + col_w / 2.0, left + col_w + colon_w + col_w / 2.0)
}

#[test]
fn time_picker_wheels_are_centered_in_the_available_width() {
    use egui_components::time_picker::{CalTime, TimePicker};
    let ctx = ctx();
    let mut value = CalTime::new(9, 30);
    let rect = render(&ctx, |ui| {
        ui.scope(|ui| {
            TimePicker::new("centered", &mut value).inline(ui);
        })
        .response
        .rect
    });
    let (hour_x, minute_x) = wheel_centers(rect);
    // The colon sits between the wheels, and the pair straddles the box center.
    assert!(
        ((hour_x + minute_x) / 2.0 - rect.center().x).abs() < 1.0,
        "wheels must straddle the box center: {hour_x}..{minute_x} in {rect:?}"
    );
    assert!(
        hour_x - rect.left() > 100.0,
        "on an 800 px screen the wheels must not hug the left edge: {hour_x}"
    );
}

#[test]
fn time_picker_wheel_tap_moves_hour_and_minute() {
    use egui_components::time_picker::{CalTime, TimePicker};
    let ctx = ctx();
    let mut value = CalTime::new(9, 30);

    let build = |v: &mut CalTime, ui: &mut egui::Ui| {
        ui.scope(|ui| {
            TimePicker::new("wheel", v).inline(ui);
        })
        .response
        .rect
    };

    let rect = render(&ctx, |ui| build(&mut value, ui));
    let (hour_x, minute_x) = wheel_centers(rect);
    // Size::Default on a wide viewport → 36 + 4 px rows.
    let row_h = 40.0;

    // Tapping the row below the center brings it up: 09 → 10.
    frame(
        &ctx,
        click_input(egui::pos2(hour_x, rect.center().y + row_h)),
        |ui| {
            build(&mut value, ui);
        },
    );
    assert_eq!(value.hour, 10, "tap below center should advance the hour");
    assert_eq!(value.minute, 30, "the minute wheel must not move");

    // And the row above it goes back: 30 → 29 on the minute wheel.
    frame(
        &ctx,
        click_input(egui::pos2(minute_x, rect.center().y - row_h)),
        |ui| {
            build(&mut value, ui);
        },
    );
    assert_eq!(
        value.minute, 29,
        "tap above center should rewind the minute"
    );
    assert_eq!(value.hour, 10, "the hour wheel must not move");
}

#[test]
fn time_picker_wheel_wraps_and_honours_minute_step() {
    use egui_components::time_picker::{CalTime, TimePicker};
    let ctx = ctx();
    let mut value = CalTime::new(0, 0);

    let build = |v: &mut CalTime, ui: &mut egui::Ui| {
        ui.scope(|ui| {
            TimePicker::new("wrap", v).minute_step(5).inline(ui);
        })
        .response
        .rect
    };

    let rect = render(&ctx, |ui| build(&mut value, ui));
    let (hour_x, minute_x) = wheel_centers(rect);
    let row_h = 40.0;

    // Above 00:00 sits the far end of each wheel — they are cyclic.
    frame(
        &ctx,
        click_input(egui::pos2(hour_x, rect.center().y - row_h)),
        |ui| {
            build(&mut value, ui);
        },
    );
    assert_eq!(value.hour, 23, "hour wheel wraps 00 → 23");

    frame(
        &ctx,
        click_input(egui::pos2(minute_x, rect.center().y - row_h)),
        |ui| {
            build(&mut value, ui);
        },
    );
    assert_eq!(
        value.minute, 55,
        "minute wheel wraps 00 → 55 in 5-min steps"
    );
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
fn toggle_show_with_click_toggles_and_disabled_is_inert() {
    use egui_components::toggle::Toggle;
    let ctx = ctx();

    // Clicking custom content flips the toggle — the content's own labels must
    // not swallow the click.
    let mut pressed = false;
    let build = |pressed: &mut bool, enabled: bool, ui: &mut egui::Ui| {
        Toggle::custom(pressed)
            .enabled(enabled)
            .show_with(ui, |ui| ui.label("Starred"))
            .response
            .rect
    };
    let rect = render(&ctx, |ui| build(&mut pressed, true, ui));
    frame(&ctx, click_input(rect.center()), |ui| {
        build(&mut pressed, true, ui);
    });
    assert!(pressed, "show_with toggle should flip pressed on click");

    // Disabled: the same click must not flip it.
    let mut pressed_d = false;
    let rect_d = render(&ctx, |ui| build(&mut pressed_d, false, ui));
    frame(&ctx, click_input(rect_d.center()), |ui| {
        build(&mut pressed_d, false, ui);
    });
    assert!(!pressed_d, "disabled show_with toggle must stay unpressed");
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

    let rect_d =
        render(&ctx, |ui| Button::new("Go").enabled(false).show(ui).rect);
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

/// Two `Select`s side by side, their trigger rects, and their selections.
fn two_selects(
    ctx: &egui::Context,
    input: egui::RawInput,
    a: &mut Option<usize>,
    b: &mut Option<usize>,
) -> (egui::Rect, egui::Rect) {
    use egui_components::select::Select;
    let opts = ["one", "two", "three"];
    let mut rects = (egui::Rect::NOTHING, egui::Rect::NOTHING);
    frame(ctx, input, |ui| {
        ui.horizontal(|ui| {
            rects.0 = ui
                .scope(|ui| Select::new(a, &opts).width(200.0).show(ui))
                .response
                .rect;
            rects.1 = ui
                .scope(|ui| Select::new(b, &opts).width(200.0).show(ui))
                .response
                .rect;
        });
    });
    rects
}

#[test]
fn opening_one_select_does_not_open_another() {
    let ctx = ctx();
    let (mut a, mut b) = (None, None);
    let (ra, rb) = two_selects(&ctx, base_input(), &mut a, &mut b);
    // Open A only.
    two_selects(&ctx, click_input(ra.center()), &mut a, &mut b);
    two_selects(&ctx, base_input(), &mut a, &mut b);
    // Where B's first option would be if B had opened too.
    let under_b = egui::pos2(rb.center().x, rb.bottom() + 26.0);
    two_selects(&ctx, click_input(under_b), &mut a, &mut b);
    assert_eq!(b, None, "B's list was open although only A was clicked");

    // And A's list really was open: its first option is where we expect it.
    two_selects(&ctx, click_input(ra.center()), &mut a, &mut b);
    two_selects(&ctx, base_input(), &mut a, &mut b);
    let under_a = egui::pos2(ra.center().x, ra.bottom() + 26.0);
    two_selects(&ctx, click_input(under_a), &mut a, &mut b);
    assert_eq!(a, Some(0), "A's first option should have been picked");
}

fn number_frame(
    ctx: &egui::Context,
    input: egui::RawInput,
    v: &mut f32,
) -> (egui::Rect, bool) {
    use egui_components::number_input::NumberInput;
    let mut out = (egui::Rect::NOTHING, false);
    frame(ctx, input, |ui| {
        let r = ui.scope(|ui| {
            NumberInput::new("n", v)
                .range(0.0..=10.0)
                .step(0.5)
                .unit("V")
                .show(ui)
        });
        out = (r.response.rect, r.inner);
    });
    out
}

fn keys(events: Vec<Event>) -> egui::RawInput {
    let mut input = base_input();
    input.events = events;
    input
}

fn key(k: egui::Key) -> Event {
    Event::Key {
        key: k,
        physical_key: None,
        pressed: true,
        repeat: false,
        modifiers: egui::Modifiers::default(),
    }
}

#[test]
fn number_input_commits_on_enter_clamps_and_reverts_garbage() {
    let ctx = ctx();
    let mut v = 1.0f32;
    let (rect, _) = number_frame(&ctx, base_input(), &mut v);
    // Focus the field (click near its left edge, where the text is).
    number_frame(
        &ctx,
        click_input(egui::pos2(rect.left() + 20.0, rect.center().y)),
        &mut v,
    );

    // Select all, type a new value: nothing is committed while typing.
    let (_, changed) = number_frame(
        &ctx,
        keys(vec![
            Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            Event::Text("2,5".into()),
        ]),
        &mut v,
    );
    assert!(!changed);
    assert_eq!(v, 1.0, "typing must not commit");

    let (_, changed) =
        number_frame(&ctx, keys(vec![key(egui::Key::Enter)]), &mut v);
    assert!(changed, "Enter commits");
    assert_eq!(v, 2.5, "decimal comma accepted");

    // Out of range is clamped on commit.
    number_frame(
        &ctx,
        click_input(egui::pos2(rect.left() + 20.0, rect.center().y)),
        &mut v,
    );
    number_frame(
        &ctx,
        keys(vec![
            Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            Event::Text("99".into()),
        ]),
        &mut v,
    );
    number_frame(&ctx, keys(vec![key(egui::Key::Enter)]), &mut v);
    assert_eq!(v, 10.0, "clamped to the range");

    // Garbage is reverted when focus leaves.
    number_frame(
        &ctx,
        click_input(egui::pos2(rect.left() + 20.0, rect.center().y)),
        &mut v,
    );
    number_frame(
        &ctx,
        keys(vec![
            Event::Key {
                key: egui::Key::A,
                physical_key: None,
                pressed: true,
                repeat: false,
                modifiers: egui::Modifiers::COMMAND,
            },
            Event::Text("abc".into()),
        ]),
        &mut v,
    );
    number_frame(
        &ctx,
        click_input(egui::pos2(rect.right() + 300.0, rect.bottom() + 200.0)),
        &mut v,
    );
    number_frame(&ctx, base_input(), &mut v);
    assert_eq!(v, 10.0, "text that does not parse leaves the value alone");
}

#[test]
fn number_input_arrow_keys_step_the_value() {
    let ctx = ctx();
    let mut v = 1.0f32;
    let (rect, _) = number_frame(&ctx, base_input(), &mut v);
    number_frame(
        &ctx,
        click_input(egui::pos2(rect.left() + 20.0, rect.center().y)),
        &mut v,
    );
    let (_, changed) =
        number_frame(&ctx, keys(vec![key(egui::Key::ArrowUp)]), &mut v);
    assert!(changed);
    assert_eq!(v, 1.5);
    number_frame(
        &ctx,
        keys(vec![key(egui::Key::ArrowDown), key(egui::Key::ArrowDown)]),
        &mut v,
    );
    assert!(v < 1.5, "stepped down to {v}");
}
