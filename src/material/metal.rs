use std::sync::Arc;

use glam::DVec3;

use crate::{
    dvec_extensions::{RandomDVec3, ReflectableDVec3},
    hit::Collision,
    ray::Ray,
    Color,
};

use super::Material;

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }

    pub fn new_arc(albedo: Color, fuzziness: f64) -> Arc<Self> {
        Arc::new(Self::new(albedo, fuzziness))
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Color, Ray)> {
        let reflection_direction = ray.delta().reflect(collision.normal).normalize();

        let scatter_direction =
            reflection_direction + self.fuzziness * DVec3::random_in_unit_sphere();

        let scattered_ray = Ray::new(collision.position, scatter_direction);

        (scattered_ray.delta().dot(collision.normal) > 0.0).then_some((self.albedo, scattered_ray))
    }
}
