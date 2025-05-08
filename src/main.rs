mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::io;

use crate::rtweekend::*;

fn main() -> io::Result<()> {
    // world
    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Arc::new(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 480;
    let sample_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);

    cam.render(&world)?;

    Ok(())
}
