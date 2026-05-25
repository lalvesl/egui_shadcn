use egui::Ui;
use crate::theme::ShadcnTheme;

pub struct Label<'a> {
    text:  &'a str,
    size:  f32,
    muted: bool,
    bold:  bool,
}

impl<'a> Label<'a> {
    pub fn new(text: &'a str) -> Self {
        Self { text, size: 14.0, muted: false, bold: false }
    }

    pub fn size(mut self, s: f32) -> Self  { self.size = s; self }
    pub fn muted(mut self, m: bool) -> Self { self.muted = m; self }
    pub fn bold(mut self, b: bool) -> Self  { self.bold = b; self }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme  = ShadcnTheme::get(ui.ctx());
        let color  = if self.muted { theme.muted_foreground } else { theme.foreground };
        let mut rt = egui::RichText::new(self.text).size(self.size).color(color);
        if self.bold { rt = rt.strong(); }
        ui.label(rt)
    }
}
