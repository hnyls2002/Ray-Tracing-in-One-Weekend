pub mod dielectric;
pub mod diffuse_light;
pub mod isotropic;
pub mod lambertian;
pub mod metal;

use crate::{
    basic::{
        ray::Ray,
        vec3::{Color, Point3, Vec3},
    },
    hittable::HitRecord,
    pdf::PDF,
};

pub struct ScatterRecord {
    // the pdf bound to a kind of material, can be cloned
    pub specular_ray: Ray,
    pub is_specular: bool,
    pub attenuation: Color,
    pub pdf_func: Option<Box<dyn PDF>>,
}

pub trait Material: Send + Sync {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        0.0
    }
    #[allow(unused_variables)]
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Vec3(0.0, 0.0, 0.0)
    }
}
