//! Equirectangular geographic projection — `(lon, lat)` → screen pixels.
//!
//! Phase 3 only ships this projection; alternatives (Mercator,
//! Robinson, …) can land later. All series in the "Geo" family agree
//! on the `(longitude, latitude)` ordering, in **degrees**.

use egui::{Pos2, Rect};

/// Lon/lat bounding box in degrees. `lon` runs left→right (`-180`..`180`),
/// `lat` runs bottom→top (`-90`..`90`).
#[derive(Clone, Copy, Debug)]
pub struct GeoBbox {
    pub min_lon: f64,
    pub max_lon: f64,
    pub min_lat: f64,
    pub max_lat: f64,
}

impl GeoBbox {
    pub fn world() -> Self {
        Self {
            min_lon: -180.0,
            max_lon: 180.0,
            min_lat: -90.0,
            max_lat: 90.0,
        }
    }

    pub fn aspect(&self) -> f32 {
        ((self.max_lon - self.min_lon) / (self.max_lat - self.min_lat))
            .max(1e-6) as f32
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GeoLayout {
    pub plot_rect: Rect,
    pub bbox: GeoBbox,
}

impl GeoLayout {
    /// Fit a square-pixel projection into `rect`, preserving aspect ratio.
    pub fn fit(rect: Rect, bbox: GeoBbox) -> Self {
        let want_aspect = bbox.aspect();
        let have_aspect = rect.width() / rect.height().max(1e-6);
        let plot_rect = if have_aspect > want_aspect {
            let w = rect.height() * want_aspect;
            let dx = (rect.width() - w) * 0.5;
            Rect::from_min_size(
                egui::pos2(rect.min.x + dx, rect.min.y),
                egui::vec2(w, rect.height()),
            )
        } else {
            let h = rect.width() / want_aspect;
            let dy = (rect.height() - h) * 0.5;
            Rect::from_min_size(
                egui::pos2(rect.min.x, rect.min.y + dy),
                egui::vec2(rect.width(), h),
            )
        };
        Self { plot_rect, bbox }
    }

    pub fn project(&self, lon: f64, lat: f64) -> Pos2 {
        let dx =
            (lon - self.bbox.min_lon) / (self.bbox.max_lon - self.bbox.min_lon);
        let dy =
            (lat - self.bbox.min_lat) / (self.bbox.max_lat - self.bbox.min_lat);
        Pos2::new(
            self.plot_rect.min.x + dx as f32 * self.plot_rect.width(),
            self.plot_rect.max.y - dy as f32 * self.plot_rect.height(),
        )
    }
}
