//! Minimal 3D pipeline: world-space `Vec3`, orbit `Camera3D`, perspective
//! projection. Renderers sort their primitives by camera-space depth (painter's
//! algorithm) before drawing.

use egui::{Pos2, Rect};

#[derive(Clone, Copy, Debug, Default)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vec3 {
    pub const fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    pub fn scale(self, k: f32) -> Self {
        Self::new(self.x * k, self.y * k, self.z * k)
    }

    pub fn length(self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalized(self) -> Self {
        let l = self.length().max(1e-6);
        Self::new(self.x / l, self.y / l, self.z / l)
    }

    pub fn cross(self, o: Self) -> Self {
        Self::new(
            self.y * o.z - self.z * o.y,
            self.z * o.x - self.x * o.z,
            self.x * o.y - self.y * o.x,
        )
    }

    pub fn dot(self, o: Self) -> f32 {
        self.x * o.x + self.y * o.y + self.z * o.z
    }
}

impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, o: Self) -> Self {
        Self::new(self.x + o.x, self.y + o.y, self.z + o.z)
    }
}

impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, o: Self) -> Self {
        Self::new(self.x - o.x, self.y - o.y, self.z - o.z)
    }
}

/// Orbit camera: looks at `target`, sits at angle `(yaw, pitch)` and `distance`.
#[derive(Clone, Copy, Debug)]
pub struct Camera3D {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub fov: f32,
}

impl Camera3D {
    pub fn iso() -> Self {
        Self {
            target: Vec3::default(),
            yaw: 0.7,
            pitch: 0.5,
            distance: 4.0,
            fov: 0.9,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct ThreeDLayout {
    pub rect: Rect,
    pub camera: Camera3D,
}

impl ThreeDLayout {
    pub fn fit(rect: Rect) -> Self {
        Self::with_camera(rect, Camera3D::iso())
    }

    pub fn with_camera(rect: Rect, camera: Camera3D) -> Self {
        Self { rect, camera }
    }

    /// Project a world-space point into screen space + its camera-space depth.
    pub fn project(&self, p: Vec3) -> (Pos2, f32) {
        let cam = self.camera;
        let cp = cam.pitch.cos();
        let sp = cam.pitch.sin();
        let cy = cam.yaw.cos();
        let sy = cam.yaw.sin();
        // Eye basis: forward = -look_dir; right + up follow from yaw/pitch.
        let forward = Vec3::new(-sy * cp, -sp, -cy * cp);
        let right = Vec3::new(cy, 0.0, -sy);
        let up = forward.cross(right).normalized();
        let eye = cam.target + forward.scale(-cam.distance);
        let rel = p - eye;
        let cx = rel.dot(right);
        let cy_ = rel.dot(up);
        let cz = rel.dot(forward);
        // Perspective divide; clamp behind-camera points to a small epsilon
        // so they don't snap to infinity.
        let z = cz.max(0.05);
        let half = 0.5 * self.rect.size().min_elem();
        let f = half / (cam.fov.tan().max(1e-3));
        let sx = self.rect.center().x + (cx / z) * f;
        let sy_screen = self.rect.center().y - (cy_ / z) * f;
        (Pos2::new(sx, sy_screen), cz)
    }
}
