use egui::Color32;
use egui_shadcn::{
    ICON_BRIGHTNESS_4, ICON_BRIGHTNESS_7, ICON_PALETTE,
    ICON_MAIL, ICON_SEND, ICON_DOWNLOAD, ICON_SEARCH, ICON_ADD,
    ICON_FORMAT_BOLD, ICON_FORMAT_ITALIC, ICON_FORMAT_UNDERLINE, ICON_FORMAT_STRIKETHROUGH,
    ShadcnTheme,
    accordion::Accordion,
    alert::{Alert, AlertVariant},
    alert_dialog::AlertDialog,
    avatar::{Avatar, AvatarSize},
    badge::{Badge, BadgeVariant},
    breadcrumb::Breadcrumb,
    button::{Button, ButtonSize, ButtonVariant},
    button_group::{ButtonGroup, ButtonGroupVariant},
    calendar::{CalDate, Calendar},
    card::{Card, card_header},
    carousel::Carousel,
    chart::{Chart, ChartDataset, ChartKind},
    checkbox::Checkbox,
    collapsible::Collapsible,
    combobox::Combobox,
    command::{Command, CommandGroup, CommandItem},
    context_menu::{ContextMenu, ContextItem},
    data_table::{DataTable, DataColumn},
    date_picker::DatePicker,
    dialog::Dialog,
    drawer::Drawer,
    dropdown_menu::{DropdownMenu, DropdownItem},
    hover_card::HoverCard,
    input::Input,
    input_otp::InputOtp,
    label::Label,
    menubar::{Menubar, MenubarItem, MenubarMenuItem},
    navigation_menu::{NavigationMenu, NavItem},
    pagination::Pagination,
    popover::Popover,
    progress::Progress,
    radio::Radio,
    resizable::{Resizable, ResizeDir},
    select::Select,
    separator::Separator,
    sheet::Sheet,
    skeleton::Skeleton,
    slider::Slider,
    spinner::Spinner,
    switch::Switch,
    table::{Table, TableColumn},
    tabs::Tabs,
    textarea::Textarea,
    toast::{Toaster, ToastVariant},
    toggle::Toggle,
    toggle_group::{ToggleGroup, ToggleGroupItem},
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
    toggle1: bool,
    toggle2: bool,
    toggle_bold: bool,
    toggle_italic: bool,
    toggle_underline: bool,
    toggle_strikethrough: bool,
    toggle_group_val: u8,
    collapsible_open: bool,
    pagination_page: usize,
    alert_dialog_open: bool,
    alert_dialog_confirmed: bool,
    button_group_sel: Option<usize>,
    combobox_val: Option<usize>,
    date_picker_val: Option<CalDate>,
    otp_val: String,
    sheet_open: bool,
    drawer_open: bool,
    nav_active: usize,
    command_open: bool,
    data_table_filter: String,
    context_last: Option<String>,
    dropdown_last: Option<String>,
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
            alert_dialog_open: false,
            alert_dialog_confirmed: false,
            button_group_sel: Some(0),
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

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "With Icon", None);
            ui.horizontal_wrapped(|ui| {
                Button::new("Send").icon(ICON_SEND).show(ui);
                ui.add_space(8.0);
                Button::new("Mail")
                    .icon(ICON_MAIL)
                    .variant(ButtonVariant::Outline)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Download")
                    .icon(ICON_DOWNLOAD)
                    .variant(ButtonVariant::Secondary)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Search")
                    .icon(ICON_SEARCH)
                    .variant(ButtonVariant::Ghost)
                    .show(ui);
                ui.add_space(8.0);
                Button::new("Add")
                    .icon(ICON_ADD)
                    .variant(ButtonVariant::Destructive)
                    .show(ui);
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

    fn section_toggle(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Toggle", "A two-state button that can be turned on or off.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Toggle", None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle1, "Bold").show(ui);
                Toggle::new(&mut self.toggle2, "Italic").show(ui);
            });
        });

        ui.add_space(16.0);
        Card::new().show(ui, |ui| {
            card_header(ui, "With Icon", None);
            ui.horizontal(|ui| {
                Toggle::new(&mut self.toggle_bold, "Bold")
                    .icon(ICON_FORMAT_BOLD)
                    .show(ui);
                ui.add_space(4.0);
                Toggle::new(&mut self.toggle_italic, "Italic")
                    .icon(ICON_FORMAT_ITALIC)
                    .show(ui);
                ui.add_space(4.0);
                Toggle::new(&mut self.toggle_underline, "Underline")
                    .icon(ICON_FORMAT_UNDERLINE)
                    .show(ui);
                ui.add_space(4.0);
                Toggle::new(&mut self.toggle_strikethrough, "Strikethrough")
                    .icon(ICON_FORMAT_STRIKETHROUGH)
                    .show(ui);
            });
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Disabled", None);
            let mut dummy = true;
            ui.horizontal(|ui| {
                Toggle::new(&mut dummy, "Disabled On").enabled(false).show(ui);
                let mut dummy2 = false;
                Toggle::new(&mut dummy2, "Disabled Off").enabled(false).show(ui);
            });
        });
    }

    fn section_toggle_group(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Toggle Group", "A set of two-state buttons — only one active at a time.");

        let items: &[(u8, ToggleGroupItem<'_>)] = &[
            (0, ToggleGroupItem { label: "Day", icon: None }),
            (1, ToggleGroupItem { label: "Week", icon: None }),
            (2, ToggleGroupItem { label: "Month", icon: None }),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "View", None);
            ToggleGroup::new(items, &mut self.toggle_group_val).show(ui);
            ui.add_space(4.0);
            muted_text(ui, &format!("Selected: {}", ["Day", "Week", "Month"][self.toggle_group_val as usize]));
        });
    }

    fn section_collapsible(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Collapsible", "An interactive component that expands/collapses a section.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Collapsible", None);
            Collapsible::new("demo_collapsible", "Starred repositories", &mut self.collapsible_open)
                .show(ui, |ui| {
                    ui.add_space(4.0);
                    muted_text(ui, "@radix-ui/primitives");
                    ui.add_space(4.0);
                    muted_text(ui, "@radix-ui/colors");
                    ui.add_space(4.0);
                    muted_text(ui, "@stitches/react");
                });
        });
    }

    fn section_breadcrumb(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Breadcrumb", "Displays the path to the current resource using a hierarchy of links.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Breadcrumb", None);
            let items = ["Home", "Components", "Breadcrumb"];
            if let Some(idx) = Breadcrumb::new(&items).show(ui) {
                muted_text(ui, &format!("Navigated to: {}", items[idx]));
            }
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Custom separator", None);
            let items = ["Docs", "Components", "Breadcrumb"];
            Breadcrumb::new(&items).separator("›").show(ui);
        });
    }

    fn section_pagination(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Pagination", "Navigation for pages of data.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Pagination", None);
            muted_text(ui, &format!("Page {} of 10", self.pagination_page));
            ui.add_space(12.0);
            if let Some(p) = Pagination::new(self.pagination_page, 10).show(ui) {
                self.pagination_page = p;
            }
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Many pages", None);
            let page2 = 7usize;
            Pagination::new(page2, 20).siblings(2).show(ui);
        });
    }

    fn section_alert_dialog(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Alert Dialog", "A modal dialog that interrupts with important content.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Default", None);
            if self.alert_dialog_confirmed {
                muted_text(ui, "Action confirmed!");
                ui.add_space(8.0);
            }
            if Button::new("Open Alert Dialog").show(ui).clicked() {
                self.alert_dialog_open = true;
                self.alert_dialog_confirmed = false;
            }
        });

        ui.add_space(16.0);

        let ctx = ui.ctx().clone();
        let mut confirmed = false;
        AlertDialog::new(
            "Are you absolutely sure?",
            "This action cannot be undone. This will permanently delete your account and remove your data from our servers.",
            &mut self.alert_dialog_open,
        )
        .destructive(true)
        .confirm_label("Delete Account")
        .show(&ctx, || { confirmed = true; });
        if confirmed {
            self.alert_dialog_confirmed = true;
        }
    }

    fn section_button_group(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Button Group", "Multiple actions in a connected button strip.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Single select", None);
            let labels = &["Months", "Weeks", "Days"];
            if let Some(i) = ButtonGroup::new(labels).selected(self.button_group_sel).show(ui) {
                self.button_group_sel = Some(i);
            }
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Outline variant", None);
            ButtonGroup::new(&["Left", "Center", "Right"]).variant(ButtonGroupVariant::Outline).show(ui);
        });
    }

    fn section_combobox(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Combobox", "An input with autocomplete and filtering.");

        let frameworks = &["Next.js", "SvelteKit", "Nuxt.js", "Remix", "Astro", "SolidStart"];
        Card::new().show(ui, |ui| {
            card_header(ui, "Framework", Some("Select a framework to use."));
            Combobox::new("demo_combobox", &mut self.combobox_val, frameworks)
                .placeholder("Select framework…")
                .show(ui);
            if let Some(i) = self.combobox_val {
                ui.add_space(4.0);
                muted_text(ui, &format!("Selected: {}", frameworks[i]));
            }
        });
    }

    fn section_date_picker(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Date Picker", "A date picker with popover calendar.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Pick a date", None);
            DatePicker::new("demo_date_picker", &mut self.date_picker_val).show(ui);
            if let Some(d) = self.date_picker_val {
                ui.add_space(4.0);
                muted_text(ui, &format!("Selected: {:04}-{:02}-{:02}", d.year, d.month, d.day));
            }
        });
    }

    fn section_dropdown_menu(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Dropdown Menu", "A menu triggered by a button.");

        let items = &[
            DropdownItem::Item { label: "Profile", disabled: false },
            DropdownItem::Item { label: "Billing", disabled: false },
            DropdownItem::Item { label: "Settings", disabled: false },
            DropdownItem::Separator,
            DropdownItem::Item { label: "Log out", disabled: false },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Account Menu", None);
            if let Some(i) = DropdownMenu::new("demo_dropdown", "My Account", items).show(ui) {
                let label = match i {
                    0 => "Profile", 1 => "Billing", 2 => "Settings", _ => "Log out",
                };
                self.dropdown_last = Some(label.to_owned());
            }
            if let Some(ref last) = self.dropdown_last {
                ui.add_space(4.0);
                muted_text(ui, &format!("Clicked: {last}"));
            }
        });
    }

    fn section_context_menu(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Context Menu", "A menu that appears on right-click.");

        let items = &[
            ContextItem::Item { label: "Back", shortcut: Some("⌘["), disabled: false },
            ContextItem::Item { label: "Forward", shortcut: Some("⌘]"), disabled: true },
            ContextItem::Item { label: "Reload", shortcut: Some("⌘R"), disabled: false },
            ContextItem::Separator,
            ContextItem::Item { label: "Save as…", shortcut: Some("⌘S"), disabled: false },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Right-click area", Some("Right-click anywhere in the box below."));

            let clicked = ContextMenu::new("demo_ctx", items).show(ui, |ui| {
                let (rect, resp) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 80.0),
                    egui::Sense::click(),
                );
                let theme = ShadcnTheme::get(ui.ctx());
                ui.painter().rect_filled(rect, egui::CornerRadius::same(6), theme.muted);
                ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Right-click here",
                    egui::FontId::new(13.0, egui::FontFamily::Proportional), theme.muted_foreground);
                resp
            });

            if let Some(i) = clicked {
                let label = match i { 0 => "Back", 1 => "Forward", 2 => "Reload", _ => "Save as…" };
                self.context_last = Some(label.to_owned());
            }
            if let Some(ref last) = self.context_last {
                ui.add_space(4.0);
                muted_text(ui, &format!("Clicked: {last}"));
            }
        });
    }

    fn section_hover_card(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Hover Card", "A card that appears when hovering over a trigger.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Profile link", Some("Hover over the username below."));
            ui.add_space(8.0);
            HoverCard::new("demo_hover_card").show(ui,
                |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    let resp = ui.add(egui::Label::new(
                        egui::RichText::new("@egui_shadcn")
                            .color(theme.primary)
                            .underline()
                    ).sense(egui::Sense::hover()));
                    resp
                },
                |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.horizontal(|ui| {
                        Avatar::new("ES").size(AvatarSize::Sm).show(ui);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("egui-shadcn").color(theme.foreground).strong());
                            ui.label(egui::RichText::new("@egui_shadcn").color(theme.muted_foreground).size(12.0));
                        });
                    });
                    ui.add_space(8.0);
                    muted_text(ui, "Shadcn/ui components for Rust egui. Dark/light, 25+ components.");
                }
            );
        });
    }

    fn section_input_otp(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Input OTP", "Accessible one-time password component.");

        Card::new().show(ui, |ui| {
            card_header(ui, "One-Time Password", Some("Enter your one-time password."));
            InputOtp::new(&mut self.otp_val, 6).separator_after(3).show(ui);
            if self.otp_val.len() == 6 {
                ui.add_space(4.0);
                muted_text(ui, &format!("Code: {}", self.otp_val));
            }
        });
    }

    fn section_popover(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Popover", "Displays rich content in a portal, triggered by a button.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Open popover", None);
            Popover::new("demo_popover").show(ui,
                |ui| Button::new("Open").show(ui),
                |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.label(egui::RichText::new("Dimensions").color(theme.foreground).strong());
                    ui.add_space(8.0);
                    muted_text(ui, "Set the dimensions for the layer.");
                    ui.add_space(12.0);
                    ui.label(egui::RichText::new("Width").color(theme.foreground).size(13.0));
                    ui.add_space(4.0);
                    let mut w = String::from("100%");
                    Input::new(&mut w).show(ui);
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Max. width").color(theme.foreground).size(13.0));
                    ui.add_space(4.0);
                    let mut mw = String::from("300px");
                    Input::new(&mut mw).show(ui);
                }
            );
        });
    }

    fn section_table(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Table", "A simple table with header and striped rows.");

        let columns = &[
            TableColumn { header: "Invoice", width: Some(120.0) },
            TableColumn { header: "Status", width: Some(100.0) },
            TableColumn { header: "Method", width: Some(120.0) },
            TableColumn { header: "Amount", width: None },
        ];

        let data = &[
            ("INV001", "Paid", "Credit Card", "$250.00"),
            ("INV002", "Pending", "PayPal", "$150.00"),
            ("INV003", "Unpaid", "Bank Transfer", "$350.00"),
            ("INV004", "Paid", "Credit Card", "$450.00"),
            ("INV005", "Paid", "PayPal", "$550.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Recent Invoices", None);
            Table::new(columns).show(ui, data.len(), |row_idx, row| {
                let (inv, status, method, amount) = data[row_idx];
                row.cell(|ui| { muted_text(ui, inv); });
                row.cell(|ui| { muted_text(ui, status); });
                row.cell(|ui| { muted_text(ui, method); });
                row.cell(|ui| { muted_text(ui, amount); });
            });
        });
    }

    fn section_data_table(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Data Table", "A powerful table with sorting and filtering.");

        let columns = &[
            DataColumn { header: "Status", width: Some(100.0), sortable: true },
            DataColumn { header: "Email", width: None, sortable: true },
            DataColumn { header: "Amount", width: Some(120.0), sortable: true },
        ];

        let data = &[
            ("Success", "ken99@yahoo.com", "$316.00"),
            ("Success", "abe45@gmail.com", "$242.00"),
            ("Processing", "monserrat44@gmail.com", "$837.00"),
            ("Success", "silas22@gmail.com", "$874.00"),
            ("Failed", "carmella@hotmail.com", "$721.00"),
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Payments", None);
            DataTable::new("demo_data_table", columns, &mut self.data_table_filter)
                .page_size(3)
                .show(ui, data.len(), |row_idx, row| {
                    let (status, email, amount) = data[row_idx];
                    row.cell(|ui| { muted_text(ui, status); });
                    row.cell(|ui| { muted_text(ui, email); });
                    row.cell(|ui| { muted_text(ui, amount); });
                });
        });
    }

    fn section_sheet(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Sheet", "Extends the dialog component to display content that slides in from the edge.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Right sheet", None);
            if Button::new("Open Sheet").show(ui).clicked() {
                self.sheet_open = true;
            }
        });

        let ctx = ui.ctx().clone();
        Sheet::new("Edit Profile", &mut self.sheet_open).show(&ctx, |ui| {
            let theme = ShadcnTheme::get(ui.ctx());
            ui.label(egui::RichText::new("Make changes to your profile here.").color(theme.muted_foreground).size(13.0));
            ui.add_space(16.0);
            let mut name = String::from("Pedro Duarte");
            Input::new(&mut name).show(ui);
            ui.add_space(8.0);
            let mut username = String::from("@peduarte");
            Input::new(&mut username).show(ui);
            ui.add_space(16.0);
            if Button::new("Save changes").show(ui).clicked() {
                // saved
            }
        });
    }

    fn section_drawer(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Drawer", "A bottom panel that slides up from the edge.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Bottom drawer", None);
            if Button::new("Open Drawer").show(ui).clicked() {
                self.drawer_open = true;
            }
        });

        let ctx = ui.ctx().clone();
        Drawer::new("Move Goal", &mut self.drawer_open).show(&ctx, |ui| {
            let theme = ShadcnTheme::get(ui.ctx());
            ui.label(egui::RichText::new("Set your daily activity goal.").color(theme.muted_foreground).size(13.0));
            ui.add_space(16.0);
            let mut goal = String::from("350");
            Input::new(&mut goal).show(ui);
            ui.add_space(16.0);
            if Button::new("Submit").show(ui).clicked() {
                // submitted
            }
        });
    }

    fn section_toast(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Toast", "A succinct message that appears temporarily.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Toast variants", Some("Click to show a toast notification."));
            ui.horizontal(|ui| {
                let ctx = ui.ctx().clone();
                if Button::new("Default").show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Scheduled", "Monday, January 3rd at 6:00pm", ToastVariant::Default);
                }
                if Button::new("Success").variant(ButtonVariant::Default).show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Success", "Your changes have been saved.", ToastVariant::Success);
                }
                if Button::new("Warning").show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Warning", "This action may have consequences.", ToastVariant::Warning);
                }
                if Button::new("Destructive").variant(ButtonVariant::Destructive).show(ui).clicked() {
                    Toaster::push_with_desc(&ctx, "Destructive", "Your session has expired.", ToastVariant::Destructive);
                }
            });
        });
    }

    fn section_navigation_menu(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Navigation Menu", "A navigation component with horizontal and vertical layouts.");

        let items = &[
            NavItem { label: "Dashboard", icon: None, badge: None },
            NavItem { label: "Projects", icon: None, badge: Some("3") },
            NavItem { label: "Team", icon: None, badge: None },
            NavItem { label: "Settings", icon: None, badge: None },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Horizontal", None);
            if let Some(i) = NavigationMenu::new(items, self.nav_active).show(ui) {
                self.nav_active = i;
            }
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Vertical (sidebar)", None);
            NavigationMenu::new(items, self.nav_active).vertical().show(ui);
        });
    }

    fn section_menubar(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Menubar", "A visually persistent menu common in desktop apps.");

        let menus = &[
            MenubarItem {
                label: "File",
                items: &[
                    MenubarMenuItem::Item { label: "New Tab", shortcut: Some("Ctrl+T"), disabled: false },
                    MenubarMenuItem::Item { label: "New Window", shortcut: Some("Ctrl+N"), disabled: false },
                    MenubarMenuItem::Separator,
                    MenubarMenuItem::Item { label: "Share", shortcut: None, disabled: false },
                    MenubarMenuItem::Item { label: "Print…", shortcut: Some("Ctrl+P"), disabled: false },
                ],
            },
            MenubarItem {
                label: "Edit",
                items: &[
                    MenubarMenuItem::Item { label: "Undo", shortcut: Some("Ctrl+Z"), disabled: false },
                    MenubarMenuItem::Item { label: "Redo", shortcut: Some("Ctrl+Y"), disabled: false },
                    MenubarMenuItem::Separator,
                    MenubarMenuItem::Item { label: "Cut", shortcut: Some("Ctrl+X"), disabled: false },
                    MenubarMenuItem::Item { label: "Copy", shortcut: Some("Ctrl+C"), disabled: false },
                    MenubarMenuItem::Item { label: "Paste", shortcut: Some("Ctrl+V"), disabled: false },
                ],
            },
            MenubarItem {
                label: "View",
                items: &[
                    MenubarMenuItem::Item { label: "Zoom In", shortcut: Some("Ctrl++"), disabled: false },
                    MenubarMenuItem::Item { label: "Zoom Out", shortcut: Some("Ctrl+-"), disabled: false },
                ],
            },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, "Application Menu", None);
            Menubar::new(menus).show(ui);
        });
    }

    fn section_carousel(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Carousel", "A carousel with motion and swipe capabilities.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Items", None);
            Carousel::new("demo_carousel", 5).height(180.0).show(ui, |idx, ui| {
                let theme = ShadcnTheme::get(ui.ctx());
                let (rect, _) = ui.allocate_exact_size(
                    egui::Vec2::new(ui.available_width(), 160.0),
                    egui::Sense::hover(),
                );
                ui.painter().rect_filled(rect, egui::CornerRadius::same(8), theme.muted);
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    &format!("Slide {}", idx + 1),
                    egui::FontId::new(24.0, egui::FontFamily::Proportional),
                    theme.muted_foreground,
                );
            });
        });
    }

    fn section_chart(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Chart", "Bar and line charts for data visualization.");

        let months = &["Jan", "Feb", "Mar", "Apr", "May", "Jun"];
        let desktop = &[186.0, 305.0, 237.0, 73.0, 209.0, 214.0];
        let mobile = &[80.0, 200.0, 120.0, 190.0, 130.0, 140.0];

        Card::new().show(ui, |ui| {
            card_header(ui, "Bar chart – Desktop vs Mobile", None);
            let datasets = &[
                ChartDataset { label: "Desktop", values: desktop, color: None },
                ChartDataset { label: "Mobile", values: mobile, color: Some(egui::Color32::from_rgb(100, 160, 240)) },
            ];
            Chart::new(datasets, months).height(200.0).show_legend(true).show(ui);
        });

        ui.add_space(16.0);

        Card::new().show(ui, |ui| {
            card_header(ui, "Line chart", None);
            let datasets = &[
                ChartDataset { label: "Desktop", values: desktop, color: None },
            ];
            Chart::new(datasets, months).kind(ChartKind::Line).height(200.0).show_grid(true).show(ui);
        });
    }

    fn section_command(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Command", "Fast, composable, keyboard-first command menu.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Command palette", Some("Press the button or ⌘K to open."));
            if Button::new("Open Command Palette").variant(ButtonVariant::Outline).show(ui).clicked() {
                self.command_open = true;
            }
        });

        let groups = &[
            CommandGroup {
                heading: Some("Suggestions"),
                items: &[
                    CommandItem { label: "Calendar", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Search Emoji", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Calculator", description: None, shortcut: None, icon: None },
                ],
            },
            CommandGroup {
                heading: Some("Settings"),
                items: &[
                    CommandItem { label: "Profile", description: Some("⌘P"), shortcut: None, icon: None },
                    CommandItem { label: "Billing", description: None, shortcut: None, icon: None },
                    CommandItem { label: "Settings", description: Some("⌘S"), shortcut: None, icon: None },
                ],
            },
        ];

        let ctx = ui.ctx().clone();
        Command::new("demo_command", groups, &mut self.command_open)
            .placeholder("Type a command or search…")
            .show(&ctx);
    }

    fn section_resizable(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Resizable", "Accessible resizable panel groups and layouts.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Horizontal panels", None);
            Resizable::new("demo_resizable_h")
                .dir(ResizeDir::Horizontal)
                .initial_split(0.5)
                .show(ui,
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 120.0), egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(4), theme.muted);
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Panel A",
                            egui::FontId::new(13.0, egui::FontFamily::Proportional), theme.muted_foreground);
                    },
                    |ui| {
                        let theme = ShadcnTheme::get(ui.ctx());
                        let (rect, _) = ui.allocate_exact_size(
                            egui::Vec2::new(ui.available_width(), 120.0), egui::Sense::hover()
                        );
                        ui.painter().rect_filled(rect, egui::CornerRadius::same(4), theme.muted);
                        ui.painter().text(rect.center(), egui::Align2::CENTER_CENTER, "Panel B",
                            egui::FontId::new(13.0, egui::FontFamily::Proportional), theme.muted_foreground);
                    }
                );
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
