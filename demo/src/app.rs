use egui::Color32;
use egui_shadcn::{
    ICON_BRIGHTNESS_4, ICON_BRIGHTNESS_7, ICON_PALETTE, ShadcnTheme,
    accordion::Accordion,
    alert::{Alert, AlertVariant},
    avatar::{Avatar, AvatarSize},
    badge::{Badge, BadgeVariant},
    button::{Button, ButtonSize, ButtonVariant},
    calendar::{CalDate, Calendar},
    card::{Card, card_header},
    checkbox::Checkbox,
    dialog::Dialog,
    input::Input,
    label::Label,
    progress::Progress,
    radio::Radio,
    select::Select,
    separator::Separator,
    skeleton::Skeleton,
    slider::Slider,
    spinner::Spinner,
    switch::Switch,
    tabs::Tabs,
    textarea::Textarea,
    tooltip::Tooltip,
    typography::*,
};

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
];

// ── App state ─────────────────────────────────────────────────────────────────

pub struct DemoApp {
    dark: bool,
    primary_hue: Option<f32>,
    show_hue_picker: bool,
    current_section: usize,

    btn_clicked: bool,
    checkbox1: bool,
    checkbox2: bool,
    checkbox3: bool,
    switch1: bool,
    switch2: bool,
    slider_val: f32,
    progress_val: f32,
    radio_val: u32,
    tab_index: usize,
    input_text: String,
    password_text: String,
    textarea_text: String,
    textarea_scroll: String,
    textarea_grow_y: String,
    textarea_grow_x: String,
    textarea_grow_both: String,
    select_val: Option<usize>,
    accordion_open: [bool; 3],
    dialog_open: bool,
    dialog_input: String,
    cal_single: Option<CalDate>,
    cal_range_start: Option<CalDate>,
    cal_range_end: Option<CalDate>,
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
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    ui.add_space(24.0);
                    egui::Frame::new()
                        .inner_margin(egui::Margin::symmetric(24, 0))
                        .show(ui, |ui| {
                            self.render_section(ui);
                        });
                    ui.add_space(48.0);
                });
        });

        // ── Dialog (floating) ────────────────────────────────────────────────
        self.render_dialog(&ctx);
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

                ui.add_space(8.0);

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
                            ui.add_space(12.0);

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

                            ui.add_space(12.0);
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

                            ui.add_space(8.0);
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
            _ => {}
        }
    }

    fn section_title(&self, ui: &mut egui::Ui, title: &str, subtitle: &str) {
        heading2(ui, title);
        ui.add_space(4.0);
        muted_text(ui, subtitle);
        ui.add_space(24.0);
        Separator::horizontal().show(ui);
        ui.add_space(24.0);
    }

    // ── Sections ──────────────────────────────────────────────────────────────

    fn section_overview(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "egui-shadcn", "Shadcn/ui components for Rust egui.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Welcome", Some("A beautiful, accessible component library."));
            muted_text(ui, "Use the sidebar to explore all components. Toggle dark mode or pick a primary color with the toolbar icons above.");
            ui.add_space(16.0);
            ui.horizontal(|ui| {
                Badge::new("v0.1.0").show(ui);
                ui.add_space(4.0);
                Badge::new("egui 0.34").variant(BadgeVariant::Secondary).show(ui);
                ui.add_space(4.0);
                Badge::new("Rust").variant(BadgeVariant::Outline).show(ui);
            });
        });

        ui.add_space(24.0);
        let theme = ShadcnTheme::get(ui.ctx());

        ui.horizontal(|ui| {
            for (n, desc) in [("19", "Components"), ("2", "Themes")] {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(n)
                            .font(egui::FontId::new(28.0, egui::FontFamily::Proportional))
                            .color(theme.primary)
                            .strong(),
                    );
                    muted_text(ui, desc);
                });
                ui.add_space(12.0);
            }
        });
        ui.add_space(12.0);
        ui.horizontal(|ui| {
            for (n, desc) in [("8", "Color presets"), ("∞", "Customizable")] {
                Card::new().padding(16.0).show(ui, |ui| {
                    ui.label(
                        egui::RichText::new(n)
                            .font(egui::FontId::new(28.0, egui::FontFamily::Proportional))
                            .color(theme.primary)
                            .strong(),
                    );
                    muted_text(ui, desc);
                });
                ui.add_space(12.0);
            }
        });
    }

    fn section_typography(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Typography", "Text styles and type scale.");
        Card::new().show(ui, |ui| {
            heading1(ui, "Heading 1");
            ui.add_space(8.0);
            heading2(ui, "Heading 2");
            ui.add_space(8.0);
            heading3(ui, "Heading 3");
            ui.add_space(8.0);
            heading4(ui, "Heading 4");
            ui.add_space(8.0);
            lead_text(ui, "Lead paragraph — larger, muted text for introductions.");
            ui.add_space(8.0);
            body_text(ui, "Body text — the default paragraph size.");
            ui.add_space(8.0);
            muted_text(ui, "Muted text — de-emphasised, helper copy.");
            ui.add_space(8.0);
            small_text(ui, "Small text — footnotes and captions.");
            ui.add_space(8.0);
            ui.horizontal(|ui| {
                body_text(ui, "Inline ");
                code_text(ui, "code_snippet");
                body_text(ui, " inside a sentence.");
            });
        });
    }

    fn section_buttons(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Button", "Trigger actions and events.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Variants", None);
            ui.horizontal_wrapped(|ui| {
                if Button::new("Default").show(ui).clicked() {
                    self.btn_clicked = true;
                }
                ui.add_space(8.0);
                Button::new("Destructive")
                    .variant(ButtonVariant::Destructive)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Outline")
                    .variant(ButtonVariant::Outline)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Secondary")
                    .variant(ButtonVariant::Secondary)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui);
                ui.add_space(8.0);
                Button::new("Link").variant(ButtonVariant::Link).show(ui);
            });
            if self.btn_clicked {
                ui.add_space(8.0);
                Alert::new("Button clicked!").show(ui);
                if ui.small_button("Dismiss").clicked() {
                    self.btn_clicked = false;
                }
            }
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Sizes", None);
            ui.horizontal(|ui| {
                Button::new("Small").size(ButtonSize::Sm).show(ui);
                ui.add_space(8.0);
                Button::new("Default").show(ui);
                ui.add_space(8.0);
                Button::new("Large").size(ButtonSize::Lg).show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "States", None);
            ui.horizontal(|ui| {
                Button::new("Enabled").show(ui);
                ui.add_space(8.0);
                Button::new("Disabled").enabled(false).show(ui);
            });
        });
    }

    fn section_badges(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Badge", "Labels and status indicators.");
        Card::new().show(ui, |ui| {
            ui.horizontal(|ui| {
                Badge::new("Default").show(ui);
                ui.add_space(8.0);
                Badge::new("Secondary")
                    .variant(BadgeVariant::Secondary)
                    .show(ui);
                ui.add_space(8.0);
                Badge::new("Destructive")
                    .variant(BadgeVariant::Destructive)
                    .show(ui);
                ui.add_space(8.0);
                Badge::new("Outline")
                    .variant(BadgeVariant::Outline)
                    .show(ui);
            });
        });
    }

    fn section_alerts(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Alert", "Informational messages.");
        Alert::new("Heads up!")
            .description("This is an informational alert with a description.")
            .show(ui);
        ui.add_space(12.0);
        Alert::new("Destructive")
            .description("Something went wrong. Please check your inputs.")
            .variant(AlertVariant::Destructive)
            .show(ui);
        ui.add_space(12.0);
        Alert::new("Warning")
            .description("This action cannot be undone. Proceed with caution.")
            .variant(AlertVariant::Warning)
            .show(ui);
    }

    fn section_cards(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Card", "Container with optional header and footer.");
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Card Title",
                Some("A brief description of the card content."),
            );
            body_text(ui, "Card body content goes here.");
        });
        ui.add_space(16.0);
        Card::new().padding(16.0).show(ui, |ui| {
            card_header(ui, "User Card", None);
            ui.horizontal(|ui| {
                Avatar::new("JD").show(ui);
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    body_text(ui, "John Doe");
                    muted_text(ui, "john@example.com");
                });
            });
        });
    }

    fn section_form(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Form Controls", "Checkbox, radio, and switch.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Checkbox", None);
            Checkbox::new(&mut self.checkbox1)
                .label("Accept terms and conditions")
                .show(ui);
            ui.add_space(8.0);
            Checkbox::new(&mut self.checkbox2)
                .label("Subscribe to newsletter")
                .show(ui);
            ui.add_space(8.0);
            Checkbox::new(&mut self.checkbox3)
                .label("Disabled option")
                .enabled(false)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Radio Group", None);
            Radio::new(&mut self.radio_val, 0u32)
                .label("Option A")
                .show(ui);
            ui.add_space(8.0);
            Radio::new(&mut self.radio_val, 1u32)
                .label("Option B")
                .show(ui);
            ui.add_space(8.0);
            Radio::new(&mut self.radio_val, 2u32)
                .label("Option C")
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Switch", None);
            Switch::new(&mut self.switch1)
                .label("Enable notifications")
                .show(ui);
            ui.add_space(8.0);
            Switch::new(&mut self.switch2)
                .label("Disabled switch")
                .enabled(false)
                .show(ui);
        });
    }

    fn section_inputs(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Inputs", "Text fields and selects.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Text Input", None);
            Input::new(&mut self.input_text)
                .label("Username")
                .placeholder("Enter your username…")
                .show(ui);
            ui.add_space(12.0);
            Input::new(&mut self.password_text)
                .label("Password")
                .placeholder("••••••••")
                .password(true)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Textarea", None);

            // Default — fixed rows
            Textarea::new(&mut self.textarea_text)
                .label("Default (fixed 4 rows)")
                .placeholder("Tell us about yourself…")
                .rows(4)
                .show(ui);

            ui.add_space(12.0);

            // Scroll — fixed height, vertical scrollbar
            Textarea::new(&mut self.textarea_scroll)
                .label("Scroll (fixed 3 rows)")
                .placeholder("Keep typing — content scrolls inside…")
                .rows(3)
                .scroll(true)
                .show(ui);

            ui.add_space(12.0);

            // Grow Y — expands up to 8 rows, then scrolls
            Textarea::new(&mut self.textarea_grow_y)
                .label("Grow Y (2 → 8 rows, then scrolls)")
                .placeholder("Start typing — textarea grows…")
                .rows(2)
                .max_rows(8)
                .show(ui);

            ui.add_space(12.0);

            // Grow X — capped width (doesn't fill panel)
            Textarea::new(&mut self.textarea_grow_x)
                .label("Grow X (max width 280 px)")
                .placeholder("Width capped at 280 px…")
                .rows(3)
                .max_width(280.0)
                .show(ui);

            ui.add_space(12.0);

            // Grow both — capped width + grow Y
            Textarea::new(&mut self.textarea_grow_both)
                .label("Grow both (2 → 6 rows, max width 320 px)")
                .placeholder("Grows in height, capped in width…")
                .rows(2)
                .max_rows(6)
                .max_width(320.0)
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Select", None);
            Label::new("Framework").show(ui);
            ui.add_space(4.0);
            Select::new(&mut self.select_val, &["React", "Vue", "Svelte", "egui"])
                .placeholder("Pick a framework…")
                .show(ui);
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Slider", None);
            muted_text(ui, &format!("Value: {:.0}", self.slider_val));
            ui.add_space(8.0);
            Slider::new(&mut self.slider_val, 0.0, 100.0).show(ui);
        });
    }

    fn section_feedback(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Feedback", "Progress, spinners, and skeletons.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Progress", None);
            muted_text(ui, &format!("{:.0}%", self.progress_val * 100.0));
            ui.add_space(8.0);
            Progress::new(self.progress_val).show(ui);
            ui.add_space(12.0);
            ui.horizontal(|ui| {
                if Button::new("−10%")
                    .size(ButtonSize::Sm)
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    self.progress_val = (self.progress_val - 0.1).max(0.0);
                }
                ui.add_space(4.0);
                if Button::new("+10%").size(ButtonSize::Sm).show(ui).clicked() {
                    self.progress_val = (self.progress_val + 0.1).min(1.0);
                }
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Spinner", None);
            ui.horizontal(|ui| {
                Spinner::new().size(16.0).show(ui);
                ui.add_space(4.0);
                Spinner::new().size(24.0).show(ui);
                ui.add_space(4.0);
                Spinner::new().size(40.0).show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Skeleton", None);
            ui.horizontal(|ui| {
                Skeleton::circle(40.0).show(ui);
                ui.add_space(12.0);
                ui.vertical(|ui| {
                    Skeleton::new(180.0, 14.0).radius(4.0).show(ui);
                    ui.add_space(6.0);
                    Skeleton::new(120.0, 12.0).radius(4.0).show(ui);
                });
            });
            ui.add_space(12.0);
            Skeleton::new(ui.available_width(), 120.0)
                .radius(8.0)
                .show(ui);
        });
    }

    fn section_navigation(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Navigation", "Tabs and accordions.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Tabs", None);
            Tabs::new(
                "demo_tabs",
                &["Account", "Password", "Notifications"],
                &mut self.tab_index,
            )
            .show(ui, |ui, tab| {
                ui.add_space(8.0);
                match tab {
                    0 => body_text(ui, "Account settings go here."),
                    1 => body_text(ui, "Change your password."),
                    2 => body_text(ui, "Notification preferences."),
                    _ => {}
                }
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Accordion", None);
            Accordion::new("acc_0", "What is egui-shadcn?", &mut self.accordion_open[0]).show(
                ui,
                |ui| {
                    muted_text(
                        ui,
                        "A Shadcn/ui implementation for the egui Rust GUI library.",
                    );
                },
            );
            Accordion::new(
                "acc_1",
                "Does it work on WASM?",
                &mut self.accordion_open[1],
            )
            .show(ui, |ui| {
                muted_text(
                    ui,
                    "Yes! Both native and web targets are supported via eframe.",
                );
            });
            Accordion::new(
                "acc_2",
                "Can I customise the theme?",
                &mut self.accordion_open[2],
            )
            .show(ui, |ui| {
                muted_text(
                    ui,
                    "Use the palette icon in the toolbar to pick a primary color.",
                );
            });
        });
    }

    fn section_overlays(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Overlays", "Dialogs and tooltips.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Dialog", None);
            if Button::new("Open Dialog").show(ui).clicked() {
                self.dialog_open = true;
            }
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Tooltip", None);
            Tooltip::new("This is a helpful tooltip").wrap(ui, |ui| {
                Button::new("Hover me")
                    .variant(ButtonVariant::Outline)
                    .show(ui)
            });
        });
    }

    fn section_data_display(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Data Display", "Avatars and labels.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Avatar", None);
            ui.horizontal(|ui| {
                Avatar::new("JD").size(AvatarSize::Sm).show(ui);
                ui.add_space(8.0);
                Avatar::new("Alice").show(ui);
                ui.add_space(8.0);
                Avatar::new("Bob").size(AvatarSize::Lg).show(ui);
                ui.add_space(8.0);
                Avatar::new("XY")
                    .color(Color32::from_rgb(139, 92, 246))
                    .show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "Label", None);
            Label::new("Username").show(ui);
            ui.add_space(4.0);
            Label::new("Email").required(true).show(ui);
        });
    }

    fn section_calendar(&mut self, ui: &mut egui::Ui) {
        self.section_title(
            ui,
            "Calendar",
            "Date and range pickers with optional cell content.",
        );

        // ── Single date ──────────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(ui, "Single date", None);

            let label = match self.cal_single {
                Some(d) => format!("Selected: {:04}-{:02}-{:02}", d.year, d.month, d.day),
                None => "No date selected".to_owned(),
            };
            muted_text(ui, &label);
            ui.add_space(12.0);

            Calendar::single("demo_cal_single", &mut self.cal_single).show(ui);
        });

        ui.add_space(16.0);

        // ── Range date ───────────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Date range",
                Some("Click a start date then an end date."),
            );

            let label = match (self.cal_range_start, self.cal_range_end) {
                (Some(s), Some(e)) => format!(
                    "{:04}-{:02}-{:02}  →  {:04}-{:02}-{:02}",
                    s.year, s.month, s.day, e.year, e.month, e.day
                ),
                (Some(s), None) => {
                    format!("{:04}-{:02}-{:02}  →  (pick end)", s.year, s.month, s.day)
                }
                _ => "No range selected".to_owned(),
            };
            muted_text(ui, &label);
            ui.add_space(12.0);

            Calendar::range(
                "demo_cal_range",
                &mut self.cal_range_start,
                &mut self.cal_range_end,
            )
            .show(ui);

            if self.cal_range_start.is_some() {
                ui.add_space(8.0);
                if ui.button("Clear").clicked() {
                    self.cal_range_start = None;
                    self.cal_range_end = None;
                }
            }
        });

        ui.add_space(16.0);

        // ── With cell content ─────────────────────────────────────────────────
        Card::new().show(ui, |ui| {
            card_header(
                ui,
                "Cell content",
                Some("Custom badges inside specific date cells."),
            );
            muted_text(ui, "Cells on the 1st, 10th, and 20th have event badges.");
            ui.add_space(12.0);

            let theme = ShadcnTheme::get(ui.ctx());
            let accent = theme.primary;

            Calendar::single("demo_cal_events", &mut self.cal_single)
                .cell_height(52.0)
                .cell_content(move |ui, date| {
                    if matches!(date.day, 1 | 10 | 20) {
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 12.0),
                            egui::Sense::hover(),
                        );
                        ui.painter().rect_filled(
                            rect.shrink2(egui::Vec2::new(2.0, 0.0)),
                            egui::CornerRadius::same(3),
                            accent,
                        );
                    }
                })
                .show(ui);
        });
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
            ui.add_space(16.0);
            ui.label(
                egui::RichText::new("Type CONFIRM to proceed")
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground),
            );
            ui.add_space(4.0);
            Input::new(input).placeholder("CONFIRM").show(ui);
            ui.add_space(16.0);

            let ok = input.as_str() == "CONFIRM";
            ui.horizontal(|ui| {
                if Button::new("Cancel")
                    .variant(ButtonVariant::Outline)
                    .show(ui)
                    .clicked()
                {
                    close = true;
                }
                ui.add_space(8.0);
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
