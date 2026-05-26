use egui::{Color32, Context, FontFamily, FontId, Visuals};

// ── HSL helpers ──────────────────────────────────────────────────────────────

pub fn hsl(h: f32, s: f32, l: f32) -> Color32 {
    let h = h / 360.0;
    let (r, g, b) = if s == 0.0 {
        (l, l, l)
    } else {
        let q = if l < 0.5 {
            l * (1.0 + s)
        } else {
            l + s - l * s
        };
        let p = 2.0 * l - q;
        (
            hue2rgb(p, q, h + 1.0 / 3.0),
            hue2rgb(p, q, h),
            hue2rgb(p, q, h - 1.0 / 3.0),
        )
    };
    Color32::from_rgb((r * 255.0) as u8, (g * 255.0) as u8, (b * 255.0) as u8)
}

fn hue2rgb(p: f32, q: f32, mut t: f32) -> f32 {
    if t < 0.0 {
        t += 1.0;
    }
    if t > 1.0 {
        t -= 1.0;
    }
    if t < 1.0 / 6.0 {
        return p + (q - p) * 6.0 * t;
    }
    if t < 1.0 / 2.0 {
        return q;
    }
    if t < 2.0 / 3.0 {
        return p + (q - p) * (2.0 / 3.0 - t) * 6.0;
    }
    p
}

// relative luminance (WCAG)
fn luminance(c: Color32) -> f32 {
    let f = |v: u8| {
        let v = v as f32 / 255.0;
        if v <= 0.03928 {
            v / 12.92
        } else {
            ((v + 0.055) / 1.055).powf(2.4)
        }
    };
    0.2126 * f(c.r()) + 0.7152 * f(c.g()) + 0.0722 * f(c.b())
}

fn foreground_for(bg: Color32) -> Color32 {
    if luminance(bg) > 0.3 {
        hsl(0.0, 0.0, 0.08)
    } else {
        hsl(0.0, 0.0, 0.98)
    }
}

// ── Theme storage key ────────────────────────────────────────────────────────

const THEME_KEY: &str = "egui_shadcn_theme";

// ── ShadcnTheme ──────────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct ShadcnTheme {
    pub dark: bool,
    pub primary_hue: Option<f32>, // None → zinc default

    // Shadcn color tokens
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
    pub radius: f32,
}

impl ShadcnTheme {
    pub fn light() -> Self {
        Self::build(false, None)
    }
    pub fn dark() -> Self {
        Self::build(true, None)
    }

    pub fn build(dark: bool, primary_hue: Option<f32>) -> Self {
        if dark {
            Self::build_dark(primary_hue)
        } else {
            Self::build_light(primary_hue)
        }
    }

    fn build_light(hue: Option<f32>) -> Self {
        let (primary, primary_fg) = match hue {
            None => (hsl(240.0, 0.059, 0.10), hsl(0.0, 0.0, 0.98)),
            Some(h) => {
                let p = hsl(h, 0.83, 0.47);
                (p, foreground_for(p))
            }
        };
        Self {
            dark: false,
            primary_hue: hue,

            background: hsl(0.0, 0.0, 1.00),
            foreground: hsl(240.0, 0.10, 0.039),
            card: hsl(0.0, 0.0, 1.00),
            card_foreground: hsl(240.0, 0.10, 0.039),
            secondary: hsl(240.0, 0.048, 0.959),
            secondary_foreground: hsl(240.0, 0.059, 0.10),
            muted: hsl(240.0, 0.048, 0.959),
            muted_foreground: hsl(240.0, 0.038, 0.461),
            accent: hsl(240.0, 0.048, 0.959),
            accent_foreground: hsl(240.0, 0.059, 0.10),
            destructive: hsl(0.0, 0.842, 0.602),
            destructive_foreground: hsl(0.0, 0.0, 0.98),
            border: hsl(240.0, 0.059, 0.90),
            input: hsl(240.0, 0.059, 0.90),
            ring: primary,
            radius: 6.0,
            primary,
            primary_foreground: primary_fg,
        }
    }

