use crate::{Animations, ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT, ShadcnTheme};
use egui::{CornerRadius, Pos2, Rect, Sense, Stroke, Ui, Vec2};

/// In-flight slide transition: the outgoing slide index, when it started, and
/// the direction (`+1` = new slide enters from the right, `-1` = from the left).
#[derive(Clone, Copy)]
struct CarAnim {
    from: usize,
    start: f64,
    dir: f32,
}

const SLIDE_DUR_BASE: f32 = 0.32;

pub struct Carousel {
    id: egui::Id,
    item_count: usize,
    height: f32,
    width: f32,
    loop_: bool,
}

impl Carousel {
    pub fn new(id: impl std::hash::Hash, item_count: usize) -> Self {
        Self {
            id: egui::Id::new(id),
            item_count,
            height: 200.0,
            width: 0.0,
            loop_: true,
        }
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = h;
        self
    }

    pub fn width(mut self, w: f32) -> Self {
        self.width = w;
        self
    }

    pub fn loop_(mut self, l: bool) -> Self {
        self.loop_ = l;
        self
    }

    pub fn show(self, ui: &mut Ui, mut item_fn: impl FnMut(usize, &mut Ui)) -> usize {
        let theme = ShadcnTheme::get(ui.ctx());
        let cr = CornerRadius::same(theme.radius as u8);

        let avail_width = if self.width > 0.0 {
            self.width
        } else {
            ui.available_width()
        };

        let idx_id = self.id.with("index");
        let anim_id = self.id.with("anim");
        let mut current = ui.ctx().data(|d| d.get_temp::<usize>(idx_id).unwrap_or(0));
        let mut anim = ui
            .ctx()
            .data(|d| d.get_temp::<Option<CarAnim>>(anim_id))
            .flatten();

        // Clamp in case item_count changed
        if self.item_count > 0 {
            current = current.min(self.item_count - 1);
        } else {
            current = 0;
        }

        let btn_size = 32.0;
        let dot_area = 24.0;
        let total_height = self.height + dot_area;

        let (outer_rect, _) =
            ui.allocate_exact_size(Vec2::new(avail_width, total_height), Sense::hover());

        // Background
        ui.painter().rect_filled(outer_rect, cr, theme.card);
        ui.painter().rect_stroke(
            outer_rect,
            cr,
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );

        // Slide area (excluding dot row)
        let slide_rect = Rect::from_min_size(outer_rect.min, Vec2::new(avail_width, self.height));

        let now = ui.input(|i| i.time);
        let slide_dur = Animations::duration(ui.ctx(), SLIDE_DUR_BASE) as f64;

        // ── Render the slide(s) from persisted state, sliding them during a
        //    swap. A click is handled *after* the buttons (below) and the
        //    transition plays from the next frame, so the swap never pops. ──────
        let mut draw_slide = |ui: &mut Ui, idx: usize, dx: f32| {
            let r = slide_rect.translate(Vec2::new(dx, 0.0));
            let mut child = ui.new_child(
                egui::UiBuilder::new()
                    .max_rect(r)
                    .layout(egui::Layout::top_down(egui::Align::LEFT)),
            );
            child.set_clip_rect(slide_rect.intersect(ui.clip_rect()));
            item_fn(idx, &mut child);
        };

        if self.item_count > 0 {
            match anim {
                Some(a) if (now - a.start) < slide_dur && a.from < self.item_count => {
                    let t = ((now - a.start) / slide_dur).clamp(0.0, 1.0) as f32;
                    let e = 1.0 - (1.0 - t).powi(3); // ease-out cubic
                    let w = slide_rect.width();
                    let new_dx = a.dir * (1.0 - e) * w;
                    let old_dx = -a.dir * e * w;
                    draw_slide(ui, a.from, old_dx);
                    draw_slide(ui, current, new_dx);
                    ui.ctx().request_repaint();
                }
                _ => {
                    anim = None;
                    draw_slide(ui, current, 0.0);
                }
            }
        }

        // ── Navigation buttons, registered AFTER the slides so they sit on top
        //    for hit-testing even when a slide holds interactive content. ────────
        let prev_center = Pos2::new(
            outer_rect.left() + btn_size / 2.0 + 8.0,
            slide_rect.center().y,
        );
        let prev_rect = Rect::from_center_size(prev_center, Vec2::splat(btn_size));
        let prev_resp = ui.interact(prev_rect, self.id.with("prev"), Sense::click());

        let next_center = Pos2::new(
            outer_rect.right() - btn_size / 2.0 - 8.0,
            slide_rect.center().y,
        );
        let next_rect = Rect::from_center_size(next_center, Vec2::splat(btn_size));
        let next_resp = ui.interact(next_rect, self.id.with("next"), Sense::click());

        // ── Button visuals (drawn over the slides) ─────────────────────────────
        let prev_bg = if prev_resp.hovered() {
            theme.accent
        } else {
            ShadcnTheme::with_alpha(theme.background, 200)
        };
        ui.painter()
            .rect_filled(prev_rect, CornerRadius::same(btn_size as u8 / 2), prev_bg);
        ui.painter().rect_stroke(
            prev_rect,
            CornerRadius::same(btn_size as u8 / 2),
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            prev_center,
            egui::Align2::CENTER_CENTER,
            ICON_CHEVRON_LEFT,
            ShadcnTheme::icon_font(18.0),
            theme.foreground,
        );
        if prev_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        let next_bg = if next_resp.hovered() {
            theme.accent
        } else {
            ShadcnTheme::with_alpha(theme.background, 200)
        };
        ui.painter()
            .rect_filled(next_rect, CornerRadius::same(btn_size as u8 / 2), next_bg);
        ui.painter().rect_stroke(
            next_rect,
            CornerRadius::same(btn_size as u8 / 2),
            Stroke::new(1.0, theme.border),
            egui::StrokeKind::Inside,
        );
        ui.painter().text(
            next_center,
            egui::Align2::CENTER_CENTER,
            ICON_CHEVRON_RIGHT,
            ShadcnTheme::icon_font(18.0),
            theme.foreground,
        );
        if next_resp.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
        }

