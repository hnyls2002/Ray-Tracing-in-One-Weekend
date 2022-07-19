use crate::{
    basic::{ray::Ray, vec3::Point3},
    bvh::aabb::Aabb,
    hittable::{hittable_list::HittableList, HitRecord, Hittable},
    material::Material,
};

use super::aarect::{XYRect, XZRect, YZRect};

pub struct MyBox {
    pub box_min: Point3,
    pub box_max: Point3,
    pub sides: HittableList,
}

impl MyBox {
    pub fn new<TM>(p0: Point3, p1: Point3, mat: TM) -> MyBox
    where
        TM: Material + Clone + Copy + 'static,
    {
        let mut list = HittableList { objects: vec![] };
        list.add(Box::new(XYRect {
            x0: p0.0,
            x1: p1.0,
            y0: p0.1,
            y1: p1.1,
            k: p1.2,
            mat,
        }));
        list.add(Box::new(XYRect {
            x0: p0.0,
            x1: p1.0,
            y0: p0.1,
            y1: p1.1,
            k: p0.2,
            mat,
        }));
        list.add(Box::new(XZRect {
            x0: p0.0,
            x1: p1.0,
            z0: p0.2,
            z1: p1.2,
            k: p1.1,
            mat,
        }));
        list.add(Box::new(XZRect {
            x0: p0.0,
            x1: p1.0,
            z0: p0.2,
            z1: p1.2,
            k: p0.1,
            mat,
        }));
        list.add(Box::new(YZRect {
            y0: p0.1,
            y1: p1.1,
            z0: p0.2,
            z1: p1.2,
            k: p1.0,
            mat,
        }));
        list.add(Box::new(YZRect {
            y0: p0.1,
            y1: p1.1,
            z0: p0.2,
            z1: p1.2,
            k: p0.0,
            mat,
        }));
        MyBox {
            box_min: p0,
            box_max: p1,
            sides: list,
        }
    }
}

impl Hittable for MyBox {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        self.sides.hit(r, t_min, t_max, rec)
    }

    fn bounding_box(
        &self,
        _time0: f64,
        _time1: f64,
        output_box: &mut crate::bvh::aabb::Aabb,
    ) -> bool {
        *output_box = Aabb {
            minimum: self.box_min,
            maximum: self.box_max,
        };
        true
    }
}
