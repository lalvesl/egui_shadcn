use crate::ShadcnTheme;
use egui::{FontFamily, FontId, RichText, Ui};

pub fn heading1(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(36.0, FontFamily::Proportional))
            .color(theme.foreground)
            .strong(),
    );
}

pub fn heading2(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(28.0, FontFamily::Proportional))
            .color(theme.foreground)
            .strong(),
    );
}

pub fn heading3(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(22.0, FontFamily::Proportional))
            .color(theme.foreground)
            .strong(),
    );
}

pub fn heading4(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(18.0, FontFamily::Proportional))
            .color(theme.foreground)
            .strong(),
    );
}

pub fn body_text(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(14.0, FontFamily::Proportional))
            .color(theme.foreground),
    );
}

pub fn muted_text(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(14.0, FontFamily::Proportional))
            .color(theme.muted_foreground),
    );
}

pub fn small_text(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(12.0, FontFamily::Proportional))
            .color(theme.muted_foreground),
    );
}

pub fn code_text(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    let bg = ShadcnTheme::with_alpha(theme.muted, 200);

    let galley = ui.painter().layout_no_wrap(
        text.to_owned(),
        FontId::new(13.0, FontFamily::Monospace),
        theme.foreground,
    );

    let h_pad = 4.0;
    let v_pad = 2.0;
    let size = egui::Vec2::new(
        galley.size().x + h_pad * 2.0,
        galley.size().y + v_pad * 2.0,
    );
    let (rect, _) = ui.allocate_exact_size(size, egui::Sense::hover());

    if ui.is_rect_visible(rect) {
        ui.painter()
            .rect_filled(rect, egui::CornerRadius::same(4), bg);
        ui.painter().galley(
            egui::Pos2::new(rect.left() + h_pad, rect.top() + v_pad),
            galley,
            theme.foreground,
        );
    }
}

pub fn lead_text(ui: &mut Ui, text: &str) {
    let theme = ShadcnTheme::get(ui.ctx());
    ui.label(
        RichText::new(text)
            .font(FontId::new(20.0, FontFamily::Proportional))
            .color(theme.muted_foreground),
    );
}
