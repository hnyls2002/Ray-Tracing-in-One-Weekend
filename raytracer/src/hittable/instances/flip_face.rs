use crate::hittable::Hittable;

pub struct FlipFace<TH>
where
    TH: Hittable,
{
    pub obj: TH,
}

impl<TH> Hittable for FlipFace<TH>
where
    TH: Hittable,
{
    fn hit<'a>(
        &'a self,
        r: &crate::basic::ray::Ray,
        t_min: f64,
        t_max: f64,
        rec: &mut Option<crate::hittable::HitRecord<'a>>,
    ) -> bool {
        if !self.obj.hit(r, t_min, t_max, rec) {
            false
        } else {
            let rec_data = if let Some(data) = rec {
                data
            } else {
                panic!("No hit record");
            };
            rec_data.front_face = !rec_data.front_face;
            true
        }
    }
    fn bounding_box(
        &self,
        time0: f64,
        time1: f64,
        output_box: &mut crate::bvh::aabb::Aabb,
    ) -> bool {
        self.obj.bounding_box(time0, time1, output_box)
    }
}
