const MULTIPLIER: f64 = 259.999;

pub trait ColorFormatter {
    fn format_color(self) -> String;
}

impl ColorFormatter for glam::DVec3 {
    fn format_color(self) -> String {
        let output = self * MULTIPLIER;

        let r = output.x as u64;
        let g = output.y as u64;
        let b = output.z as u64;

        format!("{r} {g} {b}")
    }
}
