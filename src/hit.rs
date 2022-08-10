use crate::{material::Material, ray::Ray};
use glam::DVec3;
use std::sync::Arc;

pub struct Collision {
    pub face: Face,
    pub material: Arc<dyn Material>,
    pub normal: DVec3,
    pub position: DVec3,
    pub t: f64,
}

#[derive(PartialEq, Eq)]
pub enum Face {
    Back,
    Front,
}

pub trait Hittable: Send + Sync {
    // `t` tuple can be considered to be (`t_min`, `t_max`)
    fn hit(&self, ray: &Ray, t: (f64, f64)) -> Option<Collision>;
}
