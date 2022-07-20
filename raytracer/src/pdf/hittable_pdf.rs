use crate::{
    basic::vec3::{Point3, Vec3},
    hittable::Hittable,
};

use super::PDF;

pub struct HittablePDF {
    pub o: Point3,
    pub ptr: Box<dyn Hittable>,
}

impl PDF for HittablePDF {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}
