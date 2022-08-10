use std::sync::Arc;

use glam::DVec3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};

use crate::{
    camera::Camera,
    dvec_extensions::{ColorFormat, RandomDVec3},
    hit::{Collision, Hittable},
    material::*,
    ray::Ray,
    sphere::Sphere,
    Color, IMAGE_HEIGHT, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL,
};

pub struct World(Vec<Box<dyn Hittable>>);

impl World {
    pub fn new_random() -> Self {
        let mut world = Self(Vec::with_capacity(50));

        let ground_sphere = Sphere::new_boxed(
            DVec3::new(0.0, -1000.0, 0.0),
            1000.0,
            Arc::new(Lambertian::new(Color::splat(0.5))),
        );

        world.0.push(ground_sphere);

        world.add_random_spheres();

        let mut fixed_spheres: Vec<Box<dyn Hittable>> = vec![
            Sphere::new_boxed(DVec3::Y, 1.0, Arc::new(Dielectric::new(1.5))),
            Sphere::new_boxed(
                DVec3::new(-4.0, 1.0, 0.0),
                1.0,
                Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1))),
            ),
            Sphere::new_boxed(
                DVec3::new(4.0, 1.0, 0.0),
                1.0,
                Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)),
            ),
        ];

        world.0.append(&mut fixed_spheres);

        world
    }

    fn add_random_spheres(&mut self) {
        (-11..=11).for_each(|x| {
            (-11..=11).for_each(|z| {
                let sphere_position = DVec3::new(
                    x as f64 + fastrand::f64() * 0.9,
                    0.2,
                    z as f64 + fastrand::f64() * 0.9,
                );

                let rand = fastrand::f64();

                let material: Arc<dyn Material> = if rand < 0.8 {
                    Lambertian::new_arc(Color::random() * Color::random())
                } else if rand < 0.95 {
                    Metal::new_arc(Color::random() * 0.6 + 0.4, fastrand::f64() * 0.5)
                } else {
                    Dielectric::new_arc(1.5)
                };

                let sphere = Sphere::new_boxed(sphere_position, 0.2, material);

                self.0.push(sphere);
            })
        });
    }

    pub fn get_scanline(&self, camera: &Camera, line: u32) -> Vec<String> {
        let width = (IMAGE_WIDTH - 1) as f64;
        let height = (IMAGE_HEIGHT - 1) as f64;

        let y = f64::from(line);

        (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(f64::from)
            .map(|x| {
                (0..SAMPLES_PER_PIXEL).fold(DVec3::ZERO, |pixel, _| {
                    let u = (x + fastrand::f64()) / width;
                    let v = (y + fastrand::f64()) / height;

                    let ray = camera.get_ray(u, v);

                    pixel + self.ray_color(&ray, MAX_DEPTH)
                })
            })
            .map(|pixel| pixel.format_color(SAMPLES_PER_PIXEL))
            .collect()
    }

    pub fn ray_color(&self, ray: &Ray, recursion_depth: u8) -> Color {
        if recursion_depth == 0 {
            return Color::ZERO;
        }

        if let Some(collision) = self.hit(ray, (0.001, f64::INFINITY)) {
            if let Some((attenuation, scattered_ray)) = collision.material.scatter(ray, &collision)
            {
                attenuation * self.ray_color(&scattered_ray, recursion_depth - 1)
            } else {
                Color::ZERO
            }
        } else {
            let unit_delta = ray.delta().normalize();

            let t = 0.5 * unit_delta.y + 1.0;

            (1.0 - t) * Color::splat(1.0) + t * Color::new(0.5, 0.7, 1.0)
        }
    }
}

impl Hittable for World {
    fn hit(&self, ray: &Ray, (t_min, t_max): (f64, f64)) -> Option<Collision> {
        self.0
            .iter()
            .flat_map(|hittable| hittable.hit(ray, (t_min, t_max)))
            .min_by(|collision, collision2| collision.t.total_cmp(&collision2.t))
    }
}
