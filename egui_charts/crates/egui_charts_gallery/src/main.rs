//! Gallery + theme playground for egui_charts — chrome built from
//! `egui_components` (shadcn) widgets.

mod controls;
mod samples;

use controls::ControlsState;
use eframe::egui;
use egui_charts::{ChartKind, ChartWidget, ThemeMode};
use egui_components::{
    ICON_AUTO_GRAPH, ShadcnTheme, Size,
    accordion::Accordion,
    alert::Alert,
    badge::{Badge, BadgeVariant},
    card::Card,
    icon_font_id,
    separator::Separator,
    typography::{heading2, heading4, muted_text, small_text},
};
use std::collections::BTreeMap;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1240.0, 760.0])
            .with_title("egui_charts gallery"),
        ..Default::default()
    };
    eframe::run_native(
        "egui_charts_gallery",
        options,
        Box::new(|cc| Ok(Box::new(GalleryApp::new(cc)))),
    )
}

struct GalleryApp {
    selected: ChartKind,
    controls: ControlsState,
    expanded: BTreeMap<String, bool>,
}

impl Default for GalleryApp {
    fn default() -> Self {
        let mut expanded = BTreeMap::new();
        for k in ChartKind::all() {
            expanded.entry(k.category().to_string()).or_insert(true);
        }
        Self {
            selected: ChartKind::Line,
            controls: ControlsState::default(),
            expanded,
        }
    }
}

impl GalleryApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // MaterialIcons font — required for the shadcn chevrons / icons.
        egui_components::register_font(&cc.egui_ctx);

        let app = Self::default();
        app.apply_shadcn_theme(&cc.egui_ctx);
        app
    }

    /// Sync the shadcn chrome theme to the chart controls so the whole gallery
    /// reskins from one hue + light/dark choice.
    fn apply_shadcn_theme(&self, ctx: &egui::Context) {
        let dark = self.controls.mode == ThemeMode::Dark;
        let theme = ShadcnTheme::build(dark, Some(self.controls.hue));
        ShadcnTheme::set(ctx, theme.clone());
        theme.apply(ctx);
    }
}

impl eframe::App for GalleryApp {
    fn ui(&mut self, ui: &mut egui::Ui, _: &mut eframe::Frame) {
        self.render(ui);
    }
}

impl GalleryApp {
    /// Render the gallery into `ui`. Split out from [`eframe::App::ui`] so it can
    /// be driven headlessly in tests without an `eframe::Frame`.
    fn render(&mut self, ui: &mut egui::Ui) {
        let ctx = ui.ctx().clone();
        self.apply_shadcn_theme(&ctx);
        let theme = ShadcnTheme::get(&ctx);

        let card_frame = egui::Frame::new()
            .fill(theme.card)
            .inner_margin(egui::Margin::symmetric(10, 14))
            .stroke(egui::Stroke::new(1.0, theme.border));

        // ── Sidebar (chart catalog) ─────────────────────────────────────────
        egui::Panel::left("sidebar")
            .resizable(false)
            .exact_size(248.0)
            .frame(card_frame)
            .show_inside(ui, |ui| {
                self.draw_sidebar(ui);
            });

        // ── Controls (theme playground) ─────────────────────────────────────
        egui::Panel::right("controls")
            .resizable(false)
            .exact_size(284.0)
            .frame(card_frame)
            .show_inside(ui, |ui| {
                controls::show(ui, &mut self.controls);
            });

        // ── Central (active chart) ──────────────────────────────────────────
        egui::CentralPanel::default()
            .frame(
                egui::Frame::new()
                    .fill(theme.background)
                    .inner_margin(egui::Margin::same(20)),
            )
            .show_inside(ui, |ui| {
                self.draw_canvas(ui);
            });
    }
}

impl GalleryApp {
    fn draw_sidebar(&mut self, ui: &mut egui::Ui) {
        let theme = ShadcnTheme::get(ui.ctx());

        // Brand header.
        ui.horizontal(|ui| {
            ui.label(
                egui::RichText::new(ICON_AUTO_GRAPH)
                    .font(icon_font_id(22.0))
                    .color(theme.primary),
            );
            ui.add_space(6.0);
            heading4(ui, "egui_charts");
        });
        small_text(ui, "gallery");
        ui.add_space(8.0);
        Separator::horizontal().show(ui);
        ui.add_space(8.0);

        // Category accordions → chart kinds.
        let Self {
            selected, expanded, ..
        } = self;

        egui::ScrollArea::vertical()
            .auto_shrink([false, false])
            .show(ui, |ui| {
                for (cat, kinds) in grouped_kinds() {
                    let open = expanded.entry(cat.to_string()).or_insert(true);
                    Accordion::new(cat, cat, open).show(ui, |ui| {
                        for k in kinds {
                            if sidebar_item(ui, k.label(), *selected == k, k.is_implemented()) {
                                *selected = k;
                            }
                        }
                    });
                }
            });
    }

