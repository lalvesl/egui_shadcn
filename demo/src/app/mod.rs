use crate::i18n as t;
use ::i18n::t as tr;
use egui::Color32;
use egui_sc::egui_components::spacing::Spacing;
use egui_sc::egui_components::{
    Animations, ICON_BRIGHTNESS_4, ICON_BRIGHTNESS_7, ICON_MENU, ICON_PALETTE,
    ShadcnTheme,
    button::{Button, ButtonVariant},
    calendar::CalDate,
    dialog::Dialog,
    i18n::{self, Languages},
    input::Input,
    popover::Popover,
    select::Select,
    separator::Separator,
    slider::Slider,
    switch::Switch,
    time_picker::CalTime,
    toast::Toaster,
    typography::{body_text, heading2, heading4, muted_text, small_text},
};

const LOCALES: &[(Languages, &str)] =
    &[(Languages::EnUs, "EN"), (Languages::PtBr, "PT")];

fn locale_idx_to_lang(idx: usize) -> Languages {
    LOCALES.get(idx).map(|(l, _)| *l).unwrap_or(Languages::EnUs)
}

mod sections;

// ── Nav sections ──────────────────────────────────────────────────────────────

const SECTION_COUNT: usize = t::SECTION_COUNT;

/// Below this viewport width (logical px) the layout switches to its compact
/// mobile form: the sidebar becomes a tap-to-open overlay drawer and margins
/// tighten. Matches Tailwind's `sm` breakpoint, so narrow desktop windows and
/// phones (Android / mobile web) get the same treatment.
const MOBILE_MAX_WIDTH: f32 = 640.0;

// ── App state ─────────────────────────────────────────────────────────────────

pub struct DemoApp {
    pub(super) dark: bool,
    pub(super) primary_hue: Option<f32>,
    pub(super) current_section: usize,
    pub(super) sidebar_open: bool,

