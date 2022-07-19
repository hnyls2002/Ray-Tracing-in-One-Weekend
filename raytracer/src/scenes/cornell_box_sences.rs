use crate::{
    basic::vec3::Vec3,
    hittable::{
        hittable_list::HittableList,
        instances::{constant_medium::ConstantMedium, rotate_y::RotateY, translate::Translate},
        objects::{
            aarect::{XYRect, XZRect, YZRect},
            my_box::MyBox,
        },
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian},
    texture::solid_color_texture::SolidColor,
};

pub fn cornell_box() -> HittableList {
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
    list.add(Box::new(XZRect {
        x0: 213.0,
        x1: 343.0,
        z0: 227.0,
        z1: 332.0,
        k: 554.0,
        mat: light,
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
    let box1 = MyBox::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 330.0, 165.0), white);
    let box1 = RotateY::new_by_angle(box1, 15.0);
    let box1 = Translate {
        obj: box1,
        offset: Vec3(265.0, 0.0, 295.0),
    };
    list.add(Box::new(box1));

    let box2 = MyBox::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 165.0, 165.0), white);
    let box2 = RotateY::new_by_angle(box2, -18.0);
    let box2 = Translate {
        obj: box2,
        offset: Vec3(130.0, 0.0, 65.0),
    };
    list.add(Box::new(box2));

    list
}

pub fn cornell_smoke() -> HittableList {
    let white = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.73, 0.73, 0.73));
    let light = DiffuseLight::new_by_color(Vec3(7.0, 7.0, 7.0));
    let mut list = cornell_box();
    list.objects[2] = Box::new(XZRect {
        x0: 113.0,
        x1: 443.0,
        z0: 127.0,
        z1: 432.0,
        k: 554.0,
        mat: light,
    });

    let box1 = MyBox::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 330.0, 165.0), white);
    let box1 = RotateY::new_by_angle(box1, 15.0);
    let box1 = Translate {
        obj: box1,
        offset: Vec3(265.0, 0.0, 295.0),
    };

    let box2 = MyBox::new(Vec3(0.0, 0.0, 0.0), Vec3(165.0, 165.0, 165.0), white);
    let box2 = RotateY::new_by_angle(box2, -18.0);
    let box2 = Translate {
        obj: box2,
        offset: Vec3(130.0, 0.0, 65.0),
    };

    list.objects[6] = Box::new(ConstantMedium::new_by_color(
        box1,
        0.01,
        Vec3(0.0, 0.0, 0.0),
    ));
    list.objects[7] = Box::new(ConstantMedium::new_by_color(
        box2,
        0.01,
        Vec3(1.0, 1.0, 1.0),
    ));

    list
}
