use std::{f64::INFINITY, sync::Arc};

use crate::{
    bvh::aabb::Aabb,
    material::Material,
    rtweekend::{
        degrees_to_radians,
        ray::Ray,
        vec3::{dot, Point3, Vec3},
    },
};

#[derive(Default, Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: Option<Arc<dyn Material>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
}

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

pub struct RotateY {
    ptr: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    hasbox: bool,
    bbox: Aabb,
}

impl Hittable for RotateY {
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.bbox;
        self.hasbox
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut origin = r.orig;
        let mut direction = r.direction();

        origin[0] = self.cos_theta * r.orig[0] - self.sin_theta * r.orig[2];
        origin[2] = self.sin_theta * r.orig[0] + self.cos_theta * r.orig[2];

        direction[0] = self.cos_theta * r.dir[0] - self.sin_theta * r.dir[2];
        direction[2] = self.sin_theta * r.dir[0] + self.cos_theta * r.dir[2];

        let rotated_r = Ray {
            orig: origin,
            dir: direction,
            tm: r.tm,
        };

        if !self.ptr.hit(&rotated_r, t_min, t_max, rec) {
            return false;
        }

        let mut p = rec.p;
        let mut normal = rec.normal;

        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.set_face_normal(&rotated_r, &normal);

        true
    }
}

impl RotateY {
    pub fn new_by_angle(p: Arc<dyn Hittable>, angle: f64) -> RotateY {
        let ptr = p.clone();
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let mut bbox = Default::default();
        let hasbox = ptr.bounding_box(0.0, 1.0, &mut bbox);
        let mut min = Vec3(INFINITY, INFINITY, INFINITY);
        let mut max = Vec3(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.max().0 + (1 - i) as f64 * bbox.min().0;
                    let y = j as f64 * bbox.max().1 + (1 - j) as f64 * bbox.min().1;
                    let z = k as f64 * bbox.max().2 + (1 - k) as f64 * bbox.min().2;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3(newx, y, newz);

                    for c in 0..3 {
                        min[c] = f64::min(min[c], tester[c]);
                        max[c] = f64::max(max[c], tester[c]);
                    }
                }
            }
        }
        bbox = Aabb {
            minimum: min,
            maximum: max,
        };
        RotateY {
            ptr,
            sin_theta,
            cos_theta,
            hasbox,
            bbox,
        }
    }
}
