pub use crate::camera::Camera;
pub use crate::color::{Color, write_color};
pub use crate::hittable::{HitRecord, Hittable};
pub use crate::hittable_list::HittableList;
pub use crate::interval::Interval;
pub use crate::material::{Lambertian, Material, Metal, NullMaterial};
pub use crate::ray::Ray;
pub use crate::sphere::Sphere;
pub use crate::vec3::{Point3, Vec3};

use rand::prelude::*;
pub use std::sync::Arc;

pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = std::f64::consts::PI;

#[inline]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[inline]
pub fn random_f64() -> f64 {
    let mut rng = rand::rng();
    rng.random::<f64>()
}

#[inline]
pub fn random_f64_range(min: f64, max: f64) -> f64 {
    min + (max - min) * random_f64()
}
