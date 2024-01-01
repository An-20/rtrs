use crate::ray::Ray;
use crate::types::{Point3, Vec3};


pub struct HitRecord {
    p: Point3,
    t: f64,
    normal: Vec3,
    front_face: bool
}

impl HitRecord {
    pub fn new(p: Point3, t: f64, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction().dot(&outward_normal) < 0.0;
        let normal = if front_face { outward_normal } else { -outward_normal };
        Self {
            p, t, normal, front_face
        }
    }
}


pub trait Hittable {
    fn hit(ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord>;
}
