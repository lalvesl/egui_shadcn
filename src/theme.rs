use egui::Color32;

#[derive(Clone, Debug)]
pub struct ShadcnTheme {
    pub background: Color32,
    pub foreground: Color32,
    pub card: Color32,
    pub card_foreground: Color32,
    pub primary: Color32,
    pub primary_foreground: Color32,
    pub secondary: Color32,
    pub secondary_foreground: Color32,
    pub muted: Color32,
    pub muted_foreground: Color32,
    pub accent: Color32,
    pub accent_foreground: Color32,
    pub destructive: Color32,
    pub destructive_foreground: Color32,
    pub border: Color32,
    pub input: Color32,
    pub ring: Color32,
    pub radius: u8,
}

impl ShadcnTheme {
    pub fn light() -> Self {
        Self {
            background:           Color32::from_rgb(255, 255, 255),
            foreground:           Color32::from_rgb(9, 9, 11),
            card:                 Color32::from_rgb(255, 255, 255),
            card_foreground:      Color32::from_rgb(9, 9, 11),
            primary:              Color32::from_rgb(9, 9, 11),
            primary_foreground:   Color32::from_rgb(250, 250, 250),
            secondary:            Color32::from_rgb(244, 244, 245),
            secondary_foreground: Color32::from_rgb(9, 9, 11),
            muted:                Color32::from_rgb(244, 244, 245),
            muted_foreground:     Color32::from_rgb(113, 113, 122),
            accent:               Color32::from_rgb(244, 244, 245),
            accent_foreground:    Color32::from_rgb(9, 9, 11),
            destructive:          Color32::from_rgb(239, 68, 68),
            destructive_foreground: Color32::from_rgb(250, 250, 250),
            border:               Color32::from_rgb(228, 228, 231),
            input:                Color32::from_rgb(228, 228, 231),
            ring:                 Color32::from_rgb(9, 9, 11),
            radius:               6,
        }
    }

    pub fn dark() -> Self {
        Self {
            background:           Color32::from_rgb(9, 9, 11),
            foreground:           Color32::from_rgb(250, 250, 250),
            card:                 Color32::from_rgb(9, 9, 11),
            card_foreground:      Color32::from_rgb(250, 250, 250),
            primary:              Color32::from_rgb(250, 250, 250),
            primary_foreground:   Color32::from_rgb(9, 9, 11),
            secondary:            Color32::from_rgb(39, 39, 42),
            secondary_foreground: Color32::from_rgb(250, 250, 250),
            muted:                Color32::from_rgb(39, 39, 42),
            muted_foreground:     Color32::from_rgb(161, 161, 170),
            accent:               Color32::from_rgb(39, 39, 42),
            accent_foreground:    Color32::from_rgb(250, 250, 250),
            destructive:          Color32::from_rgb(153, 27, 27),
            destructive_foreground: Color32::from_rgb(250, 250, 250),
            border:               Color32::from_rgb(39, 39, 42),
            input:                Color32::from_rgb(39, 39, 42),
            ring:                 Color32::from_rgb(212, 212, 216),
            radius:               6,
        }
    }

    pub fn get(ctx: &egui::Context) -> Self {
        ctx.data(|d| d.get_temp::<ShadcnTheme>(egui::Id::new("shadcn_theme")))
            .unwrap_or_else(Self::light)
    }

    pub fn set(ctx: &egui::Context, theme: ShadcnTheme) {
        ctx.data_mut(|d| d.insert_temp(egui::Id::new("shadcn_theme"), theme));
    }

    pub fn apply_to_ctx(&self, ctx: &egui::Context) {
        let mut visuals = if self.is_dark() {
            egui::Visuals::dark()
        } else {
            egui::Visuals::light()
        };

        visuals.window_fill       = self.background;
        visuals.panel_fill        = self.background;
        visuals.faint_bg_color    = self.muted;
        visuals.extreme_bg_color  = self.background;
        visuals.window_stroke     = egui::Stroke::new(1.0, self.border);

        visuals.widgets.noninteractive.bg_fill    = self.background;
        visuals.widgets.noninteractive.fg_stroke  = egui::Stroke::new(1.0, self.foreground);
        visuals.widgets.inactive.bg_fill          = self.secondary;
        visuals.widgets.inactive.fg_stroke        = egui::Stroke::new(1.0, self.foreground);
        visuals.widgets.hovered.bg_fill           = self.accent;
        visuals.widgets.hovered.fg_stroke         = egui::Stroke::new(1.0, self.accent_foreground);
        visuals.widgets.active.bg_fill            = self.primary;
        visuals.widgets.active.fg_stroke          = egui::Stroke::new(1.0, self.primary_foreground);

        visuals.override_text_color = Some(self.foreground);

        ctx.set_visuals(visuals);
    }

    fn is_dark(&self) -> bool {
        self.background.r() < 128
    }

    pub fn with_alpha(color: Color32, alpha: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(color.r(), color.g(), color.b(), alpha)
    }
}
