use crate::basic::{random_double_unit, vec3::Vec3};

use super::PDF;

pub struct MixturePDF<'a> {
    pub p: [Box<dyn PDF + 'a>; 2],
}

impl<'a> MixturePDF<'a> {
    pub fn new(p0: Box<dyn PDF + 'a>, p1: Box<dyn PDF + 'a>) -> MixturePDF<'a> {
        MixturePDF { p: [p0, p1] }
    }
}

impl<'a> PDF for MixturePDF<'a> {
    // p0 hittable-pdf p1 self-scatter-pdf
    fn value(&self, direction: &Vec3) -> f64 {
        if self.p[0].value(direction) < 0.0 {
            self.p[1].value(direction)
        } else {
            0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
        }
    }
    fn generate(&self) -> Vec3 {
        if (self.p[0].generate() - Vec3(0.0, 0.0, 0.0)).length() < 1e-10 {
            self.p[1].generate()
        } else {
            if random_double_unit() < 0.5 {
                self.p[0].generate()
            } else {
                self.p[1].generate()
            }
        }
    }
}
