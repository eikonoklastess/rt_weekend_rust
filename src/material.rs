use crate::rtweekend::*;

pub trait Material: Send + Sync {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)>;
}

#[derive(Debug, Clone, Copy)]
pub struct NullMaterial;

impl Material for NullMaterial {
    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord) -> Option<(Ray, Color)> {
        // A null material typically absorbs all light or doesn't scatter
        None
    }
}

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        let scattered = Ray::new(rec.p, scatter_direction);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = Vec3::reflect(&r_in.dir, &rec.normal);
        reflected =
            reflected.normalized() + (self.fuzz.min(1.0).max(0.0) * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;
        Some((scattered, attenuation))
    }
}
