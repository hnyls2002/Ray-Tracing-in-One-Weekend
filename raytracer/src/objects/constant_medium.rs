use std::{
    f64::{consts::E, INFINITY},
    sync::Arc,
};

use crate::{
    bvh::aabb::Aabb,
    hittablelist::hittable::{HitRecord, Hittable},
    material::{Isotropic, Material},
    rtweekend::{
        random_double_unit,
        ray::Ray,
        vec3::{Color, Vec3},
    },
    texture::Texture,
};

pub struct ConstantMedium {
    pub boundary: Arc<dyn Hittable>,
    pub phase_function: Arc<dyn Material>,
    pub neg_inv_density: f64,
}

impl ConstantMedium {
    #[allow(dead_code)]
    pub fn new_by_texture(b: Arc<dyn Hittable>, d: f64, a: Arc<dyn Texture>) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::new_by_texture(a)),
            neg_inv_density: -1.0 / d,
        }
    }
    pub fn new_by_color(b: Arc<dyn Hittable>, d: f64, c: Color) -> ConstantMedium {
        ConstantMedium {
            boundary: b,
            phase_function: Arc::new(Isotropic::new_by_color(c)),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl Hittable for ConstantMedium {
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random_double_unit() < 0.00001;
        let mut rec1: HitRecord = Default::default();
        let mut rec2: HitRecord = Default::default();

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        if !self.boundary.hit(r, rec1.t + 0.0001, INFINITY, &mut rec2) {
            return false;
        }

        if debugging {
            println!("\n t_min = {} , t_max = {}\n", rec1.t, rec2.t);
        }

        rec1.t = rec1.t.max(t_min);
        rec2.t = rec2.t.min(t_max);

        if rec1.t >= rec2.t {
            return false;
        }

        rec1.t = rec1.t.max(0.0);

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2.t - rec1.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double_unit().log(E);

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1.t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        if debugging {
            println!("hit_distance = {}", hit_distance);
            println!("rec.t = {}", rec.t);
            println!("rec.p = {:?}", rec.p);
        }

        rec.normal = Vec3(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat_ptr = Some(self.phase_function.clone());

        true
    }
}
