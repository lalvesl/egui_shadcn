mod components;
mod theme;

use eframe::egui;
use egui::Margin;
use theme::ShadcnTheme;
use components::{
    accordion::Accordion,
    alert::{Alert, AlertVariant},
    avatar::Avatar,
    badge::{Badge, BadgeVariant},
    breadcrumb::{Breadcrumb, BreadcrumbItem, CollapsedBreadcrumb},
    button::{Button, ButtonSize, ButtonVariant},
    card::{Card, card_content, card_footer, card_header},
    checkbox::Checkbox,
    date_picker::{Date, DatePicker, DateRangePicker},
    dialog::{Dialog, dialog_description, dialog_footer},
    input::TextInput,
    label::Label,
    progress::Progress,
    radio::RadioGroup,
    select::Select,
    separator::Separator,
    slider::Slider,
    switch::Switch,
    table::{DataTable, DataTableState, TableColumn},
    tabs::Tabs,
    textarea::Textarea,
    theme_picker::{ThemePicker, ThemePreset},
    toast::{Toaster, ToastVariant},
    tooltip::Tooltip,
};

#[cfg(not(target_arch = "wasm32"))]
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("egui-shadcn")
            .with_inner_size([1200.0, 900.0]),
        ..Default::default()
    };
    eframe::run_native("egui-shadcn", options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
}

#[cfg(target_arch = "wasm32")]
fn main() {
    use wasm_bindgen::JsCast as _;
    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        let canvas = web_sys::window()
            .unwrap()
            .document()
            .unwrap()
            .get_element_by_id("egui_canvas")
            .unwrap()
            .dyn_into::<web_sys::HtmlCanvasElement>()
            .unwrap();
        eframe::WebRunner::new()
            .start(canvas, web_options, Box::new(|cc| Ok(Box::new(App::new(cc)))))
            .await
            .unwrap();
    });
}

struct App {
    dark_mode: bool,
    counter:   u32,

    text_input: String,
    email:      String,
    password:   String,

    check1: bool,
    check2: bool,
    check3: bool,

    radio_val:  u32,
    select_val: usize,

    slider_a: f32,
    slider_b: f32,

    switch_notif: bool,
    switch_sync:  bool,

    textarea:    String,
    active_tab:  usize,
    dialog_name: String,

    // breadcrumb
    crumb_expanded: bool,

    // table
    table_state: DataTableState,

    // theme picker
    picker_idx: usize,

    // date pickers
    date_selected: Option<Date>,
    date_open:     bool,
    date_view:     Date,
    range_start:   Option<Date>,
    range_end:     Option<Date>,
    range_open:    bool,
    range_view:    Date,
}

impl App {
    fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            dark_mode:    false,
            counter:      0,
            text_input:   String::new(),
            email:        String::new(),
            password:     String::new(),
            check1:       true,
            check2:       false,
            check3:       true,
            radio_val:    0,
            select_val:   99,
            slider_a:     33.0,
            slider_b:     66.0,
            switch_notif: true,
            switch_sync:  false,
            textarea:     String::new(),
            active_tab:   0,
            dialog_name:  String::new(),
            crumb_expanded: false,
            table_state:    DataTableState::new(5, 4),
            picker_idx:   0,
            date_selected: None,
            date_open:    false,
            date_view:    Date::new(2026, 5, 1),
            range_start:  None,
            range_end:    None,
            range_open:   false,
            range_view:   Date::new(2026, 5, 1),
        }
    }
}

impl eframe::App for App {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();

        let mut theme = if self.dark_mode { ShadcnTheme::dark() } else { ShadcnTheme::light() };
        ThemePreset::all()[self.picker_idx].apply_to(&mut theme);
        ShadcnTheme::set(&ctx, theme.clone());
        theme.apply_to_ctx(&ctx);

