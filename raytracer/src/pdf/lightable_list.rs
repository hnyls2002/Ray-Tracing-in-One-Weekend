use rand::{thread_rng, Rng};

use crate::basic::vec3::{Point3, Vec3};

pub trait Lightable: Sync + Send {
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64;
    fn random(&self, o: &Vec3) -> Vec3;
}

#[derive(Default)]
pub struct LightableList {
    pub lights: Vec<Box<dyn Lightable>>,
}

impl LightableList {
    #[allow(dead_code)]
    pub fn add(&mut self, object: Box<dyn Lightable>) {
        self.lights.push(object);
    }
}

impl Lightable for LightableList {
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        if self.lights.is_empty() {
            return -1.0;
        }
        let weight = 1.0 / self.lights.len() as f64;
        let mut sum = 0.0;
        for obj in self.lights.iter() {
            sum += weight * obj.pdf_value(o, v);
        }
        sum
    }
    fn random(&self, o: &crate::basic::vec3::Vec3) -> Vec3 {
        if self.lights.is_empty() {
            return Vec3(0.0, 0.0, 0.0);
        }
        let mut rng = thread_rng();
        self.lights[rng.gen_range(0..self.lights.len())].random(o)
    }
}
