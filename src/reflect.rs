use glam::DVec3;

pub trait ReflectableDVec3 {
    fn reflect(self, normal: DVec3) -> DVec3;

    fn refract(self, normal: DVec3, eta_over_eta_prime: f64) -> DVec3;
}

impl ReflectableDVec3 for DVec3 {
    fn reflect(self, normal: DVec3) -> DVec3 {
        self - 2.0 * self.dot(normal) * normal
    }

    fn refract(self, normal: DVec3, eta_over_eta_prime: f64) -> DVec3 {
        let cos_theta = (-1.0 * self).dot(normal).min(1.0);

        let refracted_ray_perp = eta_over_eta_prime * (self + cos_theta * normal);
        let refracted_ray_parallel =
            (1.0 - refracted_ray_perp.length_squared()).abs().sqrt() * -normal;

        refracted_ray_perp + refracted_ray_parallel
    }
}
