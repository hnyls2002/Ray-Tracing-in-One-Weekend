use image::{DynamicImage, GenericImageView};

use crate::{
    basic::{
        clamp,
        onb::Onb,
        ray::Ray,
        vec3::{Point3, Vec3},
    },
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct BumpSurface<TH: Hittable> {
    pub obj: TH,
    pub norm_img: DynamicImage,
    pub rate: u32,
}

impl<TH: Hittable> BumpSurface<TH> {
    pub fn new_from_obj_and_normal_map(obj: TH, filename: &str, rate: u32) -> Self {
        BumpSurface::<TH> {
            obj,
            norm_img: image::open(filename).unwrap(),
            rate,
        }
    }
    fn bilinear_interpolation(&self, u: f64, v: f64) -> Vec3 {
        let width = self.norm_img.dimensions().0 * self.rate;
        let height = self.norm_img.dimensions().1 * self.rate;
        let color_scale = 1.0 / 255.0;
        if u.floor() as u32 == width - 1 || v.floor() as u32 == height - 1 {
            let mut i = u as u32;
            let mut j = v as u32;
            i = i.min(width - 1);
            j = j.min(height - 1);
            let col = self
                .norm_img
                .get_pixel(
                    i % self.norm_img.dimensions().0,
                    j % self.norm_img.dimensions().1,
                )
                .0;
            return Vec3(col[0] as f64, col[1] as f64, col[2] as f64) * color_scale;
        }
        let p = [u.floor(), u.floor() + 1.0];
        let q = [v.floor(), v.floor() + 1.0];
        let mut ret_color = Vec3(0.0, 0.0, 0.0);
        for i in p.iter() {
            for j in q.iter() {
                let col = self
                    .norm_img
                    .get_pixel(
                        *i as u32 % self.norm_img.dimensions().0,
                        *j as u32 % self.norm_img.dimensions().1,
                    )
                    .0;
                ret_color += Vec3(col[0] as f64, col[1] as f64, col[2] as f64)
                    * color_scale
                    * ((u - i) * (v - j)).abs();
            }
        }
        ret_color
    }
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Vec3 {
        if self.norm_img.dimensions().0 == 0 || !self.norm_img.dimensions().1 == 0 {
            return Vec3(0.0, 1.0, 1.0);
        }

        let width = self.norm_img.dimensions().0 * self.rate;
        let height = self.norm_img.dimensions().1 * self.rate;
        u = clamp(u, 0.0, 1.0) * width as f64;
        v = (1.0 - clamp(v, 0.0, 1.0)) * height as f64;
        self.bilinear_interpolation(u, v)
    }
}

impl<TH: Hittable> Hittable for BumpSurface<TH> {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        if !self.obj.hit(r, t_min, t_max, rec) {
            return false;
        }
        let mut rec_data = rec.clone().expect("No hit record");
        let u = rec_data.u;
        let v = rec_data.v;
        let norm_p = self.value(u, v, &rec_data.p) * 2.0 - Vec3(1.0, 1.0, 1.0);
        let tagent_space = Onb::build_from_w(&rec_data.normal);
        rec_data.normal = tagent_space.local_by_vec3(norm_p);
        *rec = Some(rec_data);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        self.obj.bounding_box(time0, time1, output_box)
    }
}
