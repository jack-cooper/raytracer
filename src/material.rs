use glam::DVec3;

use crate::{
    hit::HitRecord, random::RandomInUnitSphere, ray::Ray, reflect::ReflectableDVec3, Color,
};

pub trait Scatter {
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
}

impl Metal {
    pub fn new(albdeo: Color) -> Self {
        Self { albedo: albdeo }
    }
}

impl Scatter for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Color, Ray)> {
        let reflection_direction = ray.direction().reflect(record.normal).normalize_or_zero();

        let scattered_ray = Ray::new(record.position, reflection_direction);

        (scattered_ray.direction().dot(record.normal) > 0.0).then_some((self.albedo, scattered_ray))
    }
}
