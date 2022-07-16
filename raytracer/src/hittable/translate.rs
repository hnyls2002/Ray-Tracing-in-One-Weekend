use std::sync::Arc;

use crate::{
    basic::{ray::Ray, vec3::Vec3},
    bvh::aabb::Aabb,
};

use super::{HitRecord, Hittable};

pub struct Translate {
    pub ptr: Arc<dyn Hittable>,
    pub offset: Vec3,
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            tm: r.tm,
        };
        if !self.ptr.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        rec.p += self.offset;
        rec.set_face_normal(&moved_r, &rec.normal.clone());
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.ptr.bounding_box(time0, time1, output_box) {
            return false;
        }
        *output_box = Aabb {
            minimum: output_box.min() + self.offset,
            maximum: output_box.max() + self.offset,
        };
        true
    }
}
