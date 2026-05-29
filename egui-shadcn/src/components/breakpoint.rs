use egui::Ui;

/// Screen-size breakpoint derived from `ui.available_width()`.
///
/// Sm  < 480 px
/// Md  < 768 px
/// Lg  < 1024 px
/// Xl  ≥ 1024 px
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum Breakpoint {
    Sm,
    Md,
    Lg,
    Xl,
}

impl Breakpoint {
    pub fn current(ui: &Ui) -> Self {
        match ui.available_width() {
            w if w < 480.0  => Self::Sm,
            w if w < 768.0  => Self::Md,
            w if w < 1024.0 => Self::Lg,
            _               => Self::Xl,
        }
    }

    pub fn is_mobile(self) -> bool { self <= Self::Sm }
    pub fn is_narrow(self) -> bool { self <= Self::Md }
}
