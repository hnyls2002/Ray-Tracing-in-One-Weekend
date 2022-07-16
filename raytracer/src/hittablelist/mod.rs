use std::sync::Arc;

use crate::{
    bvh::{
        aabb::{surrounding_box, Aabb},
        BvhNode,
    },
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    objects::{
        aarect::{XYRect, XZRect, YZRect},
        constant_medium::ConstantMedium,
        moving_sphere::MovingSphere,
        my_box::MyBox,
        sphere::Sphere,
    },
    rtweekend::{
        random_double, random_double_unit,
        ray::Ray,
        vec3::{Color, Point3, Vec3},
    },
    texture::{CheckerTexture, ImageTexture, NoiseTexture, SolidColor, Texture},
};

use self::hittable::{HitRecord, Hittable, RotateY, Translate};

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
    fn bounding_box(&self, time0: f64, time1: f64, output_box: &mut Aabb) -> bool {
        if self.objects.is_empty() {
            return false;
        }
        let mut tmp_box = Aabb::default();
        let mut first_box = true;
        for object in &self.objects {
            if object.bounding_box(time0, time1, &mut tmp_box) {
                return false;
            }
            *output_box = if first_box {
                tmp_box
            } else {
                surrounding_box(&output_box, &tmp_box)
            };
            first_box = false;
        }
        true
    }
}

pub fn random_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::new_by_color(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::new_by_texture(checker));
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
                    Arc::new(Lambertian::new_by_texture(Arc::new(SolidColor::new(
                        Color::random_unit() * Color::random_unit(),
                    ))))
                } else if choose_mat < 0.95 {
                    Arc::new(Metal {
                        albedo: Color::random(0.5, 1.0),
                        fuzz: random_double(0.0, 0.5),
                    })
                } else {
                    Arc::new(Dielectric { ir: 1.5 })
                };
                world.add(if choose_mat < 0.8 {
                    let center2 = center + Vec3(0.0, random_double(0.0, 0.5), 0.0);
                    Arc::new(MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material),
                    })
                } else {
                    Arc::new(Sphere {
                        center,
                        radius: 0.2,
                        mat_ptr: Some(sphere_material),
                    })
                });
            }
        }
    }
    let material1 = Arc::new(Dielectric { ir: 1.5 });
    let material2 = Arc::new(Lambertian::new_by_texture(Arc::new(SolidColor::new(
        Color::new(0.4, 0.2, 0.1),
    ))));
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

pub fn two_spheres() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    let checker: Arc<dyn Texture> = Arc::new(CheckerTexture::new_by_color(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0.9, 0.9, 0.9),
    ));
    list.add(Arc::new(Sphere {
        center: Point3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat_ptr: Some(Arc::new(Lambertian::new_by_texture(checker.clone()))),
    }));
    list.add(Arc::new(Sphere {
        center: Point3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat_ptr: Some(Arc::new(Lambertian::new_by_texture(checker))),
    }));
    list
}

pub fn two_perlin_spheres() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    // Generate texture
    let pertext = Arc::new(NoiseTexture::new_by_sc(4.0));
    // Generate material
    let permat = Arc::new(Lambertian { albedo: pertext });

    list.add(Arc::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Some(permat.clone()),
    }));

    list.add(Arc::new(Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: Some(permat),
    }));

    list
}

pub fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::load_image_file(
        "./raytracer/sources/earthmap.jpg",
    ));
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 2.0,
        mat_ptr: Some(earth_surface),
    });
    let mut list = HittableList { objects: vec![] };
    list.add(globe);
    list
}

pub fn simple_light() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    let pertext = Arc::new(NoiseTexture::new_by_sc(4.0));
    let permat = Arc::new(Lambertian { albedo: pertext });

    list.add(Arc::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat_ptr: Some(permat.clone()),
    }));

    list.add(Arc::new(Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        mat_ptr: Some(permat),
    }));

    let difflight = Arc::new(DiffuseLight::new_by_color(Color::new(4.0, 4.0, 4.0)));
    list.add(Arc::new(XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mp: difflight.clone(),
    }));
    list.add(Arc::new(Sphere {
        center: Vec3(0.0, 7.0, 0.0),
        radius: 2.0,
        mat_ptr: Some(difflight),
    }));
    list
}

