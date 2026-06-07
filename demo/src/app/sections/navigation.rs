use crate::i18n as t;
use ::i18n::t as tr;
use egui_sc::egui_components::spacing::Spacing;
use egui_sc::egui_components::{
    accordion::Accordion,
    breadcrumb::Breadcrumb,
    card::{Card, card_header},
    menubar::{Menubar, MenubarItem, MenubarMenuItem},
    navigation_menu::{NavItem, NavigationMenu},
    pagination::Pagination,
    tabs::Tabs,
    typography::{body_text, muted_text},
};

use crate::app::DemoApp;

impl DemoApp {
    pub(in crate::app) fn section_accordion(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(1);
        let subtitle = tr!(t::AccordionSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::AccordionSec::HFaq).as_ref(), None);
            let q1 = tr!(t::AccordionSec::Q1);
            Accordion::new("acc_0", q1.as_ref(), &mut self.accordion_open[0])
                .show(ui, |ui| {
                    muted_text(ui, tr!(t::AccordionSec::A1).as_ref());
                });
            let q2 = tr!(t::AccordionSec::Q2);
            Accordion::new("acc_1", q2.as_ref(), &mut self.accordion_open[1])
                .show(ui, |ui| {
                    muted_text(ui, tr!(t::AccordionSec::A2).as_ref());
                });
            let q3 = tr!(t::AccordionSec::Q3);
            Accordion::new("acc_2", q3.as_ref(), &mut self.accordion_open[2])
                .show(ui, |ui| {
                    muted_text(ui, tr!(t::AccordionSec::A3).as_ref());
                });
        });
    }

    pub(in crate::app) fn section_breadcrumb(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(7);
        let subtitle = tr!(t::BreadcrumbSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BreadcrumbSec::HDefault).as_ref(), None);
            let home = tr!(t::BreadcrumbSec::Home);
            let comps = tr!(t::BreadcrumbSec::Components);
            let bc = tr!(t::BreadcrumbSec::Breadcrumb);
            let items = [home.as_ref(), comps.as_ref(), bc.as_ref()];
            if let Some(idx) = Breadcrumb::new(&items).show(ui) {
                self.breadcrumb_nav = Some(items[idx].to_string());
            }
            if let Some(nav) = &self.breadcrumb_nav {
                Spacing::Xs.show(ui);
                muted_text(ui, tr!(t::BreadcrumbSec::Nav, item = nav).as_ref());
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::BreadcrumbSec::HCustom).as_ref(), None);
            let docs = tr!(t::BreadcrumbSec::Docs);
            let comps = tr!(t::BreadcrumbSec::Components);
            let bc = tr!(t::BreadcrumbSec::Breadcrumb);
            let items = [docs.as_ref(), comps.as_ref(), bc.as_ref()];
            if let Some(idx) = Breadcrumb::new(&items).separator("›").show(ui)
            {
                self.breadcrumb_custom_nav = Some(items[idx].to_string());
            }
            if let Some(nav) = &self.breadcrumb_custom_nav {
                Spacing::Xs.show(ui);
                muted_text(ui, tr!(t::BreadcrumbSec::Nav, item = nav).as_ref());
            }
        });
    }

    pub(in crate::app) fn section_menubar(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(28);
        let subtitle = tr!(t::MenubarSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let file = tr!(t::MenubarSec::File);
        let edit = tr!(t::MenubarSec::Edit);
        let view = tr!(t::MenubarSec::View);
        let new_tab = tr!(t::MenubarSec::NewTab);
        let new_win = tr!(t::MenubarSec::NewWindow);
        let share = tr!(t::MenubarSec::Share);
        let print = tr!(t::MenubarSec::Print);
        let undo = tr!(t::MenubarSec::Undo);
        let redo = tr!(t::MenubarSec::Redo);
        let cut = tr!(t::MenubarSec::Cut);
        let copy = tr!(t::MenubarSec::Copy);
        let paste = tr!(t::MenubarSec::Paste);
        let zoom_in = tr!(t::MenubarSec::ZoomIn);
        let zoom_out = tr!(t::MenubarSec::ZoomOut);

        let file_items = [
            MenubarMenuItem::Item {
                label: new_tab.as_ref(),
                shortcut: Some("Ctrl+T"),
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: new_win.as_ref(),
                shortcut: Some("Ctrl+N"),
                disabled: false,
            },
            MenubarMenuItem::Separator,
            MenubarMenuItem::Item {
                label: share.as_ref(),
                shortcut: None,
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: print.as_ref(),
                shortcut: Some("Ctrl+P"),
                disabled: false,
            },
        ];
        let edit_items = [
            MenubarMenuItem::Item {
                label: undo.as_ref(),
                shortcut: Some("Ctrl+Z"),
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: redo.as_ref(),
                shortcut: Some("Ctrl+Y"),
                disabled: false,
            },
            MenubarMenuItem::Separator,
            MenubarMenuItem::Item {
                label: cut.as_ref(),
                shortcut: Some("Ctrl+X"),
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: copy.as_ref(),
                shortcut: Some("Ctrl+C"),
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: paste.as_ref(),
                shortcut: Some("Ctrl+V"),
                disabled: false,
            },
        ];
        let view_items = [
            MenubarMenuItem::Item {
                label: zoom_in.as_ref(),
                shortcut: Some("Ctrl++"),
                disabled: false,
            },
            MenubarMenuItem::Item {
                label: zoom_out.as_ref(),
                shortcut: Some("Ctrl+-"),
                disabled: false,
            },
        ];
        let menus = [
            MenubarItem {
                label: file.as_ref(),
                items: &file_items,
            },
            MenubarItem {
                label: edit.as_ref(),
                items: &edit_items,
            },
            MenubarItem {
                label: view.as_ref(),
                items: &view_items,
            },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::MenubarSec::HApp).as_ref(), None);
            Menubar::new(&menus).show(ui);
        });
    }

    pub(in crate::app) fn section_navigation_menu(
        &mut self,
        ui: &mut egui::Ui,
    ) {
        let title = t::section_name(29);
        let subtitle = tr!(t::NavMenuSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        let dashboard = tr!(t::NavMenuSec::Dashboard);
        let projects = tr!(t::NavMenuSec::Projects);
        let team = tr!(t::NavMenuSec::Team);
        let settings = tr!(t::NavMenuSec::Settings);
        let items = [
            NavItem {
                label: dashboard.as_ref(),
                icon: None,
                badge: None,
            },
            NavItem {
                label: projects.as_ref(),
                icon: None,
                badge: Some("3"),
            },
            NavItem {
                label: team.as_ref(),
                icon: None,
                badge: None,
            },
            NavItem {
                label: settings.as_ref(),
                icon: None,
                badge: None,
            },
        ];

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::NavMenuSec::HHoriz).as_ref(), None);
            if let Some(i) =
                NavigationMenu::new(&items, self.nav_active).show(ui)
            {
                self.nav_active = i;
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::NavMenuSec::HVert).as_ref(), None);
            NavigationMenu::new(&items, self.nav_active)
                .vertical()
                .show(ui);
        });
    }

    pub(in crate::app) fn section_pagination(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(30);
        let subtitle = tr!(t::PaginationSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::PaginationSec::HBasic).as_ref(), None);
            let page1 = self.pagination_page;
            muted_text(ui, tr!(t::PaginationSec::Page10, n = page1).as_ref());
            Spacing::Md.show(ui);
            if let Some(p) = Pagination::new(self.pagination_page, 10).show(ui)
            {
                self.pagination_page = p;
            }
        });

        Spacing::Lg.show(ui);
        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::PaginationSec::HMany).as_ref(), None);
            let page2 = self.pagination_page2;
            muted_text(ui, tr!(t::PaginationSec::Page20, n = page2).as_ref());
            Spacing::Md.show(ui);
            if let Some(p) = Pagination::new(self.pagination_page2, 20)
                .siblings(2)
                .show(ui)
            {
                self.pagination_page2 = p;
            }
        });
    }

    pub(in crate::app) fn section_tabs(&mut self, ui: &mut egui::Ui) {
        let title = t::section_name(44);
        let subtitle = tr!(t::TabsSec::Subtitle);
        self.section_title(ui, title.as_ref(), subtitle.as_ref());

        Card::new().show(ui, |ui| {
            card_header(ui, tr!(t::TabsSec::HAccount).as_ref(), None);
            let account = tr!(t::TabsSec::Account);
            let password = tr!(t::TabsSec::Password);
            let notifs = tr!(t::TabsSec::Notifications);
            let tabs = [account.as_ref(), password.as_ref(), notifs.as_ref()];
            Tabs::new("demo_tabs", &tabs, &mut self.tab_index).show(
                ui,
                |ui, tab| {
                    Spacing::Sm.show(ui);
                    match tab {
                        0 => {
                            body_text(ui, tr!(t::TabsSec::BodyAccount).as_ref())
                        }
                        1 => body_text(
                            ui,
                            tr!(t::TabsSec::BodyPassword).as_ref(),
                        ),
                        2 => body_text(
                            ui,
                            tr!(t::TabsSec::BodyNotifications).as_ref(),
                        ),
                        _ => {}
                    }
                },
            );
        });
    }
}
