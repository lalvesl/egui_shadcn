use super::spacing::Spacing;
use crate::{ICON_CHECK_CIRCLE, ICON_CLOSE, ICON_ERROR, ICON_WARNING, ShadcnTheme};
use egui::{Color32, CornerRadius, Frame, Stroke, Vec2};

#[derive(Clone, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Warning,
    Destructive,
}

#[derive(Clone)]
pub struct ToastMessage {
    pub id: u64,
    pub title: String,
    pub description: Option<String>,
    pub variant: ToastVariant,
    pub duration_secs: f32,
    pub elapsed: f32,
}

const TOAST_QUEUE_KEY: &str = "shadcn_toasts";
const TOAST_COUNTER_KEY: &str = "shadcn_toast_counter";

pub struct Toaster;

impl Toaster {
    pub fn push(ctx: &egui::Context, title: impl Into<String>, variant: ToastVariant) {
        Self::push_internal(ctx, title.into(), None, variant);
    }

    pub fn push_with_desc(
        ctx: &egui::Context,
        title: impl Into<String>,
        description: impl Into<String>,
        variant: ToastVariant,
    ) {
        Self::push_internal(ctx, title.into(), Some(description.into()), variant);
    }

    fn push_internal(
        ctx: &egui::Context,
        title: String,
        description: Option<String>,
        variant: ToastVariant,
    ) {
        let id = ctx.data_mut(|d| {
            let counter_id = egui::Id::new(TOAST_COUNTER_KEY);
            let counter: u64 = d.get_temp(counter_id).unwrap_or(0);
            let next = counter + 1;
            d.insert_temp(counter_id, next);
            next
        });

        let msg = ToastMessage {
            id,
            title,
            description,
            variant,
            duration_secs: 5.0,
            elapsed: 0.0,
        };

        ctx.data_mut(|d| {
            let queue_id = egui::Id::new(TOAST_QUEUE_KEY);
            let mut queue: Vec<ToastMessage> = d.get_temp(queue_id).unwrap_or_default();
            queue.push(msg);
            d.insert_temp(queue_id, queue);
        });
    }

    pub fn show(ctx: &egui::Context) {
        let queue_id = egui::Id::new(TOAST_QUEUE_KEY);

        let dt = ctx.input(|i| i.unstable_dt);

        // Age all toasts and collect survivors
        let toasts: Vec<ToastMessage> = ctx.data_mut(|d| {
            let mut queue: Vec<ToastMessage> = d.get_temp(queue_id).unwrap_or_default();
            for t in &mut queue {
                t.elapsed += dt;
            }
            queue.retain(|t| t.elapsed < t.duration_secs);
            d.insert_temp(queue_id, queue.clone());
            queue
        });

        if toasts.is_empty() {
            return;
        }

        let theme = ShadcnTheme::get(ctx);
        let toast_width = 360.0;
        let screen = ctx.viewport_rect();

        // Render from bottom-right, stacking upward
        let mut ids_to_remove: Vec<u64> = Vec::new();

        let area_id = egui::Id::new("shadcn_toaster_area");
        egui::Area::new(area_id)
            .fixed_pos(egui::Pos2::new(
                screen.right() - toast_width - 16.0,
                screen.bottom() - 16.0,
            ))
            .order(egui::Order::Tooltip)
            .anchor(egui::Align2::RIGHT_BOTTOM, [0.0, 0.0])
            .show(ctx, |ui| {
                ui.set_max_width(toast_width);
                ui.vertical(|ui| {
                    // Render in reverse so newest is at bottom
                    for toast in toasts.iter().rev() {
                        let (accent_color, icon) = match toast.variant {
                            ToastVariant::Default => (theme.primary, ICON_CHECK_CIRCLE),
                            ToastVariant::Success => {
                                (Color32::from_rgb(34, 197, 94), ICON_CHECK_CIRCLE)
                            }
                            ToastVariant::Warning => (Color32::from_rgb(234, 179, 8), ICON_WARNING),
                            ToastVariant::Destructive => (theme.destructive, ICON_ERROR),
                        };

                        let toast_id = toast.id;
                        let item_height = if toast.description.is_some() {
                            72.0
                        } else {
                            52.0
                        };

                        let (toast_rect, _) = ui.allocate_exact_size(
                            Vec2::new(toast_width, item_height),
                            egui::Sense::hover(),
                        );

                        let cr = CornerRadius::same(theme.radius as u8);

                        // Card background
                        ui.painter().rect_filled(toast_rect, cr, theme.card);
                        ui.painter().rect_stroke(
                            toast_rect,
                            cr,
                            Stroke::new(1.0, theme.border),
                            egui::StrokeKind::Inside,
                        );

                        // Colored left border stripe
                        let stripe_rect =
                            egui::Rect::from_min_size(toast_rect.min, Vec2::new(4.0, item_height));
                        ui.painter().rect_filled(
                            stripe_rect,
                            CornerRadius {
                                nw: theme.radius as u8,
                                ne: 0,
                                sw: theme.radius as u8,
                                se: 0,
                            },
                            accent_color,
                        );

                        // Icon
                        ui.painter().text(
                            egui::Pos2::new(toast_rect.left() + 20.0, toast_rect.top() + 18.0),
                            egui::Align2::CENTER_CENTER,
                            icon,
                            crate::icon_font_id(16.0),
                            accent_color,
                        );

                        // Title
                        ui.painter().text(
                            egui::Pos2::new(toast_rect.left() + 36.0, toast_rect.top() + 18.0),
                            egui::Align2::LEFT_CENTER,
                            &toast.title,
                            egui::FontId::new(14.0, egui::FontFamily::Proportional),
                            theme.foreground,
                        );

                        // Description
                        if let Some(desc) = &toast.description {
                            ui.painter().text(
                                egui::Pos2::new(toast_rect.left() + 36.0, toast_rect.top() + 36.0),
                                egui::Align2::LEFT_CENTER,
                                desc,
                                egui::FontId::new(12.0, egui::FontFamily::Proportional),
                                theme.muted_foreground,
                            );
                        }

                        // Close button
                        let close_size = 20.0;
                        let close_rect = egui::Rect::from_center_size(
                            egui::Pos2::new(toast_rect.right() - 14.0, toast_rect.top() + 14.0),
                            Vec2::splat(close_size),
                        );
                        let close_resp = ui.interact(
                            close_rect,
                            egui::Id::new("toast_close").with(toast_id),
                            egui::Sense::click(),
                        );
                        if close_resp.hovered() {
                            ctx.set_cursor_icon(egui::CursorIcon::PointingHand);
                        }
                        ui.painter().text(
                            close_rect.center(),
                            egui::Align2::CENTER_CENTER,
                            ICON_CLOSE,
                            crate::icon_font_id(14.0),
                            if close_resp.hovered() {
                                theme.foreground
                            } else {
                                theme.muted_foreground
                            },
                        );
                        if close_resp.clicked() {
                            ids_to_remove.push(toast_id);
                        }

                        Spacing::Sm.show(ui);
                    }
                });
            });

        // Remove manually dismissed toasts
        if !ids_to_remove.is_empty() {
            ctx.data_mut(|d| {
                let mut queue: Vec<ToastMessage> = d.get_temp(queue_id).unwrap_or_default();
                queue.retain(|t| !ids_to_remove.contains(&t.id));
                d.insert_temp(queue_id, queue);
            });
        }

        // Request repaint while there are active toasts
        ctx.request_repaint();
    }
}

// Convenience re-export so callers can use `Frame` inside toast content if needed
#[allow(unused_imports)]
use Frame as _FrameReexport;
