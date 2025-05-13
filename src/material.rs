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
        Self {
            albedo,
            fuzz: fuzz.clamp(0.0, 1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let mut reflected = Vec3::reflect(&r_in.dir, &rec.normal);
        reflected = reflected.normalized() + (self.fuzz * Vec3::random_unit_vector());
        let scattered = Ray::new(rec.p, reflected);
        let attenuation = self.albedo;

        if scattered.dir.dot(rec.normal) > 0.0 {
            Some((scattered, attenuation))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    ir: f64,
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            ir: refraction_index,
        }
    }

    pub fn reflanctance(cosine: f64, refraction_index: f64) -> f64 {
        let mut r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord) -> Option<(Ray, Color)> {
        let attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.dir.normalized();

        let cos_theta = rec.normal.dot(-unit_direction).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cant_refract = { ri * sin_theta > 1.0 };
        let direction: Vec3;

        if cant_refract || Dielectric::reflanctance(cos_theta, ri) > random_f64() {
            direction = Vec3::reflect(&unit_direction, &rec.normal);
        } else {
            direction = Vec3::refract(&unit_direction, &rec.normal, ri);
        }

        let scattered = Ray::new(rec.p, direction);

        Some((scattered, attenuation))
    }
}
