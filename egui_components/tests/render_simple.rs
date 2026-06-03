//! Render-without-panic ("smoke") tests for the leaf widgets — those whose
//! `show()` paints directly and takes only `&mut Ui`. Each is exercised across
//! both themes and, where relevant, every variant/size, asserting the returned
//! geometry is sane.

mod common;
use common::*;

use egui_components::ICON_CHECK;
use egui_components::size::Size;

const THEMES: [bool; 2] = [true, false];
const SIZES: [Size; 3] = [Size::Sm, Size::Default, Size::Lg];

#[test]
fn alert_all_variants() {
    use egui_components::alert::{Alert, AlertVariant};
    for dark in THEMES {
        let ctx = test_ctx(dark);
        for v in [
            AlertVariant::Default,
            AlertVariant::Destructive,
            AlertVariant::Warning,
        ] {
            render(&ctx, |ui| {
                Alert::new("Heads up!")
                    .description("Something happened.")
                    .variant(v)
                    .show(ui)
            });
        }
    }
}

#[test]
fn avatar_initials_and_sizes() {
    use egui_components::avatar::Avatar;
    let ctx = ctx();
    for s in SIZES {
        render(&ctx, |ui| {
            Avatar::new("LL").size(s).show(ui);
            // Long initials get truncated to two chars; must not panic on slicing.
            Avatar::new("ABCDEF").size(s).show(ui);
            Avatar::new("").size(s).show(ui);
        });
    }
}

#[test]
fn badge_all_variants_sizes() {
    use egui_components::badge::{Badge, BadgeVariant};
    let ctx = ctx();
    for v in [
        BadgeVariant::Default,
        BadgeVariant::Secondary,
        BadgeVariant::Destructive,
        BadgeVariant::Outline,
    ] {
        for s in SIZES {
            render(&ctx, |ui| Badge::new("New").variant(v).size(s).show(ui));
        }
    }
}

#[test]
fn button_variants_sizes_states() {
    use egui_components::button::{Button, ButtonSize, ButtonVariant};
    for dark in THEMES {
        let ctx = test_ctx(dark);
        for v in [
            ButtonVariant::Default,
            ButtonVariant::Destructive,
            ButtonVariant::Outline,
            ButtonVariant::Secondary,
            ButtonVariant::Ghost,
            ButtonVariant::Link,
        ] {
            for s in [
                ButtonSize::Sm,
                ButtonSize::Default,
                ButtonSize::Lg,
                ButtonSize::Icon,
            ] {
                for enabled in [true, false] {
                    let resp = render(&ctx, |ui| {
                        Button::new("Click")
                            .variant(v)
                            .size(s)
                            .icon(ICON_CHECK)
                            .enabled(enabled)
                            .show(ui)
                    });
                    assert_rect_sane(resp.rect);
                    assert!(resp.rect.height() > 0.0);
                }
            }
        }
    }
}

#[test]
fn checkbox_states() {
    use egui_components::checkbox::Checkbox;
    let ctx = ctx();
    for s in SIZES {
        for mut checked in [true, false] {
            let resp = render(&ctx, |ui| {
                Checkbox::new(&mut checked).label("Accept").size(s).show(ui)
            });
            assert_rect_sane(resp.rect);
        }
    }
}

#[test]
fn icon_renders() {
    use egui_components::icon::Icon;
    let ctx = ctx();
    let resp = render(&ctx, |ui| Icon::new(ICON_CHECK).show(ui));
    assert_rect_sane(resp.rect);
}

#[test]
fn input_variants() {
    use egui_components::input::Input;
    let ctx = ctx();
    let mut s = String::from("hello");
    for password in [true, false] {
        let resp = render(&ctx, |ui| {
            Input::new(&mut s)
                .label("Email")
                .placeholder("you@example.com")
                .password(password)
                .icon_left(ICON_CHECK)
                .show(ui)
        });
        assert_rect_sane(resp.rect);
    }
}

#[test]
fn input_otp_renders() {
    use egui_components::input_otp::InputOtp;
    let ctx = ctx();
    let mut v = String::from("12");
    let resp = render(&ctx, |ui| InputOtp::new(&mut v, 6).show(ui));
    assert_rect_sane(resp.rect);
}

#[test]
fn label_required_and_plain() {
    use egui_components::label::Label;
    let ctx = ctx();
    render(&ctx, |ui| {
        Label::new("Name").show(ui);
        Label::new("Name").required(true).show(ui);
    });
}

#[test]
fn progress_clamps_and_renders() {
    use egui_components::progress::Progress;
    let ctx = ctx();
    render(&ctx, |ui| {
        for v in [-0.5, 0.0, 0.33, 1.0, 2.0] {
            Progress::new(v).show(ui);
        }
    });
}

#[test]
fn radio_group() {
    use egui_components::radio::Radio;
    let ctx = ctx();
    let mut selected: u32 = 1;
    for s in SIZES {
        let resp = render(&ctx, |ui| {
            Radio::new(&mut selected, 1u32)
                .label("One")
                .size(s)
                .show(ui)
        });
        assert_rect_sane(resp.rect);
    }
}

#[test]
fn separator_orientations() {
    use egui_components::separator::Separator;
    let ctx = ctx();
    render(&ctx, |ui| {
        Separator::horizontal().show(ui);
        Separator::horizontal()
            .thickness(3.0)
            .length(120.0)
            .show(ui);
        ui.horizontal(|ui| {
            Separator::vertical().show(ui);
        });
    });
}

