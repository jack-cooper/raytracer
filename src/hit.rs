use glam::DVec3;

use crate::ray::Ray;

pub struct HitRecord {
    pub face: Face,
    pub position: DVec3,
    pub normal: DVec3,
    pub t: f64,
}

pub enum Face {
    Back,
    Front,
}

pub type World = Vec<Box<dyn Hittable>>;

impl Hittable for World {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<HitRecord> {
        self.iter()
            .flat_map(|hittable| hittable.hit(ray, (t_min, t_max)))
            .min_by(|hit_record, hit_record2| hit_record.t.total_cmp(&hit_record2.t))
    }
}

pub trait Hittable {
    // `t` tuple can be considered to be (`t_min`, `t_max`)
    fn hit(&self, ray: &Ray, t: (f64, f64)) -> Option<HitRecord>;
}
