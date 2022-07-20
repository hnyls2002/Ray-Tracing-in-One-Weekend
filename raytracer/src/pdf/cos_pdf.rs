use std::f64::consts::PI;

use crate::basic::{
    onb::Onb,
    vec3::{dot, random_cosine_direction, Vec3},
};

use super::PDF;

pub struct CosPDF {
    uvw: Onb,
}

impl CosPDF {
    pub fn new_from_normal(w: &Vec3) -> CosPDF {
        CosPDF {
            uvw: Onb::build_from_w(w),
        }
    }
}

impl PDF for CosPDF {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine = dot(direction, &self.uvw.w());
        (cosine / PI).max(0.0)
    }
    fn generate(&self) -> Vec3 {
        self.uvw.local_by_vec3(random_cosine_direction())
    }
}
