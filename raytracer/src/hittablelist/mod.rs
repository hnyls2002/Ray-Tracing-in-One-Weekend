use std::sync::Arc;

use crate::{
    material::{Dielectric, Lambertian, Material, Metal},
    rtweekend::{
        random_double, random_double_unit,
        ray::Ray,
        vec3::{Color, Point3},
    },
    sphere::Sphere,
};

use self::hittable::{HitRecord, Hittable};

pub mod hittable;

#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable>>,
}

#[allow(dead_code)]
impl HittableList {
    pub fn clear(&mut self) {
        self.objects.clear();
    }
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r: &Ray, t_min: f64, t_max: f64, rec: &mut HitRecord) -> bool {
        let mut tmp_rec = Default::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, &mut tmp_rec) {
                hit_anything = true;
                closest_so_far = tmp_rec.t;
                *rec = tmp_rec.clone();
            }
        }

        hit_anything
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let ground_material = Arc::new(Lambertian {
        albedo: Color::new(0.5, 0.5, 0.5),
    });
    world.add(Arc::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Some(ground_material),
    }));
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double_unit();
            let center = Point3::new(
                a as f64 + 0.9 * random_double_unit(),
                0.2,
                b as f64 + 0.9 * random_double_unit(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material> = if choose_mat < 0.8 {
                    Arc::new(Lambertian {
                        albedo: Color::random_unit() * Color::random_unit(),
                    })
                } else if choose_mat < 0.95 {
                    Arc::new(Metal {
                        albedo: Color::random(0.5, 1.0),
                        fuzz: random_double(0.0, 0.5),
                    })
                } else {
                    Arc::new(Dielectric { ir: 1.5 })
                };
                world.add(Arc::new(Sphere {
                    center,
                    radius: 0.2,
                    mat_ptr: Some(sphere_material),
                }));
            }
        }
    }
    let material1 = Arc::new(Dielectric { ir: 1.5 });
    let material2 = Arc::new(Lambertian {
        albedo: Color::new(0.4, 0.2, 0.1),
    });
    let material3 = Arc::new(Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    });

    world.add(Arc::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: Some(material1),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: Some(material2),
    }));

    world.add(Arc::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat_ptr: Some(material3),
    }));
    world
}