        egui::Panel::top("toolbar")
            .frame(
                egui::Frame::new()
                    .fill(theme.background)
                    .stroke(egui::Stroke::new(1.0, theme.border))
                    .inner_margin(Margin::symmetric(16, 12)),
            )
            .show_inside(ui, |ui| {
                ui.horizontal(|ui| {
                    ui.label(egui::RichText::new("egui-shadcn").size(18.0).strong().color(theme.foreground));
                    ui.add_space(8.0);
                    ui.label(egui::RichText::new("Shadcn UI Components for Rust").size(14.0).color(theme.muted_foreground));

                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let lbl = if self.dark_mode { "☀ Light" } else { "☾ Dark" };
                        if Button::new(lbl).variant(ButtonVariant::Outline).show(ui).clicked() {
                            self.dark_mode = !self.dark_mode;
                        }
                    });
                });
            });

        Toaster::show(&ctx);

        egui::CentralPanel::default()
            .frame(egui::Frame::new().fill(theme.background).inner_margin(Margin::symmetric(24, 0)))
            .show_inside(ui, |ui| {
                egui::ScrollArea::both().show(ui, |ui| {
                    ui.add_space(24.0);

                    let col_w = ((ui.available_width() - 24.0) * 0.5).max(400.0);

                    egui::Grid::new("grid")
                        .num_columns(2)
                        .spacing([24.0, 24.0])
                        .min_col_width(col_w)
                        .max_col_width(col_w)
                        .show(ui, |ui| {
                            self.section_buttons(ui, &theme);
                            self.section_input(ui);
                            ui.end_row();

                            self.section_badges(ui);
                            self.section_checkbox(ui);
                            ui.end_row();

                            self.section_card(ui, &theme);
                            self.section_radio(ui);
                            ui.end_row();

                            self.section_select(ui, &theme);
                            self.section_slider(ui);
                            ui.end_row();

                            self.section_switch(ui);
                            self.section_textarea(ui);
                            ui.end_row();

                            self.section_progress(ui);
                            self.section_avatar(ui, &theme);
                            ui.end_row();

                            self.section_alert(ui);
                            self.section_separator_label(ui, &theme);
                            ui.end_row();

                            self.section_tabs(ui, &theme);
                            self.section_accordion(ui, &theme);
                            ui.end_row();

                            self.section_dialog(ui, &ctx);
                            self.section_tooltip(ui);
                            ui.end_row();

                            self.section_date_pickers(ui);
                            self.section_theme_picker(ui);
                            ui.end_row();

                            self.section_toast(ui, &ctx);
                            ui.end_row();

                            self.section_table(ui);
                            self.section_breadcrumb(ui);
                            ui.end_row();
                        });

                    ui.add_space(32.0);

                    ui.separator();
                    ui.add_space(8.0);
                    ui.centered_and_justified(|ui| {
                        ui.label(
                            egui::RichText::new("egui-shadcn — Shadcn UI for Rust / egui 0.31")
                                .size(12.0)
                                .color(theme.muted_foreground),
                        );
                    });
                    ui.add_space(16.0);
                });
            });
    }
}

