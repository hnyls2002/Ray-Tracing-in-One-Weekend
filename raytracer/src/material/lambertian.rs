use std::f64::consts::PI;

use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, random_in_hemisphere, Color},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::Material;

#[derive(Clone, Copy)]
pub struct Lambertian<TT>
where
    TT: Texture,
{
    pub albedo: TT,
}

impl<TT> Lambertian<TT>
where
    TT: Texture,
{
    pub fn new_by_texture(tex: TT) -> Lambertian<TT> {
        Lambertian { albedo: tex }
    }
    pub fn new_by_solid_color(c: Color) -> Lambertian<SolidColor> {
        Lambertian {
            albedo: SolidColor::new_from_color(c),
        }
    }
}

impl<TT> Material for Lambertian<TT>
where
    TT: Texture,
{
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        alb: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let direction = random_in_hemisphere(&rec.normal);
        *scattered = Ray {
            orig: rec.p,
            dir: direction.unit_vec(),
            tm: r_in.tm,
        };
        *alb = self.albedo.value(rec.u, rec.v, &rec.p);
        *pdf = 0.5 / PI;
        true
    }
    #[allow(unused_variables)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &mut Ray) -> f64 {
        let cosine = dot(&rec.normal, &scattered.dir.unit_vec());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
