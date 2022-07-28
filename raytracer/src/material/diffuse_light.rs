use crate::{
    basic::{
        ray::Ray,
        vec3::{Color, Point3, Vec3},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::Material;

#[derive(Clone)]
pub struct DiffuseLight<TT: Texture> {
    pub emit: TT,
}

impl DiffuseLight<SolidColor> {
    pub fn new_by_color(c: Color) -> DiffuseLight<SolidColor> {
        DiffuseLight {
            emit: SolidColor::new_from_color(c),
        }
    }
}

impl<TT: Texture> DiffuseLight<TT> {
    pub fn new_by_texture(tex: TT) -> DiffuseLight<TT> {
        DiffuseLight { emit: tex }
    }
}

impl<TT: Texture> Material for DiffuseLight<TT> {
    #[allow(unused_variables)]
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if rec.front_face {
            self.emit.value(u, v, p)
        } else {
            Vec3(0.0, 0.0, 0.0)
        }
    }
}
