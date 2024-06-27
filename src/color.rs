use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(output: &mut dyn Write, color: &Color) -> std::io::Result<()> {
    let r = Color::linear_to_gamma(color.x);
    let g = Color::linear_to_gamma(color.y);
    let b = Color::linear_to_gamma(color.z);

    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte = 256.0 * intensity.clamp(r);
    let gbyte = 256.0 * intensity.clamp(g);
    let bbyte = 256.0 * intensity.clamp(b);

    writeln!(output, "{} {} {}", rbyte as i32, gbyte as i32, bbyte as i32)
}

impl Color {
    #[inline]
    pub fn linear_to_gamma(comp: f64) -> f64 {
        if comp > 0.0 {
            f64::sqrt(comp)
        } else {
            0.0
        }
    }
}
