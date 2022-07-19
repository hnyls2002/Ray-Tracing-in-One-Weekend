use crate::{
    basic::{
        ray::Ray,
        vec3::{dot, Point3, Vec3},
    },
    bvh::aabb::{surrounding_box, Aabb},
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub struct MovingSphere<TM>
where
    TM: Material,
{
    pub center0: Point3,
    pub center1: Point3,
    pub time0: f64,
    pub time1: f64,
    pub radius: f64,
    pub mat: TM,
}

impl<TM> MovingSphere<TM>
where
    TM: Material,
{
    fn center(&self, t: f64) -> Point3 {
        self.center0
            + (self.center1 - self.center0) * ((t - self.time0) / (self.time1 - self.time0))
    }
}

impl<TM> Hittable for MovingSphere<TM>
where
    TM: Material,
{
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let oc = r.orig - self.center(r.tm);
        let a = r.direction().length().powi(2);
        let half_b = dot(&oc, &r.direction());
        let c = oc.length().powi(2) - self.radius.powi(2);

        let discriminant = half_b.powi(2) - a * c;
        if discriminant < 0.0 {
            return false;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }
        let mut rec_data = HitRecord {
            p: r.at(root),
            normal: Default::default(),
            mat_ptr: &self.mat,
            t: root,
            u: Default::default(),
            v: Default::default(),
            front_face: Default::default(),
        };
        let outward_normal = (r.at(root) - self.center(r.tm)) / self.radius;
        rec_data.set_face_normal(r, &outward_normal);
        *rec = Some(rec_data);
        true
    }
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        let cube = Vec3(self.radius, self.radius, self.radius);
        let box0 = Aabb {
            minimum: self.center(time0) - cube,
            maximum: self.center(time0) + cube,
        };
        let box1 = Aabb {
            minimum: self.center(time1) - cube,
            maximum: self.center(time1) + cube,
        };
        *output_box = surrounding_box(&box0, &box1);
        true
    }
}