impl App {
    fn section_buttons(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Button", Some("6 variants. 3 sizes."));
                card_content(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::splat(8.0);
                        if Button::new("Default").show(ui).clicked() { self.counter += 1; }
                        Button::new("Destructive").variant(ButtonVariant::Destructive).show(ui);
                        Button::new("Outline").variant(ButtonVariant::Outline).show(ui);
                        Button::new("Secondary").variant(ButtonVariant::Secondary).show(ui);
                        Button::new("Ghost").variant(ButtonVariant::Ghost).show(ui);
                        Button::new("Link").variant(ButtonVariant::Link).show(ui);
                    });
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        Button::new("Small").size(ButtonSize::Sm).show(ui);
                        Button::new("Default").show(ui);
                        Button::new("Large").size(ButtonSize::Lg).show(ui);
                        Button::new("Disabled").enabled(false).show(ui);
                    });
                    if self.counter > 0 {
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(format!("Clicked {} time{}", self.counter, if self.counter == 1 { "" } else { "s" }))
                                .size(13.0)
                                .color(theme.muted_foreground),
                        );
                    }
                });
            });
        });
    }

    fn section_input(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Input", Some("Text entry with focus states."));
                card_content(ui, |ui| {
                    TextInput::new(&mut self.text_input).placeholder("John Doe").label("Name").show(ui);
                    ui.add_space(8.0);
                    TextInput::new(&mut self.email).placeholder("you@example.com").label("Email").show(ui);
                    ui.add_space(8.0);
                    TextInput::new(&mut self.password).placeholder("••••••••").label("Password").password(true).show(ui);
                    ui.add_space(8.0);
                    TextInput::new(&mut String::new()).placeholder("Disabled").disabled(true).show(ui);
                });
            });
        });
    }

    fn section_badges(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Badge", Some("Status labels and tags."));
                card_content(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing = egui::Vec2::new(8.0, 8.0);
                        Badge::new("Default").show(ui);
                        Badge::new("Secondary").variant(BadgeVariant::Secondary).show(ui);
                        Badge::new("Destructive").variant(BadgeVariant::Destructive).show(ui);
                        Badge::new("Outline").variant(BadgeVariant::Outline).show(ui);
                        Badge::new("New").show(ui);
                        Badge::new("Beta").variant(BadgeVariant::Secondary).show(ui);
                        Badge::new("Error").variant(BadgeVariant::Destructive).show(ui);
                        Badge::new("v1.0.0").variant(BadgeVariant::Outline).show(ui);
                    });
                });
            });
        });
    }

    fn section_checkbox(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Checkbox", Some("Boolean toggles."));
                card_content(ui, |ui| {
                    Checkbox::new(&mut self.check1).label("Accept terms and conditions").show(ui);
                    ui.add_space(8.0);
                    Checkbox::new(&mut self.check2).label("Subscribe to newsletter").show(ui);
                    ui.add_space(8.0);
                    Checkbox::new(&mut self.check3).label("Enable notifications").show(ui);
                    ui.add_space(8.0);
                    let mut disabled = false;
                    Checkbox::new(&mut disabled).label("Disabled checkbox").disabled(true).show(ui);
                });
            });
        });
    }

    fn section_card(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Card", Some("Content grouping surface."));
                card_content(ui, |ui| {
                    Card::new().show(ui, |ui| {
                        card_header(ui, "Project Alpha", Some("Last updated 2 hours ago"));
                        card_content(ui, |ui| {
                            ui.label(
                                egui::RichText::new("A platform to build and ship your next idea faster than ever before.")
                                    .size(14.0)
                                    .color(theme.muted_foreground),
                            );
                        });
                        card_footer(ui, |ui| {
                            Button::new("View Project").size(ButtonSize::Sm).show(ui);
                        });
                    });
                });
            });
        });
    }

    fn section_radio(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Radio Group", Some("Single selection from options."));
                card_content(ui, |ui| {
                    let mut rg = RadioGroup::new(&mut self.radio_val);
                    rg.item(ui, 0, "Default");
                    ui.add_space(6.0);
                    rg.item(ui, 1, "Comfortable");
                    ui.add_space(6.0);
                    rg.item(ui, 2, "Compact");
                    ui.add_space(6.0);
                    let mut dummy = 99u32;
                    RadioGroup::new(&mut dummy).disabled(true).item(ui, 99, "Disabled");
                });
            });
        });
    }

    fn section_select(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Select", Some("Dropdown selection."));
                card_content(ui, |ui| {
                    let fruits = &["Apple", "Banana", "Cherry", "Durian", "Elderberry"];
                    Select::new(&mut self.select_val, fruits)
                        .placeholder("Pick a fruit...")
                        .show(ui);
                    ui.add_space(8.0);
                    let mut dummy = 0usize;
                    Select::new(&mut dummy, &["Option A", "Option B"]).disabled(true).show(ui);
                    if self.select_val < fruits.len() {
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(format!("Selected: {}", fruits[self.select_val]))
                                .size(13.0)
                                .color(theme.muted_foreground),
                        );
                    }
                });
            });
        });
    }

    fn section_slider(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Slider", Some("Range value input."));
                card_content(ui, |ui| {
                    let lbl_a = format!("Progress: {:.0}%", self.slider_a);
                    let lbl_b = format!("Volume:   {:.0}%", self.slider_b);
                    Slider::new(&mut self.slider_a, 0.0, 100.0).label(&lbl_a).show(ui);
                    ui.add_space(16.0);
                    Slider::new(&mut self.slider_b, 0.0, 100.0).label(&lbl_b).show(ui);
                    ui.add_space(16.0);
                    let mut locked = 50.0f32;
                    Slider::new(&mut locked, 0.0, 100.0).label("Disabled: 50%").disabled(true).show(ui);
                });
            });
        });
    }

    fn section_switch(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Switch", Some("Toggle on/off states."));
                card_content(ui, |ui| {
                    Switch::new(&mut self.switch_notif).label("Push notifications").show(ui);
                    ui.add_space(8.0);
                    Switch::new(&mut self.switch_sync).label("Dark mode sync").show(ui);
                    ui.add_space(8.0);
                    let mut off = false;
                    Switch::new(&mut off).label("Disabled (off)").disabled(true).show(ui);
                    ui.add_space(8.0);
                    let mut on = true;
                    Switch::new(&mut on).label("Disabled (on)").disabled(true).show(ui);
                });
            });
        });
    }

    fn section_textarea(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Textarea", Some("Multiline text input."));
                card_content(ui, |ui| {
                    Textarea::new(&mut self.textarea)
                        .placeholder("Write your message here...")
                        .rows(4)
                        .show(ui);
                    ui.add_space(8.0);
                    let mut locked = "Disabled textarea content".to_string();
                    Textarea::new(&mut locked).rows(3).disabled(true).show(ui);
                });
            });
        });
    }

    fn section_progress(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Progress", Some("Task completion indicators."));
                card_content(ui, |ui| {
                    Progress::new(0.33).label("Loading assets").show(ui);
                    ui.add_space(12.0);
                    Progress::new(self.slider_a / 100.0).label("Dynamic (drag slider above)").show(ui);
                    ui.add_space(12.0);
                    Progress::new(1.0).label("Complete").show(ui);
                    ui.add_space(12.0);
                    Progress::new(0.0).show(ui);
                });
            });
        });
    }

    fn section_avatar(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Avatar", Some("User profile representations."));
                card_content(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        Avatar::new("Lucas Alves").size(56.0).bg(egui::Color32::from_rgb(99, 102, 241)).show(ui);
                        Avatar::new("AB").size(48.0).bg(egui::Color32::from_rgb(236, 72, 153)).show(ui);
                        Avatar::new("CD").size(40.0).show(ui);
                        Avatar::new("EF").size(32.0).bg(egui::Color32::from_rgb(34, 197, 94)).show(ui);
                        Avatar::new("GH").size(24.0).bg(egui::Color32::from_rgb(234, 179, 8)).show(ui);
                    });
                    ui.add_space(16.0);
                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 12.0;
                        Avatar::new("Lucas Alves").size(40.0).bg(egui::Color32::from_rgb(79, 70, 229)).show(ui);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("Lucas Alves").size(14.0).strong().color(theme.foreground));
                            ui.label(egui::RichText::new("alvesdelima.lucas45@gmail.com").size(12.0).color(theme.muted_foreground));
                        });
                    });
                });
            });
        });
    }

    fn section_alert(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Alert", Some("Informational and error messages."));
                card_content(ui, |ui| {
                    Alert::new("Heads up!")
                        .description("You can add components to your app using the CLI.")
                        .show(ui);
                    ui.add_space(8.0);
                    Alert::new("Error")
                        .description("Your session has expired. Please log in again.")
                        .variant(AlertVariant::Destructive)
                        .show(ui);
                });
            });
        });
    }

    fn section_separator_label(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Separator & Label", Some("Dividers and text labels."));
                card_content(ui, |ui| {
                    Label::new("Default label").show(ui);
                    ui.add_space(4.0);
                    Label::new("Muted label").muted(true).show(ui);
                    ui.add_space(4.0);
                    Label::new("Large bold label").size(20.0).bold(true).show(ui);
                    ui.add_space(8.0);

                    Separator::horizontal().show(ui);

                    ui.label(egui::RichText::new("Content after separator").size(14.0).color(theme.foreground));
                    ui.add_space(8.0);

                    ui.horizontal(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;
                        ui.label(egui::RichText::new("Left").size(14.0).color(theme.foreground));
                        Separator::vertical().length(20.0).show(ui);
                        ui.label(egui::RichText::new("Center").size(14.0).color(theme.foreground));
                        Separator::vertical().length(20.0).show(ui);
                        ui.label(egui::RichText::new("Right").size(14.0).color(theme.foreground));
                    });
                });
            });
        });
    }

    fn section_tabs(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Tabs", Some("Tabbed content panels."));
                card_content(ui, |ui| {
                    let names = ["Account", "Password", "Notifications"];
                    let mut tabs = Tabs::new(&mut self.active_tab, &names);
                    tabs.show_bar(ui);
                    ui.add_space(16.0);

                    Tabs::content(self.active_tab, 0, ui, |ui| {
                        ui.label(egui::RichText::new("Account Settings").size(14.0).strong().color(theme.foreground));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("Manage your account information and preferences.").size(13.0).color(theme.muted_foreground));
                    });
                    Tabs::content(self.active_tab, 1, ui, |ui| {
                        ui.label(egui::RichText::new("Password & Security").size(14.0).strong().color(theme.foreground));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("Update your password and secure your account.").size(13.0).color(theme.muted_foreground));
                    });
                    Tabs::content(self.active_tab, 2, ui, |ui| {
                        ui.label(egui::RichText::new("Notification Preferences").size(14.0).strong().color(theme.foreground));
                        ui.add_space(6.0);
                        ui.label(egui::RichText::new("Control what alerts and emails you receive.").size(13.0).color(theme.muted_foreground));
                    });
                });
            });
        });
    }

    fn section_accordion(&mut self, ui: &mut egui::Ui, theme: &ShadcnTheme) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Accordion", Some("Collapsible content sections."));
                card_content(ui, |ui| {
                    Accordion::item(ui, egui::Id::new("acc1"), "Is it accessible?", |ui| {
                        ui.label(egui::RichText::new("Yes. It adheres to the WAI-ARIA design pattern.")
                            .size(14.0).color(theme.muted_foreground));
                    });
                    Accordion::item(ui, egui::Id::new("acc2"), "Is it styled?", |ui| {
                        ui.label(egui::RichText::new("Yes. It uses Shadcn color tokens for consistent styling.")
                            .size(14.0).color(theme.muted_foreground));
                    });
                    Accordion::item(ui, egui::Id::new("acc3"), "Is it animated?", |ui| {
                        ui.label(egui::RichText::new("Yes. The chevron rotates using egui's animate_bool.")
                            .size(14.0).color(theme.muted_foreground));
                    });
                    let (sep, _) = ui.allocate_exact_size(egui::Vec2::new(ui.available_width(), 1.0), egui::Sense::hover());
                    ui.painter().rect_filled(sep, 0u8, theme.border);
                });
            });
        });
    }

    fn section_dialog(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Dialog", Some("Modal overlay dialogs."));
                card_content(ui, |ui| {
                    let id = egui::Id::new("demo_dialog");

                    if Button::new("Open Dialog").show(ui).clicked() {
                        Dialog::open(ctx, id);
                    }

                    Dialog::show(ctx, id, "Edit Profile", |ui, close| {
                        dialog_description(ui, "Make changes to your profile. Click save when done.");
                        TextInput::new(&mut self.dialog_name)
                            .placeholder("Your name")
                            .label("Name")
                            .show(ui);
                        dialog_footer(ui, |ui| {
                            if Button::new("Save changes").show(ui).clicked() { *close = true; }
                            ui.add_space(8.0);
                            if Button::new("Cancel").variant(ButtonVariant::Outline).show(ui).clicked() { *close = true; }
                        });
                    });
                });
            });
        });
    }

    fn section_tooltip(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Tooltip", Some("Hover-triggered contextual hints."));
                card_content(ui, |ui| {
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        Tooltip::new("Add a new item to the list").show(ui, |ui| {
                            Button::new("Add Item").show(ui)
                        });

                        Tooltip::new("Permanently delete this record").show(ui, |ui| {
                            Button::new("Delete").variant(ButtonVariant::Destructive).show(ui)
                        });

                        Tooltip::new("Copy value to clipboard").show(ui, |ui| {
                            Button::new("Copy").variant(ButtonVariant::Outline).show(ui)
                        });
                    });
                    ui.add_space(12.0);
                    ui.label(
                        egui::RichText::new("Hover any button above to see its tooltip.")
                            .size(12.0)
                            .italics(),
                    );
                });
            });
        });
    }

    fn section_date_pickers(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Date Picker", Some("Single date and date range selection."));
                card_content(ui, |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.label(egui::RichText::new("Date").size(13.0).color(theme.muted_foreground));
                    ui.add_space(4.0);
                    DatePicker::new(&mut self.date_selected, &mut self.date_open, &mut self.date_view)
                        .today(Date::new(2026, 5, 22))
                        .show(ui);

                    if let Some(d) = self.date_selected {
                        ui.add_space(6.0);
                        ui.label(
                            egui::RichText::new(format!("Selected: {}/{:02}/{}", d.month, d.day, d.year))
                                .size(12.0)
                                .color(theme.muted_foreground),
                        );
                    }

                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("Date Range").size(13.0).color(theme.muted_foreground));
                    ui.add_space(4.0);
                    DateRangePicker::new(
                        &mut self.range_start,
                        &mut self.range_end,
                        &mut self.range_open,
                        &mut self.range_view,
                    )
                    .today(Date::new(2026, 5, 22))
                    .show(ui);
                });
            });
        });
    }

    fn section_toast(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Toast", Some("Timed overlay notifications."));
                card_content(ui, |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.horizontal_wrapped(|ui| {
                        ui.spacing_mut().item_spacing.x = 8.0;

                        if Button::new("Default").variant(ButtonVariant::Secondary).show(ui).clicked() {
                            Toaster::push_desc(ctx, "Heads up!", "Your request has been received.", ToastVariant::Default);
                        }
                        if Button::new("Success").show(ui).clicked() {
                            Toaster::push_desc(ctx, "Changes saved", "Your profile has been updated.", ToastVariant::Success);
                        }
                        if Button::new("Error").variant(ButtonVariant::Destructive).show(ui).clicked() {
                            Toaster::push_desc(ctx, "Something went wrong", "Please try again later.", ToastVariant::Destructive);
                        }
                    });
                    ui.add_space(10.0);
                    ui.label(
                        egui::RichText::new("Toasts auto-dismiss after 4 s. Stack up to 4.")
                            .size(12.0)
                            .color(theme.muted_foreground),
                    );
                });
            });
        });
    }

    fn section_table(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Table", Some("Filter, multi-select, sort, paginate."));
                card_content(ui, |ui| {
                    let cols = [
                        TableColumn::new("Status"),
                        TableColumn::new("Email"),
                        TableColumn::new("Amount"),
                    ];
                    let rows: Vec<Vec<String>> = vec![
                        vec!["Success".into(),    "ken99@example.com".into(),       "$316.00".into()],
                        vec!["Success".into(),    "abe45@example.com".into(),       "$242.00".into()],
                        vec!["Processing".into(), "monserrat44@example.com".into(), "$837.00".into()],
                        vec!["Success".into(),    "silas22@example.com".into(),     "$874.00".into()],
                        vec!["Failed".into(),     "carmella@example.com".into(),    "$721.00".into()],
                    ];
                    DataTable::new(&cols, &rows, &mut self.table_state)
                        .filter_hint("Filter emails...")
                        .page_size(5)
                        .show(ui);
                });
            });
        });
    }

    fn section_breadcrumb(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Breadcrumb", Some("Navigation path with collapsed variant."));
                card_content(ui, |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());

                    ui.label(egui::RichText::new("Standard").size(12.0).color(theme.muted_foreground));
                    ui.add_space(4.0);
                    let items = [
                        BreadcrumbItem::link("Home"),
                        BreadcrumbItem::link("Components"),
                        BreadcrumbItem::current("Breadcrumb"),
                    ];
                    Breadcrumb::new(&items).separator("/").show(ui);

                    ui.add_space(16.0);
                    ui.label(egui::RichText::new("Collapsed").size(12.0).color(theme.muted_foreground));
                    ui.add_space(4.0);
                    CollapsedBreadcrumb::new(
                        "Home",
                        "Breadcrumb",
                        &mut self.crumb_expanded,
                        &["Docs", "Components"],
                    ).show(ui);
                });
            });
        });
    }

    fn section_theme_picker(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            Card::new().show(ui, |ui| {
                card_header(ui, "Theme", Some("Pick a color palette — applies live to this app."));
                card_content(ui, |ui| {
                    ThemePicker::new(&mut self.picker_idx).show(ui);
                });
            });
        });
    }
}
