use egui::Ui;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Spacing {
    Xs,
    Sm,
    Md,
    Lg,
    Xl,
    Xl2,
    Xl3,
}

impl Spacing {
    pub fn px(self) -> f32 {
        f32::from(self)
    }

    pub fn show(self, ui: &mut Ui) {
        ui.add_space(self.px());
    }
}

impl From<Spacing> for f32 {
    fn from(s: Spacing) -> f32 {
        match s {
            Spacing::Xs => 4.0,
            Spacing::Sm => 8.0,
            Spacing::Md => 12.0,
            Spacing::Lg => 16.0,
            Spacing::Xl => 24.0,
            Spacing::Xl2 => 32.0,
            Spacing::Xl3 => 48.0,
        }
    }
}

impl From<Spacing> for f64 {
    fn from(s: Spacing) -> f64 {
        f32::from(s) as f64
    }
}

impl From<Spacing> for u32 {
    fn from(s: Spacing) -> u32 {
        f32::from(s) as u32
    }
}

impl From<Spacing> for i32 {
    fn from(s: Spacing) -> i32 {
        f32::from(s) as i32
    }
}
