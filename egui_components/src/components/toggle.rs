use super::size::Size;
use crate::ShadcnTheme;
use egui::{
    Align, Color32, CornerRadius, InnerResponse, Layout, Rect, Response,
    RichText, Sense, Stroke, StrokeKind, Ui, UiBuilder, Vec2,
};

/// Horizontal gap between items on the toggle surface — icon to label in the
/// default content, and between whatever `show_with` puts there.
const ICON_GAP: f32 = 6.0;

pub struct Toggle<'a> {
    pressed: &'a mut bool,
    label: &'a str,
    icon: Option<&'a str>,
    enabled: bool,
    size: Size,
    corner_radius: Option<CornerRadius>,
    bordered: bool,
}

impl<'a> Toggle<'a> {
    pub fn new(pressed: &'a mut bool, label: &'a str) -> Self {
        Self {
            pressed,
            label,
            icon: None,
            enabled: true,
            size: Size::Default,
            corner_radius: None,
            bordered: false,
        }
    }
    /// A toggle whose content the caller supplies through [`Toggle::show_with`].
    ///
    /// Same as `Toggle::new(pressed, "")` — the label is unused on that path.
    pub fn custom(pressed: &'a mut bool) -> Self {
        Self::new(pressed, "")
    }

    pub fn icon(mut self, i: &'a str) -> Self {
        self.icon = Some(i);
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }
    pub fn size(mut self, s: Size) -> Self {
        self.size = s;
        self
    }
    pub fn corner_radius(mut self, cr: CornerRadius) -> Self {
        self.corner_radius = Some(cr);
        self
    }
    pub fn bordered(mut self, b: bool) -> Self {
        self.bordered = b;
        self
    }

    /// Surface colors for the current state. `hovered` is the *previous*
    /// frame's hover — the surface has to be colored before the content is laid
    /// out, so this frame's rect does not exist yet.
    fn colors(&self, theme: &ShadcnTheme, hovered: bool) -> (Color32, Color32) {
        if *self.pressed {
            (theme.primary, theme.primary_foreground)
        } else if hovered && self.enabled {
            (theme.muted, theme.muted_foreground)
        } else {
            (theme.secondary, theme.secondary_foreground)
        }
    }

    /// Icon (optional) + label, the standard Shadcn toggle content.
    pub fn show(self, ui: &mut Ui) -> Response {
        let (icon, label, fs) = (self.icon, self.label, self.size.font_size());
        self.show_with(ui, |ui| {
            if let Some(ic) = icon {
                ui.label(RichText::new(ic).font(crate::icon_font_id(fs + 2.0)));
            }
            ui.label(RichText::new(label).size(fs));
        })
        .response
    }

    /// The same toggle surface, filled with arbitrary widgets.
    ///
    /// The content is laid out left-to-right, vertically centered in a row of
    /// [`Size::height`], inset by [`Size::h_pad`] on both sides; the surface
    /// grows to fit it. `override_text_color` is set to the state's foreground,
    /// so plain `ui.label` calls pick up the pressed/hover color for free.
    ///
    /// `label` and `icon` are ignored on this path — use [`Toggle::custom`].
    ///
    /// Two caveats inherited from `UiBuilder::sense`:
    /// - A click-sensing widget inside the content eats the click instead of
    ///   flipping the toggle.
    /// - `enabled(false)` dims the surface and stops it sensing clicks, but it
    ///   does not disable widgets in the content — guard those yourself.
    ///
    /// ```ignore
    /// Toggle::custom(&mut self.starred).bordered(true).show_with(ui, |ui| {
    ///     ui.label(RichText::new(ICON_STAR).font(icon_font_id(16.0)));
    ///     ui.label("Starred");
    ///     Badge::new("12").variant(BadgeVariant::Secondary).show(ui);
    /// });
    /// ```
    pub fn show_with<R>(
        self,
        ui: &mut Ui,
        content: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        let theme = ShadcnTheme::get(ui.ctx());
        let prev_opacity = ui.opacity();
        if !self.enabled {
            ui.multiply_opacity(ShadcnTheme::DISABLED_OPACITY);
        }
        let h_pad = self.size.h_pad();
        let height = self.size.height();

        // Reserve a slot so the surface paints *under* the content, which is
        // drawn before the rect it sits on is known.
        let bg_idx = ui.painter().add(egui::Shape::Noop);

        // Fixed-height row, inset by the horizontal padding: the cross axis is
        // exactly `height`, so `Align::Center` centers the content in it.
        let avail = ui.available_rect_before_wrap();
        let row = Rect::from_min_size(
            avail.min + Vec2::new(h_pad, 0.0),
            Vec2::new((avail.width() - h_pad * 2.0).max(0.0), height),
        );

        let mut bg = Color32::TRANSPARENT;
        let inner = ui.scope_builder(
            UiBuilder::new()
                .max_rect(row)
                .layout(Layout::left_to_right(Align::Center))
                .sense(if self.enabled {
                    Sense::click()
                } else {
                    Sense::hover()
                }),
            |ui| {
                let (b, fg) = self.colors(&theme, ui.response().hovered());
                bg = b;
                ui.visuals_mut().override_text_color = Some(fg);
                ui.spacing_mut().item_spacing.x = ICON_GAP;
                // Selectable labels sense click-and-drag, which would eat the
                // toggle's own click — text on a button-like surface is not
                // meant to be selectable anyway.
                ui.style_mut().interaction.selectable_labels = false;

                let ret = content(ui);

                // Pad the content bounds back out to the toggle surface, so
                // both the sensed rect and the space taken from the parent
                // cover the whole pill.
                let content_rect = ui.min_rect();
                ui.expand_to_include_rect(Rect::from_min_size(
                    egui::Pos2::new(content_rect.left() - h_pad, row.top()),
                    Vec2::new(
                        (content_rect.width() + h_pad * 2.0).max(height),
                        height,
                    ),
                ));
                ret
            },
        );

        let resp = inner.response;
        if resp.clicked() && self.enabled {
            *self.pressed = !*self.pressed;
        }

        let cr = self
            .corner_radius
            .unwrap_or(CornerRadius::same(theme.radius as u8));
        let stroke = if self.bordered {
            Stroke::new(1.0, theme.border)
        } else {
            Stroke::NONE
        };
        ui.painter().set(
            bg_idx,
            egui::epaint::RectShape::new(
                resp.rect,
                cr,
                bg,
                stroke,
                StrokeKind::Inside,
            ),
        );

        ui.set_opacity(prev_opacity);
        InnerResponse::new(inner.inner, resp)
    }
}
