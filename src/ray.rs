use glam::DVec3;

pub struct Ray {
    origin: DVec3,
    direction: DVec3,
}

impl Ray {
    pub fn new(origin: DVec3, direction: DVec3) -> Self {
        Self { origin, direction }
    }

    pub fn direction(&self) -> DVec3 {
        self.direction
    }

    pub fn origin(&self) -> DVec3 {
        self.origin
    }

    pub fn position_at(&self, t: f64) -> DVec3 {
        self.origin + self.direction * t
    }
}
