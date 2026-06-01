//! Anchored text + axis-label helpers.

use egui::FontId;

pub fn small_font() -> FontId {
    FontId::proportional(11.0)
}

pub fn label_font() -> FontId {
    FontId::proportional(12.0)
}

pub fn title_font() -> FontId {
    FontId::proportional(15.0)
}

/// Truncate `s` to fit `max_chars`, appending ellipsis.
pub fn ellipsis(s: &str, max_chars: usize) -> String {
    if s.chars().count() <= max_chars {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max_chars.saturating_sub(1)).collect();
        format!("{cut}…")
    }
}
