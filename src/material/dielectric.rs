use super::Material;
use crate::{
    dvec_extensions::ReflectableDVec3,
    hit::{Collision, Face},
    ray::Ray,
    Color,
};
use std::sync::Arc;

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    pub fn new_arc(refractive_index: f64) -> Arc<Self> {
        Arc::new(Self::new(refractive_index))
    }

    fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
        // Magic Schlick formula, just trust it
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0.powi(2);

        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Color, Ray)> {
        let refraction_ratio = if collision.face == Face::Front {
            self.refractive_index.recip()
        } else {
            self.refractive_index
        };

        let unit_delta = ray.delta().normalize();

        let cos_theta = unit_delta.dot(-collision.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let can_not_refract = refraction_ratio * sin_theta > 1.0;

        let will_reflect = fastrand::f64() < Self::reflectance(cos_theta, refraction_ratio);

        let new_direction = if can_not_refract || will_reflect {
            unit_delta.reflect(collision.normal) // total internal reflection
        } else {
            unit_delta.refract(collision.normal, refraction_ratio) // refraction
        };

        let scattered_ray = Ray::new(collision.position, new_direction);

        Some((Color::ONE, scattered_ray))
    }
}
