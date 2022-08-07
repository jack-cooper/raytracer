use glam::DVec3;

use crate::ray::Ray;

pub struct Camera {
    horizontal: DVec3,
    lower_left_corner: DVec3,
    origin: DVec3,
    vertical: DVec3,
}

impl Camera {
    pub fn new(
        position: DVec3,
        target: DVec3,
        up_vector: DVec3,
        vertical_fov: f64,
        aspect_ratio: f64,
    ) -> Self {
        let theta = std::f64::consts::PI / 180.0 * vertical_fov;
        let viewport_height = 2.0 * (theta * 0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let camera_z = (position - target).normalize();
        let camera_x = up_vector.cross(camera_z).normalize();
        let camera_y = camera_z.cross(camera_x);

        let horizontal = camera_x * viewport_width;
        let vertical = camera_y * viewport_height;

        let lower_left_corner = position - horizontal / 2.0 - vertical / 2.0 - camera_z;

        Self {
            horizontal,
            lower_left_corner,
            origin: position,
            vertical,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + x * self.horizontal + y * self.vertical - self.origin,
        )
    }
}
