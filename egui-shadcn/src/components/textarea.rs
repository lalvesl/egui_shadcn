use crate::ShadcnTheme;
use egui::{CornerRadius, Sense, Stroke, Ui, Vec2};

pub struct Textarea<'a> {
    value: &'a mut String,
    placeholder: &'a str,
    label: Option<&'a str>,
    rows: usize,
    enabled: bool,
    /// Grow Y up to this many rows, then scroll. `None` = fixed at `rows`.
    max_rows: Option<usize>,
    /// Cap width. `None` = fill available width.
    max_width: Option<f32>,
    /// Always-scroll mode: fixed height (`rows`), vertical scrollbar inside.
    scroll: bool,
}

impl<'a> Textarea<'a> {
    pub fn new(value: &'a mut String) -> Self {
        Self {
            value,
            placeholder: "",
            label: None,
            rows: 4,
            enabled: true,
            max_rows: None,
            max_width: None,
            scroll: false,
        }
    }

    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = p;
        self
    }
    pub fn label(mut self, l: &'a str) -> Self {
        self.label = Some(l);
        self
    }
    pub fn rows(mut self, r: usize) -> Self {
        self.rows = r;
        self
    }
    pub fn enabled(mut self, e: bool) -> Self {
        self.enabled = e;
        self
    }
    /// Grow height up to `n` rows, then scroll when content exceeds.
    pub fn max_rows(mut self, n: usize) -> Self {
        self.max_rows = Some(n);
        self
    }
    /// Cap width at `w` pixels instead of filling all available space.
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }
    /// Fixed height equal to `rows`, with a vertical scrollbar inside.
    pub fn scroll(mut self, s: bool) -> Self {
        self.scroll = s;
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        let theme = ShadcnTheme::get(ui.ctx());

        if let Some(lbl) = self.label {
            ui.label(
                egui::RichText::new(lbl)
                    .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
                    .color(theme.foreground),
            );
            ui.add_space(4.0);
        }

        let line_h = 20.0;
        let v_pad = 8.0;
        let h_pad = 12.0;

        let width = {
            let avail = ui.available_width();
            self.max_width.map(|mw| avail.min(mw)).unwrap_or(avail)
        };

        // Determine display rows and whether to scroll.
        let content_lines = self.value.lines().count().max(1);
        let (display_rows, do_scroll) = if self.scroll {
            // explicit scroll: fixed height at rows (or max_rows if set), always scroll
            (self.max_rows.unwrap_or(self.rows), true)
        } else if let Some(max) = self.max_rows {
            // grow Y: expand from rows up to max, then scroll
            let rows = content_lines.min(max).max(self.rows);
            (rows, content_lines > max)
        } else {
            // fixed: no grow, no scroll
            (self.rows, false)
        };

        let height = line_h * display_rows as f32 + v_pad * 2.0;

        let cr = CornerRadius::same(theme.radius as u8);
        let bg = if self.enabled { theme.background } else { theme.muted };

        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());

        ui.painter().rect_filled(rect, cr, bg);
        ui.painter().rect_stroke(
            rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        let inner_rect = rect.shrink2(Vec2::new(h_pad, v_pad));

        let te = egui::TextEdit::multiline(self.value)
            .desired_width(inner_rect.width())
            .desired_rows(display_rows)
            .hint_text(self.placeholder)
            .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
            .text_color(theme.foreground)
            .interactive(self.enabled)
            .frame(egui::Frame::new());

        let mut child = ui.new_child(
            egui::UiBuilder::new()
                .max_rect(inner_rect)
                .layout(egui::Layout::top_down(egui::Align::LEFT)),
        );

        let resp = if do_scroll {
            egui::ScrollArea::vertical()
                .max_height(inner_rect.height())
                .auto_shrink([false, false])
                .show(&mut child, |ui| ui.add(te))
                .inner
        } else {
            child.add(te)
        };

        if resp.has_focus() {
            ui.painter().rect_stroke(
                rect.expand(1.0),
                cr,
                Stroke::new(2.0, theme.ring),
                egui::StrokeKind::Outside,
            );
        }

        resp
    }
}
