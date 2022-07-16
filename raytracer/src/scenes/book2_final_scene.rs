use std::sync::Arc;

use crate::{
    basic::{random_double, vec3::Vec3},
    bvh::BvhNode,
    hittable::{hittable_list::HittableList, rotate_y::RotateY, translate::Translate},
    material::{
        dielectric::Dielectric, diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal,
    },
    objects::{
        aarect::XZRect, constant_medium::ConstantMedium, moving_sphere::MovingSphere,
        my_box::MyBox, sphere::Sphere,
    },
    texture::{image_texture::ImageTexture, noise_texture::NoiseTexture},
};

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
        ImageTexture::load_image_file("./raytracer/sources/yiyan.jpg"),
    )));

    let image_sphere = Arc::new(Sphere {
        center: Vec3(400.0, 200.0, 400.0),
        radius: 100.0,
        mat_ptr: Some(emat),
    });
    objects.add(image_sphere);

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