    pub(super) btn_clicked: bool,
    pub(super) checkbox1: bool,
    pub(super) checkbox2: bool,
    pub(super) checkbox3: bool,
    pub(super) switch1: bool,
    pub(super) switch2: bool,
    pub(super) slider_val: f32,
    pub(super) progress_val: f32,
    pub(super) radio_val: u32,
    pub(super) tab_index: usize,
    pub(super) input_text: String,
    pub(super) password_text: String,
    pub(super) textarea_text: String,
    pub(super) textarea_scroll: String,
    pub(super) textarea_grow_y: String,
    pub(super) select_val: Option<usize>,
    pub(super) accordion_open: [bool; 3],
    pub(super) dialog_open: bool,
    pub(super) dialog_input: String,
    pub(super) cal_single: Option<CalDate>,
    pub(super) cal_range_start: Option<CalDate>,
    pub(super) cal_range_end: Option<CalDate>,
    pub(super) cal_prices_selected: Option<CalDate>,
    pub(super) cal_range_compact_start: Option<CalDate>,
    pub(super) cal_range_compact_end: Option<CalDate>,
    pub(super) toggle1: bool,
    pub(super) toggle2: bool,
    pub(super) toggle_bold: bool,
    pub(super) toggle_italic: bool,
    pub(super) toggle_underline: bool,
    pub(super) toggle_strikethrough: bool,
    pub(super) toggle_starred: bool,
    pub(super) toggle_notify: bool,
    pub(super) toggle_group_val: u8,
    pub(super) collapsible_open: bool,
    pub(super) pagination_page: usize,
    pub(super) pagination_page2: usize,
    pub(super) alert_dialog_open: bool,
    pub(super) alert_dialog_confirmed: bool,
    pub(super) button_group_sel: Option<usize>,
    pub(super) button_group_outline_sel: Option<usize>,
    pub(super) combobox_val: Option<usize>,
    pub(super) date_picker_val: Option<CalDate>,
    pub(super) time_picker_val: CalTime,
    pub(super) time_picker_inline: CalTime,
    pub(super) time_picker_step: CalTime,
    pub(super) otp_val: String,
    pub(super) sheet_open: bool,
    pub(super) drawer_open: bool,
    pub(super) nav_active: usize,
    pub(super) command_open: bool,
    pub(super) data_table_filter: String,
    pub(super) context_last: Option<String>,
    pub(super) dropdown_last: Option<String>,
    pub(super) breadcrumb_nav: Option<String>,
    pub(super) breadcrumb_custom_nav: Option<String>,
    pub(super) scroll_to_section: Option<usize>,
    pub(super) locale_idx: Option<usize>,
    pub(super) sidebar_last_interaction: f64,
    /// Last known "is the viewport narrow?" state. `None` until the first frame;
    /// a change flips the sidebar to its default open state for the new size.
    pub(super) prev_mobile: Option<bool>,
    pub(super) anim_enabled: bool,
    pub(super) anim_speed: f32,
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            dark: true,
            primary_hue: None,
            current_section: 0,
            sidebar_open: true,
            btn_clicked: false,
            checkbox1: true,
            checkbox2: false,
            checkbox3: true,
            switch1: true,
            switch2: false,
            slider_val: 40.0,
            progress_val: 0.65,
            radio_val: 1,
            tab_index: 0,
            input_text: String::new(),
            password_text: String::new(),
            textarea_text: String::new(),
            textarea_scroll: String::new(),
            textarea_grow_y: String::new(),
            select_val: None,
            accordion_open: [true, false, false],
            dialog_open: false,
            dialog_input: String::new(),
            cal_single: None,
            cal_range_start: None,
            cal_range_end: None,
            cal_prices_selected: None,
            cal_range_compact_start: None,
            cal_range_compact_end: None,
            toggle1: false,
            toggle2: true,
            toggle_bold: false,
            toggle_italic: false,
            toggle_underline: false,
            toggle_strikethrough: false,
            toggle_starred: true,
            toggle_notify: false,
            toggle_group_val: 0,
            collapsible_open: false,
            pagination_page: 1,
            pagination_page2: 7,
            alert_dialog_open: false,
            alert_dialog_confirmed: false,
            button_group_sel: Some(0),
            button_group_outline_sel: None,
            combobox_val: None,
            date_picker_val: None,
            time_picker_val: CalTime::new(9, 30),
            time_picker_inline: CalTime::new(14, 0),
            time_picker_step: CalTime::new(18, 45),
            otp_val: String::new(),
            sheet_open: false,
            drawer_open: false,
            nav_active: 0,
            command_open: false,
            data_table_filter: String::new(),
            context_last: None,
            dropdown_last: None,
            breadcrumb_nav: None,
            breadcrumb_custom_nav: None,
            scroll_to_section: None,
            locale_idx: Some(0),
            sidebar_last_interaction: 0.0,
            prev_mobile: None,
            anim_enabled: true,
            anim_speed: 1.0,
        }
    }
}

impl DemoApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        crate::fonts::register_demo_fonts(&cc.egui_ctx);

        let dark = cc.egui_ctx.data_mut(|d| {
            d.get_persisted::<bool>(egui::Id::new("demo_dark"))
                .unwrap_or(true)
        });
        let hue = cc.egui_ctx.data_mut(|d| {
            d.get_persisted::<Option<f32>>(egui::Id::new("demo_hue"))
                .unwrap_or(None)
        });
        let locale_idx = cc.egui_ctx.data_mut(|d| {
            d.get_persisted::<usize>(egui::Id::new("demo_locale_idx"))
                .unwrap_or(0)
        });

        let lang = locale_idx_to_lang(locale_idx);
        i18n::set_language(lang);
        // wasm: register the async catalog source and prefetch the active
        // language so the first paint already has strings. No-op on native.
        #[cfg(target_arch = "wasm32")]
        t::init_web(&cc.egui_ctx);
        t::ensure_loaded(lang);

        let app = Self {
            dark,
            primary_hue: hue,
            locale_idx: Some(locale_idx),
            ..Default::default()
        };

        let theme = ShadcnTheme::build(dark, hue);
        ShadcnTheme::set(&cc.egui_ctx, theme.clone());
        theme.apply(&cc.egui_ctx);

        app
    }

    /// Test/embedding helper: pick the initial color scheme on a default app.
    pub fn with_dark(mut self, dark: bool) -> Self {
        self.dark = dark;
        self
    }

    /// Whether the sidebar is currently shown (inline on desktop, overlay on
    /// mobile). Exposed for tests/embedders that drive the app headlessly.
    pub fn sidebar_open(&self) -> bool {
        self.sidebar_open
    }

    fn apply_theme(&self, ctx: &egui::Context) {
        let theme = ShadcnTheme::build(self.dark, self.primary_hue);
        ShadcnTheme::set(ctx, theme.clone());
        theme.apply(ctx);

        Animations::set(
            ctx,
            Animations {
                enabled: self.anim_enabled,
                speed: self.anim_speed,
            },
        );

        ctx.data_mut(|d| {
            d.insert_persisted(egui::Id::new("demo_dark"), self.dark);
            d.insert_persisted(egui::Id::new("demo_hue"), self.primary_hue);
        });
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.show(ui);
    }
}

