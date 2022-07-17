use std::sync::Arc;

use crate::{
    basic::{
        random_double, random_double_unit,
        vec3::{Color, Point3, Vec3},
    },
    hittable::{
        hittable_list::HittableList,
        objects::{moving_sphere::MovingSphere, sphere::Sphere},
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal, Material},
    texture::{checker_texture::CheckerTexture, solid_color_texture::SolidColor, Texture},
};

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
