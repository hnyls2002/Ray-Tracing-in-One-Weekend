use std::{cmp::Ordering, sync::Arc};

use rand::Rng;

use crate::{
    hittablelist::hittable::{HitRecord, Hittable},
    rtweekend::ray::Ray,
};

use self::aabb::{surrounding_box, Aabb};

pub mod aabb;

#[derive(Clone)]
pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    box_: Aabb,
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        if !self.box_.hit(r, t_min, t_max) {
            return false;
        }
        // for option<Arc<dyn Hittable>>
        /*
        let hit_left = if let Some(node_left) = self.left.clone() {
            node_left.hit(r, t_min, t_max, rec)
        } else {
            false
        };

        let hit_right = if let Some(node_right) = self.right.clone() {
            node_right.hit(r, t_min, if hit_left { rec.t } else { t_max }, rec)
        } else {
            false
        };
        */

        let hit_left = self.left.hit(r, t_min, t_max, rec);
        let hit_right = self
            .right
            .hit(r, t_min, if hit_left { rec.t } else { t_max }, rec);

        hit_left || hit_right
    }
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = self.box_;
        true
    }
}

impl BvhNode {
    pub fn new(
        src_objects: Vec<Arc<dyn Hittable>>,
        start: usize,
        end: usize,
        time0: f64,
        time1: f64,
    ) -> BvhNode {
        let mut objects = src_objects.clone();
        let axis: u32 = rand::thread_rng().gen_range(0..3);

        let comparator = if axis == 0 {
            BvhNode::box_x_compare
        } else if axis == 1 {
            BvhNode::box_y_compare
        } else {
            BvhNode::box_z_compare
        };

        let object_span = end - start;

        let (left, right) = if object_span == 1 {
            (objects[start].clone(), objects[start].clone())
        } else if object_span == 2 {
            if comparator(objects[start].clone(), objects[start + 1].clone()) {
                (objects[start].clone(), objects[start + 1].clone())
            } else {
                (objects[start + 1].clone(), objects[start].clone())
            }
        } else {
            objects[start..end].sort_by(|x, y| {
                if comparator(x.clone(), y.clone()) {
                    Ordering::Less
                } else if comparator(y.clone(), x.clone()) {
                    Ordering::Greater
                } else {
                    Ordering::Equal
                }
            });

            let mid = start + object_span / 2;

            (
                Arc::new(BvhNode::new(objects.clone(), start, mid, time0, time1))
                    as Arc<dyn Hittable>,
                Arc::new(BvhNode::new(objects, mid, end, time0, time1)) as Arc<dyn Hittable>,
            )
        };

        let mut box_left = Aabb::default();
        let mut box_right = Aabb::default();

        if !left.bounding_box(time0, time1, &mut box_left)
            || !right.bounding_box(time0, time1, &mut box_right)
        {
            panic!("No bounding box in BvhNode constructor!");
        }

        BvhNode {
            left,
            right,
            box_: surrounding_box(&box_left, &box_right),
        }
    }
    fn box_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>, axis: usize) -> bool {
        let mut box_a = Aabb::default();
        let mut box_b = Aabb::default();
        if !a.bounding_box(0.0, 0.0, &mut box_a) || !b.bounding_box(0.0, 0.0, &mut box_b) {
            panic!("No bounding box in BvhNode constructor.");
        }
        box_a.min()[axis] < box_b.min()[axis]
    }
    fn box_x_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 0)
    }
    fn box_y_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 1)
    }
    fn box_z_compare(a: Arc<dyn Hittable>, b: Arc<dyn Hittable>) -> bool {
        BvhNode::box_compare(a, b, 2)
    }
}