impl DemoApp {
    /// Render the entire demo into `ui`. Split out from [`eframe::App::ui`] so
    /// the app can be driven headlessly in tests without an `eframe::Frame`.
    pub fn show(&mut self, ui: &mut egui::Ui) {
        self.apply_theme(ui.ctx());
        let ctx = ui.ctx().clone();

        // ── Responsive mode ──────────────────────────────────────────────────
        // On a narrow viewport (Android / phones / small windows) the sidebar
        // becomes a tap-to-open overlay and starts collapsed; on a wide viewport
        // it's an inline left panel. Flip to the new size's default whenever we
        // cross the breakpoint (initial launch, window resize, device rotation).
        let mobile = ctx.viewport_rect().width() < MOBILE_MAX_WIDTH;
        if self.prev_mobile != Some(mobile) {
            self.sidebar_open = !mobile;
            self.prev_mobile = Some(mobile);
        }

        let theme = ShadcnTheme::get(&ctx);

        // ── Toolbar ─────────────────────────────────────────────────────────
        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(theme.background)
                    .inner_margin(egui::Margin::symmetric(
                        if mobile { 10 } else { 16 },
                        10,
                    ))
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ui, |ui| {
                self.show_toolbar(ui, mobile);
            });

        // ── Sidebar (inline, wide screens only) ──────────────────────────────
        if self.sidebar_open && !mobile {
            let theme = ShadcnTheme::get(&ctx);
            egui::Panel::left("sidebar")
                .exact_size(200.0)
                .frame(
                    egui::Frame::new()
                        .fill(theme.card)
                        .inner_margin(egui::Margin::symmetric(8, 12))
                        .stroke(egui::Stroke::new(1.0, theme.border)),
                )
                .show(ui, |ui| {
                    self.show_sidebar(ui);
                });
        }

        // ── Content ─────────────────────────────────────────────────────────
        let theme = ShadcnTheme::get(&ctx);
        let content_margin = if mobile { 12 } else { 24 };
        egui::Frame::new().fill(theme.background).show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("main_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(
                            content_margin,
                            0,
                        ))
                        .show(ui, |ui| {
                            self.render_all_sections(ui);
                        });
                    Spacing::Xl3.show(ui);
                });
        });

        // ── Sidebar (overlay, narrow screens — drawn above content) ───────────
        if self.sidebar_open && mobile {
            self.show_sidebar_overlay(&ctx);
        }

        // ── Dialog (floating) ────────────────────────────────────────────────
        self.render_dialog(&ctx);
        Toaster::show(&ctx);
    }
}

// ── Toolbar & Sidebar ─────────────────────────────────────────────────────────

