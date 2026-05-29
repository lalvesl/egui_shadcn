use egui_shadcn::spacing::Spacing;
use egui::Color32;
use egui_shadcn::{
    ICON_BRIGHTNESS_4, ICON_BRIGHTNESS_7, ICON_PALETTE,
    ShadcnTheme,
    calendar::CalDate,
    dialog::Dialog,
    input::Input,
    button::{Button, ButtonVariant},
    separator::Separator,
    toast::Toaster,
    typography::{heading2, muted_text},
    slider::Slider,
};

mod sections;

// ── Nav sections ──────────────────────────────────────────────────────────────

const SECTIONS: &[&str] = &[
    "Overview",
    "Typography",
    "Buttons",
    "Badges",
    "Alerts",
    "Cards",
    "Form",
    "Inputs",
    "Feedback",
    "Navigation",
    "Overlays",
    "Data Display",
    "Calendar",
    "Toggle",
    "Toggle Group",
    "Collapsible",
    "Breadcrumb",
    "Pagination",
    "Alert Dialog",
    "Button Group",
    "Combobox",
    "Date Picker",
    "Dropdown Menu",
    "Context Menu",
    "Hover Card",
    "Input OTP",
    "Popover",
    "Table",
    "Data Table",
    "Sheet",
    "Drawer",
    "Toast",
    "Navigation Menu",
    "Menubar",
    "Carousel",
    "Chart",
    "Command",
    "Resizable",
    "Separator",
    "Spacing",
    "Boxed",
];

// ── App state ─────────────────────────────────────────────────────────────────

pub struct DemoApp {
    pub(super) dark: bool,
    pub(super) primary_hue: Option<f32>,
    pub(super) show_hue_picker: bool,
    pub(super) current_section: usize,

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
    pub(super) textarea_grow_x: String,
    pub(super) textarea_grow_both: String,
    pub(super) select_val: Option<usize>,
    pub(super) accordion_open: [bool; 3],
    pub(super) dialog_open: bool,
    pub(super) dialog_input: String,
    pub(super) cal_single: Option<CalDate>,
    pub(super) cal_range_start: Option<CalDate>,
    pub(super) cal_range_end: Option<CalDate>,
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
}

