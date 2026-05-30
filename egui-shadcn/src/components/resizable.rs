use crate::ShadcnTheme;
use egui::{CursorIcon, Rect, Sense, Ui, Vec2};

const HANDLE_HIT: f32 = 8.0;
const HANDLE_LINE: f32 = 1.0;
const GRIP_LONG: f32 = 24.0;
const GRIP_SHORT: f32 = 3.0;

pub enum ResizeDir {
    Horizontal,
    Vertical,
}

pub struct Resizable {
    id: egui::Id,
    dir: ResizeDir,
    initial_split: f32,
    min_size: f32,
    height: Option<f32>,
}

impl Resizable {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            dir: ResizeDir::Horizontal,
            initial_split: 0.5,
            min_size: 50.0,
            height: None,
        }
    }

    pub fn dir(mut self, d: ResizeDir) -> Self {
        self.dir = d;
        self
    }

    pub fn initial_split(mut self, s: f32) -> Self {
        self.initial_split = s.clamp(0.0, 1.0);
        self
    }

    pub fn min_size(mut self, m: f32) -> Self {
        self.min_size = m;
        self
    }

    pub fn height(mut self, h: f32) -> Self {
        self.height = Some(h);
        self
    }

    pub fn show(
        self,
        ui: &mut Ui,
        first_fn: impl FnOnce(&mut Ui),
        second_fn: impl FnOnce(&mut Ui),
    ) {
        let theme = ShadcnTheme::get(ui.ctx());
        let split_id = self.id.with("split");

        let mut split: f32 = ui
            .ctx()
            .data_mut(|d| d.get_persisted::<f32>(split_id).unwrap_or(self.initial_split));

        let avail = ui.available_rect_before_wrap();

        match self.dir {
            ResizeDir::Horizontal => {
                let total_w = avail.width();
                let total_h = self.height.unwrap_or_else(|| avail.height());

                let min_frac = self.min_size / total_w;
                let max_frac = 1.0 - self.min_size / total_w;
                split = split.clamp(min_frac, max_frac);

                let first_w = total_w * split - HANDLE_HIT / 2.0;
                let second_w = total_w * (1.0 - split) - HANDLE_HIT / 2.0;
                let handle_left = avail.left() + first_w;

                // First panel
                let first_rect = Rect::from_min_size(avail.min, Vec2::new(first_w, total_h));
                let mut first_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(first_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                first_fn(&mut first_child);

                // Handle hit area
                let handle_rect = Rect::from_min_size(
                    egui::Pos2::new(handle_left, avail.top()),
                    Vec2::new(HANDLE_HIT, total_h),
                );
                let handle_resp = ui.interact(handle_rect, self.id.with("handle"), Sense::drag());
                let active = handle_resp.hovered() || handle_resp.dragged();

                // Separator line (1px, centered in hit area)
                let line_cx = handle_left + HANDLE_HIT / 2.0;
                let line_rect = Rect::from_min_size(
                    egui::Pos2::new(line_cx - HANDLE_LINE / 2.0, avail.top()),
                    Vec2::new(HANDLE_LINE, total_h),
                );
                ui.painter().rect_filled(
                    line_rect,
                    egui::CornerRadius::ZERO,
                    if active { theme.ring } else { theme.border },
                );

                // Grip pill centered on line
                let grip_rect = Rect::from_center_size(
                    egui::Pos2::new(line_cx, avail.top() + total_h / 2.0),
                    Vec2::new(GRIP_SHORT, GRIP_LONG),
                );
                ui.painter().rect_filled(
                    grip_rect,
                    egui::CornerRadius::same(2),
                    if active { theme.ring } else { theme.muted_foreground },
                );

                if active {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
                }

                if handle_resp.dragged() {
                    let delta = handle_resp.drag_delta().x;
                    split = ((split * total_w + delta) / total_w).clamp(min_frac, max_frac);
                    ui.ctx().data_mut(|d| d.insert_persisted(split_id, split));
                }

                // Second panel
                let second_rect = Rect::from_min_size(
                    egui::Pos2::new(handle_left + HANDLE_HIT, avail.top()),
                    Vec2::new(second_w, total_h),
                );
                let mut second_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(second_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                second_fn(&mut second_child);

                ui.allocate_exact_size(Vec2::new(total_w, total_h), Sense::hover());
            }
            ResizeDir::Vertical => {
                let total_w = avail.width();
                let total_h = self.height.unwrap_or_else(|| avail.height());

                let min_frac = self.min_size / total_h;
                let max_frac = 1.0 - self.min_size / total_h;
                split = split.clamp(min_frac, max_frac);

                let first_h = total_h * split - HANDLE_HIT / 2.0;
                let second_h = total_h * (1.0 - split) - HANDLE_HIT / 2.0;
                let handle_top = avail.top() + first_h;

                // First panel
                let first_rect = Rect::from_min_size(avail.min, Vec2::new(total_w, first_h));
                let mut first_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(first_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                first_fn(&mut first_child);

                // Handle hit area
                let handle_rect = Rect::from_min_size(
                    egui::Pos2::new(avail.left(), handle_top),
                    Vec2::new(total_w, HANDLE_HIT),
                );
                let handle_resp = ui.interact(handle_rect, self.id.with("handle"), Sense::drag());
                let active = handle_resp.hovered() || handle_resp.dragged();

                // Separator line (1px, centered in hit area)
                let line_cy = handle_top + HANDLE_HIT / 2.0;
                let line_rect = Rect::from_min_size(
                    egui::Pos2::new(avail.left(), line_cy - HANDLE_LINE / 2.0),
                    Vec2::new(total_w, HANDLE_LINE),
                );
                ui.painter().rect_filled(
                    line_rect,
                    egui::CornerRadius::ZERO,
                    if active { theme.ring } else { theme.border },
                );

                // Grip pill centered on line
                let grip_rect = Rect::from_center_size(
                    egui::Pos2::new(avail.center().x, line_cy),
                    Vec2::new(GRIP_LONG, GRIP_SHORT),
                );
                ui.painter().rect_filled(
                    grip_rect,
                    egui::CornerRadius::same(2),
                    if active { theme.ring } else { theme.muted_foreground },
                );

                if active {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
                }

                if handle_resp.dragged() {
                    let delta = handle_resp.drag_delta().y;
                    split = ((split * total_h + delta) / total_h).clamp(min_frac, max_frac);
                    ui.ctx().data_mut(|d| d.insert_persisted(split_id, split));
                }

                // Second panel
                let second_rect = Rect::from_min_size(
                    egui::Pos2::new(avail.left(), handle_top + HANDLE_HIT),
                    Vec2::new(total_w, second_h),
                );
                let mut second_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(second_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                second_fn(&mut second_child);

                ui.allocate_exact_size(Vec2::new(total_w, total_h), Sense::hover());
            }
        }
    }
}
