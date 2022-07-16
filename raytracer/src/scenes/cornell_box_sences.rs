use std::sync::Arc;

use crate::{
    basic::vec3::Vec3,
    hittable::{hittable_list::HittableList, rotate_y::RotateY, translate::Translate, Hittable},
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    objects::{
        aarect::{XYRect, XZRect, YZRect},
        constant_medium::ConstantMedium,
        my_box::MyBox,
    },
};

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