#[test]
fn skeleton_rect_and_circle() {
    use egui_components::skeleton::Skeleton;
    let ctx = ctx();
    render(&ctx, |ui| {
        Skeleton::new(200.0, 16.0).show(ui);
        Skeleton::new(200.0, 16.0).radius(8.0).show(ui);
        Skeleton::circle(40.0).show(ui);
    });
}

#[test]
fn slider_renders_all_sizes() {
    use egui_components::slider::Slider;
    let ctx = ctx();
    let mut v = 40.0_f32;
    for s in SIZES {
        let resp = render(&ctx, |ui| {
            Slider::new(&mut v, 0.0, 100.0).step(5.0).size(s).show(ui)
        });
        assert_rect_sane(resp.rect);
    }
}

#[test]
fn spinner_renders() {
    use egui_components::spinner::Spinner;
    let ctx = ctx();
    render(&ctx, |ui| {
        for s in SIZES {
            Spinner::new().size(s).show(ui);
        }
        Spinner::new().thickness(4.0).show(ui);
    });
}

#[test]
fn switch_states() {
    use egui_components::switch::Switch;
    let ctx = ctx();
    for s in SIZES {
        for mut on in [true, false] {
            let resp = render(&ctx, |ui| {
                Switch::new(&mut on).enabled(true).size(s).show(ui)
            });
            assert_rect_sane(resp.rect);
        }
    }
}

#[test]
fn textarea_renders() {
    use egui_components::textarea::Textarea;
    let ctx = ctx();
    let mut text = String::from("line 1\nline 2");
    let resp = render(&ctx, |ui| {
        Textarea::new(&mut text)
            .placeholder("Type…")
            .rows(4)
            .show(ui)
    });
    assert_rect_sane(resp.rect);
}

#[test]
fn toggle_states() {
    use egui_components::toggle::Toggle;
    let ctx = ctx();
    for s in SIZES {
        for mut pressed in [true, false] {
            let resp = render(&ctx, |ui| {
                Toggle::new(&mut pressed, "Bold")
                    .icon(ICON_CHECK)
                    .bordered(true)
                    .size(s)
                    .show(ui)
            });
            assert_rect_sane(resp.rect);
        }
    }
}

#[test]
fn typography_helpers() {
    use egui_components::typography::*;
    let ctx = ctx();
    render(&ctx, |ui| {
        heading1(ui, "H1");
        heading2(ui, "H2");
        heading3(ui, "H3");
        heading4(ui, "H4");
        body_text(ui, "body");
        muted_text(ui, "muted");
        small_text(ui, "small");
        code_text(ui, "code()");
        lead_text(ui, "lead");
    });
}

#[test]
fn breadcrumb_returns_none_without_click() {
    use egui_components::breadcrumb::Breadcrumb;
    let ctx = ctx();
    let items = ["Home", "Library", "Data"];
    let clicked = render(&ctx, |ui| Breadcrumb::new(&items).show(ui));
    assert_eq!(clicked, None);
}

#[test]
fn button_group_renders() {
    use egui_components::button_group::{ButtonGroup, ButtonGroupVariant};
    let ctx = ctx();
    let buttons = ["Left", "Center", "Right"];
    for v in [ButtonGroupVariant::Default, ButtonGroupVariant::Outline] {
        let sel = render(&ctx, |ui| {
            ButtonGroup::new(&buttons)
                .selected(Some(0))
                .variant(v)
                .show(ui)
        });
        // No click this frame.
        assert_eq!(sel, None);
    }
}

#[test]
fn pagination_renders() {
    use egui_components::pagination::Pagination;
    let ctx = ctx();
    let clicked = render(&ctx, |ui| Pagination::new(3, 10).show(ui));
    assert_eq!(clicked, None);
}

#[test]
fn navigation_menu_orientations() {
    use egui_components::navigation_menu::{NavItem, NavigationMenu};
    let ctx = ctx();
    let items = [
        NavItem {
            label: "Home",
            icon: Some(ICON_CHECK),
            badge: None,
        },
        NavItem {
            label: "Inbox",
            icon: None,
            badge: Some("9"),
        },
    ];
    render(&ctx, |ui| {
        let _ = NavigationMenu::new(&items, 0).show(ui);
    });
    render(&ctx, |ui| {
        let _ = NavigationMenu::new(&items, 1).vertical().show(ui);
    });
}

#[test]
fn select_combobox_datepicker_closed() {
    use egui_components::combobox::Combobox;
    use egui_components::date_picker::DatePicker;
    use egui_components::select::Select;
    let ctx = ctx();
    let opts = ["Apple", "Banana", "Cherry"];
    let mut sel: Option<usize> = Some(1);
    let mut sel2: Option<usize> = None;
    let mut date = None;
    render(&ctx, |ui| {
        let _ = Select::new(&mut sel, &opts).show(ui);
    });
    render(&ctx, |ui| {
        let _ = Combobox::new("cb", &mut sel2, &opts).show(ui);
    });
    render(&ctx, |ui| {
        let _ = DatePicker::new("dp", &mut date).show(ui);
    });
}

#[test]
fn spacing_renders() {
    use egui_components::spacing::Spacing;
    let ctx = ctx();
    render(&ctx, |ui| {
        for sp in [
            Spacing::Xs,
            Spacing::Sm,
            Spacing::Md,
            Spacing::Lg,
            Spacing::Xl,
            Spacing::Xl2,
            Spacing::Xl3,
        ] {
            sp.show(ui);
        }
    });
}
