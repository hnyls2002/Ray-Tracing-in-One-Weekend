use crate::camera::rtweekend::ray::Ray;
use crate::camera::rtweekend::vec3::{dot, Point3};
use crate::hittablelist::hittable::{HitRecord, Hittable};
use crate::material::Material;

pub struct Sphere<'a> {
    pub center: Point3,
    pub radius: f64,
    pub mat_ptr: &'a dyn Material,
}

impl<'a> Hittable<'a> for Sphere<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let oc = r.orig - self.center;
        let a = r.direction().length().powi(2);
        let half_b = dot(&oc, &r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            let root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        let mut rec_data = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            mat_ptr: self.mat_ptr,
            t: root,
            front_face: Default::default(),
        };
        let outward_normal = (rec_data.p - self.center) / self.radius;
        rec_data.set_face_normal(r, &outward_normal);
        *rec = Some(rec_data);
        true
    }
}
