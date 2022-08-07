mod camera;
mod color_formatter;
mod hit;
mod material;
mod random;
mod ray;
mod reflect;
mod sphere;

use camera::Camera;
use color_formatter::ColorFormatter;
use glam::DVec3;
use hit::{Hittable, World};
use material::Lambertian;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sphere::Sphere;
use std::{io::Write, sync::Arc};

use crate::material::{Dielectric, Metal};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u64 = 512;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

const MAX_DEPTH: u8 = 100;

const SAMPLES_PER_PIXEL: u64 = 100;

type Color = DVec3;

fn main() {
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.5));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.14));

    let sphere_ground = Sphere::new_boxed(DVec3::new(0.0, -100.5, -1.0), 100.0, material_ground);
    let sphere_center = Sphere::new_boxed(DVec3::new(0.0, 0.0, -1.0), 0.5, material_center);
    let sphere_left = Sphere::new_boxed(DVec3::new(-1.0, 0.0, -1.0), 0.5, material_left);
    let sphere_right = Sphere::new_boxed(DVec3::new(1.0, 0.0, -1.0), 0.5, material_right);

    let world: Vec<Box<dyn Hittable>> =
        vec![sphere_ground, sphere_center, sphere_left, sphere_right];

    let camera = Camera::new(
        DVec3::new(-2.0, 2.0, 1.0),
        DVec3::NEG_Z,
        DVec3::Y,
        15.0,
        ASPECT_RATIO,
    );

    println!("P3");
    println!("{IMAGE_WIDTH} {IMAGE_HEIGHT}");
    println!("255");

    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;

    (0..IMAGE_HEIGHT).rev().for_each(|y| {
        eprint!("\rScanlines remaining: {:3}", y);
        std::io::stderr().flush().unwrap();

        let scanline: Vec<Color> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|x| {
                let x = x as f64;
                let y = y as f64;

                let mut pixel = DVec3::ZERO;

                (0..SAMPLES_PER_PIXEL).for_each(|_| {
                    let u = (x + fastrand::f64()) / width;
                    let v = (y + fastrand::f64()) / height;

                    let ray = camera.get_ray(u, v);

                    pixel += ray_color(&ray, &world, MAX_DEPTH);
                });

                pixel
            })
            .collect();

        scanline.into_iter().for_each(|pixel| {
            println!("{}", pixel.format_color(SAMPLES_PER_PIXEL));
        })
    });

    eprintln!();
    eprintln!("Finished.");
}

fn ray_color(ray: &Ray, world: &World, recursion_depth: u8) -> Color {
    if recursion_depth == 0 {
        return Color::ZERO;
    }

    if let Some(record) = world.hit(ray, (0.001, f64::INFINITY)) {
        if let Some((attenuation, scattered_ray)) = record.material.scatter(ray, &record) {
            attenuation * ray_color(&scattered_ray, world, recursion_depth - 1)
        } else {
            Color::ZERO
        }
    } else {
        let unit_direction = ray.direction().normalize();

        let t = 0.5 * unit_direction.y + 1.0;

        (1.0 - t) * Color::splat(1.0) + t * Color::new(0.5, 0.7, 1.0)
    }
}
