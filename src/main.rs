mod camera;
mod color_formatter;
mod hit;
mod random;
mod ray;
mod sphere;

use crate::{color_formatter::ColorFormatter, sphere::Sphere};
use camera::Camera;
use glam::DVec3;
use hit::{Hittable, World};
use random::RandomInUnitSphere;
use ray::Ray;
use std::io::Write;

const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / Camera::ASPECT_RATIO) as u64;

const MAX_DEPTH: u8 = 5;

const SAMPLES_PER_PIXEL: u64 = 100;

type Color = DVec3;

fn main() {
    let camera = Camera::new();

    // World setup
    let world: Vec<Box<dyn Hittable>> = vec![
        Box::new(Sphere::new(DVec3::new(0.0, 0.0, -1.0), 0.5)),
        Box::new(Sphere::new(DVec3::new(0.0, -100.5, -1.0), 100.0)),
    ];

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;

    (0..IMAGE_HEIGHT).rev().for_each(|y| {
        eprint!("\rScanlines remaining: {:3}", y);
        std::io::stderr().flush().unwrap();

        (0..IMAGE_WIDTH).for_each(|x| {
            let x = x as f64;
            let y = y as f64;

            let mut pixel = DVec3::ZERO;

            (0..SAMPLES_PER_PIXEL).for_each(|_| {
                let u = (x + fastrand::f64()) / width;
                let v = (y + fastrand::f64()) / height;

                let ray = camera.get_ray(u, v);

                pixel += ray_color(&ray, &world, MAX_DEPTH);
            });

            println!("{}", pixel.format_color(SAMPLES_PER_PIXEL));
        });
    });

    eprintln!();
    eprintln!("Finished.");
}

fn ray_color(ray: &Ray, world: &World, recursion_depth: u8) -> Color {
    if recursion_depth == 0 {
        return DVec3::ZERO;
    }

    if let Some(record) = world.hit(ray, (0.001, f64::INFINITY)) {
        let target =
            record.position + record.normal + DVec3::random_in_unit_sphere().normalize_or_zero();

        let ray = Ray::new(record.position, target - record.position);

        0.5 * ray_color(&ray, world, recursion_depth - 1)
    } else {
        let unit_direction = ray.direction().normalize();

        let t = 0.5 * unit_direction.y + 1.0;

        (1.0 - t) * Color::splat(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}