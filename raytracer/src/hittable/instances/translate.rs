use crate::{
    basic::{ray::Ray, vec3::Vec3},
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
};

pub struct Translate<TH>
where
    TH: Hittable,
{
    pub obj: TH,
    pub offset: Vec3,
}

impl<TH> Hittable for Translate<TH>
where
    TH: Hittable,
{
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let moved_r = Ray {
            orig: r.orig - self.offset,
            dir: r.dir,
            tm: r.tm,
        };
        if !self.obj.hit(&moved_r, t_min, t_max, rec) {
            return false;
        }
        let rec_data = if let Some(data) = rec {
            data
        } else {
            panic!("No hit record");
        };
        rec_data.p += self.offset;
        rec_data.set_face_normal(&moved_r, &rec_data.normal.clone());
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if !self.obj.bounding_box(time0, time1, output_box) {
            return false;
        }
        *output_box = Aabb {
            minimum: output_box.min() + self.offset,
            maximum: output_box.max() + self.offset,
        };
        true
    }
}
