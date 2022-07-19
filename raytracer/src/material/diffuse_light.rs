use crate::{
    basic::{
        ray::Ray,
        vec3::{Color, Point3},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::Material;

#[derive(Clone, Copy)]
pub struct DiffuseLight {
    pub emit: SolidColor,
}

impl DiffuseLight {
    pub fn new_by_color(c: Color) -> DiffuseLight {
        DiffuseLight {
            emit: SolidColor::new_from_color(c),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
    ) -> bool {
        false
    }
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.emit.value(u, v, p)
    }
}