impl DemoApp {
    fn show_toolbar(&mut self, ui: &mut egui::Ui, mobile: bool) {
        let theme = ShadcnTheme::get(ui.ctx());

        ui.horizontal(|ui| {
            // Pin the row height to the tallest control (the 36px Select) *before*
            // adding any items, so egui vertically centers every element — icons
            // included — against the same height. Without this, the icons are laid
            // out before the taller Select grows the row and end up top-aligned.
            ui.set_min_height(36.0);
            let hamburger_resp = ui.add(
                egui::Label::new(
                    egui::RichText::new(ICON_MENU)
                        .font(egui::FontId::new(
                            20.0,
                            egui::FontFamily::Name("MaterialIcons".into()),
                        ))
                        .color(theme.foreground),
                )
                .sense(egui::Sense::click()),
            );
            if hamburger_resp.clicked() {
                self.sidebar_open = !self.sidebar_open;
            }
            if hamburger_resp.hovered() {
                ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            }

            Spacing::Sm.show(ui);
            heading4(ui, tr!(t::Toolbar::AppName).as_ref());
            // Subtitle is the first thing to drop on a narrow toolbar so the name
            // and the right-aligned controls keep their room.
            if !mobile {
                Spacing::Sm.show(ui);
                muted_text(ui, tr!(t::Toolbar::AppSubtitle).as_ref());
            }

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Dark/light toggle
                let moon_sun = if self.dark {
                    ICON_BRIGHTNESS_7
                } else {
                    ICON_BRIGHTNESS_4
                };
                let icon_resp = ui.add(
                    egui::Label::new(
                        egui::RichText::new(moon_sun)
                            .font(egui::FontId::new(
                                20.0,
                                egui::FontFamily::Name("MaterialIcons".into()),
                            ))
                            .color(theme.foreground),
                    )
                    .sense(egui::Sense::click()),
                );
                if icon_resp.clicked() {
                    self.dark = !self.dark;
                }
                if icon_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                Spacing::Sm.show(ui);

                // Language switcher
                let locale_labels: Vec<&str> = LOCALES.iter().map(|(_, n)| *n).collect();
                let mut idx = self.locale_idx;
                if Select::new(&mut idx, &locale_labels).width(64.0).show(ui)
                    && let Some(i) = idx
                {
                    let lang = locale_idx_to_lang(i);
                    self.locale_idx = Some(i);
                    i18n::set_language(lang);
                    t::ensure_loaded(lang);
                    ui.ctx().data_mut(|d| {
                        d.insert_persisted(egui::Id::new("demo_locale_idx"), i);
                    });
                }

                Spacing::Sm.show(ui);

                // Theme color picker via Popover
                let palette_color = self
                    .primary_hue
                    .map(|h| egui_sc::egui_components::theme::hsl(h, 0.8, 0.55))
                    .unwrap_or(theme.foreground);
                let dark = self.dark;

                Popover::new("hue_picker").width(260.0).show(
                    ui,
                    |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(ICON_PALETTE)
                                    .font(egui::FontId::new(
                                        20.0,
                                        egui::FontFamily::Name("MaterialIcons".into()),
                                    ))
                                    .color(palette_color),
                            )
                            .sense(egui::Sense::click()),
                        )
                    },
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        heading4(ui, tr!(t::Toolbar::PrimaryColor).as_ref());
                        Spacing::Md.show(ui);

                        let presets: &[(Option<f32>, t::Toolbar)] = &[
                            (None, t::Toolbar::ColorZinc),
                            (Some(0.0), t::Toolbar::ColorRed),
                            (Some(25.0), t::Toolbar::ColorOrange),
                            (Some(48.0), t::Toolbar::ColorYellow),
                            (Some(142.0), t::Toolbar::ColorGreen),
                            (Some(217.0), t::Toolbar::ColorBlue),
                            (Some(263.0), t::Toolbar::ColorViolet),
                            (Some(300.0), t::Toolbar::ColorPink),
                        ];

                        ui.horizontal_wrapped(|ui| {
                            for (hue, name_variant) in presets {
                                let color = match hue {
                                    None => egui_sc::egui_components::theme::hsl(
                                        240.0,
                                        0.059,
                                        if dark { 0.5 } else { 0.3 },
                                    ),
                                    Some(h) => egui_sc::egui_components::theme::hsl(*h, 0.8, 0.55),
                                };
                                let is_sel = *hue == self.primary_hue;
                                let (swatch_rect, swatch_resp) = ui.allocate_exact_size(
                                    egui::Vec2::splat(28.0),
                                    egui::Sense::click(),
                                );
                                ui.painter()
                                    .circle_filled(swatch_rect.center(), 12.0, color);
                                if is_sel {
                                    ui.painter().circle_stroke(
                                        swatch_rect.center(),
                                        14.0,
                                        egui::Stroke::new(2.0, theme.foreground),
                                    );
                                }
                                if swatch_resp.hovered() {
                                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                                    let name = tr!(*name_variant);
                                    ui.painter().text(
                                        swatch_rect.center_bottom() + egui::Vec2::new(0.0, 4.0),
                                        egui::Align2::CENTER_TOP,
                                        name.as_ref(),
                                        egui::FontId::new(10.0, egui::FontFamily::Proportional),
                                        theme.foreground,
                                    );
                                }
                                if swatch_resp.clicked() {
                                    self.primary_hue = *hue;
                                }
                            }
                        });

                        Spacing::Md.show(ui);
                        small_text(ui, tr!(t::Toolbar::CustomHue).as_ref());
                        let mut hue = self.primary_hue.unwrap_or(240.0);
                        let hue_resp = Slider::new(&mut hue, 0.0, 360.0).show(ui);
                        if hue_resp.dragged() || hue_resp.changed() {
                            self.primary_hue = Some(hue);
                        }

                        Spacing::Sm.show(ui);
                        let reset_label = tr!(t::Toolbar::ResetToZinc);
                        if Button::new(reset_label.as_ref())
                            .variant(ButtonVariant::Secondary)
                            .show(ui)
                            .clicked()
                        {
                            self.primary_hue = None;
                        }

                        // ── Animation controls (velocity / disable) ───────────
                        Spacing::Lg.show(ui);
                        Separator::horizontal().show(ui);
                        Spacing::Md.show(ui);
                        heading4(ui, "Animations");
                        Spacing::Sm.show(ui);
                        Switch::new(&mut self.anim_enabled)
                            .label("Enabled")
                            .show(ui);
                        Spacing::Md.show(ui);
                        small_text(ui, "Speed");
                        let _ = Slider::new(&mut self.anim_speed, 0.25, 3.0).show(ui);
                    },
                );
            });
        });
    }

    /// Renders the section list. Returns `true` if the user picked a section this
    /// frame — the mobile overlay uses that to auto-close after navigation.
    fn show_sidebar(&mut self, ui: &mut egui::Ui) -> bool {
        let theme = ShadcnTheme::get(ui.ctx());

        // Idle period before sidebar auto-scrolls to track the active section.
        // Resets whenever the cursor sits inside the sidebar.
        const AUTO_SCROLL_TIMEOUT: f64 = 1.5;

        let now = ui.input(|i| i.time);
        let sidebar_rect = ui.max_rect();
        let pointer_over_sidebar = ui.ctx().input(|i| {
            i.pointer
                .hover_pos()
                .map(|p| sidebar_rect.contains(p))
                .unwrap_or(false)
        });
        if pointer_over_sidebar {
            self.sidebar_last_interaction = now;
        }
        let elapsed = now - self.sidebar_last_interaction;
        let can_auto_scroll = elapsed >= AUTO_SCROLL_TIMEOUT;
        if !pointer_over_sidebar && !can_auto_scroll {
            ui.ctx()
                .request_repaint_after(std::time::Duration::from_secs_f64(
                    (AUTO_SCROLL_TIMEOUT - elapsed).max(0.05),
                ));
        }

        // Separator after letter groups (A→4, B→9, C→18, D→23, H–L→27, M–P→32, R–S→42)
        const SEP_AFTER: &[usize] = &[0, 4, 9, 18, 23, 27, 32, 42];

        egui::ScrollArea::vertical()
            .show(ui, |ui| {
                ui.set_width(184.0);
                let mut picked = false;

                for i in 0..SECTION_COUNT {
                    let section = t::section_name(i);
                    let is_active = self.current_section == i;
                    let bg = if is_active {
                        theme.accent
                    } else {
                        Color32::TRANSPARENT
                    };
                    let fg = if is_active {
                        theme.accent_foreground
                    } else {
                        theme.foreground
                    };

                    let (rect, resp) = ui.allocate_exact_size(
                        egui::Vec2::new(184.0, 32.0),
                        egui::Sense::click(),
                    );

                    if resp.hovered() && !is_active {
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(theme.radius as u8),
                            ShadcnTheme::with_alpha(theme.accent, 80),
                        );
                        ui.ctx()
                            .set_cursor_icon(egui::CursorIcon::PointingHand);
                    } else {
                        ui.painter().rect_filled(
                            rect,
                            egui::CornerRadius::same(theme.radius as u8),
                            bg,
                        );
                    }

                    ui.painter().text(
                        egui::Pos2::new(rect.left() + 12.0, rect.center().y),
                        egui::Align2::LEFT_CENTER,
                        section.as_ref(),
                        egui::FontId::new(14.0, egui::FontFamily::Proportional),
                        fg,
                    );

                    if is_active && can_auto_scroll {
                        let visible = ui.clip_rect();
                        if !visible.contains(rect.center()) {
                            resp.scroll_to_me(Some(egui::Align::Center));
                        }
                    }

                    if resp.clicked() {
                        self.current_section = i;
                        self.scroll_to_section = Some(i);
                        self.sidebar_last_interaction = now;
                        resp.surrender_focus();
                        picked = true;
                    }

                    if SEP_AFTER.contains(&i) {
                        Spacing::Xs.show(ui);
                        Separator::horizontal().show(ui);
                        Spacing::Xs.show(ui);
                    } else {
                        ui.add_space(2.0);
                    }
                }

                picked
            })
            .inner
    }

    /// Mobile sidebar: a left-anchored drawer floating above the content with a
    /// dim scrim. Tapping the scrim or choosing a section closes it.
    fn show_sidebar_overlay(&mut self, ctx: &egui::Context) {
        let theme = ShadcnTheme::get(ctx);

        // Dim scrim. Order::Middle puts it above the content panels; the drawer
        // window below uses Order::Foreground so it always sits above the scrim.
        let scrim = egui::LayerId::new(
            egui::Order::Middle,
            egui::Id::new("sidebar_scrim"),
        );
        ctx.layer_painter(scrim).rect_filled(
            egui::Rect::EVERYTHING,
            0.0,
            Color32::from_black_alpha(120),
        );

        let screen = ctx.viewport_rect();
        let width = 260.0_f32.min(screen.width() * 0.85);

        let win = egui::Window::new("sidebar_overlay")
            .id(egui::Id::new("sidebar_overlay_win"))
            .order(egui::Order::Foreground)
            .title_bar(false)
            .resizable(false)
            .collapsible(false)
            .fixed_size(egui::Vec2::new(width, screen.height()))
            .anchor(egui::Align2::LEFT_TOP, [0.0, 0.0])
            .frame(
                egui::Frame::new()
                    .fill(theme.card)
                    .inner_margin(egui::Margin::symmetric(8, 12))
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show(ctx, |ui| self.show_sidebar(ui));

        let picked = win.as_ref().and_then(|r| r.inner).unwrap_or(false);

        // Tap outside the drawer closes it.
        if ctx.input(|i| i.pointer.primary_pressed()) {
            let pos = ctx.input(|i| i.pointer.interact_pos());
            let inside = matches!((win.as_ref(), pos), (Some(r), Some(p)) if r.response.rect.contains(p));
            if !inside {
                self.sidebar_open = false;
            }
        }

        // Choosing a section also dismisses the drawer (standard mobile nav).
        if picked {
            self.sidebar_open = false;
        }
    }
}

