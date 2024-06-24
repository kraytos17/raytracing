use std::io::Write;

use crate::{interval::Interval, vec3::Vec3};

pub type Color = Vec3;

pub fn write_color(output: &mut dyn Write, color: &Color) -> std::io::Result<()> {
    let intensity: Interval = Interval::new(0.000, 0.999);
    let rbyte = 256.0 * intensity.clamp(color.x);
    let gbyte = 256.0 * intensity.clamp(color.y);
    let bbyte = 256.0 * intensity.clamp(color.z);
    writeln!(output, "{} {} {}", rbyte as i32, gbyte as i32, bbyte as i32)
}
