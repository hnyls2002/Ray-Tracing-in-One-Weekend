use crate::{
    basic::vec3::{Point3, Vec3},
    hittable::Hittable,
};

use super::PDF;

pub struct HittablePDF<'a> {
    pub o: Point3,
    pub ptr: &'a dyn Hittable,
}

impl<'a> PDF for HittablePDF<'a> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.ptr.pdf_value(&self.o, direction)
    }
    fn generate(&self) -> Vec3 {
        self.ptr.random(&self.o)
    }
}
