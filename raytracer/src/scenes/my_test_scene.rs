use crate::{
    basic::vec3::{Color, Vec3},
    camera::Camera,
    hittable::{hittable_list::HittableList, objects::sphere::Sphere},
    material::lambertian::Lambertian,
    obj_loader::{my_loader, LoadOption},
    pdf::lightable_list::LightableList,
    texture::solid_color_texture::SolidColor,
    ASPECT_RATIO,
};

use super::SceneOption;

pub fn my_test_scene(_id: u32) -> SceneOption {
    let mut world_list = HittableList::default();
    let light_list = LightableList::default();
    let background = Color::new(0.73, 0.73, 0.73);
    let red = Lambertian::<SolidColor>::new_by_solid_color(Vec3(1.0, 0.0, 0.0));
    let green = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.0, 1.0, 0.0));
    world_list.add(Box::new(Sphere {
        center: Vec3(600.0, 0.0, 400.0),
        radius: 10.0,
        mat: red,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 10.0,
        mat: red,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, 800.0),
        radius: 10.0,
        mat: red,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(1200.0, 0.0, 0.0),
        radius: 10.0,
        mat: red,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(1200.0, 0.0, 800.0),
        radius: 10.0,
        mat: red,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(600.0, -100000.0, 400.0),
        radius: 100000.0,
        mat: green,
    }));

    let _hutao_option = LoadOption {
        path: "./raytracer/sources/HutaoObj/",
        file_name: "Hutao",
        zoom_rate: 20.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(490.0, 250.0, 230.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 0.0,
        r_z: 0.0,
    };

    let _patrick_option = LoadOption {
        path: "./raytracer/sources/someobj/",
        file_name: "patrick",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(600.0, 180.0, 400.0),
        set_flag: true,
        r_x: -20.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    let _baseball_option = LoadOption {
        path: "./raytracer/sources/someobj/",
        file_name: "10483_baseball_v1_L3",
        zoom_rate: 7.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(440.0, 270.0, 150.0),
        set_flag: true,
        r_x: -20.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    let _baseball_bat_option = LoadOption {
        path: "./raytracer/sources/someobj/",
        file_name: "10485_Baseball_bat_v1_max2011_iteration-2",
        zoom_rate: 7.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(270.0, 230.0, 150.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 0.0,
        r_z: 60.0,
    };

    let _babara_option = LoadOption {
        path: "./raytracer/sources/BabaraObj/",
        file_name: "Babara",
        zoom_rate: 24.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(700.0, 290.0, 200.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 180.0,
        r_z: 0.0,
    };

    let _moon_option = LoadOption {
        path: "./raytracer/sources/MoonObj/",
        file_name: "Moon",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(1200.0, 420.0, 800.0),
        set_flag: true,
        r_x: 0.0,
        r_y: -180.0,
        r_z: 0.0,
    };

    world_list.add(my_loader(_id, _patrick_option));
    world_list.add(my_loader(_id, _moon_option));
    world_list.add(my_loader(_id, _hutao_option));
    world_list.add(my_loader(_id, _babara_option));

    SceneOption {
        world: world_list,
        lights: light_list,
        cam: camera_generator(),
        background,
    }
}

pub fn camera_generator() -> Camera {
    // Camera
    let lookfrom = Vec3(400.0, 500.0, -1000.0);
    let lookat = Vec3(600.0, 0.0, 400.0);
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