        // ── Handle clicks (after drawing): commit the new index and start (or
        //    refresh) the slide animation, which plays from the next frame. ──────
        if prev_resp.clicked() && self.item_count > 0 {
            let from = current;
            if current == 0 {
                if self.loop_ {
                    current = self.item_count - 1;
                }
            } else {
                current -= 1;
            }
            if current != from {
                anim = Some(CarAnim {
                    from,
                    start: now,
                    dir: -1.0,
                }); // enters from left
                ui.ctx().request_repaint();
            }
        }
        if next_resp.clicked() && self.item_count > 0 {
            let from = current;
            if current + 1 >= self.item_count {
                if self.loop_ {
                    current = 0;
                }
            } else {
                current += 1;
            }
            if current != from {
                anim = Some(CarAnim {
                    from,
                    start: now,
                    dir: 1.0,
                }); // enters from right
                ui.ctx().request_repaint();
            }
        }

        // Persist index + in-flight animation.
        ui.ctx().data_mut(|d| {
            d.insert_temp(idx_id, current);
            d.insert_temp(anim_id, anim);
        });

        // Dot indicators
        if self.item_count > 0 {
            let dot_row_y = outer_rect.top() + self.height + dot_area / 2.0;
            let dot_r = 4.0;
            let dot_spacing = 12.0;
            let total_dots_width =
                self.item_count as f32 * dot_spacing - (dot_spacing - dot_r * 2.0);
            let mut dot_x = outer_rect.center().x - total_dots_width / 2.0 + dot_r;

            for i in 0..self.item_count {
                let dot_center = Pos2::new(dot_x, dot_row_y);
                let dot_color = if i == current {
                    theme.primary
                } else {
                    theme.muted_foreground
                };
                let radius = if i == current { dot_r } else { dot_r * 0.7 };
                ui.painter().circle_filled(dot_center, radius, dot_color);
                dot_x += dot_spacing;
            }
        }

        current
    }
}
