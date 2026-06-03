//! Render-without-panic ("smoke") tests for container / popup / overlay
//! components — those taking content closures or a `&Context`. Each is driven
//! through at least one real frame in both the closed and (where meaningful)
//! open state.

mod common;
use common::*;

use egui_components::ICON_CHECK;

#[test]
fn accordion_open_and_closed() {
    use egui_components::accordion::Accordion;
    let ctx = ctx();
    for mut open in [true, false] {
        frame(&ctx, base_input(), |ui| {
            Accordion::new("acc", "Section", &mut open).show(ui, |ui| {
                ui.label("hidden content");
            });
        });
    }
}

#[test]
fn boxed_returns_inner() {
    use egui_components::boxed::Boxed;
    use egui_components::spacing::Spacing;
    let ctx = ctx();
    let out = render(&ctx, |ui| {
        Boxed::new()
            .padding(Spacing::Lg)
            .margin(Spacing::Sm)
            .show(ui, |ui| {
                ui.label("inside");
                42
            })
            .inner
    });
    assert_eq!(out, 42);
}

#[test]
fn card_with_header_and_footer() {
    use egui_components::card::{Card, card_footer, card_header};
    let ctx = ctx();
    frame(&ctx, base_input(), |ui| {
        Card::new().padding(16.0).show(ui, |ui| {
            card_header(ui, "Title", Some("Description"));
            ui.label("body");
            card_footer(ui, |ui| {
                ui.label("footer");
            });
        });
    });
}

#[test]
fn carousel_renders_items() {
    use egui_components::carousel::Carousel;
    let ctx = ctx();
    let active = render(&ctx, |ui| {
        Carousel::new("car", 3).show(ui, |i, ui| {
            ui.label(format!("slide {i}"));
        })
    });
    assert!(active < 3);
}

#[test]
fn chart_bar_and_line() {
    use egui_components::chart::{Chart, ChartDataset, ChartKind};
    let ctx = ctx();
    let ds = [
        ChartDataset {
            label: "Sales",
            values: &[3.0, 7.0, 4.0, 9.0],
            color: None,
        },
        ChartDataset {
            label: "Costs",
            values: &[2.0, 5.0, 3.0, 6.0],
            color: Some(egui::Color32::RED),
        },
    ];
    let labels = ["Q1", "Q2", "Q3", "Q4"];
    for kind in [ChartKind::Bar, ChartKind::Line] {
        render(&ctx, |ui| {
            Chart::new(&ds, &labels)
                .kind(kind)
                .height(180.0)
                .show_grid(true)
                .show_legend(true)
                .show(ui)
        });
    }
    // Degenerate: empty datasets must not divide-by-zero / panic.
    render(&ctx, |ui| Chart::new(&[], &[]).show(ui));
}

#[test]
fn collapsible_open_and_closed() {
    use egui_components::collapsible::Collapsible;
    let ctx = ctx();
    for mut open in [true, false] {
        frame(&ctx, base_input(), |ui| {
            Collapsible::new("col", "More", &mut open).show(ui, |ui| {
                ui.label("content");
            });
        });
    }
}

#[test]
fn command_palette_open() {
    use egui_components::command::{Command, CommandGroup, CommandItem};
    let ctx = ctx();
    let items = [
        CommandItem {
            label: "New File",
            description: Some("Create a file"),
            shortcut: Some("Ctrl+N"),
            icon: Some(ICON_CHECK),
        },
        CommandItem {
            label: "Open",
            description: None,
            shortcut: None,
            icon: None,
        },
    ];
    let groups = [CommandGroup {
        heading: Some("Actions"),
        items: &items,
    }];
    let mut open = true;
    frame(&ctx, base_input(), |ui| {
        let _ = Command::new("cmd", &groups, &mut open).show(ui.ctx());
    });
}

