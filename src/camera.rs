use glam::DVec3;

use crate::ray::Ray;

pub struct Camera {
    horizontal: DVec3,
    lower_left_corner: DVec3,
    origin: DVec3,
    vertical: DVec3,
}

impl Camera {
    pub const ASPECT_RATIO: f64 = 16.0 / 9.0;

    const VIEWPORT_HEIGHT: f64 = 2.0;
    const VIEWPORT_WIDTH: f64 = Self::VIEWPORT_HEIGHT * Self::ASPECT_RATIO;

    const FOCAL_LENGTH: f64 = 1.0;

    pub fn new() -> Self {
        let origin = DVec3::ZERO;
        let horizontal = DVec3::X * Self::VIEWPORT_WIDTH;
        let vertical = DVec3::Y * Self::VIEWPORT_HEIGHT;
        let lower_left_corner =
            origin - horizontal / 2.0 - vertical / 2.0 - DVec3::Z * Self::FOCAL_LENGTH;

        Self {
            horizontal,
            lower_left_corner,
            origin,
            vertical,
        }
    }

    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        )
    }
}
