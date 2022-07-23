use crate::{
    basic::vec3::{Color, Point3, Vec3},
    hittable::{
        hittable_list::HittableList,
        objects::{aarect::XYRect, sphere::Sphere},
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    texture::{
        checker_texture::CheckerTexture, image_texture::ImageTexture, noise_texture::NoiseTexture,
        solid_color_texture::SolidColor,
    },
};

#[allow(dead_code)]
pub fn two_spheres() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    let checker = CheckerTexture::<SolidColor, SolidColor>::new_by_color(
        Color::new(1.0, 0.5, 0.0),
        Color::new(0.9, 0.9, 0.9),
    );
    list.add(Box::new(Sphere {
        center: Point3::new(0.0, -10.0, 0.0),
        radius: 10.0,
        mat: Lambertian::new_by_texture(checker),
    }));
    list.add(Box::new(Sphere {
        center: Point3::new(0.0, 10.0, 0.0),
        radius: 10.0,
        mat: Lambertian::new_by_texture(checker),
    }));
    list
}

#[allow(dead_code)]
pub fn two_perlin_spheres() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    // Generate texture
    let pertext = NoiseTexture::new_by_sc(4.0);
    // Generate material
    let permat = Lambertian { albedo: pertext };

    list.add(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: permat.clone(),
    }));

    list.add(Box::new(Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        mat: permat,
    }));

    list
}

#[allow(dead_code)]
pub fn earth() -> HittableList {
    let earth_texture = ImageTexture::load_image_file("./raytracer/sources/yiyan.jpg");
    let earth_surface = Lambertian {
        albedo: earth_texture,
    };
    let globe = Box::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 2.0,
        mat: earth_surface,
    });
    let mut list = HittableList { objects: vec![] };
    list.add(globe);
    list
}

#[allow(dead_code)]
pub fn simple_light() -> HittableList {
    let mut list = HittableList { objects: vec![] };
    let pertext = NoiseTexture::new_by_sc(4.0);
    let permat = Lambertian { albedo: pertext };

    list.add(Box::new(Sphere {
        center: Vec3(0.0, -1000.0, 0.0),
        radius: 1000.0,
        mat: permat.clone(),
    }));

    list.add(Box::new(Sphere {
        center: Vec3(0.0, 2.0, 0.0),
        radius: 2.0,
        mat: permat,
    }));

    let difflight = DiffuseLight::new_by_color(Color::new(4.0, 4.0, 4.0));
    list.add(Box::new(XYRect {
        x0: 3.0,
        x1: 5.0,
        y0: 1.0,
        y1: 3.0,
        k: -2.0,
        mat: difflight,
    }));
    list.add(Box::new(Sphere {
        center: Vec3(0.0, 7.0, 0.0),
        radius: 2.0,
        mat: difflight,
    }));
    list
}
