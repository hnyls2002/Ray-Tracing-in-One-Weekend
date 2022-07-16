use std::sync::Arc;

use crate::{
    basic::{
        ray::Ray,
        vec3::{random_unit_vector, Color},
    },
    hittable::HitRecord,
    texture::{solid_color_texture::SolidColor, Texture},
};

use super::Material;

pub struct Lambertian {
    pub albedo: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn new_by_texture(tex: Arc<dyn Texture>) -> Lambertian {
        Lambertian { albedo: tex }
    }
    pub fn new_by_solid_color(c: &Color) -> Lambertian {
        Lambertian {
            albedo: Arc::new(SolidColor::new(*c)),
        }
    }
}

impl Material for Lambertian {
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
