//! Numeric text field: an [`Input`] that edits a number.
//!
//! The text is free while the field has focus and is committed — parsed,
//! clamped to the range, written back — on Enter or when focus leaves. Text
//! that does not parse draws the destructive border and is reverted on
//! commit, so the bound value is never anything but a valid number. Arrow
//! Up/Down step the value by `step` and commit at once.
//!
//! State (the text being edited) lives in `ctx.data` under the component's
//! `Id`, so two instances never share an edit buffer.

use std::ops::RangeInclusive;

use egui::{CornerRadius, Id, Key, Stroke, Ui, emath::Numeric};

use super::input::Input;
use crate::ShadcnTheme;

pub struct NumberInput<'a, T: Numeric> {
    id: Id,
    value: &'a mut T,
    range: RangeInclusive<f64>,
    step: f64,
    decimals: Option<usize>,
    unit: Option<&'a str>,
    width: f32,
    enabled: bool,
}

impl<'a, T: Numeric> NumberInput<'a, T> {
    /// A field editing `value`. `id_salt` must be unique among the number
    /// inputs shown together — it keys the edit buffer.
    pub fn new(
        id_salt: impl std::hash::Hash + std::fmt::Debug,
        value: &'a mut T,
    ) -> Self {
        Self {
            id: Id::new("shadcn_number_input").with(id_salt),
            value,
            range: T::MIN.to_f64()..=T::MAX.to_f64(),
            step: 1.0,
            decimals: None,
            unit: None,
            width: 120.0,
            enabled: true,
        }
    }

    /// Clamp committed values to this range.
    pub fn range(mut self, range: RangeInclusive<T>) -> Self {
        self.range = range.start().to_f64()..=range.end().to_f64();
        self
    }

    /// Amount Arrow Up/Down add or subtract.
    pub fn step(mut self, step: f64) -> Self {
        self.step = step;
        self
    }

    /// Decimals shown when the field is not being edited. Default: as many as
    /// the value needs, up to 6.
    pub fn decimals(mut self, d: usize) -> Self {
        self.decimals = Some(d);
        self
    }

    /// Unit shown after the field ("V", "Hz", "ms").
    pub fn unit(mut self, unit: &'a str) -> Self {
        self.unit = Some(unit);
        self
    }

    /// Width of the text field itself, not counting the unit.
    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }

    fn format(&self, v: f64) -> String {
        if T::INTEGRAL {
            return format!("{}", v.round() as i64);
        }
        match self.decimals {
            Some(d) => format!("{v:.d$}"),
            None => {
                let s = format!("{v:.6}");
                let s = s.trim_end_matches('0');
                s.strip_suffix('.').unwrap_or(s).to_string()
            }
        }
    }

    fn parse(&self, text: &str) -> Option<f64> {
        // A decimal comma is accepted: half the world types one.
        let v: f64 = text.trim().replace(',', ".").parse().ok()?;
        v.is_finite().then_some(v)
    }

    fn clamp(&self, v: f64) -> f64 {
        let v = v.clamp(*self.range.start(), *self.range.end());
        if T::INTEGRAL { v.round() } else { v }
    }

    /// Returns true in the frame the bound value was committed with a new
    /// value — not on every keystroke, which is what a caller that restarts
    /// something on change needs.
    pub fn show(self, ui: &mut Ui) -> bool {
        let theme = ShadcnTheme::get(ui.ctx());
        let current = self.value.to_f64();
        let editing: Option<String> = ui.ctx().data(|d| d.get_temp(self.id));
        let mut text = editing.clone().unwrap_or_else(|| self.format(current));
        let invalid = self.parse(&text).is_none();

        let inner = ui.horizontal(|ui| {
            let field = ui.scope(|ui| {
                Input::new(&mut text)
                    .width(self.width)
                    .enabled(self.enabled)
                    .show(ui)
            });
            if invalid {
                ui.painter().rect_stroke(
                    field.response.rect,
                    CornerRadius::same(theme.radius as u8),
                    Stroke::new(1.5, theme.destructive),
                    egui::StrokeKind::Inside,
                );
            }
            if let Some(unit) = self.unit {
                ui.label(
                    egui::RichText::new(unit).color(theme.muted_foreground),
                );
            }
            field.inner
        });
        let resp = inner.inner;

        let mut commit: Option<f64> = None;
        if resp.has_focus() {
            let (up, down, enter) = ui.input(|i| {
                (
                    i.key_pressed(Key::ArrowUp),
                    i.key_pressed(Key::ArrowDown),
                    i.key_pressed(Key::Enter),
                )
            });
            if up || down {
                let base = self.parse(&text).unwrap_or(current);
                commit = Some(base + if up { self.step } else { -self.step });
            } else if enter {
                commit = Some(self.parse(&text).unwrap_or(current));
            }
            if commit.is_none() {
                ui.ctx().data_mut(|d| d.insert_temp(self.id, text.clone()));
            }
        } else if resp.lost_focus() || editing.is_some() {
            // Focus left (Tab, click elsewhere, Enter on a single-line
            // field): commit what parses, revert what does not.
            commit = Some(self.parse(&text).unwrap_or(current));
        }

        if let Some(v) = commit {
            let v = self.clamp(v);
            // Read focus first: `has_focus` locks the context, and so does
            // `data_mut` — nesting them deadlocks.
            let focused = resp.has_focus();
            let text = self.format(v);
            ui.ctx().data_mut(|d| {
                if focused {
                    d.insert_temp(self.id, text);
                } else {
                    d.remove::<String>(self.id);
                }
            });
            if v != current {
                *self.value = T::from_f64(v);
                return true;
            }
        }
        false
    }
}
