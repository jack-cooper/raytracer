mod color_formatter;
mod hit;
mod ray;
mod sphere;

use crate::{color_formatter::ColorFormatter, sphere::Sphere};
use glam::DVec3;
use hit::{HitRecord, Hittable, World};
use ray::Ray;
use std::io::Write;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u64 = 256;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

type Color = DVec3;

fn main() {
    // Camera setup
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = DVec3::ZERO;
    let horizontal = DVec3::X * viewport_width;
    let vertical = DVec3::Y * viewport_height;
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - DVec3::Z * focal_length;

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

            let u = x / width;
            let v = y / height;

            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            let pixel = ray_color(&ray, &world);

            println!("{}", pixel.format_color());
        });
    });

    eprintln!();
    eprintln!("Finished.");
}

fn ray_color(ray: &Ray, world: &World) -> Color {
    if let Some(HitRecord { normal, .. }) = world.hit(ray, (0.0, f64::INFINITY)) {
        0.5 * (normal + 1.0)
    } else {
        let unit_direction = ray.direction().normalize();

        let t = 0.5 * unit_direction.y + 1.0;

        (1.0 - t) * Color::splat(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
