mod dielectric;
mod lambertian;
mod metal;

pub use dielectric::Dielectric;
pub use lambertian::Lambertian;
pub use metal::Metal;

use crate::{hit::Collision, ray::Ray, Color};

pub trait Material: Send + Sync {
    fn scatter(&self, ray: &Ray, collision: &Collision) -> Option<(Color, Ray)>;
}
