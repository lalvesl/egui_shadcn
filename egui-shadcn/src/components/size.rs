#[derive(Clone, Copy, PartialEq, Eq, Default, Debug)]
pub enum Size {
    Sm,
    #[default]
    Default,
    Lg,
}

impl Size {
    pub fn height(self) -> f32 {
        match self { Size::Sm => 28.0, Size::Default => 36.0, Size::Lg => 44.0 }
    }
    pub fn font_size(self) -> f32 {
        match self { Size::Sm => 12.0, Size::Default => 14.0, Size::Lg => 16.0 }
    }
    pub fn h_pad(self) -> f32 {
        match self { Size::Sm => 8.0, Size::Default => 12.0, Size::Lg => 16.0 }
    }
    pub fn v_pad(self) -> f32 {
        match self { Size::Sm => 3.0, Size::Default => 5.0, Size::Lg => 8.0 }
    }
    pub fn icon_size(self) -> f32 {
        match self { Size::Sm => 14.0, Size::Default => 16.0, Size::Lg => 20.0 }
    }
    /// Circle diameter — avatar, etc.
    pub fn diameter(self) -> f32 {
        match self { Size::Sm => 32.0, Size::Default => 40.0, Size::Lg => 56.0 }
    }
    /// Square box — checkbox, radio indicator.
    pub fn box_size(self) -> f32 {
        match self { Size::Sm => 14.0, Size::Default => 16.0, Size::Lg => 20.0 }
    }
    /// (track_w, track_h, thumb_r) for Switch.
    pub fn switch_track(self) -> (f32, f32, f32) {
        match self {
            Size::Sm      => (36.0, 20.0,  8.0),
            Size::Default => (44.0, 24.0, 10.0),
            Size::Lg      => (54.0, 28.0, 12.0),
        }
    }
    /// (size, thickness) for Spinner.
    pub fn spinner(self) -> (f32, f32) {
        match self {
            Size::Sm      => (16.0, 2.0),
            Size::Default => (24.0, 2.5),
            Size::Lg      => (36.0, 3.0),
        }
    }
    /// (track_h, thumb_r) for Slider.
    pub fn slider_track(self) -> (f32, f32) {
        match self {
            Size::Sm      => (4.0, 7.0),
            Size::Default => (6.0, 9.0),
            Size::Lg      => (8.0, 12.0),
        }
    }
}
