use super::button::{Button, ButtonSize, ButtonVariant};
use super::size::Size;
use egui::{CornerRadius, Ui};

#[derive(Clone, Copy, PartialEq, Default, Debug)]
pub enum ButtonGroupVariant {
    #[default]
    Default,
    Outline,
}

pub struct ButtonGroup<'a> {
    buttons: &'a [&'a str],
    selected: Option<usize>,
    variant: ButtonGroupVariant,
    size: Size,
}

impl<'a> ButtonGroup<'a> {
    pub fn new(buttons: &'a [&'a str]) -> Self {
        Self { buttons, selected: None, variant: ButtonGroupVariant::Default, size: Size::Default }
    }

    pub fn selected(mut self, s: Option<usize>) -> Self { self.selected = s; self }
    pub fn variant(mut self, v: ButtonGroupVariant) -> Self { self.variant = v; self }
    pub fn size(mut self, s: Size) -> Self { self.size = s; self }

    pub fn show(self, ui: &mut Ui) -> Option<usize> {
        let r = crate::ShadcnTheme::get(ui.ctx()).radius as u8;
        let n = self.buttons.len();
        let mut clicked_idx: Option<usize> = None;

        let btn_size = match self.size {
            Size::Sm => ButtonSize::Sm,
            Size::Default => ButtonSize::Default,
            Size::Lg => ButtonSize::Lg,
        };

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for (i, &label) in self.buttons.iter().enumerate() {
                let is_first = i == 0;
                let is_last = i == n - 1;
                let is_selected = self.selected == Some(i);

                let cr = CornerRadius {
                    nw: if is_first { r } else { 0 },
                    ne: if is_last  { r } else { 0 },
                    sw: if is_first { r } else { 0 },
                    se: if is_last  { r } else { 0 },
                };

                let variant = if is_selected {
                    ButtonVariant::Default
                } else {
                    ButtonVariant::Outline
                };

                let resp = Button::new(label)
                    .variant(variant)
                    .size(btn_size)
                    .corner_radius(cr)
                    .show(ui);

                if resp.clicked() {
                    clicked_idx = Some(i);
                }
            }
        });

        clicked_idx
    }
}
