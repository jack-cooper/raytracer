use crate::{camera::Camera, world::World, IMAGE_HEIGHT, IMAGE_WIDTH, OUTPUT_FILE};
use std::{fmt::Write as _, fs, io::Write};

pub fn write(camera: &Camera, world: &World) {
    let output_size: usize = (3 + 10 + 4 + (IMAGE_HEIGHT) * IMAGE_WIDTH * 12)
        .try_into()
        .unwrap();

    let mut output = String::with_capacity(output_size);

    write_ppm_headers(&mut output);

    (0..IMAGE_HEIGHT).rev().for_each(|y| {
        eprint!("\rScanlines remaining: {:3}", y);
        std::io::stderr().flush().unwrap();

        let scanline = world.get_scanline(camera, y);

        write_ppm_body_line(&mut output, scanline);
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

fn write_ppm_body_line(output: &mut String, scanline: Vec<String>) {
    scanline.into_iter().for_each(|pixel| {
        writeln!(output, "{}", pixel).unwrap();
    });
}
