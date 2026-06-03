use crate::ShadcnTheme;
use egui::{Color32, FontFamily, FontId, Response, Sense, Ui};

const ICON_FONT: &str = "MaterialIcons";

pub fn icon_font_id(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(ICON_FONT.into()))
}

pub struct Icon {
    glyph: &'static str,
    size: f32,
    color: Option<Color32>,
    sense: Sense,
}

impl Icon {
    pub fn new(glyph: &'static str) -> Self {
        Self {
            glyph,
            size: 20.0,
            color: None,
            sense: Sense::hover(),
        }
    }

    pub fn size(mut self, s: f32) -> Self {
        self.size = s;
        self
    }

    pub fn color(mut self, c: Color32) -> Self {
        self.color = Some(c);
        self
    }

    pub fn clickable(mut self) -> Self {
        self.sense = Sense::click();
        self
    }

    pub fn show(self, ui: &mut Ui) -> Response {
        let theme = ShadcnTheme::get(ui.ctx());
        let color = self.color.unwrap_or(theme.foreground);
        ui.add(
            egui::Label::new(
                egui::RichText::new(self.glyph)
                    .font(icon_font_id(self.size))
                    .color(color),
            )
            .sense(self.sense),
        )
    }

    pub fn paint(self, ui: &Ui, pos: egui::Pos2, align: egui::Align2, color: Color32) {
        ui.painter()
            .text(pos, align, self.glyph, icon_font_id(self.size), color);
    }
}