#[test]
fn context_menu_trigger() {
    use egui_components::context_menu::{ContextItem, ContextMenu};
    let ctx = ctx();
    let items = [
        ContextItem::Item {
            label: "Copy",
            shortcut: Some("Ctrl+C"),
            disabled: false,
        },
        ContextItem::Separator,
        ContextItem::Item {
            label: "Delete",
            shortcut: None,
            disabled: true,
        },
    ];
    let chosen = render(&ctx, |ui| {
        ContextMenu::new("ctx", &items).show(ui, |ui| {
            ui.allocate_response(egui::vec2(120.0, 40.0), egui::Sense::click_and_drag())
        })
    });
    assert_eq!(chosen, None);
}

#[test]
fn data_table_with_rows() {
    use egui_components::data_table::{DataColumn, DataTable};
    let ctx = ctx();
    let cols = [
        DataColumn {
            header: "Name",
            width: Some(120.0),
            sortable: true,
        },
        DataColumn {
            header: "Email",
            width: None,
            sortable: false,
        },
    ];
    let mut filter = String::new();
    let sort = render(&ctx, |ui| {
        DataTable::new("dt", &cols, &mut filter).show(ui, 3, |i, row| {
            row.cell(|ui| {
                ui.label(format!("Person {i}"));
            });
            row.cell(|ui| {
                ui.label(format!("p{i}@x.com"));
            });
        })
    });
    let _ = sort;
}

#[test]
fn dialog_open_and_closed() {
    use egui_components::dialog::Dialog;
    let ctx = ctx();
    for mut open in [true, false] {
        frame(&ctx, base_input(), |ui| {
            Dialog::new("Confirm", &mut open).show(ui.ctx(), |ui| {
                ui.label("Are you sure?");
            });
        });
    }
}

#[test]
fn drawer_and_sheet() {
    use egui_components::drawer::Drawer;
    use egui_components::sheet::Sheet;
    let ctx = ctx();
    let mut d_open = true;
    let mut s_open = true;
    frame(&ctx, base_input(), |ui| {
        Drawer::new("Drawer", &mut d_open).show(ui.ctx(), |ui| {
            ui.label("drawer body");
        });
    });
    frame(&ctx, base_input(), |ui| {
        Sheet::new("Sheet", &mut s_open).show(ui.ctx(), |ui| {
            ui.label("sheet body");
        });
    });
}

#[test]
fn dropdown_menu_trigger() {
    use egui_components::dropdown_menu::{DropdownItem, DropdownMenu};
    let ctx = ctx();
    let items = [
        DropdownItem::Item {
            label: "Profile",
            disabled: false,
        },
        DropdownItem::Separator,
        DropdownItem::Item {
            label: "Logout",
            disabled: false,
        },
    ];
    let chosen = render(&ctx, |ui| DropdownMenu::new("dd", "Menu", &items).show(ui));
    assert_eq!(chosen, None);
}

#[test]
fn grid_lays_out_items() {
    use egui_components::grid::Grid;
    let ctx = ctx();
    frame(&ctx, base_input(), |ui| {
        Grid::new()
            .min_col_width(80.0)
            .gap(8.0)
            .show(ui, 6, |ui, i| {
                ui.label(format!("cell {i}"));
            });
    });
}

#[test]
fn hover_card_and_popover() {
    use egui_components::hover_card::HoverCard;
    use egui_components::popover::Popover;
    let ctx = ctx();
    frame(&ctx, base_input(), |ui| {
        HoverCard::new("hc").show(
            ui,
            |ui| ui.label("hover me"),
            |ui| {
                ui.label("card content");
            },
        );
    });
    frame(&ctx, base_input(), |ui| {
        Popover::new("pop").show(
            ui,
            |ui| ui.allocate_response(egui::vec2(80.0, 32.0), egui::Sense::click()),
            |ui| {
                ui.label("popover content");
            },
        );
    });
}

