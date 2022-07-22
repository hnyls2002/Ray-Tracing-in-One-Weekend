use crate::{
    basic::vec3::Vec3,
    hittable::{
        hittable_list::HittableList,
        instances::{flip_face::FlipFace, rotate_y::RotateY, translate::Translate},
        objects::{
            aarect::{XYRect, XZRect, YZRect},
            triangle::Triangle,
        },
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    texture::solid_color_texture::SolidColor,
};

pub fn test_scene() -> (HittableList, HittableList) {
    let mut list = HittableList { objects: vec![] };
    let red = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.65, 0.05, 0.05));
    let white = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let green = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.12, 0.45, 0.15));
    let light = DiffuseLight::new_by_color(Vec3(15.0, 15.0, 15.0));

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
    list.add(Box::new(FlipFace {
        obj: XZRect {
            x0: 213.0,
            x1: 343.0,
            z0: 227.0,
            z1: 332.0,
            k: 554.0,
            mat: light,
        },
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

    /*
    let alumium = Metal {
        albedo: Vec3(0.8, 0.85, 0.88),
        fuzz: 0.0,
    };
    let glass = Dielectric { ir: 1.5 };
    */

    let yellow = Lambertian::<SolidColor>::new_by_solid_color(Vec3(1.0, 69.0 / 255.0, 0.0));
    let triangle = Triangle::new(
        Vec3(0.0, 0.0, 100.0),
        Vec3(365.0, 0.0, 100.0),
        Vec3(365.0, 200.0, 100.0),
        yellow,
    );
    let triangle = RotateY::new_by_angle(triangle, 15.0);
    let triangle = Translate {
        obj: triangle,
        offset: Vec3(50.0, 0.0, 300.0),
    };
    list.add(Box::new(triangle));

    let mut lights = HittableList::default();
    lights.add(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mat: light, // the material doesn't matter
    }));

    (list, lights)
}