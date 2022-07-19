use crate::{
    basic::{
        ray::Ray,
        vec3::{random_in_unit_sphere, Color},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::Material;

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
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *scattered = Ray {
            orig: rec.p,
            dir: random_in_unit_sphere(),
            tm: r_in.tm,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