#[test]
fn menubar_renders() {
    use egui_components::menubar::{Menubar, MenubarItem, MenubarMenuItem};
    let ctx = ctx();
    let file_items = [
        MenubarMenuItem::Item {
            label: "New",
            shortcut: Some("Ctrl+N"),
            disabled: false,
        },
        MenubarMenuItem::Separator,
        MenubarMenuItem::Item {
            label: "Exit",
            shortcut: None,
            disabled: false,
        },
    ];
    let menus = [MenubarItem {
        label: "File",
        items: &file_items,
    }];
    let chosen = render(&ctx, |ui| Menubar::new(&menus).show(ui));
    assert_eq!(chosen, None);
}

#[test]
fn resizable_split() {
    use egui_components::resizable::Resizable;
    let ctx = ctx();
    frame(&ctx, base_input(), |ui| {
        Resizable::new("rz").show(
            ui,
            |ui| {
                ui.label("left");
            },
            |ui| {
                ui.label("right");
            },
        );
    });
}

#[test]
fn table_with_rows() {
    use egui_components::table::{Table, TableColumn};
    let ctx = ctx();
    let cols = [
        TableColumn {
            header: "Item",
            width: Some(100.0),
        },
        TableColumn {
            header: "Qty",
            width: None,
        },
    ];
    frame(&ctx, base_input(), |ui| {
        Table::new(&cols).show(ui, 4, |i, row| {
            row.cell(|ui| {
                ui.label(format!("item {i}"));
            });
            row.cell(|ui| {
                ui.label(format!("{}", i * 2));
            });
        });
    });
}

#[test]
fn tabs_switch_content() {
    use egui_components::tabs::Tabs;
    let ctx = ctx();
    let labels = ["Account", "Password", "Team"];
    let mut current = 0usize;
    frame(&ctx, base_input(), |ui| {
        Tabs::new("tabs", &labels, &mut current).show(ui, |ui, idx| {
            ui.label(format!("panel {idx}"));
        });
    });
}

#[test]
fn toaster_with_queued_toast() {
    use egui_components::toast::{ToastVariant, Toaster};
    let ctx = ctx();
    Toaster::push(&ctx, "Saved", ToastVariant::Success);
    Toaster::push_with_desc(&ctx, "Heads up", "Something to note", ToastVariant::Warning);
    frame(&ctx, base_input(), |ui| {
        Toaster::show(ui.ctx());
    });
}

#[test]
fn toggle_group_single_select() {
    use egui_components::toggle_group::{ToggleGroup, ToggleGroupItem};
    let ctx = ctx();
    let items = [
        (
            0u8,
            ToggleGroupItem {
                label: "Left",
                icon: None,
            },
        ),
        (
            1u8,
            ToggleGroupItem {
                label: "Center",
                icon: Some(ICON_CHECK),
            },
        ),
    ];
    let mut selected = 0u8;
    frame(&ctx, base_input(), |ui| {
        ToggleGroup::new(&items, &mut selected).show(ui);
    });
}

#[test]
fn alert_dialog_open() {
    use egui_components::alert_dialog::AlertDialog;
    let ctx = ctx();
    let mut open = true;
    let mut confirmed = false;
    frame(&ctx, base_input(), |ui| {
        let _ = AlertDialog::new("Delete?", "This cannot be undone.", &mut open)
            .show(ui.ctx(), || confirmed = true);
    });
    let _ = confirmed;
}

#[test]
fn calendar_single_range_and_custom_cells() {
    use egui_components::calendar::{CalDate, Calendar};
    let ctx = ctx();
    let mut sel = Some(CalDate::new(2026, 6, 3));
    frame(&ctx, base_input(), |ui| {
        Calendar::single("cal", &mut sel).show(ui);
    });

    let mut start = Some(CalDate::new(2026, 6, 1));
    let mut end = Some(CalDate::new(2026, 6, 10));
    frame(&ctx, base_input(), |ui| {
        Calendar::range("cal_range", &mut start, &mut end)
            .compact()
            .show(ui);
    });

    let mut sel2 = None;
    frame(&ctx, base_input(), |ui| {
        Calendar::single("cal_cells", &mut sel2)
            .cell_height(48.0)
            .cell_content(|ui, _date| {
                ui.label("•");
            })
            .show(ui);
    });
}
