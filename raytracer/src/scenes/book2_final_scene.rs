use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    basic::vec3::Vec3,
    bvh::BvhNode,
    hittable::{
        hittable_list::HittableList,
        instances::{
            constant_medium::ConstantMedium, flip_face::FlipFace, rotate_y::RotateY,
            translate::Translate,
        },
        objects::{aarect::XZRect, moving_sphere::MovingSphere, my_box::MyBox, sphere::Sphere},
    },
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    texture::{
        image_texture::ImageTexture, noise_texture::NoiseTexture, solid_color_texture::SolidColor,
    },
};

pub fn final_scene() -> (HittableList, HittableList) {
    let mut boxes1 = HittableList { objects: vec![] };
    let ground_material = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.48, 0.83, 0.53));

    const BOXES_PER_SIDE: i32 = 20;

    let mut rng = StdRng::seed_from_u64(19260817);
    let mut fixed_seed_double = || rng.gen_range(0.0..1.0);

    for i in 0..BOXES_PER_SIDE {
        for j in 0..BOXES_PER_SIDE {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = fixed_seed_double() * 100.0 + 1.0;
            let z1 = z0 + w;
            boxes1.add(Box::new(MyBox::new(
                Vec3(x0, y0, z0),
                Vec3(x1, y1, z1),
                ground_material,
            )));
        }
    }
    let mut objects = HittableList { objects: vec![] };

    objects.add(Box::new(BvhNode::new_from_list(boxes1, 0.0, 1.0)));

    let light = DiffuseLight::new_by_color(Vec3(7.0, 7.0, 7.0));

    let light_rect = FlipFace {
        obj: XZRect {
            x0: 123.0,
            x1: 423.0,
            z0: 147.0,
            z1: 412.0,
            k: 554.0,
            mat: light,
        },
    };
    objects.add(Box::new(light_rect));

    let mut lights = HittableList::default();
    lights.add(Box::new(XZRect {
        x0: 123.0,
        x1: 423.0,
        z0: 147.0,
        z1: 412.0,
        k: 554.0,
        mat: light,
    }));

    let center1 = Vec3(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3(30.0, 0.0, 0.0);
    let moving_sphere_material = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.7, 0.3, 0.1));

    objects.add(Box::new(MovingSphere {
        center0: center1,
        center1: center2,
        time0: 0.0,
        time1: 1.0,
        radius: 50.0,
        mat: moving_sphere_material,
    }));

    objects.add(Box::new(Sphere {
        center: Vec3(260.0, 150.0, 45.0),
        radius: 50.0,
        mat: Dielectric { ir: 1.5 },
    }));

    lights.add(Box::new(Sphere {
        center: Vec3(260.0, 150.0, 45.0),
        radius: 50.0,
        mat: Dielectric { ir: 1.5 },
    }));

    objects.add(Box::new(Sphere {
        center: Vec3(0.0, 150.0, 145.0),
        radius: 50.0,
        mat: Metal {
            albedo: Vec3(0.8, 0.8, 0.9),
            fuzz: 1.0,
        },
    }));

    let mut boundary = Sphere {
        center: Vec3(360.0, 150.0, 145.0),
        radius: 70.0,
        mat: Dielectric { ir: 1.5 },
    };

    objects.add(Box::new(boundary));
    objects.add(Box::new(ConstantMedium::new_by_color(
        boundary,
        0.2,
        Vec3(0.2, 0.4, 0.9),
    )));

    boundary = Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 5000.0,
        mat: Dielectric { ir: 1.5 },
    };

    objects.add(Box::new(ConstantMedium::new_by_color(
        boundary,
        0.0001,
        Vec3(1.0, 1.0, 1.0),
    )));

    let earth_material = Lambertian::new_by_texture(ImageTexture::load_image_file(
        "./raytracer/sources/earthmap.jpg",
    ));

    let image_sphere = Box::new(Sphere {
        center: Vec3(400.0, 200.0, 400.0),
        radius: 100.0,
        mat: earth_material,
    });
    objects.add(image_sphere);

    let pertext = NoiseTexture::new_by_sc(0.1);

    objects.add(Box::new(Sphere {
        center: Vec3(220.0, 280.0, 300.0),
        radius: 80.0,
        mat: Lambertian::new_by_texture(pertext),
    }));

    let mut boxes2 = HittableList { objects: vec![] };
    let white = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Box::new(Sphere {
            center: Vec3(
                fixed_seed_double() * 165.0,
                fixed_seed_double() * 165.0,
                fixed_seed_double() * 165.0,
            ),
            radius: 10.0,
            mat: white,
        }));
    }

    objects.add(Box::new(Translate {
        obj: RotateY::new_by_angle(BvhNode::new_from_list(boxes2, 0.0, 1.0), 15.0),
        offset: Vec3(-100.0, 270.0, 395.0),
    }));

    (objects, lights)
}
