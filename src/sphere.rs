use std::sync::Arc;

use crate::{
    hit::{Collision, Face, Hittable},
    material::Material,
};
use glam::DVec3;

pub struct Sphere {
    pub center: DVec3,
    pub material: Arc<dyn Material>,
    pub radius: f64,
}

impl Sphere {
    pub fn new_boxed(center: DVec3, radius: f64, material: Arc<dyn Material>) -> Box<Self> {
        Box::new(Self {
            center,
            material,
            radius,
        })
    }
}

impl Hittable for Sphere {
    fn hit(&self, ray: &crate::ray::Ray, (t_min, t_max): (f64, f64)) -> Option<Collision> {
        let distance_to_center = ray.origin() - self.center;

        let a = ray.delta().length_squared();
        let half_b = distance_to_center.dot(ray.delta());
        let c = distance_to_center.length_squared() - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;

        if discriminant < 0.0 {
            return None;
        }

        // Check both roots (+/- discriminant_sqrt) to see if either lies between t_min and t_max
        let t = {
            let discriminant_sqrt = discriminant.sqrt();

            let mut candidate_t = (-half_b - discriminant_sqrt) / a;

            if candidate_t < t_min || t_max < candidate_t {
                candidate_t = (-half_b + discriminant_sqrt) / a;
                if candidate_t < t_min || t_max < candidate_t {
                    return None;
                }
            }

            candidate_t
        };

        let position = ray.position_at(t);
        let outward_normal = (position - self.center) / self.radius;

        let (face, normal) = if ray.delta().dot(outward_normal) < 0.0 {
            (Face::Front, outward_normal)
        } else {
            (Face::Back, -outward_normal)
        };

        Some(Collision {
            face,
            material: self.material.clone(),
            normal,
            position,
            t,
        })
    }
}
