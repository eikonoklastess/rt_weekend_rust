mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use std::io;

use crate::rtweekend::*;

fn main() -> io::Result<()> {
    // world
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.1));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 0.5));

    let mut world = HittableList::new();
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    // camera
    let aspect_ratio = 16.0 / 9.0;
    let image_width: u32 = 1080;
    let sample_per_pixel: u32 = 100;
    let max_depth: u32 = 50;
    let cam = Camera::new(aspect_ratio, image_width, sample_per_pixel, max_depth);

    cam.render(&world)?;

    Ok(())
}
