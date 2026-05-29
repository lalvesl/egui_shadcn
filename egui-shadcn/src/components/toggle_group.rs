use super::size::Size;
use super::toggle::Toggle;
use egui::{CornerRadius, Ui};

pub struct ToggleGroupItem<'a> {
    pub label: &'a str,
    pub icon: Option<&'a str>,
}

pub struct ToggleGroup<'a, T: PartialEq + Clone> {
    items: &'a [(T, ToggleGroupItem<'a>)],
    selected: &'a mut T,
    enabled: bool,
    size: Size,
}

impl<'a, T: PartialEq + Clone> ToggleGroup<'a, T> {
    pub fn new(items: &'a [(T, ToggleGroupItem<'a>)], selected: &'a mut T) -> Self {
        Self { items, selected, enabled: true, size: Size::Default }
    }
    pub fn enabled(mut self, e: bool) -> Self { self.enabled = e; self }
    pub fn size(mut self, s: Size) -> Self { self.size = s; self }

    pub fn show(self, ui: &mut Ui) {
        let r = crate::ShadcnTheme::get(ui.ctx()).radius as u8;
        let n = self.items.len();

        ui.horizontal(|ui| {
            ui.spacing_mut().item_spacing.x = 0.0;

            for (i, (value, item)) in self.items.iter().enumerate() {
                let is_first = i == 0;
                let is_last = i == n - 1;

                let cr = CornerRadius {
                    nw: if is_first { r } else { 0 },
                    ne: if is_last  { r } else { 0 },
                    sw: if is_first { r } else { 0 },
                    se: if is_last  { r } else { 0 },
                };

                let mut pressed = *self.selected == *value;

                let mut toggle = Toggle::new(&mut pressed, item.label)
                    .enabled(self.enabled)
                    .size(self.size)
                    .corner_radius(cr)
                    .bordered(true);

                if let Some(ic) = item.icon {
                    toggle = toggle.icon(ic);
                }

                let resp = toggle.show(ui);

                if resp.clicked() && self.enabled {
                    *self.selected = value.clone();
                }
            }
        });
    }
}
