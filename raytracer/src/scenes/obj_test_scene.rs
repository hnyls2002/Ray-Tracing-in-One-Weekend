use super::SceneOption;
use crate::{
    basic::vec3::Vec3,
    camera::Camera,
    hittable::{
        hittable_list::HittableList,
        instances::flip_face::FlipFace,
        objects::aarect::{XYRect, XZRect, YZRect},
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    obj_loader::{my_loader, LoadOption},
    pdf::lightable_list::LightableList,
    texture::solid_color_texture::SolidColor,
    ASPECT_RATIO,
};

#[allow(dead_code)]
fn light_get(f: f64) -> DiffuseLight<SolidColor> {
    DiffuseLight::new_by_color(Vec3(f, f, f))
}

#[allow(dead_code)]
pub fn obj_test_scene(id: u32) -> SceneOption {
    let mut list = HittableList { objects: vec![] };
    let red = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.12, 0.45, 0.15));

    list.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mat: green,
    }));
    list.add(Box::new(YZRect {
        y0: 0.0,
        y1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mat: red,
    }));
    let light_top = XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 127.0,
        z1: 232.0,
        k: 554.0,
        mat: light_get(25.0),
    };

    list.add(Box::new(FlipFace {
        obj: light_top.clone(),
    }));

    list.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 0.0,
        mat: white,
    }));
    list.add(Box::new(XZRect {
        x0: 0.0,
        x1: 555.0,
        z0: 0.0,
        z1: 555.0,
        k: 555.0,
        mat: white,
    }));
    list.add(Box::new(XYRect {
        x0: 0.0,
        x1: 555.0,
        y0: 0.0,
        y1: 555.0,
        k: 555.0,
        mat: white,
    }));

    let _hutao_option = LoadOption {
        path: "./raytracer/sources/Hutao/",
        file_name: "hutao",
        zoom_rate: 22.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(270.0, 0.0, 200.0),
        set_flag: false,
        r_x: 0.0,
        r_y: 0.0,
        r_z: 0.0,
    };

    let _patrick_option = LoadOption {
        path: "./raytracer/sources/Patrick/",
        file_name: "patrick",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(270.0, 0.0, 250.0),
        set_flag: false,
        r_x: -20.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    let _babara_option = LoadOption {
        path: "./raytracer/sources/Babara/",
        file_name: "babara",
        zoom_rate: 24.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(270.0, 0.0, 200.0),
        set_flag: false,
        r_x: 0.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    //list.add(my_loader(_patrick_option));
    //list.add(my_loader(id, _babara_option));
    list.add(my_loader(id, _patrick_option));

    let mut lights = LightableList::default();
    lights.add(Box::new(light_top));

    let light_front = XYRect {
        x0: 100.0,
        x1: 400.0,
        y0: 0.0,
        y1: 500.0,
        k: -800.0,
        mat: light_get(15.0),
    };
    list.add(Box::new(light_front.clone()));
    lights.add(Box::new(light_front));

    SceneOption {
        world: list,
        lights,
        cam: camera_generator(),
        background: Vec3(0.0, 0.0, 0.0),
    }
}

fn camera_generator() -> Camera {
    // Camera
    let lookfrom = Vec3(278.0, 278.0, -800.0);
    let lookat = Vec3(278.0, 278.0, 0.0);
    let vfov: f64 = 40.0;
    let aperture = 0.0;

    // Camera
    let vup: Vec3 = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    )
}
