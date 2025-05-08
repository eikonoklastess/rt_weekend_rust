use crate::rtweekend::*;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat: Arc<dyn Material + Send + Sync>,
}

impl Sphere {
    pub fn new(center: Point3, radius: f64, mat: Arc<dyn Material + Send + Sync>) -> Self {
        Self {
            center,
            radius,
            mat,
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord> {
        let oc = self.center - r.orig;
        let a = r.dir.length_squared();
        let h = r.dir.dot(oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();

        let root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            let root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return None;
            }
        }

        let mut rec = HitRecord::default();
        rec.t = root;
        rec.p = r.at(rec.t);
        rec.normal = (rec.p - self.center) / self.radius;
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = self.mat.clone();

        Some(rec)
    }
}
