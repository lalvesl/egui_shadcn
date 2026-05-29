use egui_shadcn::spacing::Spacing;
use egui_shadcn::{
    ShadcnTheme,
    avatar::Avatar, size::Size,
    button::{Button, ButtonVariant},
    card::{Card, card_header},
    context_menu::{ContextMenu, ContextItem},
    drawer::Drawer,
    dropdown_menu::{DropdownMenu, DropdownItem},
    hover_card::HoverCard,
    input::Input,
    popover::Popover,
    sheet::Sheet,
    tooltip::Tooltip,
    typography::muted_text,
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_overlays(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Overlays", "Dialogs and tooltips.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Dialog", None);
            if Button::new("Open Dialog").show(ui).clicked() {
                self.dialog_open = true;
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, "Tooltip", None);
            Tooltip::new("This is a helpful tooltip").wrap(ui, |ui| {
                Button::new("Hover me")
                    .variant(ButtonVariant::Outline)
                    .show(ui)
            });
        });
    }

    pub(in crate::app) fn section_dropdown_menu(&mut self, ui: &mut egui::Ui) {
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
                Spacing::Xs.show(ui);
                muted_text(ui, &format!("Clicked: {last}"));
            }
        });
    }

    pub(in crate::app) fn section_context_menu(&mut self, ui: &mut egui::Ui) {
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
                Spacing::Xs.show(ui);
                muted_text(ui, &format!("Clicked: {last}"));
            }
        });
    }

    pub(in crate::app) fn section_hover_card(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Hover Card", "A card that appears when hovering over a trigger.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Profile link", Some("Hover over the username below."));
            Spacing::Sm.show(ui);
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
                        Avatar::new("ES").size(Size::Sm).show(ui);
                        ui.vertical(|ui| {
                            ui.label(egui::RichText::new("egui-shadcn").color(theme.foreground).strong());
                            ui.label(egui::RichText::new("@egui_shadcn").color(theme.muted_foreground).size(12.0));
                        });
                    });
                    Spacing::Sm.show(ui);
                    muted_text(ui, "Shadcn/ui components for Rust egui. Dark/light, 25+ components.");
                }
            );
        });
    }

    pub(in crate::app) fn section_popover(&mut self, ui: &mut egui::Ui) {
        self.section_title(ui, "Popover", "Displays rich content in a portal, triggered by a button.");

        Card::new().show(ui, |ui| {
            card_header(ui, "Open popover", None);
            Popover::new("demo_popover").show(ui,
                |ui| Button::new("Open").show(ui),
                |ui| {
                    let theme = ShadcnTheme::get(ui.ctx());
                    ui.label(egui::RichText::new("Dimensions").color(theme.foreground).strong());
                    Spacing::Sm.show(ui);
                    muted_text(ui, "Set the dimensions for the layer.");
                    Spacing::Md.show(ui);
                    ui.label(egui::RichText::new("Width").color(theme.foreground).size(13.0));
                    Spacing::Xs.show(ui);
                    let mut w = String::from("100%");
                    Input::new(&mut w).show(ui);
                    Spacing::Sm.show(ui);
                    ui.label(egui::RichText::new("Max. width").color(theme.foreground).size(13.0));
                    Spacing::Xs.show(ui);
                    let mut mw = String::from("300px");
                    Input::new(&mut mw).show(ui);
                }
            );
        });
    }

    pub(in crate::app) fn section_sheet(&mut self, ui: &mut egui::Ui) {
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
            Spacing::Lg.show(ui);
            let mut name = String::from("Pedro Duarte");
            Input::new(&mut name).show(ui);
            Spacing::Sm.show(ui);
            let mut username = String::from("@peduarte");
            Input::new(&mut username).show(ui);
            Spacing::Lg.show(ui);
            if Button::new("Save changes").show(ui).clicked() {
                // saved
            }
        });
    }

    pub(in crate::app) fn section_drawer(&mut self, ui: &mut egui::Ui) {
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
            Spacing::Lg.show(ui);
            let mut goal = String::from("350");
            Input::new(&mut goal).show(ui);
            Spacing::Lg.show(ui);
            if Button::new("Submit").show(ui).clicked() {
                // submitted
            }
        });
    }
}
