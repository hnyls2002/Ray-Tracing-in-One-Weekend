use std::f64::INFINITY;

use crate::{
    basic::{
        ray::Ray,
        vec3::{cross, dot, Point3, Vec3},
    },
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub struct Triangle<TM: Material> {
    pub p: [Point3; 3],
    pub norm: Vec3,
    pub mat: TM,
}

impl<TM: Material> Triangle<TM> {
    pub fn new(p0: Point3, p1: Point3, p2: Point3, mat: TM) -> Triangle<TM> {
        Triangle::<TM> {
            p: [p0, p1, p2],
            norm: cross(&(p1 - p0), &(p2 - p0)).unit_vec(),
            mat,
        }
    }
}

impl<TM: Material> Hittable for Triangle<TM> {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let e1 = self.p[1] - self.p[0];
        let e2 = self.p[2] - self.p[0];
        let s = r.orig - self.p[0];
        let s1 = cross(&r.dir, &e2);
        let s2 = cross(&s, &e1);

        let div = 1.0 / dot(&s1, &e1);

        let t = dot(&s2, &e2) * div;
        let b1 = dot(&s1, &s) * div;
        let b2 = dot(&s2, &r.dir) * div;
        let hit_point = self.p[0] * (1.0 - b1 - b2) + self.p[1] * b1 + self.p[2] * b2;

        if t < t_min || t > t_max {
            return false;
        }

        let mut rec_data = HitRecord {
            p: hit_point,
            normal: self.norm,
            mat_ptr: &self.mat,
            t,
            u: Default::default(),
            v: Default::default(),
            front_face: Default::default(),
        };

        rec_data.set_face_normal(r, &self.norm);

        if (b1 >= 0.0) && (b2 >= 0.0) && (b1 + b2 <= 1.0) {
            // Attention!!!
            // only when hit object, the rec can be Some(_)
            *rec = Some(rec_data);
            return true;
        }

        false
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        let mut min_p = Vec3::default();
        let mut max_p = Vec3::default();
        for i in 0..3 {
            // x,y,z
            let mut min = INFINITY;
            let mut max = -INFINITY;
            for j in 0..3 {
                // three points
                min = min.min(self.p[j][i]);
                max = max.max(self.p[j][i]);
            }
            min_p[i] = min;
            max_p[i] = max;
        }
        *output_box = Aabb {
            minimum: min_p - Vec3(0.001, 0.001, 0.001),
            maximum: max_p + Vec3(0.001, 0.001, 0.001),
        };
        true
    }
}
