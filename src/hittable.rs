use crate::rtweekend::*;

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f64,
    pub mat: Arc<dyn Material + Send + Sync>,
    pub front_face: bool,
}

impl HitRecord {
    #[inline]
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        self.front_face = r.dir.dot(outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            -outward_normal
        };
    }
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Point3::default(),    // or Point3::zero()
            normal: Vec3::default(), // or Vec3::zero()
            t: 0.0,
            // Use your placeholder material for the default
            mat: Arc::new(NullMaterial),
            front_face: true, //false,
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: Interval) -> Option<HitRecord>;
}
