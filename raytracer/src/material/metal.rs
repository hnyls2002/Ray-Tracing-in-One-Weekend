use crate::{
    basic::{
        ray::Ray,
        vec3::{random_in_unit_sphere, reflect, Color},
    },
    hittable::HitRecord,
};

use super::{Material, ScatterRecord};

#[derive(Clone, Copy)]
pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Material for Metal {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        let reflected = reflect(&r_in.direction().unit_vec(), &rec.normal);
        *srec = Some(ScatterRecord {
            specular_ray: Ray {
                orig: rec.p,
                dir: reflected + random_in_unit_sphere() * self.fuzz.min(1.0),
                tm: r_in.tm,
            },
            is_specular: true,
            attenuation: self.albedo,
            pdf_func: None,
        });
        true
    }
}