    fn build_dark(hue: Option<f32>) -> Self {
        let (primary, primary_fg) = match hue {
            None => (hsl(0.0, 0.0, 0.98), hsl(240.0, 0.059, 0.10)),
            Some(h) => {
                let p = hsl(h, 0.85, 0.62);
                (p, foreground_for(p))
            }
        };
        Self {
            dark: true,
            primary_hue: hue,

            background: hsl(240.0, 0.10, 0.039),
            foreground: hsl(0.0, 0.0, 0.98),
            card: hsl(240.0, 0.10, 0.039),
            card_foreground: hsl(0.0, 0.0, 0.98),
            secondary: hsl(240.0, 0.037, 0.159),
            secondary_foreground: hsl(0.0, 0.0, 0.98),
            muted: hsl(240.0, 0.037, 0.159),
            muted_foreground: hsl(240.0, 0.05, 0.649),
            accent: hsl(240.0, 0.037, 0.159),
            accent_foreground: hsl(0.0, 0.0, 0.98),
            destructive: hsl(0.0, 0.628, 0.306),
            destructive_foreground: hsl(0.0, 0.0, 0.98),
            border: hsl(240.0, 0.037, 0.159),
            input: hsl(240.0, 0.037, 0.159),
            ring: primary,
            radius: 6.0,
            primary,
            primary_foreground: primary_fg,
        }
    }

    // ── Persistence ──────────────────────────────────────────────────────────

    pub fn get(ctx: &Context) -> Self {
        ctx.data(|d| d.get_temp::<ShadcnTheme>(egui::Id::new(THEME_KEY)))
            .unwrap_or_else(ShadcnTheme::dark)
    }

    pub fn set(ctx: &Context, theme: ShadcnTheme) {
        ctx.data_mut(|d| d.insert_temp(egui::Id::new(THEME_KEY), theme));
    }

    // ── Apply to egui visuals ────────────────────────────────────────────────

    pub fn apply(&self, ctx: &Context) {
        let mut vis = if self.dark {
            Visuals::dark()
        } else {
            Visuals::light()
        };

        vis.window_fill = self.background;
        vis.panel_fill = self.background;
        vis.override_text_color = Some(self.foreground);
        vis.window_stroke.color = self.border;
        vis.window_stroke.width = 1.0;
        vis.window_corner_radius = egui::CornerRadius::same(self.radius as u8);

        vis.widgets.noninteractive.bg_fill = self.card;
        vis.widgets.noninteractive.fg_stroke = egui::Stroke::new(1.0, self.foreground);
        vis.widgets.noninteractive.bg_stroke = egui::Stroke::new(1.0, self.border);
        vis.widgets.noninteractive.corner_radius = egui::CornerRadius::same(self.radius as u8);

        vis.widgets.inactive.bg_fill = self.secondary;
        vis.widgets.inactive.fg_stroke = egui::Stroke::new(1.0, self.secondary_foreground);
        vis.widgets.inactive.bg_stroke = egui::Stroke::new(1.0, self.border);
        vis.widgets.inactive.corner_radius = egui::CornerRadius::same(self.radius as u8);

        vis.widgets.hovered.bg_fill = self.accent;
        vis.widgets.hovered.fg_stroke = egui::Stroke::new(1.0, self.accent_foreground);
        vis.widgets.hovered.bg_stroke = egui::Stroke::new(1.0, self.ring);
        vis.widgets.hovered.corner_radius = egui::CornerRadius::same(self.radius as u8);

        vis.widgets.active.bg_fill = self.primary;
        vis.widgets.active.fg_stroke = egui::Stroke::new(1.0, self.primary_foreground);
        vis.widgets.active.bg_stroke = egui::Stroke::new(1.0, self.ring);
        vis.widgets.active.corner_radius = egui::CornerRadius::same(self.radius as u8);

        vis.selection.bg_fill = self.primary.gamma_multiply(0.25);
        vis.selection.stroke = egui::Stroke::new(1.0, self.ring);

        ctx.set_visuals(vis);
    }

    // ── Helpers ───────────────────────────────────────────────────────────────

    pub fn with_alpha(c: Color32, a: u8) -> Color32 {
        Color32::from_rgba_unmultiplied(c.r(), c.g(), c.b(), a)
    }

    pub fn text_style_body() -> egui::TextStyle {
        egui::TextStyle::Body
    }

    pub fn body_font() -> FontId {
        FontId::new(14.0, FontFamily::Proportional)
    }

    pub fn small_font() -> FontId {
        FontId::new(12.0, FontFamily::Proportional)
    }

    pub fn heading_font(size: f32) -> FontId {
        FontId::new(size, FontFamily::Proportional)
    }

    pub fn icon_font(size: f32) -> FontId {
        FontId::new(size, FontFamily::Name("MaterialIcons".into()))
    }
}
