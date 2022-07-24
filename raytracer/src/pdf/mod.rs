pub mod cos_pdf;
pub mod hittable_pdf;
pub mod mixture_pdf;
pub mod lightable_list;

use crate::basic::vec3::Vec3;

pub trait PDF {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}
