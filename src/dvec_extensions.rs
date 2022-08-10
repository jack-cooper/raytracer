use glam::{DVec2, DVec3};

pub trait ColorFormat {
    fn format_color(self, samples_per_pixel: u64) -> String;
}

impl ColorFormat for DVec3 {
    fn format_color(self, samples_per_pixel: u64) -> String {
        let output = self / (samples_per_pixel as f64);
        let output = output.powf(0.5);
        let output = output.clamp(DVec3::splat(0.0), DVec3::splat(0.999));
        let output = output * 256.0;

        let r = output.x as u64;
        let g = output.y as u64;
        let b = output.z as u64;

        format!("{r} {g} {b}")
    }
}

pub trait RandomDVec3 {
    fn random() -> Self;
    fn random_in_unit_circle() -> Self;
    fn random_in_unit_sphere() -> Self;
}

impl RandomDVec3 for DVec3 {
    fn random() -> Self {
        Self::new(fastrand::f64(), fastrand::f64(), fastrand::f64())
    }

    fn random_in_unit_circle() -> Self {
        loop {
            // Get a random 2D vector in range -1.0..1.0 (inside the unit square)
            let vector = DVec2::new(fastrand::f64(), fastrand::f64());
            let vector = vector * 2.0 - 1.0;

            // If vector is not also inside unit circle, reject and try again
            if vector.length_squared() < 1.0 {
                return vector.extend(0.0);
            }
        }
    }

    fn random_in_unit_sphere() -> Self {
        loop {
            // Get a random vector in range -1.0..1.0 (inside the unit cube)
            let vector = DVec3::new(fastrand::f64(), fastrand::f64(), fastrand::f64());
            let vector = vector * 2.0 - 1.0;

            // If vector is not also inside unit sphere, reject and try again
            if vector.length_squared() < 1.0 {
                return vector;
            }
        }
    }
}

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
