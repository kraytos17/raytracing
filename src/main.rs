mod camera;
mod color;
mod hittable;
mod interval;
mod ray;
mod sphere;
mod utils;
mod vec3;

use camera::Camera;
use hittable::HittableList;
use sphere::Sphere;
use std::{fs::File, rc::Rc};
use vec3::Vec3;

fn main() -> std::io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let samples_per_pixel = 100;
    let camera = Camera::new(aspect_ratio, image_width, samples_per_pixel);

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)));

    let mut file = File::create("output.ppm")?;
    camera.render(&world, &mut file)?;

    Ok(())
}
