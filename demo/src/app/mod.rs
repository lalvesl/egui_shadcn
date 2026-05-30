use egui_shadcn::spacing::Spacing;
use egui::Color32;
use egui_shadcn::{
    ICON_BRIGHTNESS_4, ICON_BRIGHTNESS_7, ICON_MENU, ICON_PALETTE,
    ShadcnTheme,
    calendar::CalDate,
    dialog::Dialog,
    i18n::{self, Languages},
    input::Input,
    button::{Button, ButtonVariant},
    popover::Popover,
    select::Select,
    separator::Separator,
    toast::Toaster,
    typography::{body_text, heading2, heading4, muted_text, small_text},
    slider::Slider,
};
use crate::i18n as t;
use ::i18n::t as tr;

const LOCALES: &[(Languages, &str)] = &[
    (Languages::EnUs, "EN"),
    (Languages::PtBr, "PT"),
];

fn locale_idx_to_lang(idx: usize) -> Languages {
    LOCALES.get(idx).map(|(l, _)| *l).unwrap_or(Languages::EnUs)
}

mod sections;

// ── Nav sections ──────────────────────────────────────────────────────────────

const SECTION_COUNT: usize = t::SECTION_COUNT;

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
    pub(super) sidebar_needs_scroll: bool,
    pub(super) locale_idx: Option<usize>,
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
            sidebar_needs_scroll: false,
            locale_idx: Some(0),
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

        i18n::set_language(locale_idx_to_lang(locale_idx));

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

    fn apply_theme(&self, ctx: &egui::Context) {
        let theme = ShadcnTheme::build(self.dark, self.primary_hue);
        ShadcnTheme::set(ctx, theme.clone());
        theme.apply(ctx);

        ctx.data_mut(|d| {
            d.insert_persisted(egui::Id::new("demo_dark"), self.dark);
            d.insert_persisted(egui::Id::new("demo_hue"), self.primary_hue);
        });
    }
}

// ── eframe::App ───────────────────────────────────────────────────────────────

impl eframe::App for DemoApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        self.apply_theme(ui.ctx());
        let ctx = ui.ctx().clone();

        let theme = ShadcnTheme::get(&ctx);

        // ── Toolbar ─────────────────────────────────────────────────────────
        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(theme.background)
                    .inner_margin(egui::Margin::symmetric(16, 10))
                    .stroke(egui::Stroke::new(1.0, theme.border)),
            )
            .show_inside(ui, |ui| {
                self.show_toolbar(ui);
            });

        // ── Sidebar ─────────────────────────────────────────────────────────
        if self.sidebar_open {
            let theme = ShadcnTheme::get(&ctx);
            egui::Panel::left("sidebar")
                .exact_size(200.0)
                .frame(
                    egui::Frame::new()
                        .fill(theme.card)
                        .inner_margin(egui::Margin::symmetric(8, 12))
                        .stroke(egui::Stroke::new(1.0, theme.border)),
                )
                .show_inside(ui, |ui| {
                    self.show_sidebar(ui);
                });
        }

        // ── Content ─────────────────────────────────────────────────────────
        let theme = ShadcnTheme::get(&ctx);
        egui::Frame::new().fill(theme.background).show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt("main_scroll")
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(24, 0))
                        .show(ui, |ui| {
                            self.render_all_sections(ui);
                        });
                    Spacing::Xl3.show(ui);
                });
        });

        // ── Dialog (floating) ────────────────────────────────────────────────
        self.render_dialog(&ctx);
        Toaster::show(&ctx);
    }
}

// ── Toolbar & Sidebar ─────────────────────────────────────────────────────────