pub fn cornell_box() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    let red = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new_by_color(Vec3(15.0, 15.0, 15.0)));

    list.add(Arc::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: green,
    }));
    list.add(Arc::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: red,
    }));
    list.add(Arc::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mp: light,
    }));
    list.add(Arc::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mp: white.clone(),
    }));
    list.add(Arc::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    list.add(Arc::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mp: white.clone(),
    }));
    let mut box1: Arc<dyn Hittable> = Arc::new(MyBox::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new_by_angle(box1, 15.0));
    box1 = Arc::new(Translate {
        ptr: box1,
        offset: Vec3(265.0, 0.0, 295.0),
    });
    list.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(MyBox::new(
        Vec3(0.0, 0.0, 0.0),
        Vec3(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new_by_angle(box2, -18.0));
    box2 = Arc::new(Translate {
        ptr: box2,
        offset: Vec3(130.0, 0.0, 65.0),
    });
    list.add(box2);

    list
}

pub fn cornell_smoke() -> HittableList {
    let light = Arc::new(DiffuseLight::new_by_color(Vec3(7.0, 7.0, 7.0)));
    let mut list = cornell_box();
    list.objects[2] = Arc::new(XZRect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
        mp: light,
    });
    list.objects[6] = Arc::new(ConstantMedium::new_by_color(
        list.objects[6].clone(),
        0.01,
        Vec3(0.0, 0.0, 0.0),
    ));
    list.objects[7] = Arc::new(ConstantMedium::new_by_color(
        list.objects[7].clone(),
        0.01,
        Vec3(1.0, 1.0, 1.0),
    ));

    list
}

pub fn final_scene() -> HittableList {
    let mut boxes1 = HittableList { objects: vec![] };
    let ground_material = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.48, 0.83, 0.53)));

    const BOXES_PER_SIDE: i32 = 20;

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double(1.0, 101.0);
            let z1 = z0 + w;
            boxes1.add(Arc::new(MyBox::new(
                Vec3(x0, y0, z0),
                Vec3(x1, y1, z1),
                ground_material.clone(),
            )));
        }
    }
    let mut objects = HittableList { objects: vec![] };

    objects.add(Arc::new(BvhNode::new_list(boxes1, 0.0, 1.0)));

    let light = Arc::new(DiffuseLight::new_by_color(Vec3(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        mp: light,
    }));

    let center1 = Vec3(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.7, 0.3, 0.1)));

    objects.add(Arc::new(MovingSphere {
        center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat_ptr: Some(moving_sphere_material),
    }));

    objects.add(Arc::new(Sphere {
        center: Vec3(260.0, 150.0, 45.0),
        radius: 50.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    }));
    objects.add(Arc::new(Sphere {
        center: Vec3(0.0, 150.0, 145.0),
        radius: 50.0,
        mat_ptr: Some(Arc::new(Metal {
            albedo: Vec3(0.8, 0.8, 0.9),
            fuzz: 1.0,
        })),
    }));

    let mut boundary = Arc::new(Sphere {
        center: Vec3(360.0, 150.0, 145.0),
        radius: 70.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    });

    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_by_color(
        boundary,
        0.2,
        Vec3(0.2, 0.4, 0.9),
    )));

    boundary = Arc::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 5000.0,
        mat_ptr: Some(Arc::new(Dielectric { ir: 1.5 })),
    });

    objects.add(Arc::new(ConstantMedium::new_by_color(
        boundary,
        0.0001,
        Vec3(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new_by_texture(Arc::new(
        ImageTexture::load_image_file("./raytracer/sources/earthmap.jpg"),
    )));

    objects.add(Arc::new(Sphere {
        center: Vec3(400.0, 200.0, 400.0),
        radius: 100.0,
        mat_ptr: Some(emat),
    }));

    let pertext = Arc::new(NoiseTexture::new_by_sc(0.1));
    objects.add(Arc::new(Sphere {
        center: Vec3(220.0, 280.0, 300.0),
        radius: 80.0,
        mat_ptr: Some(Arc::new(Lambertian::new_by_texture(pertext))),
    }));

    let mut boxes2 = HittableList { objects: vec![] };
    let white = Arc::new(Lambertian::new_by_solid_color(&Vec3(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere {
            center: Vec3::random(0.0, 165.0),
            radius: 10.0,
            mat_ptr: Some(white.clone()),
        }));
    }

    objects.add(Arc::new(Translate {
        ptr: Arc::new(RotateY::new_by_angle(
            Arc::new(BvhNode::new_list(boxes2, 0.0, 1.0)),
            15.0,
        )),
        offset: Vec3(-100.0, 270.0, 395.0),
    }));

    objects
}
