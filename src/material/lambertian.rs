use std::sync::Arc;

use super::Material;
use crate::{dvec_extensions::RandomDVec3, hit::Collision, ray::Ray, Color};
use glam::DVec3;

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }

    pub fn new_arc(albedo: Color) -> Arc<Self> {
        Arc::new(Self::new(albedo))
    }
}

impl Material for Lambertian {
    fn scatter(&self, _ray: &Ray, collision: &Collision) -> Option<(Color, Ray)> {
        // Get random scatter direction, retrying the case where the random direction perfectly cancels the normal
        let scatter_direction = {
            loop {
                let scatter_direction =
                    collision.normal + DVec3::random_in_unit_sphere().normalize();
                if !scatter_direction.abs_diff_eq(DVec3::ZERO, f64::EPSILON) {
                    break scatter_direction;
                }
            }
        };

        let scattered_ray = Ray::new(collision.position, scatter_direction);

        Some((self.albedo, scattered_ray))
    }
}
