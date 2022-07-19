use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    basic::vec3::{Color, Point3, Vec3},
    hittable::{
        hittable_list::HittableList,
        objects::{moving_sphere::MovingSphere, sphere::Sphere},
    },
    material::{dielectric::Dielectric, lambertian::Lambertian, metal::Metal},
    texture::{checker_texture::CheckerTexture, solid_color_texture::SolidColor},
};

pub fn random_scene() -> HittableList {
    let mut world = HittableList { objects: vec![] };
    let checker = CheckerTexture::<SolidColor, SolidColor>::new_by_color(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0.9, 0.9, 0.9),
    );
    let ground_material = Lambertian::new_by_texture(checker);
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: ground_material,
    }));
    let mut rng = StdRng::seed_from_u64(19260817);
    let mut fixed_seed_double = || rng.gen_range(0.0..1.0);
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = fixed_seed_double();
            let center = Point3::new(
                a as f64 + 0.9 * fixed_seed_double(),
                0.2,
                b as f64 + 0.9 * fixed_seed_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let sphere_material = Lambertian::new_by_texture(SolidColor::new_from_color(
                        Vec3(
                            fixed_seed_double(),
                            fixed_seed_double(),
                            fixed_seed_double(),
                        ) * Vec3(
                            fixed_seed_double(),
                            fixed_seed_double(),
                            fixed_seed_double(),
                        ),
                    ));

                    let center2 = center + Vec3(0.0, fixed_seed_double() * 0.5, 0.0);
                    world.add(Box::new(MovingSphere {
                        center0: center,
                        center1: center2,
                        time0: 0.0,
                        time1: 1.0,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else if choose_mat < 0.95 {
                    let sphere_material = Metal {
                        albedo: Vec3(
                            fixed_seed_double() * 0.5 + 0.5,
                            fixed_seed_double() * 0.5 + 0.5,
                            fixed_seed_double() * 0.5 + 0.5,
                        ),
                        fuzz: fixed_seed_double() * 0.5,
                    };
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                } else {
                    let sphere_material = Dielectric { ir: 1.5 };
                    world.add(Box::new(Sphere {
                        center,
                        radius: 0.2,
                        mat: sphere_material,
                    }));
                };
            }
        }
    }
    let material1 = Dielectric { ir: 1.5 };
    let material2 =
        Lambertian::new_by_texture(SolidColor::new_from_color(Color::new(0.4, 0.2, 0.1)));
    let material3 = Metal {
        albedo: Color::new(0.7, 0.6, 0.5),
        fuzz: 0.0,
    };

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 1.0, 0.0),
        radius: 1.0,
        mat: material1,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(-4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material2,
    }));

    world.add(Box::new(Sphere {
        center: Point3::new(4.0, 1.0, 0.0),
        radius: 1.0,
        mat: material3,
    }));
    world
}