    fn draw_canvas(&mut self, ui: &mut egui::Ui) {
        heading2(ui, self.selected.label());
        ui.add_space(6.0);

        ui.horizontal(|ui| {
            muted_text(ui, &format!("Category · {}", self.selected.category()));
            ui.add_space(8.0);
            let (label, variant) = if self.selected.is_implemented() {
                ("implemented", BadgeVariant::Default)
            } else {
                ("planned", BadgeVariant::Secondary)
            };
            Badge::new(label).variant(variant).size(Size::Sm).show(ui);
        });

        ui.add_space(12.0);
        Separator::horizontal().show(ui);
        ui.add_space(14.0);

        if let Some(chart) = samples::build(self.selected) {
            let chart_theme = self.controls.build_theme();
            Card::new().padding(12.0).show(ui, |ui| {
                let avail = ui.available_size();
                ChartWidget::new(&chart)
                    .theme(chart_theme)
                    .min_size(egui::vec2(avail.x.max(360.0), (avail.y - 4.0).max(320.0)))
                    .show(ui);
            });
        } else {
            ui.add_space(40.0);
            ui.vertical_centered(|ui| {
                Alert::new("Not yet implemented")
                    .description(
                        "Phase 0 ships line, bar and scatter. See the roadmap for what comes next.",
                    )
                    .show(ui);
            });
        }
    }
}

// ── Sidebar helpers ─────────────────────────────────────────────────────────

/// Chart kinds grouped by category, preserving the order from `ChartKind::all()`.
fn grouped_kinds() -> Vec<(&'static str, Vec<ChartKind>)> {
    let mut groups: Vec<(&'static str, Vec<ChartKind>)> = Vec::new();
    for k in ChartKind::all() {
        let cat = k.category();
        if let Some(g) = groups.iter_mut().find(|g| g.0 == cat) {
            g.1.push(*k);
        } else {
            groups.push((cat, vec![*k]));
        }
    }
    groups
}

/// A single clickable chart-kind row. Returns `true` when clicked.
fn sidebar_item(ui: &mut egui::Ui, label: &str, selected: bool, implemented: bool) -> bool {
    let theme = ShadcnTheme::get(ui.ctx());
    let width = ui.available_width();
    let (rect, resp) = ui.allocate_exact_size(egui::vec2(width, 30.0), egui::Sense::click());
    let cr = egui::CornerRadius::same(theme.radius as u8);

    if selected {
        ui.painter().rect_filled(rect, cr, theme.accent);
    } else if resp.hovered() {
        ui.painter()
            .rect_filled(rect, cr, ShadcnTheme::with_alpha(theme.accent, 90));
        ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
    }

    let fg = if selected {
        theme.accent_foreground
    } else if implemented {
        theme.foreground
    } else {
        theme.muted_foreground
    };
    ui.painter().text(
        egui::pos2(rect.left() + 10.0, rect.center().y),
        egui::Align2::LEFT_CENTER,
        label,
        egui::FontId::new(13.0, egui::FontFamily::Proportional),
        fg,
    );

    if !implemented {
        ui.painter().text(
            egui::pos2(rect.right() - 8.0, rect.center().y),
            egui::Align2::RIGHT_CENTER,
            "soon",
            egui::FontId::new(10.0, egui::FontFamily::Proportional),
            theme.muted_foreground,
        );
    }

    ui.add_space(2.0);
    resp.clicked()
}

// ── Headless smoke test ──────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn screen() -> egui::RawInput {
        egui::RawInput {
            screen_rect: Some(egui::Rect::from_min_size(
                egui::Pos2::ZERO,
                egui::vec2(1240.0, 760.0),
            )),
            ..Default::default()
        }
    }

    /// Render every chart kind (implemented and "planned") through the real
    /// gallery chrome in both light and dark, with no window. Catches panics in
    /// `samples::build`, the chart renderers, or the shadcn chrome.
    #[test]
    fn gallery_renders_every_chart_kind() {
        let ctx = egui::Context::default();
        egui_components::register_font(&ctx);

        let mut app = GalleryApp::default();
        for &mode in &[ThemeMode::Light, ThemeMode::Dark] {
            app.controls.mode = mode;
            for &kind in ChartKind::all() {
                app.selected = kind;
                // A couple of frames so accordions/animations settle.
                for _ in 0..2 {
                    let _ = ctx.run_ui(screen(), |ui| app.render(ui));
                }
            }
        }
    }
}
