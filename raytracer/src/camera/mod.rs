use crate::rtweekend::{
    degrees_to_radians, random_double,
    ray::Ray,
    vec3::{cross, random_in_unit_disk, Point3, Vec3},
};

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    lens_radius: f64,
    time0: f64,
    time1: f64,
}

impl Camera {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        lookfrom: Point3,
        lookat: Point3,
        vup: Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
        time0: f64,
        time1: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit_vec();
        let u = cross(&vup, &w).unit_vec();
        let v = cross(&w, &u);

        let origin = lookfrom;
        let horizontal = u * viewport_width * focus_dist;
        let vertical = v * viewport_height * focus_dist;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w * focus_dist;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
            w,
            u,
            v,
            lens_radius: aperture / 2.0,
            time0,
            time1,
        }
    }
}

impl Camera {
    pub fn get_ray(self, s: f64, t: f64) -> Ray {
        let rd = random_in_unit_disk() * self.lens_radius;
        let offset = self.u * rd.0 + self.v * rd.1;
        Ray {
            orig: self.origin + offset,
            dir: self.lower_left_corner + self.horizontal * s + self.vertical * t
                - self.origin
                - offset,
            tm: random_double(self.time0, self.time1), // shutter open/close times
        }
    }
}
