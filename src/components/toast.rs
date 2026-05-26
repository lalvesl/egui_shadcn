/// Toast notification overlay — push short-lived messages from anywhere.
use egui::{Color32, Context, Margin, Sense, Stroke, Vec2};
use crate::theme::ShadcnTheme;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum ToastVariant {
    Default,
    Success,
    Destructive,
}

#[derive(Clone, Debug)]
struct ToastItem {
    id:          u64,
    title:       String,
    description: Option<String>,
    variant:     ToastVariant,
    expires_at:  f64,
    duration:    f64,
}

fn list_key()    -> egui::Id { egui::Id::new("__shadcn_toast_list__") }
fn next_id_key() -> egui::Id { egui::Id::new("__shadcn_toast_next__") }

const DURATION: f64 = 4.0;
const TOAST_W:  f32 = 340.0;
const SLOT_H:   f32 = 88.0;
const GAP:      f32 = 8.0;
const EDGE:     f32 = 16.0;

pub struct Toaster;

impl Toaster {
    #[allow(dead_code)]
    pub fn push(ctx: &Context, title: &str, variant: ToastVariant) {
        Self::enqueue(ctx, title, None, variant);
    }

    pub fn push_desc(ctx: &Context, title: &str, description: &str, variant: ToastVariant) {
        Self::enqueue(ctx, title, Some(description.to_owned()), variant);
    }

    fn enqueue(ctx: &Context, title: &str, description: Option<String>, variant: ToastVariant) {
        let now  = ctx.input(|i| i.time);
        let next = ctx.data(|d| d.get_temp::<u64>(next_id_key()).unwrap_or(0));

        let item = ToastItem {
            id: next,
            title: title.to_owned(),
            description,
            variant,
            expires_at: now + DURATION,
            duration:   DURATION,
        };

        ctx.data_mut(|d| {
            let mut list: Vec<ToastItem> = d.get_temp(list_key()).unwrap_or_default();
            list.push(item);
            d.insert_temp(list_key(), list);
            d.insert_temp(next_id_key(), next + 1);
        });

        ctx.request_repaint();
    }

    pub fn show(ctx: &Context) {
        let theme  = ShadcnTheme::get(ctx);
        let now    = ctx.input(|i| i.time);
        let screen = ctx.viewport_rect();

        let toasts: Vec<ToastItem> = ctx.data(|d| d.get_temp(list_key()).unwrap_or_default());
        if toasts.is_empty() { return; }

        let mut dismissed = Vec::<u64>::new();

        for (i, toast) in toasts.iter().enumerate() {
            let y = screen.bottom() - EDGE - (SLOT_H + GAP) * (i as f32 + 1.0);
            let x = screen.right() - TOAST_W - EDGE;

            let (accent, icon): (Color32, Option<(&str, Color32)>) = match toast.variant {
                ToastVariant::Success =>
                    (Color32::from_rgb(34, 197, 94), Some(("✓", Color32::from_rgb(34, 197, 94)))),
                ToastVariant::Destructive =>
                    (theme.destructive, Some(("✕", theme.destructive))),
                ToastVariant::Default =>
                    (theme.primary, None),
            };

            egui::Area::new(egui::Id::new("__shadcn_toast__").with(toast.id))
                .fixed_pos(egui::pos2(x, y))
                .order(egui::Order::Tooltip)
                .show(ctx, |ui| {
                    let fr = egui::Frame::new()
                        .fill(theme.card)
                        .stroke(Stroke::new(1.0, theme.border))
                        .corner_radius(theme.radius)
                        .inner_margin(Margin::symmetric(14, 12))
                        .show(ui, |ui| {
                            ui.set_width(TOAST_W - 32.0);

                            // Header row: icon + text + dismiss
                            ui.horizontal(|ui| {
                                if let Some((sym, col)) = icon {
                                    ui.add_space(2.0);
                                    ui.label(egui::RichText::new(sym).size(15.0).strong().color(col));
                                    ui.add_space(4.0);
                                }

                                ui.vertical(|ui| {
                                    ui.label(
                                        egui::RichText::new(&toast.title)
                                            .size(14.0)
                                            .strong()
                                            .color(theme.foreground),
                                    );
                                    if let Some(desc) = &toast.description {
                                        ui.add_space(2.0);
                                        ui.label(
                                            egui::RichText::new(desc)
                                                .size(13.0)
                                                .color(theme.muted_foreground),
                                        );
                                    }
                                });

                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Min), |ui| {
                                    let btn = ui.add(
                                        egui::Button::new(
                                            egui::RichText::new("✕")
                                                .size(11.0)
                                                .color(theme.muted_foreground),
                                        )
                                        .frame(false)
                                        .min_size(Vec2::splat(20.0)),
                                    );
                                    if btn.clicked() { dismissed.push(toast.id); }
                                });
                            });

                            // Progress bar
                            ui.add_space(10.0);
                            let ratio = ((toast.expires_at - now) / toast.duration)
                                .clamp(0.0, 1.0) as f32;
                            let bar_w = ui.available_width();
                            let (bg_rect, _) = ui.allocate_exact_size(Vec2::new(bar_w, 3.0), Sense::hover());
                            ui.painter().rect_filled(
                                bg_rect,
                                1u8,
                                ShadcnTheme::with_alpha(accent, 35),
                            );
                            ui.painter().rect_filled(
                                egui::Rect::from_min_size(bg_rect.min, Vec2::new(bar_w * ratio, 3.0)),
                                1u8,
                                accent,
                            );
                        });

                    // Left accent strip over the frame border
                    let r = fr.response.rect;
                    ui.painter().rect_filled(
                        egui::Rect::from_min_max(
                            egui::pos2(r.left(), r.top() + 6.0),
                            egui::pos2(r.left() + 3.0, r.bottom() - 6.0),
                        ),
                        1u8,
                        accent,
                    );
                });
        }

        let has_live = toasts.iter().any(|t| t.expires_at > now);

        ctx.data_mut(|d| {
            let mut list: Vec<ToastItem> = d.get_temp(list_key()).unwrap_or_default();
            list.retain(|t| !dismissed.contains(&t.id) && t.expires_at > now);
            d.insert_temp(list_key(), list);
        });

        if has_live { ctx.request_repaint(); }
    }
}
