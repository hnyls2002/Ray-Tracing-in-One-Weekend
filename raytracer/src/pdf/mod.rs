pub mod cos_pdf;
pub mod hittable_pdf;
pub mod lightable_list;
pub mod mixture_pdf;

use crate::basic::vec3::Vec3;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
