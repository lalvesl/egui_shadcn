//! Theme + color tokens. Build a whole chart skin from one primary color.

pub mod color;
pub mod harmony;
pub mod palette;

pub use color::{Oklch, from_oklch, mix_oklab, to_oklch};
pub use harmony::{Harmony, harmony_palette};
pub use palette::{Distribution, categorical, diverging, sequential};

use egui::Color32;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ThemeMode {
    Light,
    Dark,
}

impl ThemeMode {
    pub fn is_dark(self) -> bool {
        matches!(self, ThemeMode::Dark)
    }
}

#[derive(Clone, Debug)]
pub struct ChartTheme {
    pub mode: ThemeMode,
    pub primary: Color32,
    pub harmony: Harmony,
    pub distribution: Distribution,

    pub background: Color32,
    pub surface: Color32,
    pub axis_line: Color32,
    pub grid_line: Color32,
    pub split_area: [Color32; 2],
    pub text: Color32,
    pub text_dim: Color32,

    pub palette: Vec<Color32>,
    pub sequential: Vec<Color32>,
    pub diverging: Vec<Color32>,
}

impl ChartTheme {
    pub fn from_primary(
        primary: Color32,
        mode: ThemeMode,
        harmony: Harmony,
        series: usize,
        distribution: Distribution,
    ) -> Self {
        let dark = mode.is_dark();

        let mut palette = harmony_palette(primary, harmony);
        if palette.len() < series.max(1) {
            palette = categorical(primary, series.max(1), dark, distribution);
        }

        Self {
            mode,
            primary,
            harmony,
            distribution,

            background: if dark {
                Color32::from_rgb(0x14, 0x16, 0x1a)
            } else {
                Color32::from_rgb(0xff, 0xff, 0xff)
            },
            surface: if dark {
                Color32::from_rgb(0x22, 0x25, 0x2b)
            } else {
                Color32::from_rgb(0xf6, 0xf7, 0xf9)
            },
            axis_line: if dark {
                Color32::from_gray(0x55)
            } else {
                Color32::from_gray(0x99)
            },
            grid_line: if dark {
                Color32::from_gray(0x33)
            } else {
                Color32::from_gray(0xe2)
            },
            split_area: if dark {
                [
                    Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 0x06),
                    Color32::from_rgba_unmultiplied(0xff, 0xff, 0xff, 0x0c),
                ]
            } else {
                [
                    Color32::from_rgba_unmultiplied(0, 0, 0, 0x04),
                    Color32::from_rgba_unmultiplied(0, 0, 0, 0x09),
                ]
            },
            text: if dark {
                Color32::from_gray(0xea)
            } else {
                Color32::from_gray(0x20)
            },
            text_dim: if dark {
                Color32::from_gray(0x9a)
            } else {
                Color32::from_gray(0x70)
            },

            palette,
            sequential: sequential(primary, 9, dark),
            diverging: diverging(primary, 11, dark),
        }
    }

    /// Follow egui's own light/dark setting automatically.
    pub fn follow_egui(
        ctx: &egui::Context,
        primary: Color32,
        harmony: Harmony,
        series: usize,
    ) -> Self {
        let mode = if ctx.global_style().visuals.dark_mode {
            ThemeMode::Dark
        } else {
            ThemeMode::Light
        };
        Self::from_primary(primary, mode, harmony, series, Distribution::Even)
    }

    pub fn series_color(&self, i: usize) -> Color32 {
        self.palette[i % self.palette.len()]
    }

    pub fn rebuild_palette(&mut self, series: usize) {
        let mut palette = harmony_palette(self.primary, self.harmony);
        if palette.len() < series.max(1) {
            palette = categorical(
                self.primary,
                series.max(1),
                self.mode.is_dark(),
                self.distribution,
            );
        }
        self.palette = palette;
    }
}

impl Default for ChartTheme {
    fn default() -> Self {
        Self::from_primary(
            Color32::from_rgb(0x4f, 0x8c, 0xff),
            ThemeMode::Light,
            Harmony::Square,
            6,
            Distribution::Even,
        )
    }
}
