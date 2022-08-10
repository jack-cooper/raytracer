use crate::{
    camera::Camera, dvec_extensions::ColorFormat, world::World, IMAGE_HEIGHT, IMAGE_WIDTH,
    MAX_DEPTH, OUTPUT_FILE, SAMPLES_PER_PIXEL,
};
use glam::DVec3;
use rayon::prelude::{IntoParallelIterator, ParallelIterator};
use std::{fmt::Write as _, fs, io::Write};

pub fn write(camera: &Camera, world: &World) {
    let output_size: usize = (3 + 10 + 4 + (IMAGE_HEIGHT) * IMAGE_WIDTH * 12)
        .try_into()
        .unwrap();

    let mut output = String::with_capacity(output_size);

    write_ppm_headers(&mut output);

    let width = (IMAGE_WIDTH - 1) as f64;
    let height = (IMAGE_HEIGHT - 1) as f64;

    (0..IMAGE_HEIGHT).rev().for_each(|y| {
        eprint!("\rScanlines remaining: {:3}", y);
        std::io::stderr().flush().unwrap();

        let scanline: Vec<_> = (0..IMAGE_WIDTH)
            .into_par_iter()
            .map(|x| {
                let x = x as f64;
                let y = y as f64;

                let mut pixel = DVec3::ZERO;

                (0..SAMPLES_PER_PIXEL).for_each(|_| {
                    let u = (x + fastrand::f64()) / width;
                    let v = (y + fastrand::f64()) / height;

                    let ray = camera.get_ray(u, v);

                    pixel += world.ray_color(&ray, MAX_DEPTH);
                });

                pixel
            })
            .collect();

        scanline.into_iter().for_each(|pixel| {
            writeln!(output, "{}", pixel.format_color(SAMPLES_PER_PIXEL)).unwrap();
        })
    });

    eprintln!();
    eprintln!("Finished.");

    fs::write(OUTPUT_FILE, output).unwrap();
}

fn write_ppm_headers(output: &mut String) {
    writeln!(output, "P3").unwrap();
    writeln!(output, "{IMAGE_WIDTH} {IMAGE_HEIGHT}").unwrap();
    writeln!(output, "255").unwrap();
}