impl DemoApp {
    fn show_toolbar(&mut self, ui: &mut egui::Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        ui.horizontal(|ui| {
            let hamburger_resp = ui.add(
                egui::Label::new(
                    egui::RichText::new(ICON_MENU)
                        .font(egui::FontId::new(20.0, egui::FontFamily::Name("MaterialIcons".into())))
                        .color(theme.foreground),
                )
                .sense(egui::Sense::click()),
            );
            if hamburger_resp.clicked() { self.sidebar_open = !self.sidebar_open; }
            if hamburger_resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }

            Spacing::Sm.show(ui);
            heading4(ui, tr!(t::Toolbar::AppName).as_ref());
            Spacing::Sm.show(ui);
            muted_text(ui, tr!(t::Toolbar::AppSubtitle).as_ref());

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Dark/light toggle
                let moon_sun = if self.dark { ICON_BRIGHTNESS_7 } else { ICON_BRIGHTNESS_4 };
                let icon_resp = ui.add(
                    egui::Label::new(
                        egui::RichText::new(moon_sun)
                            .font(egui::FontId::new(20.0, egui::FontFamily::Name("MaterialIcons".into())))
                            .color(theme.foreground),
                    )
                    .sense(egui::Sense::click()),
                );
                if icon_resp.clicked() { self.dark = !self.dark; }
                if icon_resp.hovered() { ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand); }

                Spacing::Sm.show(ui);

                // Language switcher
                let locale_labels: Vec<&str> = LOCALES.iter().map(|(_, n)| *n).collect();
                let mut idx = self.locale_idx;
                if Select::new(&mut idx, &locale_labels).width(64.0).show(ui)
                    && let Some(i) = idx
                {
                    self.locale_idx = Some(i);
                    i18n::set_language(locale_idx_to_lang(i));
                    ui.ctx().data_mut(|d| {
                        d.insert_persisted(egui::Id::new("demo_locale_idx"), i);
                    });
                }

                Spacing::Sm.show(ui);

                // Theme color picker via Popover
                let palette_color = self.primary_hue
                    .map(|h| egui_shadcn::theme::hsl(h, 0.8, 0.55))
                    .unwrap_or(theme.foreground);
                let dark = self.dark;

                Popover::new("hue_picker").width(260.0).show(
                    ui,
                    |ui| {
                        ui.add(
                            egui::Label::new(
                                egui::RichText::new(ICON_PALETTE)
                                    .font(egui::FontId::new(20.0, egui::FontFamily::Name("MaterialIcons".into())))
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
                            (None,         t::Toolbar::ColorZinc),
                            (Some(0.0),    t::Toolbar::ColorRed),
                            (Some(25.0),   t::Toolbar::ColorOrange),
                            (Some(48.0),   t::Toolbar::ColorYellow),
                            (Some(142.0),  t::Toolbar::ColorGreen),
                            (Some(217.0),  t::Toolbar::ColorBlue),
                            (Some(263.0),  t::Toolbar::ColorViolet),
                            (Some(300.0),  t::Toolbar::ColorPink),
                        ];

                        ui.horizontal_wrapped(|ui| {
                            for (hue, name_variant) in presets {
                                let color = match hue {
                                    None => egui_shadcn::theme::hsl(240.0, 0.059, if dark { 0.5 } else { 0.3 }),
                                    Some(h) => egui_shadcn::theme::hsl(*h, 0.8, 0.55),
                                };
                                let is_sel = *hue == self.primary_hue;
                                let (swatch_rect, swatch_resp) = ui.allocate_exact_size(
                                    egui::Vec2::splat(28.0),
                                    egui::Sense::click(),
                                );
                                ui.painter().circle_filled(swatch_rect.center(), 12.0, color);
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
                        if Button::new(reset_label.as_ref()).variant(ButtonVariant::Secondary).show(ui).clicked() {
                            self.primary_hue = None;
                        }
                    },
                );
            });
        });
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        // Separator after letter groups (A→4, B→9, C→18, D→23, H–L→27, M–P→32, R–S→42)
        const SEP_AFTER: &[usize] = &[0, 4, 9, 18, 23, 27, 32, 42];

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(184.0);

            for i in 0..SECTION_COUNT {
                let section = t::section_name(i);
                let is_active = self.current_section == i;
                let bg = if is_active { theme.accent } else { Color32::TRANSPARENT };
                let fg = if is_active { theme.accent_foreground } else { theme.foreground };

                let (rect, resp) =
                    ui.allocate_exact_size(egui::Vec2::new(184.0, 32.0), egui::Sense::click());

                if resp.hovered() && !is_active {
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(theme.radius as u8),
                        ShadcnTheme::with_alpha(theme.accent, 80),
                    );
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                } else {
                    ui.painter().rect_filled(rect, egui::CornerRadius::same(theme.radius as u8), bg);
                }

                ui.painter().text(
                    egui::Pos2::new(rect.left() + 12.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    section.as_ref(),
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                    fg,
                );

                if is_active && self.sidebar_needs_scroll {
                    resp.scroll_to_me(Some(egui::Align::Center));
                    self.sidebar_needs_scroll = false;
                }

                if resp.clicked() {
                    self.current_section = i;
                    self.scroll_to_section = Some(i);
                }

                if SEP_AFTER.contains(&i) {
                    Spacing::Xs.show(ui);
                    Separator::horizontal().show(ui);
                    Spacing::Xs.show(ui);
                } else {
                    ui.add_space(2.0);
                }
            }
        });
    }
}

// ── Section routing ───────────────────────────────────────────────────────────

impl DemoApp {
    fn render_section(&mut self, ui: &mut egui::Ui, index: usize) {
        match index {
            0  => self.section_overview(ui),
            1  => self.section_accordion(ui),
            2  => self.section_alert(ui),
            3  => self.section_alert_dialog(ui),
            4  => self.section_avatar(ui),
            5  => self.section_badge(ui),
            6  => self.section_boxed(ui),
            7  => self.section_breadcrumb(ui),
            8  => self.section_button(ui),
            9  => self.section_button_group(ui),
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
            49 => self.section_tooltip(ui),
            50 => self.section_typography(ui),
            _  => {}
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
                if self.current_section != i {
                    self.sidebar_needs_scroll = true;
                    // show_sidebar already ran this frame; force next frame so
                    // the sidebar can consume sidebar_needs_scroll immediately.
                    ui.ctx().request_repaint();
                }
                self.current_section = i;
            }

            // Scroll-to anchor
            let (_, anchor) = ui.allocate_exact_size(egui::Vec2::ZERO, egui::Sense::hover());
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

    pub(super) fn section_title(&self, ui: &mut egui::Ui, title: &str, subtitle: &str) {
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
