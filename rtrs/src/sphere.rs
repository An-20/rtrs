use crate::ray::Ray;
use crate::types::{Point3, Vec3};
use crate::hittable::{HitRecord, Hittable};


pub struct Sphere {
    centre: Point3,
    radius: f64
}

impl Sphere {
    pub fn new(centre: Point3, radius: f64) -> Self {
        Self { centre. radius}
    }
}

impl Hittable for Sphere {
    fn hit(ray: &Ray, ray_tmin: f64, ray_tmax: f64) -> Option<HitRecord> {
        let oc: Vec3 = ray.origin() - center;
        let a = ray.direction().norm_squared();
        let half_b = oc.dot(ray.direction());
        let c = oc.norm_squared() - radius * radius;
        let discriminant: f64 = half_b * half_b - a*c;
        
        if discriminant < 0.0 {
            return None;
        }
        let disc_sqrt: f64 = discriminant.sqrt();
        let root = (-half_b - disc_sqrt) / a;
        // check first root
        if (root <= ray_tmin || ray_tmax <= root) {
            // check other root
            root = (-half_b + disc_sqrt) / a;
            if (root <= ray_tmin || ray_tmax <= root) {
                return None;
            }
        }
        let hit_pos = ray.at(root);

        Some(HitRecord::new(hit_pos, root, ray, (hit_pos - center) / radius))
    }
}
