use egui_shadcn::{
    accordion::Accordion,
    breadcrumb::Breadcrumb,
    card::{Card, card_header},
    menubar::{Menubar, MenubarItem, MenubarMenuItem},
    navigation_menu::{NavigationMenu, NavItem},
    pagination::Pagination,
    tabs::Tabs,
    typography::{body_text, muted_text},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_navigation(&mut self, ui: &mut egui::Ui) {
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

    pub(in crate::app) fn section_breadcrumb(&mut self, ui: &mut egui::Ui) {
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

    pub(in crate::app) fn section_pagination(&mut self, ui: &mut egui::Ui) {
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

    pub(in crate::app) fn section_navigation_menu(&mut self, ui: &mut egui::Ui) {
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

    pub(in crate::app) fn section_menubar(&mut self, ui: &mut egui::Ui) {
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
}
