use crate::camera::rtweekend::ray::Ray;

use self::hittable::{HitRecord, Hittable};

pub mod hittable;

pub struct HittableList<'a> {
    pub objects: Vec<Box<dyn Hittable<'a> + 'a>>,
}

#[allow(dead_code)]
impl<'a> HittableList<'a> {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Box<dyn Hittable<'a> + 'a>) {
        self.objects.push(object);
    }
}

impl<'a> Hittable<'a> for HittableList<'a> {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let mut tmp_rec = None;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            let res = object.hit(r, t_min, closest_so_far, &mut tmp_rec);
            if res {
                if let Some(rec_data) = &tmp_rec {
                    hit_anything = true;
                    closest_so_far = rec_data.t;
                    *rec = tmp_rec.clone();
                }
            }
        }

        hit_anything
    }
}
