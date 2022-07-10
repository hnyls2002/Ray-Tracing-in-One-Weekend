use self::rtweekend::{
    degrees_to_radians,
    ray::Ray,
    vec3::{cross, Point3, Vec3},
};

pub mod rtweekend;

#[derive(Clone, Copy)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(
        lookfrom: &Point3,
        lookat: &Point3,
        vup: &Vec3,
        vfov: f64, // vertical field-of-view in degrees
        aspect_ratio: f64,
    ) -> Self {
        let theta = degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (*lookfrom - *lookat).unit_vec();
        let u = cross(vup, &w).unit_vec();
        let v = cross(&w, &u);

        let origin = *lookfrom;
        let horizontal = u * viewport_width;
        let vertical = v * viewport_height;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }
}

impl Camera {
    pub fn get_ray(self, u: f64, v: f64) -> Ray {
        Ray {
            orig: self.origin,
            dir: self.lower_left_corner + self.horizontal * u + self.vertical * v - self.origin,
        }
    }
}
