use glam::DVec3;

use crate::{dvec_extensions::RandomDVec3, ray::Ray, ASPECT_RATIO};

pub struct Camera {
    camera_x: DVec3,
    camera_y: DVec3,
    horizontal: DVec3,
    lens_radius: f64,
    lower_left_corner: DVec3,
    position: DVec3,
    vertical: DVec3,
}

impl Camera {
    pub fn new(
        position: DVec3,
        target: DVec3,
        up_vector: DVec3,
        vertical_fov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_distance: f64,
    ) -> Self {
        let theta = std::f64::consts::PI / 180.0 * vertical_fov;
        let viewport_height = 2.0 * (theta * 0.5).tan();
        let viewport_width = aspect_ratio * viewport_height;

        let camera_z = (position - target).normalize();
        let camera_x = up_vector.cross(camera_z).normalize();
        let camera_y = camera_z.cross(camera_x);

        let horizontal = camera_x * viewport_width * focus_distance;
        let vertical = camera_y * viewport_height * focus_distance;

        let lower_left_corner =
            position - horizontal / 2.0 - vertical / 2.0 - camera_z * focus_distance;

        Self {
            camera_x,
            camera_y,
            horizontal,
            lens_radius: aperture / 2.0,
            lower_left_corner,
            position,
            vertical,
        }
    }

    pub fn get_ray(&self, x: f64, y: f64) -> Ray {
        let radius = self.lens_radius * DVec3::random_in_unit_circle();

        let offset = self.camera_x * radius.x + self.camera_y * radius.y;

        Ray::new(
            self.position + offset,
            self.lower_left_corner + x * self.horizontal + y * self.vertical
                - self.position
                - offset,
        )
    }
}

impl Default for Camera {
    fn default() -> Self {
        let position = DVec3::new(13.0, 2.0, 3.0);
        let target = DVec3::ZERO;
        let focus_distance = 10.0;
        let aperture = 0.1;

        Self::new(
            position,
            target,
            DVec3::Y,
            45.0,
            ASPECT_RATIO,
            aperture,
            focus_distance,
        )
    }
}
