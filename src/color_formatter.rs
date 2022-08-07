use glam::DVec3;

const MULTIPLIER: f64 = 256.0;

pub trait ColorFormatter {
    fn format_color(self, samples_per_pixel: u64) -> String;
}

impl ColorFormatter for DVec3 {
    fn format_color(self, samples_per_pixel: u64) -> String {
        let output = self / (samples_per_pixel as f64);
        let output = output.powf(0.5);
        let output = output.clamp(DVec3::splat(0.0), DVec3::splat(0.999));
        let output = output * MULTIPLIER;

        let r = output.x as u64;
        let g = output.y as u64;
        let b = output.z as u64;

        format!("{r} {g} {b}")
    }
}
