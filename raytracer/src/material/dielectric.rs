use crate::{
    basic::{
        random_double_unit,
        ray::Ray,
        vec3::{dot, reflect, refract, Vec3},
    },
    hittable::HitRecord,
};

use super::{Material, ScatterRecord};

#[derive(Clone, Copy)]
pub struct Dielectric {
    pub ir: f64, // Index of Refraction
}

impl Dielectric {
    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    #[allow(unused_variables)]
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut Option<ScatterRecord>) -> bool {
        let refraction_ratio = if rec.front_face {
            1.0 / self.ir
        } else {
            self.ir
        };
        let unit_direction = r_in.direction().unit_vec();

        let cos_theta = dot(&-unit_direction, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();

        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_ratio) > random_double_unit()
        {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, refraction_ratio)
        };

        *srec = Some(ScatterRecord {
            specular_ray: Ray {
                orig: rec.p,
                dir: direction,
                tm: r_in.tm,
            },
            is_specular: true,
            attenuation: Vec3(1.0, 1.0, 1.0),
            pdf_func: None,
        });
        true
    }
}
