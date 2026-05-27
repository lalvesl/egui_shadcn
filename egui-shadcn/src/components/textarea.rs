use crate::ShadcnTheme;
use egui::{CornerRadius, Rect, Sense, Stroke, Ui, Vec2};

pub struct Textarea<'a> {
    value: &'a mut String,
    placeholder: &'a str,
    label: Option<&'a str>,
    rows: usize,
    enabled: bool,
    /// Grow height from `rows` up to this many rows, then scroll.
    max_rows: Option<usize>,
    /// Cap width instead of filling all available space.
    max_width: Option<f32>,
    /// Fixed height (`rows`), vertical scrollbar inside.
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
    /// Grow height from `rows` up to `n` rows, then scroll when exceeded.
    pub fn max_rows(mut self, n: usize) -> Self {
        self.max_rows = Some(n);
        self
    }
    /// Cap width at `w` pixels instead of filling all available space.
    pub fn max_width(mut self, w: f32) -> Self {
        self.max_width = Some(w);
        self
    }
    /// Fixed height equal to `rows`, vertical scrollbar inside.
    pub fn scroll(mut self, s: bool) -> Self {
        self.scroll = s;
        self
    }

    pub fn show(self, ui: &mut Ui) -> egui::Response {
        // Grab a stable per-instance ID before any allocations.
        let height_id = ui.next_auto_id();

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

        let cr = CornerRadius::same(theme.radius as u8);
        let bg = if self.enabled { theme.background } else { theme.muted };

        let te = egui::TextEdit::multiline(self.value)
            .desired_width(width - h_pad * 2.0)
            .desired_rows(self.rows)
            .hint_text(self.placeholder)
            .font(egui::FontId::new(14.0, egui::FontFamily::Proportional))
            .text_color(theme.foreground)
            .interactive(self.enabled)
            .frame(egui::Frame::new());

        if !self.scroll && self.max_rows.is_some() {
            // ── Grow mode ─────────────────────────────────────────────────────
            // Use the previous frame's measured content height so the border
            // frame matches actual rendered text (wrapping lines, not newlines).
            // One-frame lag is imperceptible during normal typing.
            let max_rows = self.max_rows.unwrap();
            let min_h = line_h * self.rows as f32;
            let max_h = line_h * max_rows as f32;

            let stored_h: f32 = ui
                .ctx()
                .data(|d| d.get_temp::<f32>(height_id).unwrap_or(min_h));
            let inner_h = stored_h.clamp(min_h, max_h);
            let height = inner_h + v_pad * 2.0;

            let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
            ui.painter().rect_filled(rect, cr, bg);
            ui.painter().rect_stroke(
                rect,
                cr,
                Stroke::new(1.0, theme.border),
                egui::StrokeKind::Inside,
            );

            let inner_rect = rect.shrink2(Vec2::new(h_pad, v_pad));
            // Tall max_rect so TextEdit can allocate its full content height;
            // set_clip_rect ensures nothing bleeds outside the border.
            let tall_rect = Rect::from_min_size(
                inner_rect.min,
                egui::vec2(inner_rect.width(), 10_000.0),
            );
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(tall_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            child.set_clip_rect(inner_rect);

            // When content has reached max height switch to a ScrollArea.
            let resp = if inner_h >= max_h {
                egui::ScrollArea::vertical()
                    .id_salt(height_id)
                    .max_height(inner_rect.height())
                    .auto_shrink([false, false])
                    .show(&mut child, |ui| ui.add(te))
                    .inner
            } else {
                child.add(te)
            };

            // Persist this frame's content height for the next frame.
            ui.ctx()
                .data_mut(|d| d.insert_temp(height_id, resp.rect.height()));

            if resp.has_focus() {
                ui.painter().rect_stroke(
                    rect.expand(1.0),
                    cr,
                    Stroke::new(2.0, theme.ring),
                    egui::StrokeKind::Outside,
                );
            }
            return resp;
        }

        // ── Scroll / fixed mode ───────────────────────────────────────────────
        let display_rows = if self.scroll {
            self.max_rows.unwrap_or(self.rows)
        } else {
            self.rows
        };
        let height = line_h * display_rows as f32 + v_pad * 2.0;

        let (rect, _) = ui.allocate_exact_size(Vec2::new(width, height), Sense::hover());
        ui.painter().rect_filled(rect, cr, bg);
        ui.painter().rect_stroke(
            rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        let inner_rect = rect.shrink2(Vec2::new(h_pad, v_pad));

        let resp = if self.scroll {
            // Tall child so TextEdit allocates full content; ScrollArea clips+scrolls.
            let tall_rect = Rect::from_min_size(
                inner_rect.min,
                egui::vec2(inner_rect.width(), 10_000.0),
            );
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(tall_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            child.set_clip_rect(inner_rect);

            egui::ScrollArea::vertical()
                .id_salt(height_id)
                .max_height(inner_rect.height())
                .auto_shrink([false, false])
                .show(&mut child, |ui| ui.add(te))
                .inner
        } else {
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(inner_rect)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            child.set_clip_rect(inner_rect);
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
