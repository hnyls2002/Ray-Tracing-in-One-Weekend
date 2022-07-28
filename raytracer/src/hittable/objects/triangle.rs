use std::f64::INFINITY;

use crate::{
    basic::{
        degrees_to_radians,
        ray::Ray,
        vec3::{cross, dot, Point3, Vec3},
    },
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
};

#[derive(Clone, Copy)]
pub struct Triangle<TM: Material> {
    pub p: [Point3; 3],
    pub tex: [(f64, f64); 3],
    pub norm: [Vec3; 3],
    pub mat: TM,
}

#[allow(dead_code)]
impl<TM: Material> Triangle<TM> {
    pub fn new_from_obj(
        pt: &[Vec3],
        nm: &[Vec3],
        tx: &[(f64, f64)],
        idx: [usize; 3],
        mat: TM,
    ) -> Triangle<TM> {
        if tx.is_empty() {
            Triangle::<TM> {
                p: [pt[idx[0]], pt[idx[1]], pt[idx[2]]],
                tex: [(0.5, 0.5), (0.5, 0.5), (0.5, 0.5)],
                norm: [nm[idx[0]], nm[idx[1]], nm[idx[2]]],
                mat,
            }
        } else {
            Triangle::<TM> {
                p: [pt[idx[0]], pt[idx[1]], pt[idx[2]]],
                tex: [tx[idx[0]], tx[idx[1]], tx[idx[2]]],
                norm: [nm[idx[0]], nm[idx[1]], nm[idx[2]]],
                mat,
            }
        }
    }
    pub fn zoom(&mut self, origin: Vec3, b: f64) {
        for i in 0..3 {
            let offset = self.p[i] - origin;
            self.p[i] = origin + offset * b;
        }
    }
    pub fn trans(&mut self, offset: Vec3) {
        for i in 0..3 {
            self.p[i] += offset;
        }
    }
    #[allow(clippy::needless_range_loop)]
    pub fn rotate_xyz(&mut self, center: Vec3, r_x: f64, r_y: f64, r_z: f64) {
        let mut op: [Vec3; 3] = Default::default();
        let mut on: [Vec3; 3] = Default::default();

        for i in 0..3 {
            on[i] = self.norm[i];
            self.p[i] = self.p[i] - center;
            op[i] = self.p[i];
        }

        let cos_x = degrees_to_radians(r_x).cos();
        let sin_x = degrees_to_radians(r_x).sin();
        for i in 0..3 {
            self.p[i].1 = cos_x * op[i].1 - sin_x * op[i].2;
            self.p[i].2 = sin_x * op[i].1 + cos_x * op[i].2;
            op[i] = self.p[i];

            self.norm[i].1 = cos_x * on[i].1 - sin_x * on[i].2;
            self.norm[i].2 = sin_x * on[i].1 + cos_x * on[i].2;
            on[i] = self.norm[i];
        }

        let cos_y = degrees_to_radians(r_y).cos();
        let sin_y = degrees_to_radians(r_y).sin();
        for i in 0..3 {
            self.p[i].0 = cos_y * op[i].0 - sin_y * op[i].2;
            self.p[i].2 = sin_y * op[i].0 + cos_y * op[i].2;
            op[i] = self.p[i];

            self.norm[i].0 = cos_y * on[i].0 - sin_y * on[i].2;
            self.norm[i].2 = sin_y * on[i].0 + cos_y * on[i].2;
            on[i] = self.norm[i];
        }

        let cos_z = degrees_to_radians(r_z).cos();
        let sin_z = degrees_to_radians(r_z).sin();
        for i in 0..3 {
            self.p[i].0 = cos_z * op[i].0 - sin_z * op[i].1;
            self.p[i].1 = sin_z * op[i].0 + cos_z * op[i].1;

            self.norm[i].0 = cos_z * on[i].0 - sin_z * on[i].1;
            self.norm[i].1 = sin_z * on[i].0 + cos_z * on[i].1;
        }

        for i in 0..3 {
            self.p[i] += center;
        }
    }
    pub fn set_position(&mut self, center_old: Vec3, center_new: Vec3) {
        for q in self.p.iter_mut() {
            *q += center_new - center_old;
        }
    }
    fn get_hit_point(&self, r: &Ray) -> (Vec3, f64) {
        let e1 = self.p[1] - self.p[0];
        let e2 = self.p[2] - self.p[0];
        let s = r.orig - self.p[0];
        let s1 = cross(&r.dir, &e2);
        let s2 = cross(&s, &e1);

        let div = 1.0 / dot(&s1, &e1);

        let t = dot(&s2, &e2) * div;
        let mut b: [f64; 3] = [0.0; 3];
        b[1] = dot(&s1, &s) * div;
        b[2] = dot(&s2, &r.dir) * div;
        b[0] = 1.0 - b[1] - b[2];

        (self.p[0] * b[0] + self.p[1] * b[1] + self.p[2] * b[2], t)
    }
    fn get_barycentric_coordinates(&self, hit_point: &Vec3) -> [f64; 3] {
        // get real barycentric coordinates
        let mut n: [Vec3; 3] = Default::default();
        let area_vec = cross(&(self.p[1] - self.p[0]), &(self.p[2] - self.p[0]));
        n[0] = cross(&(self.p[2] - self.p[1]), &(*hit_point - self.p[1]));
        n[1] = cross(&(self.p[0] - self.p[2]), &(*hit_point - self.p[2]));
        n[2] = cross(&(self.p[1] - self.p[0]), &(*hit_point - self.p[0]));

        let mut c: [f64; 3] = Default::default();
        c[0] = dot(&n[0], &area_vec) / area_vec.length().powi(2);
        c[1] = dot(&n[1], &area_vec) / area_vec.length().powi(2);
        c[2] = dot(&n[2], &area_vec) / area_vec.length().powi(2);
        c
    }
}

impl<TM: Material> Hittable for Triangle<TM> {
    #[allow(clippy::many_single_char_names)]
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let (hit_point, t) = self.get_hit_point(r);

        if t < t_min || t > t_max {
            return false;
        }

        let c = self.get_barycentric_coordinates(&hit_point);

        let normal = (self.norm[0] * c[0] + self.norm[1] * c[1] + self.norm[2] * c[2]).unit_vec();

        // get texture-coordinates
        let mut u = 0.0;
        let mut v = 0.0;
        #[allow(clippy::needless_range_loop)]
        for i in 0..3 {
            u += self.tex[i].0 * c[i];
            v += self.tex[i].1 * c[i];
        }

        let mut rec_data = HitRecord {
            p: hit_point,
            normal,
            mat_ptr: &self.mat,
            t,
            u,
            v,
            front_face: Default::default(),
        };

        rec_data.set_face_normal(r, &normal);

        if (c[1] >= 0.0) && (c[2] >= 0.0) && (c[1] + c[2] <= 1.0) {
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
