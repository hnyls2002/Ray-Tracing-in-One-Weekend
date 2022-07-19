use crate::{
    basic::{ray::Ray, vec3::Vec3},
    bvh::aabb::Aabb,
    hittable::{HitRecord, Hittable},
    material::Material,
};

pub struct XYRect<TM>
where
    TM: Material,
{
    pub x0: f64,
    pub x1: f64,
    pub y0: f64,
    pub y1: f64,
    pub k: f64,
    pub mat: TM,
}

impl<TM> Hittable for XYRect<TM>
where
    TM: Material,
{
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: Vec3(self.x0, self.y0, self.k - 0.0001),
            maximum: Vec3(self.x1, self.y1, self.k + 0.0001),
        };
        true
    }
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let t = (self.k - r.orig.2) / r.dir.2;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.0 + t * r.dir.0;
        let y = r.orig.1 + t * r.dir.1;
        if x < self.x0 || x > self.x1 || y < self.y0 || y > self.y1 {
            return false;
        }
        let outward_normal = Vec3(0.0, 0.0, 1.0);
        let mut rec_data = HitRecord {
            p: r.at(t),
            normal: outward_normal,
            mat_ptr: &self.mat,
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (y - self.y0) / (self.y1 - self.y0),
            front_face: Default::default(),
        };
        rec_data.set_face_normal(r, &outward_normal);

        *rec = Some(rec_data);
        true
    }
}

pub struct XZRect<TM>
where
    TM: Material,
{
    pub x0: f64,
    pub x1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: TM,
}

impl<TM> Hittable for XZRect<TM>
where
    TM: Material,
{
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: Vec3(self.x0, self.k - 0.0001, self.z0),
            maximum: Vec3(self.x1, self.k + 0.0001, self.z1),
        };
        true
    }
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let t = (self.k - r.orig.1) / r.dir.1;
        if t < t_min || t > t_max {
            return false;
        }
        let x = r.orig.0 + t * r.dir.0;
        let z = r.orig.2 + t * r.dir.2;
        if x < self.x0 || x > self.x1 || z < self.z0 || z > self.z1 {
            return false;
        }
        let outward_normal = Vec3(0.0, 1.0, 0.0);
        let mut rec_data = HitRecord {
            p: r.at(t),
            normal: outward_normal,
            mat_ptr: &self.mat,
            t,
            u: (x - self.x0) / (self.x1 - self.x0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: true,
        };
        rec_data.set_face_normal(r, &outward_normal);

        *rec = Some(rec_data);

        true
    }
}

pub struct YZRect<TM>
where
    TM: Material,
{
    pub y0: f64,
    pub y1: f64,
    pub z0: f64,
    pub z1: f64,
    pub k: f64,
    pub mat: TM,
}

impl<TM> Hittable for YZRect<TM>
where
    TM: Material,
{
    fn bounding_box(&self, _time0: f64, _time1: f64, output_box: &mut Aabb) -> bool {
        *output_box = Aabb {
            minimum: Vec3(self.k - 0.0001, self.y0, self.z0),
            maximum: Vec3(self.k + 0.0001, self.y1, self.z1),
        };
        true
    }
    fn hit<'a>(&'a self, r: &Ray, t_min: f64, t_max: f64, rec: &mut Option<HitRecord<'a>>) -> bool {
        let t = (self.k - r.orig.0) / r.dir.0;
        if t < t_min || t > t_max {
            return false;
        }
        let y = r.orig.1 + t * r.dir.1;
        let z = r.orig.2 + t * r.dir.2;
        if y < self.y0 || y > self.y1 || z < self.z0 || z > self.z1 {
            return false;
        }
        let outward_normal = Vec3(1.0, 0.0, 0.0);
        let mut rec_data = HitRecord {
            p: r.at(t),
            normal: outward_normal,
            mat_ptr: &self.mat,
            t,
            u: (y - self.y0) / (self.y1 - self.y0),
            v: (z - self.z0) / (self.z1 - self.z0),
            front_face: true,
        };
        rec_data.set_face_normal(r, &outward_normal);

        *rec = Some(rec_data);

        true
    }
}
