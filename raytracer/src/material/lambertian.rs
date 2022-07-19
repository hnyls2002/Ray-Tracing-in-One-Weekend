use crate::{
    basic::{
        ray::Ray,
        vec3::{random_unit_vector, Color},
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
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray {
            orig: rec.p,
            dir: scatter_direction,
            tm: r_in.tm,
        };
        *attenuation = self.albedo.value(rec.u, rec.v, &rec.p);
        true
    }
}
