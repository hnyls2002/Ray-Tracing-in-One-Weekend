use crate::{
    camera::rtweekend::{
        ray::Ray,
        vec3::{dot, random_unit_vector, reflect, Color},
    },
    hittablelist::hittable::HitRecord,
};

pub trait Material {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    pub albedo: Color,
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
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
        };
        *attenuation = self.albedo;
        true
    }
}

pub struct Metal {
    pub albedo: Color,
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = reflect(&r_in.direction().unit_vec(), &rec.normal);
        *scattered = Ray {
            orig: rec.p,
            dir: reflected,
        };
        *attenuation = self.albedo;
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
