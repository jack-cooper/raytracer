use glam::DVec3;

pub trait ReflectableDVec3 {
    fn reflect(self, other: DVec3) -> DVec3;
}

impl ReflectableDVec3 for DVec3 {
    fn reflect(self, normal: DVec3) -> DVec3 {
        self - 2.0 * self.dot(normal) * normal
    }
}
