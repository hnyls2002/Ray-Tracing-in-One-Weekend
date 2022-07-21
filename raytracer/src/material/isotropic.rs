use crate::{
    basic::{
        ray::Ray,
        vec3::{random_in_unit_sphere, Color},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::{Material, ScatterRecord};

pub struct Isotropic {
    pub albedo: SolidColor,
}

impl Isotropic {
    #[allow(dead_code)]
    pub fn new_by_texture(tex: SolidColor) -> Isotropic {
        Isotropic { albedo: tex }
    }
    pub fn new_by_color(c: Color) -> Isotropic {
        Isotropic {
            albedo: SolidColor::new_from_color(c),
        }
    }
}

impl Material for Isotropic {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        *srec = Some(ScatterRecord {
            specular_ray: Ray {
                orig: rec.p,
                dir: random_in_unit_sphere(),
                tm: r_in.tm,
            },

            is_specular: false,
            attenuation: self.albedo.value(rec.u, rec.v, &rec.p),
            pdf_func: None,
        });
        true
    }
}
