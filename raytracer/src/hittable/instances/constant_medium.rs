use std::f64::{consts::E, INFINITY};

use crate::{
    basic::{
        random_double_unit,
        ray::Ray,
        vec3::{Color, Vec3},
    },
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::isotropic::Isotropic,
    texture::{solid_color_texture::SolidColor, Texture},
};

pub struct ConstantMedium<TH> {
    pub boundary: TH,
    pub phase_function: Isotropic,
    pub neg_inv_density: f64,
}

impl<TH: Hittable> ConstantMedium<TH> {
    #[allow(dead_code)]
    pub fn new_by_texture<TT: Texture>(b: TH, d: f64, a: SolidColor) -> ConstantMedium<TH> {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic::new_by_texture(a),
            neg_inv_density: -1.0 / d,
        }
    }
    pub fn new_by_color(b: TH, d: f64, c: Color) -> ConstantMedium<TH> {
        ConstantMedium {
            boundary: b,
            phase_function: Isotropic::new_by_color(c),
            neg_inv_density: -1.0 / d,
        }
    }
}

impl<TH> Hittable for ConstantMedium<TH>
where
    TH: Hittable,
{
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.boundary.bounding_box(time0, time1, output_box)
    }
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let enable_debug = false;
        let debugging = enable_debug && random_double_unit() < 0.00001;
        let mut rec1 = None;
        let mut rec2 = None;

        if !self.boundary.hit(r, -INFINITY, INFINITY, &mut rec1) {
            return false;
        }

        let mut rec1_data = if let Some(data) = rec1 {
            data
        } else {
            panic!("No hit record");
        };

        if !self
            .boundary
            .hit(r, rec1_data.t + 0.0001, INFINITY, &mut rec2)
        {
            return false;
        }

        let mut rec2_data = if let Some(data) = rec2 {
            data
        } else {
            panic!("no hit record");
        };

        if debugging {
            println!("\n t_min = {} , t_max = {}\n", rec1_data.t, rec2_data.t);
        }

        rec1_data.t = rec1_data.t.max(t_min);
        rec2_data.t = rec2_data.t.min(t_max);

        if rec1_data.t >= rec2_data.t {
            return false;
        }

        rec1_data.t = rec1_data.t.max(0.0);

        let ray_length = r.dir.length();
        let distance_inside_boundary = (rec2_data.t - rec1_data.t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double_unit().log(E);

        if hit_distance > distance_inside_boundary {
            return false;
        }

        let tmp_t = rec1_data.t + hit_distance / ray_length;

        *rec = Some(HitRecord {
            p: r.at(tmp_t),
            normal: Vec3(1.0, 0.0, 0.0),
            mat_ptr: &self.phase_function,
            t: tmp_t,
            u: Default::default(),
            v: Default::default(),
            front_face: true,
        });

        if debugging {
            if let Some(rec_data) = rec {
                println!("hit_distance = {}", hit_distance);
                println!("rec.t = {}", rec_data.t);
                println!("rec.p = {:?}", rec_data.p);
            }
        }

        true
    }
}
