use std::mem::swap;

use crate::rtweekend::{
    ray::Ray,
    vec3::{Point3, Vec3},
};

#[derive(Default, Clone, Copy, Debug)]
pub struct Aabb {
    pub minimum: Point3,
    pub maximum: Point3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, mut t_min: f64, mut t_max: f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.dir[a];
            let mut t0 = (self.min()[a] - r.orig[a]) * inv_d;
            let mut t1 = (self.max()[a] - r.orig[a]) * inv_d;
            if inv_d < 0.0 {
                swap(&mut t0, &mut t1);
            }
            t_min = t_min.max(t0);
            t_max = t_max.min(t1);
            if t_max <= t_min {
                return false;
            }
        }
        true
    }
    pub fn min(&self) -> Point3 {
        self.minimum
    }
    pub fn max(&self) -> Point3 {
        self.maximum
    }
}

pub fn surrounding_box(box0: &Aabb, box1: &Aabb) -> Aabb {
    let small = Vec3(
        f64::min(box0.min().0, box1.min().0),
        f64::min(box0.min().1, box1.min().1),
        f64::min(box0.min().2, box1.min().2),
    );
    let big = Vec3(
        f64::max(box0.max().0, box1.max().0),
        f64::max(box0.max().1, box1.max().1),
        f64::max(box0.max().2, box1.max().2),
    );
    Aabb {
        minimum: small,
        maximum: big,
    }
}
