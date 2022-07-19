use crate::{
    basic::PI,
    hittable::{HitRecord, Hittable},
};

use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, Point3, Vec3},
    },
    bvh::aabb::Aabb,
    material::Material,
};

#[derive(Clone, Copy)]
pub struct Sphere<TM>
where
    TM: Material,
{
    pub center: Point3,
    pub radius: f64,
    pub mat: TM,
}

impl<TM: Material> Sphere<TM> {
    fn get_sphere_uv(p: &Point3, u: &mut f64, v: &mut f64) {
        let theta = (-p.1).acos();
        let phi = (-p.2).atan2(p.0) + PI;
        *u = phi / (2.0 * PI);
        *v = theta / PI;
    }
}

impl<TM: Material> Hittable for Sphere<TM> {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
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
        let outward_normal = (r.at(root) - self.center) / self.radius;
        let mut rec_data = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            mat_ptr: &self.mat,
            t: root,
            u: Default::default(),
            v: Default::default(),
            front_face: Default::default(),
        };
        rec_data.set_face_normal(r, &outward_normal);
        Sphere::<TM>::get_sphere_uv(&outward_normal, &mut rec_data.u, &mut rec_data.v);
        *rec = Some(rec_data);
        true
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let cube = Vec3(self.radius, self.radius, self.radius);
        *output_box = Aabb {
            minimum: self.center - cube,
            maximum: self.center + cube,
        };
        true
    }
}
