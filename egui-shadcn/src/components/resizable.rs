use crate::ShadcnTheme;
use egui::{CursorIcon, Rect, Sense, Stroke, Ui, Vec2};

pub enum ResizeDir {
    Horizontal,
    Vertical,
}

pub struct Resizable {
    id: egui::Id,
    dir: ResizeDir,
    initial_split: f32,
    min_size: f32,
    handle_size: f32,
    height: Option<f32>,
}

impl Resizable {
    pub fn new(id: impl std::hash::Hash) -> Self {
        Self {
            id: egui::Id::new(id),
            dir: ResizeDir::Horizontal,
            initial_split: 0.5,
            min_size: 50.0,
            handle_size: 5.0,
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

                // Clamp split so each panel respects min_size
                let min_frac = self.min_size / total_w;
                let max_frac = 1.0 - self.min_size / total_w;
                split = split.clamp(min_frac, max_frac);

                let first_w = total_w * split - self.handle_size / 2.0;
                let second_w = total_w * (1.0 - split) - self.handle_size / 2.0;

                // First panel
                let first_rect =
                    Rect::from_min_size(avail.min, Vec2::new(first_w, total_h));
                let mut first_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(first_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                first_fn(&mut first_child);

                // Handle
                let handle_x = avail.left() + first_w;
                let handle_rect = Rect::from_min_size(
                    egui::Pos2::new(handle_x, avail.top()),
                    Vec2::new(self.handle_size, total_h),
                );
                let handle_resp =
                    ui.interact(handle_rect, self.id.with("handle"), Sense::drag());

                let handle_color = if handle_resp.hovered() || handle_resp.dragged() {
                    theme.ring
                } else {
                    theme.border
                };

                ui.painter()
                    .rect_filled(handle_rect, egui::CornerRadius::ZERO, handle_color);

                if handle_resp.hovered() || handle_resp.dragged() {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeHorizontal);
                }

                if handle_resp.dragged() {
                    let delta = handle_resp.drag_delta().x;
                    split = ((split * total_w + delta) / total_w).clamp(min_frac, max_frac);
                    ui.ctx().data_mut(|d| d.insert_persisted(split_id, split));
                }

                // Second panel
                let second_x = handle_x + self.handle_size;
                let second_rect = Rect::from_min_size(
                    egui::Pos2::new(second_x, avail.top()),
                    Vec2::new(second_w, total_h),
                );
                let mut second_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(second_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                second_fn(&mut second_child);

                // Advance cursor
                ui.allocate_exact_size(Vec2::new(total_w, total_h), Sense::hover());
            }
            ResizeDir::Vertical => {
                let total_w = avail.width();
                let total_h = self.height.unwrap_or_else(|| avail.height());

                let min_frac = self.min_size / total_h;
                let max_frac = 1.0 - self.min_size / total_h;
                split = split.clamp(min_frac, max_frac);

                let first_h = total_h * split - self.handle_size / 2.0;
                let second_h = total_h * (1.0 - split) - self.handle_size / 2.0;

                // First panel
                let first_rect =
                    Rect::from_min_size(avail.min, Vec2::new(total_w, first_h));
                let mut first_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(first_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                first_fn(&mut first_child);

                // Handle
                let handle_y = avail.top() + first_h;
                let handle_rect = Rect::from_min_size(
                    egui::Pos2::new(avail.left(), handle_y),
                    Vec2::new(total_w, self.handle_size),
                );
                let handle_resp =
                    ui.interact(handle_rect, self.id.with("handle"), Sense::drag());

                let handle_color = if handle_resp.hovered() || handle_resp.dragged() {
                    theme.ring
                } else {
                    theme.border
                };

                ui.painter().hline(
                    handle_rect.x_range(),
                    handle_rect.center().y,
                    Stroke::new(self.handle_size, handle_color),
                );

                if handle_resp.hovered() || handle_resp.dragged() {
                    ui.ctx().set_cursor_icon(CursorIcon::ResizeVertical);
                }

                if handle_resp.dragged() {
                    let delta = handle_resp.drag_delta().y;
                    split = ((split * total_h + delta) / total_h).clamp(min_frac, max_frac);
                    ui.ctx().data_mut(|d| d.insert_persisted(split_id, split));
                }

                // Second panel
                let second_y = handle_y + self.handle_size;
                let second_rect = Rect::from_min_size(
                    egui::Pos2::new(avail.left(), second_y),
                    Vec2::new(total_w, second_h),
                );
                let mut second_child = ui.new_child(
                    egui::UiBuilder::new()
                        .max_rect(second_rect)
                        .layout(egui::Layout::top_down(egui::Align::LEFT)),
                );
                second_fn(&mut second_child);

                // Advance cursor
                ui.allocate_exact_size(Vec2::new(total_w, total_h), Sense::hover());
            }
        }
    }
}