// ── Section routing ───────────────────────────────────────────────────────────

impl DemoApp {
    fn render_section(&mut self, ui: &mut egui::Ui, index: usize) {
        match index {
            0 => self.section_overview(ui),
            1 => self.section_accordion(ui),
            2 => self.section_alert(ui),
            3 => self.section_alert_dialog(ui),
            4 => self.section_avatar(ui),
            5 => self.section_badge(ui),
            6 => self.section_boxed(ui),
            7 => self.section_breadcrumb(ui),
            8 => self.section_button(ui),
            9 => self.section_button_group(ui),
            10 => self.section_calendar(ui),
            11 => self.section_card(ui),
            12 => self.section_carousel(ui),
            13 => self.section_chart(ui),
            14 => self.section_checkbox(ui),
            15 => self.section_collapsible(ui),
            16 => self.section_combobox(ui),
            17 => self.section_command(ui),
            18 => self.section_context_menu(ui),
            19 => self.section_data_table(ui),
            20 => self.section_date_picker(ui),
            21 => self.section_dialog(ui),
            22 => self.section_drawer(ui),
            23 => self.section_dropdown_menu(ui),
            24 => self.section_hover_card(ui),
            25 => self.section_input(ui),
            26 => self.section_input_otp(ui),
            27 => self.section_label(ui),
            28 => self.section_menubar(ui),
            29 => self.section_navigation_menu(ui),
            30 => self.section_pagination(ui),
            31 => self.section_popover(ui),
            32 => self.section_progress(ui),
            33 => self.section_radio(ui),
            34 => self.section_resizable(ui),
            35 => self.section_select(ui),
            36 => self.section_separator(ui),
            37 => self.section_sheet(ui),
            38 => self.section_skeleton(ui),
            39 => self.section_slider(ui),
            40 => self.section_spacing(ui),
            41 => self.section_spinner(ui),
            42 => self.section_switch(ui),
            43 => self.section_table(ui),
            44 => self.section_tabs(ui),
            45 => self.section_textarea(ui),
            46 => self.section_toast(ui),
            47 => self.section_toggle(ui),
            48 => self.section_toggle_group(ui),
            49 => self.section_time_picker(ui),
            50 => self.section_tooltip(ui),
            51 => self.section_typography(ui),
            _ => {}
        }
    }

