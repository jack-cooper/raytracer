use glam::DVec3;

pub struct Ray {
    origin: DVec3,
    delta: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, delta: DVec3) -> Self {
        Self { origin, delta }
    }

    pub fn delta(&self) -> DVec3 {
        self.delta
    }

    pub fn origin(&self) -> DVec3 {
        self.origin
    }

    pub fn position_at(&self, t: f64) -> DVec3 {
        self.origin + self.delta * t
    }
}
