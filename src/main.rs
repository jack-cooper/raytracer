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
use material::{Dielectric, Lambertian, Metal};
use random::RandomInUnitSphere;
use ray::Ray;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use sphere::Sphere;
use std::{io::Write, sync::Arc};

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u64 = 1024;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

const MAX_DEPTH: u8 = 50;

const SAMPLES_PER_PIXEL: u64 = 100;

type Color = DVec3;

fn main() {
    let world = setup_random_world();

    let camera = setup_camera();

    print_output(&camera, &world);
}

fn print_output(camera: &Camera, world: &World) {
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

                    pixel += ray_color(&ray, world, MAX_DEPTH);
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

fn add_random_spheres(world: &mut World) {
    (-11..=11).for_each(|x| {
        (-11..=11).for_each(|z| {
            let rand = fastrand::f64();

            let sphere_position = DVec3::new(
                x as f64 + fastrand::f64() * 0.9,
                0.2,
                z as f64 + fastrand::f64() * 0.9,
            );

            if rand < 0.8 {
                let sphere = Sphere::new_boxed(
                    sphere_position,
                    0.2,
                    Arc::new(Lambertian::new(Color::random() * Color::random())),
                );

                world.push(sphere);
            } else if rand < 0.95 {
                let sphere = Sphere::new_boxed(
                    sphere_position,
                    0.2,
                    Arc::new(Metal::new(
                        Color::random() * 0.6 + 0.4,
                        fastrand::f64() * 0.5,
                    )),
                );

                world.push(sphere);
            } else {
                let sphere =
                    Sphere::new_boxed(sphere_position, 0.2, Arc::new(Dielectric::new(1.5)));

                world.push(sphere);
            }
        })
    });
}

fn setup_camera() -> Camera {
    let camera_position = DVec3::new(13.0, 2.0, 3.0);
    let camera_target = DVec3::ZERO;
    let focus_distance = 10.0;
    let aperture = 0.1;

    Camera::new(
        camera_position,
        camera_target,
        DVec3::Y,
        45.0,
        ASPECT_RATIO,
        aperture,
        focus_distance,
    )
}

fn setup_random_world() -> World {
    let mut world = World::with_capacity(50);

    let ground_sphere = Sphere::new_boxed(
        DVec3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(Color::splat(0.5))),
    );

    world.push(ground_sphere);

    add_random_spheres(&mut world);

    let mut fixed_spheres: World = vec![
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

    world.append(&mut fixed_spheres);

    world
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
