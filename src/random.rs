use glam::{DVec2, DVec3};

pub trait RandomInUnitSphere {
    fn random() -> Self;
    fn random_in_unit_circle() -> Self;
    fn random_in_unit_sphere() -> Self;
}

impl RandomInUnitSphere for DVec3 {
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
