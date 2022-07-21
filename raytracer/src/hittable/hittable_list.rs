use rand::{thread_rng, Rng};

use crate::{
    basic::{
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::{surrounding_box, Aabb},
};

use super::{HitRecord, Hittable};

#[derive(Default)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let mut tmp_rec = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let res = object.hit(r, t_min, closest_so_far, &mut tmp_rec);
            if res {
                hit_anything = true;
                if let Some(rec_data) = &tmp_rec {
                    closest_so_far = rec_data.t;
                    *rec = tmp_rec.clone();
                }
            }
        }

        hit_anything
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut tmp_box = Aabb::default();
        let mut first_box = true;
        for object in &self.objects {
            if object.bounding_box(time0, time1, &mut tmp_box) {
                return false;
            }
            *output_box = if first_box {
                tmp_box
            } else {
                surrounding_box(&output_box, &tmp_box)
            };
            first_box = false;
        }
        true
    }
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;
        for obj in &self.objects {
            sum += weight * obj.pdf_value(o, v);
        }
        sum
    }
    fn random(&self, o: &crate::basic::vec3::Vec3) -> Vec3 {
        let mut rng = thread_rng();
        self.objects[rng.gen_range(0..self.objects.len())].random(o)
    }
}
