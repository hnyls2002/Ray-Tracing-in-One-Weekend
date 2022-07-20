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
};

pub trait Material: Send + Sync {
    #[allow(unused_variables)]
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        false
    }
    #[allow(unused_variables)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        0.0
    }
    #[allow(unused_variables)]
    fn emitted(&self, r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        Vec3(0.0, 0.0, 0.0)
    }
}
