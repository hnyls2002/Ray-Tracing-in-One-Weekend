use std::f64::consts::PI;

use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, Color},
    },
    hittable::HitRecord,
    pdf::cos_pdf::CosPDF,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::{Material, ScatterRecord};

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
    fn scatter<'a>(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        srec: &'a mut Option<ScatterRecord>,
    ) -> bool {
        *srec = Some(ScatterRecord {
            specular_ray: Default::default(),
            is_specular: false,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_func: Some(Box::new(CosPDF::new_from_normal(&rec.normal))),
        });
        true
    }
    #[allow(unused_variables)]
    fn scattering_pdf(&self, r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cosine = dot(&rec.normal, &scattered.dir.unit_vec());
        if cosine < 0.0 {
            0.0
        } else {
            cosine / PI
        }
    }
}
