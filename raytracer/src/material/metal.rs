use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, random_in_unit_sphere, reflect, Color},
    },
    hittable::HitRecord,
};

use super::Material;

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
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
            dir: reflected
                + random_in_unit_sphere() * {
                    if self.fuzz < 1.0 {
                        self.fuzz
                    } else {
                        1.0
                    }
                },
            tm: r_in.tm,
        };
        *attenuation = self.albedo;
        dot(&scattered.direction(), &rec.normal) > 0.0
    }
}
