use std::sync::Arc;

use crate::{
    basic::vec3::{Color, Point3, Vec3},
    hittable::hittable_list::HittableList,
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    objects::{aarect::XYRect, sphere::Sphere},
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        Texture,
    },
};

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
