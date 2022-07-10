use std::sync::Arc;

use crate::camera::rtweekend::ray::Ray;
use crate::camera::rtweekend::vec3::{dot, Point3};
use crate::hittablelist::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: Option<Arc<dyn Material>>,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let oc = r.orig - self.center;
        let a = r.direction().length().powi(2);
        let half_b = dot(&oc, &r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_ptr = self.mat_ptr.clone();
        true
    }
}
