use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, Point3, Vec3},
    },
    bvh::aabb::Aabb,
    material::Material,
};

pub mod hittable_list;
pub mod instances;
pub mod objects;

#[derive(Clone)]
pub struct HitRecord<'a> {
    pub p: Point3,
    pub normal: Vec3,
    pub mat_ptr: &'a dyn Material,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

impl<'a> HitRecord<'a> {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            -*outward_normal
        }
    }
}

pub trait Hittable: Send + Sync {
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool;
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool;
    #[allow(unused_variables)]
    fn pdf_value(&self, o: &Point3, v: &Vec3) -> f64 {
        0.0
    }
    #[allow(unused_variables)]
    fn random(&self, o: &Vec3) -> Vec3 {
        Vec3(1.0, 0.0, 0.0)
    }
}
