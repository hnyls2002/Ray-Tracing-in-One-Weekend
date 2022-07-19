use std::cmp::Ordering;

use rand::Rng;

use crate::{
    basic::ray::Ray,
    hittable::{hittable_list::HittableList, HitRecord, Hittable},
};

use self::aabb::{surrounding_box, Aabb};

pub mod aabb;

pub struct BvhNode {
    left: Option<Box<dyn Hittable>>,
    right: Option<Box<dyn Hittable>>,
    box_: Aabb,
}

impl Hittable for BvhNode {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        if !self.box_.hit(r, t_min, t_max) {
            return false;
        }
        let hit_left = if let Some(node_left) = &self.left {
            node_left.hit(r, t_min, t_max, rec)
        } else {
            false
        };

        let left_t = if let Some(data) = rec { data.t } else { t_max };

        let hit_right = if let Some(node_right) = &self.right {
            node_right.hit(r, t_min, if hit_left { left_t } else { t_max }, rec)
        } else {
            false
        };

        hit_left || hit_right
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.box_;
        true
    }
}

impl BvhNode {
    pub fn new_from_vec(
        mut src_objects: Vec<Box<dyn Hittable>>,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let axis: u32 = rand::thread_rng().gen_range(0..3);

        let comparator = if axis == 0 {
            BvhNode::box_x_compare
        } else if axis == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };

        let object_span = src_objects.len();

        let (left, right) = if object_span == 1 {
            (Some(src_objects.pop().unwrap()), None)
        } else if object_span == 2 {
            let b1 = src_objects.pop().unwrap();
            let b0 = src_objects.pop().unwrap();

            if comparator(&*b0, &*b1) {
                (Some(b0), Some(b1))
            } else {
                (Some(b1), Some(b0))
            }
        } else {
            src_objects.sort_by(|x, y| {
                if comparator(&**x, &**y) {
                    Ordering::Less
                } else if comparator(&**y, &**x) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            let mid = object_span / 2;

            let mut left_vec = src_objects;
            let right_vec = left_vec.split_off(mid);

            (
                Some(Box::new(BvhNode::new_from_vec(left_vec, time0, time1)) as Box<dyn Hittable>),
                Some(Box::new(BvhNode::new_from_vec(right_vec, time0, time1)) as Box<dyn Hittable>),
            )
        };

        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();
        let mut flag_left = true;
        let mut flag_right = true;

        if let Some(obj_left) = &left {
            obj_left.bounding_box(time0, time1, &mut box_left);
        } else {
            flag_left = false;
        }

        if let Some(obj_right) = &right {
            obj_right.bounding_box(time0, time1, &mut box_right);
        } else {
            flag_right = false;
        }

        BvhNode {
            left,
            right,
            box_: if !flag_left {
                surrounding_box(&box_right, &box_right)
            } else if !flag_right {
                surrounding_box(&box_left, &box_left)
            } else {
                surrounding_box(&box_left, &box_right)
            },
        }
    }
    pub fn new_from_list(list: HittableList, time0: f64, time1: f64) -> BvhNode {
        BvhNode::new_from_vec(list.objects, time0, time1)
    }
    fn box_compare(a: &dyn Hittable, b: &dyn Hittable, axis: usize) -> bool {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BvhNode constructor.");
        }
        box_a.min()[axis] < box_b.min()[axis]
    }
    fn box_x_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 0)
    }
    fn box_y_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 1)
    }
    fn box_z_compare(a: &dyn Hittable, b: &dyn Hittable) -> bool {
        BvhNode::box_compare(a, b, 2)
    }
}