    fn render_all_sections(&mut self, ui: &mut egui::Ui) {
        let clip_top = ui.clip_rect().min.y;
        let was_scrolling = self.scroll_to_section.is_some();

        for i in 0..SECTION_COUNT {
            Spacing::Xl.show(ui);

            // Auto-highlight sidebar: last section whose top is at or above viewport top
            let section_top = ui.cursor().top();
            if !was_scrolling && section_top <= clip_top + 40.0 {
                self.current_section = i;
            }

            // Scroll-to anchor
            let (_, anchor) =
                ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover());
            if self.scroll_to_section == Some(i) {
                anchor.scroll_to_me(Some(egui::Align::TOP));
                self.scroll_to_section = None;
            }

            self.render_section(ui, i);

            if i < SECTION_COUNT - 1 {
                Spacing::Xl2.show(ui);
                Separator::horizontal().show(ui);
            }
        }
    }

    pub(super) fn section_title(
        &self,
        ui: &mut egui::Ui,
        title: &str,
        subtitle: &str,
    ) {
        heading2(ui, title);
        Spacing::Xs.show(ui);
        muted_text(ui, subtitle);
        Spacing::Xl.show(ui);
        Separator::horizontal().show(ui);
        Spacing::Xl.show(ui);
    }

    fn render_dialog(&mut self, ctx: &egui::Context) {
        let mut close = false;
        let input = &mut self.dialog_input;

        let title = tr!(t::DemoDialog::Title);
        let body = tr!(t::DemoDialog::Body);
        let type_prompt = tr!(t::DemoDialog::TypePrompt);
        let placeholder = tr!(t::DemoDialog::Placeholder);
        let cancel = tr!(t::DemoDialog::Cancel);
        let continue_lbl = tr!(t::DemoDialog::Continue);

        Dialog::new(title.as_ref(), &mut self.dialog_open).show(ctx, |ui| {
            muted_text(ui, body.as_ref());
            Spacing::Lg.show(ui);
            body_text(ui, type_prompt.as_ref());
            Spacing::Xs.show(ui);
            Input::new(input).placeholder(placeholder.as_ref()).show(ui);
            Spacing::Lg.show(ui);

            let ok = input.as_str() == "CONFIRM";
            ui.horizontal(|ui| {
                if Button::new(cancel.as_ref())
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    close = true;
                }
                Spacing::Sm.show(ui);
                if Button::new(continue_lbl.as_ref())
                    .variant(ButtonVariant::Destructive)
                    .enabled(ok)
                    .show(ui)
                    .clicked()
                {
                    close = true;
                }
            });
        });

        if close {
            self.dialog_open = false;
            self.dialog_input.clear();
        }
    }
}
