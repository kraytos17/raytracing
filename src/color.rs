use std::io::{self, Write};

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn write_color(stream: &mut impl Write, pixel_color: &Color) -> io::Result<()> {
    let r = (255.999 * pixel_color.x) as i32;
    let g = (255.999 * pixel_color.y) as i32;
    let b = (255.999 * pixel_color.z) as i32;

    writeln!(stream, "{} {} {}", r, g, b)?;
    Ok(())
}
