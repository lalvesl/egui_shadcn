use super::spacing::Spacing;
use egui::{Color32, Frame, InnerResponse, Margin, Ui};

pub struct Boxed {
    padding: Spacing,
    margin: Option<Spacing>,
}

impl Default for Boxed {
    fn default() -> Self {
        Self {
            padding: Spacing::Md,
            margin: None,
        }
    }
}

impl Boxed {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn padding(mut self, s: Spacing) -> Self {
        self.padding = s;
        self
    }

    pub fn margin(mut self, s: Spacing) -> Self {
        self.margin = Some(s);
        self
    }

    pub fn show<R>(self, ui: &mut Ui, content: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        let inner_margin = Margin::same(self.padding.px() as i8);
        let outer_margin = Margin::same(self.margin.map(|s| s.px() as i8).unwrap_or(0));

        Frame::new()
            .fill(Color32::TRANSPARENT)
            .inner_margin(inner_margin)
            .outer_margin(outer_margin)
            .show(ui, content)
    }
}
