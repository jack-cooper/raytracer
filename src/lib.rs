pub mod camera;
mod dvec_extensions;
mod hit;
mod material;
pub mod output;
mod ray;
mod sphere;
pub mod world;

const ASPECT_RATIO: f64 = 16.0 / 9.0;

const IMAGE_WIDTH: u64 = 1024;
const IMAGE_HEIGHT: u64 = ((IMAGE_WIDTH as f64) / ASPECT_RATIO) as u64;

const MAX_DEPTH: u8 = 50;

const SAMPLES_PER_PIXEL: u64 = 100;

const OUTPUT_FILE: &str = "image.ppm";

type Color = glam::DVec3;
