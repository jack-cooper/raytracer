use glam::DVec3;

use crate::{
    hit::{Face, HitRecord},
    random::RandomInUnitSphere,
    ray::Ray,
    reflect::ReflectableDVec3,
    Color,
};

pub trait Scatter: Send + Sync {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)>;
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Scatter for Lambertian {
    fn scatter(&self, _ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        // Get random scatter direction, retrying the case where the random direction perfectly cancels the normal
        let scatter_direction = {
            loop {
                let scatter_direction =
                    record.normal + DVec3::random_in_unit_sphere().normalize_or_zero();
                if !scatter_direction.abs_diff_eq(DVec3::ZERO, f64::EPSILON) {
                    break scatter_direction;
                }
            }
        };

        let scattered_ray = Ray::new(record.position, scatter_direction);

        Some((self.albedo, scattered_ray))
    }
}

pub struct Metal {
    albedo: Color,
    fuzziness: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzziness: f64) -> Self {
        Self { albedo, fuzziness }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflection_direction = ray.direction().reflect(record.normal).normalize_or_zero();

        let scatter_direction =
            reflection_direction + self.fuzziness * DVec3::random_in_unit_sphere();

        let scattered_ray = Ray::new(record.position, scatter_direction);

        (scattered_ray.direction().dot(record.normal) > 0.0).then_some((self.albedo, scattered_ray))
    }
}

pub struct Dielectric {
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Self { refractive_index }
    }

    fn reflectance(cos_theta: f64, refractive_index: f64) -> f64 {
        // Magic Schlick formula, just trust it
        let r0 = (1.0 - refractive_index) / (1.0 + refractive_index);
        let r0 = r0.powi(2);

        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Scatter for Dielectric {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let refraction_ratio = if record.face == Face::Front {
            self.refractive_index.recip()
        } else {
            self.refractive_index
        };

        let unit_direction = ray.direction().normalize();

        let cos_theta = unit_direction.dot(-record.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let can_not_refract = refraction_ratio * sin_theta > 1.0;

        let will_reflect = fastrand::f64() < Self::reflectance(cos_theta, refraction_ratio);

        let new_direction = if can_not_refract || will_reflect {
            unit_direction.reflect(record.normal) // total internal reflection
        } else {
            unit_direction.refract(record.normal, refraction_ratio) // refraction
        };

        let scattered_ray = Ray::new(record.position, new_direction);

        Some((Color::ONE, scattered_ray))
    }
}
