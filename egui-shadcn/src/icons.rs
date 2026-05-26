use std::sync::Arc;

// Material Icons unicode codepoints
pub const ICON_ADD: &str = "\u{e145}";
pub const ICON_ARROW_BACK: &str = "\u{e5c4}";
pub const ICON_ARROW_FORWARD: &str = "\u{e5c8}";
pub const ICON_ARROW_DROP_DOWN: &str = "\u{e5c5}";
pub const ICON_ARROW_DROP_UP: &str = "\u{e5c6}";
pub const ICON_CHECK: &str = "\u{e5ca}";
pub const ICON_CHEVRON_LEFT: &str = "\u{e5cb}";
pub const ICON_CHEVRON_RIGHT: &str = "\u{e5cc}";
pub const ICON_CLOSE: &str = "\u{e5cd}";
pub const ICON_DELETE: &str = "\u{e872}";
pub const ICON_EDIT: &str = "\u{e3c9}";
pub const ICON_ERROR: &str = "\u{e000}";
pub const ICON_ERROR_OUTLINE: &str = "\u{e001}";
pub const ICON_EXPAND_LESS: &str = "\u{e5ce}";
pub const ICON_EXPAND_MORE: &str = "\u{e5cf}";
pub const ICON_FAVORITE: &str = "\u{e87d}";
pub const ICON_FAVORITE_BORDER: &str = "\u{e87e}";
pub const ICON_FOLDER: &str = "\u{e2c7}";
pub const ICON_HOME: &str = "\u{e88a}";
pub const ICON_INFO: &str = "\u{e88e}";
pub const ICON_LINK: &str = "\u{e157}";
pub const ICON_LOCK: &str = "\u{e897}";
pub const ICON_LOCK_OPEN: &str = "\u{e898}";
pub const ICON_MENU: &str = "\u{e5d2}";
pub const ICON_MORE_HORIZ: &str = "\u{e5d3}";
pub const ICON_MORE_VERT: &str = "\u{e5d4}";
pub const ICON_NOTIFICATIONS: &str = "\u{e7f4}";
pub const ICON_PERSON: &str = "\u{e7fd}";
pub const ICON_REFRESH: &str = "\u{e5d5}";
pub const ICON_REMOVE: &str = "\u{e15b}";
pub const ICON_SEARCH: &str = "\u{e8b6}";
pub const ICON_SEND: &str = "\u{e163}";
pub const ICON_SETTINGS: &str = "\u{e8b8}";
pub const ICON_SHARE: &str = "\u{e80d}";
pub const ICON_STAR: &str = "\u{e838}";
pub const ICON_STAR_BORDER: &str = "\u{e83a}";
pub const ICON_VISIBILITY: &str = "\u{e8f4}";
pub const ICON_VISIBILITY_OFF: &str = "\u{e8f5}";
pub const ICON_WARNING: &str = "\u{e002}";
pub const ICON_PALETTE: &str = "\u{e40a}";
pub const ICON_BRIGHTNESS_4: &str = "\u{e3a9}";
pub const ICON_BRIGHTNESS_7: &str = "\u{e3ac}";
pub const ICON_CALENDAR_TODAY: &str = "\u{e935}";
pub const ICON_DASHBOARD: &str = "\u{e871}";
pub const ICON_SHOPPING_CART: &str = "\u{e8cc}";
pub const ICON_ACCOUNT_CIRCLE: &str = "\u{e853}";
pub const ICON_CHECK_CIRCLE: &str = "\u{e86c}";
pub const ICON_RADIO_BUTTON_OFF: &str = "\u{e836}";
pub const ICON_RADIO_BUTTON_ON: &str = "\u{e837}";
pub const ICON_CONTENT_COPY: &str = "\u{e14d}";
pub const ICON_OPEN_IN_NEW: &str = "\u{e89e}";

// ── Font bytes ─────────────────────────────────────────────────────────────

#[cfg(all(not(target_arch = "wasm32"), has_material_icons))]
pub fn material_icons_ttf() -> Option<&'static [u8]> {
    Some(include_bytes!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/assets/MaterialIcons-Regular.ttf"
    )))
}

#[cfg(all(not(target_arch = "wasm32"), not(has_material_icons)))]
pub fn material_icons_ttf() -> Option<&'static [u8]> {
    None
}

#[cfg(target_arch = "wasm32")]
pub fn material_icons_ttf() -> Option<&'static [u8]> {
    None // web: font served separately, loaded async by demo app
}

/// Register Material Icons font. Call during app creation (eframe CreationContext).
/// Uses FontDefinitions::default() as base — fonts are applied before the first frame renders.
pub fn register_font(ctx: &egui::Context) {
    let Some(bytes) = material_icons_ttf() else {
        return;
    };

    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "MaterialIcons".to_owned(),
        Arc::new(egui::FontData::from_static(bytes)),
    );
    fonts.families.insert(
        egui::FontFamily::Name("MaterialIcons".into()),
        vec!["MaterialIcons".to_owned()],
    );
    ctx.set_fonts(fonts);
}
