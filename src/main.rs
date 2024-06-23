mod color;
mod ray;
mod vec3;

use color::{write_color, Color};
use ray::Ray;
use vec3::Vec3;

use std::fs::File;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let image_width = 400;
    let aspect_ratio = 16.0 / 9.0;
    let image_height = (image_width as f64 / aspect_ratio) as i32;

    let focal_length = 1.0;
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let camera_center = Vec3::new(0.0, 0.0, 0.0);
    let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
    let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);
    let pixel_delta_u = viewport_u / image_width as f64;
    let pixel_delta_v = viewport_v / image_height as f64;
    let viewport_upper_left =
        camera_center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
    let pixel_00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    let mut file = File::create("output.ppm")?;
    writeln!(&mut file, "P3\n{} {}\n255", image_width, image_height)?;

    for j in 0..image_height {
        for i in 0..image_width {
            let pixel_center =
                pixel_00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);
            let ray_dir = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_dir);
            let pixel_color = ray_color(&r);
            write_color(&mut file, &pixel_color)?;
        }
    }

    Ok(())
}

fn ray_color(r: &Ray) -> Color {
    let t = hit_sphere(Vec3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = Vec3::normalized(r.at(t) - Vec3::new(0.0, 0.0, -1.0));
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_dir = Vec3::normalized(r.dir);
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: Vec3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin - center;
    let a = r.dir.dot(r.dir);
    let b = 2.0 * oc.dot(r.dir);
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}
