use crate::{
    basic::vec3::Vec3,
    hittable::{
        hittable_list::HittableList,
        instances::flip_face::FlipFace,
        objects::aarect::{XYRect, XZRect, YZRect},
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    obj_loader::{my_loader, LoadOption},
    texture::solid_color_texture::SolidColor,
};

pub fn test_scene() -> (HittableList, HittableList) {
    let mut list = HittableList { objects: vec![] };
    let red = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_by_color(Vec3(25.0, 25.0, 25.0));

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
    let light_rect = XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 127.0,
        z1: 232.0,
        k: 554.0,
        mat: light,
    };

    list.add(Box::new(FlipFace {
        obj: light_rect.clone(),
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
        path: "./raytracer/sources/HutaoObj/Hutao.obj",
        zoom_rate: 20.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(250.0, 0.0, 150.0),
        r_x: 0.0,
        r_y: 0.0,
        r_z: 0.0,
    };

    let _patrick_option = LoadOption {
        path: "./raytracer/sources/someobj/patrick.obj",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(250.0, 0.0, 100.0),
        r_x: 0.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    list.add(my_loader(_patrick_option, red));

    let mut lights = HittableList::default();
    lights.add(Box::new(light_rect));

    (list, lights)
}