impl Default for DemoApp {
    fn default() -> Self {
        Self {
            dark: true,
            primary_hue: None,
            show_hue_picker: false,
            current_section: 0,
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
            textarea_grow_x: String::new(),
            textarea_grow_both: String::new(),
            select_val: None,
            accordion_open: [true, false, false],
            dialog_open: false,
            dialog_input: String::new(),
            cal_single: None,
            cal_range_start: None,
            cal_range_end: None,
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

        let app = Self { dark, primary_hue: hue, ..Default::default() };

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

        // ── Content ─────────────────────────────────────────────────────────
        let theme = ShadcnTheme::get(&ctx);
        egui::Frame::new().fill(theme.background).show(ui, |ui| {
            egui::ScrollArea::vertical()
                .id_salt(self.current_section)
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    Spacing::Xl.show(ui);
                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(24, 0))
                        .show(ui, |ui| {
                            self.render_section(ui);
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
            ui.label(
                egui::RichText::new("egui-shadcn")
                    .font(egui::FontId::new(18.0, egui::FontFamily::Proportional))
                    .color(theme.foreground)
                    .strong(),
            );
            ui.label(
                egui::RichText::new("component demo")
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.muted_foreground),
            );

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

                // Color picker icon
                let palette_color = self
                    .primary_hue
                    .map(|h| egui_shadcn::theme::hsl(h, 0.8, 0.55))
                    .unwrap_or(theme.foreground);
                let palette_resp = ui.add(
                    egui::Label::new(
                        egui::RichText::new(ICON_PALETTE)
                            .font(egui::FontId::new(
                                20.0,
                                egui::FontFamily::Name("MaterialIcons".into()),
                            ))
                            .color(palette_color),
                    )
                    .sense(egui::Sense::click()),
                );
                if palette_resp.clicked() {
                    self.show_hue_picker = !self.show_hue_picker;
                }
                if palette_resp.hovered() {
                    ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
                }

                if self.show_hue_picker {
                    let theme = ShadcnTheme::get(ui.ctx());
                    let dark = self.dark;
                    egui::Window::new("Primary Color")
                        .id(egui::Id::new("hue_picker"))
                        .collapsible(false)
                        .resizable(false)
                        .default_width(260.0)
                        .anchor(egui::Align2::RIGHT_TOP, [-8.0, 48.0])
                        .frame(
                            egui::Frame::new()
                                .fill(theme.card)
                                .stroke(egui::Stroke::new(1.0, theme.border))
                                .corner_radius(egui::CornerRadius::same(theme.radius as u8))
                                .inner_margin(egui::Margin::same(16)),
                        )
                        .title_bar(false)
                        .show(ui.ctx(), |ui| {
                            ui.label(
                                egui::RichText::new("Primary Color")
                                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                                    .color(theme.foreground)
                                    .strong(),
                            );
                            Spacing::Md.show(ui);

                            let presets: &[(Option<f32>, &str)] = &[
                                (None, "Zinc"),
                                (Some(0.0), "Red"),
                                (Some(25.0), "Orange"),
                                (Some(48.0), "Yellow"),
                                (Some(142.0), "Green"),
                                (Some(217.0), "Blue"),
                                (Some(263.0), "Violet"),
                                (Some(300.0), "Pink"),
                            ];

                            ui.horizontal_wrapped(|ui| {
                                for (hue, name) in presets {
                                    let color = match hue {
                                        None => egui_shadcn::theme::hsl(
                                            240.0,
                                            0.059,
                                            if dark { 0.5 } else { 0.3 },
                                        ),
                                        Some(h) => egui_shadcn::theme::hsl(*h, 0.8, 0.55),
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
                                        ui.painter().text(
                                            swatch_rect.center_bottom() + egui::Vec2::new(0.0, 4.0),
                                            egui::Align2::CENTER_TOP,
                                            *name,
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
                            let mut hue = self.primary_hue.unwrap_or(240.0);
                            ui.label(
                                egui::RichText::new("Custom hue")
                                    .font(egui::FontId::new(12.0, egui::FontFamily::Proportional))
                                    .color(theme.muted_foreground),
                            );
                            let hue_resp = Slider::new(&mut hue, 0.0, 360.0).show(ui);
                            if hue_resp.dragged() || hue_resp.changed() {
                                self.primary_hue = Some(hue);
                            }

                            Spacing::Sm.show(ui);
                            if ui.button("Reset to Zinc").clicked() {
                                self.primary_hue = None;
                                self.show_hue_picker = false;
                            }
                        });
                }
            });
        });
    }

    fn show_sidebar(&mut self, ui: &mut egui::Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        egui::ScrollArea::vertical().show(ui, |ui| {
            ui.set_width(184.0);

            for (i, section) in SECTIONS.iter().enumerate() {
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
                    ui.painter().rect_filled(
                        rect,
                        egui::CornerRadius::same(theme.radius as u8),
                        bg,
                    );
                }

                ui.painter().text(
                    egui::Pos2::new(rect.left() + 12.0, rect.center().y),
                    egui::Align2::LEFT_CENTER,
                    *section,
                    egui::FontId::new(14.0, egui::FontFamily::Proportional),
                    fg,
                );

                if resp.clicked() {
                    self.current_section = i;
                }
                ui.add_space(2.0);
            }
        });
    }
}

// ── Section routing ───────────────────────────────────────────────────────────

impl DemoApp {
    fn render_section(&mut self, ui: &mut egui::Ui) {
        match self.current_section {
            0 => self.section_overview(ui),
            1 => self.section_typography(ui),
            2 => self.section_buttons(ui),
            3 => self.section_badges(ui),
            4 => self.section_alerts(ui),
            5 => self.section_cards(ui),
            6 => self.section_form(ui),
            7 => self.section_inputs(ui),
            8 => self.section_feedback(ui),
            9 => self.section_navigation(ui),
            10 => self.section_overlays(ui),
            11 => self.section_data_display(ui),
            12 => self.section_calendar(ui),
            13 => self.section_toggle(ui),
            14 => self.section_toggle_group(ui),
            15 => self.section_collapsible(ui),
            16 => self.section_breadcrumb(ui),
            17 => self.section_pagination(ui),
            18 => self.section_alert_dialog(ui),
            19 => self.section_button_group(ui),
            20 => self.section_combobox(ui),
            21 => self.section_date_picker(ui),
            22 => self.section_dropdown_menu(ui),
            23 => self.section_context_menu(ui),
            24 => self.section_hover_card(ui),
            25 => self.section_input_otp(ui),
            26 => self.section_popover(ui),
            27 => self.section_table(ui),
            28 => self.section_data_table(ui),
            29 => self.section_sheet(ui),
            30 => self.section_drawer(ui),
            31 => self.section_toast(ui),
            32 => self.section_navigation_menu(ui),
            33 => self.section_menubar(ui),
            34 => self.section_carousel(ui),
            35 => self.section_chart(ui),
            36 => self.section_command(ui),
            37 => self.section_resizable(ui),
            38 => self.section_separator(ui),
            39 => self.section_spacing(ui),
            40 => self.section_boxed(ui),
            _ => {}
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

        Dialog::new("Confirm Action", &mut self.dialog_open).show(ctx, |ui| {
            let theme = ShadcnTheme::get(ui.ctx());
            ui.label(
                egui::RichText::new(
                    "Are you sure you want to perform this action? This cannot be undone.",
                )
                .font(egui::FontId::new(13.0, egui::FontFamily::Proportional))
                .color(theme.muted_foreground),
            );
            Spacing::Lg.show(ui);
            ui.label(
                egui::RichText::new("Type CONFIRM to proceed")
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground),
            );
            Spacing::Xs.show(ui);
            Input::new(input).placeholder("CONFIRM").show(ui);
            Spacing::Lg.show(ui);

            let ok = input.as_str() == "CONFIRM";
            ui.horizontal(|ui| {
                if Button::new("Cancel")
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    close = true;
                }
                Spacing::Sm.show(ui);
                if Button::new("Continue")
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
