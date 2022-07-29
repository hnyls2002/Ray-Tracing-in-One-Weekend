use rand::{prelude::StdRng, Rng, SeedableRng};

use crate::{
    basic::vec3::{Color, Vec3},
    camera::Camera,
    hittable::{
        hittable_list::HittableList,
        instances::{bump::BumpSurface, flip_face::FlipFace},
        objects::{
            aarect::{XYRect, XZRect},
            my_box::MyBox,
            sphere::Sphere,
        },
    },
    material::{diffuse_light::DiffuseLight, lambertian::Lambertian, metal::Metal},
    obj_loader::{
        moon::moon_loader, my_loader, treelight::treelight_loader, ufo::ufo_loader, LoadOption,
    },
    pdf::lightable_list::LightableList,
    texture::solid_color_texture::SolidColor,
    ASPECT_RATIO,
};

use super::SceneOption;

fn light_get(f: f64) -> DiffuseLight<SolidColor> {
    DiffuseLight::new_by_color(Vec3(f, f, f))
}

fn ground_generator(world_list: &mut HittableList) {
    let _gold = Vec3(1.0, 215.0 / 255.0, 0.0);
    let _tan4 = Vec3(1.0, 215.0 / 255.0, 0.0);
    let _firebrick4 = Vec3(139.0 / 255.0, 26.0 / 255.0, 26.0 / 255.0);
    let _sienna4 = Vec3(139.0 / 255.0, 71.0 / 255.0, 38.0 / 255.0);
    let _ground_material = Lambertian::<SolidColor>::new_by_solid_color(_sienna4);
    let _ground_metal_material = Metal {
        albedo: _sienna4,
        fuzz: 0.7,
    };
    let ground = Sphere {
        center: Vec3(600.0, -100000.0, 400.0),
        radius: 100000.0,
        mat: _ground_metal_material,
        is_ground: true,
    };
    world_list.add(Box::new(BumpSurface::new_from_obj_and_normal_map(
        ground,
        "./raytracer/sources/Images/ground_norm.jpg",
        1500,
    )));
}

#[allow(dead_code)]
fn basic_points_generator(world_list: &mut HittableList) {
    let _red = Lambertian::<SolidColor>::new_by_solid_color(Vec3(1.0, 0.0, 0.0));
    world_list.add(Box::new(Sphere {
        center: Vec3(600.0, 0.0, 400.0),
        radius: 10.0,
        mat: light_get(15.0),
        is_ground: false,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, 0.0),
        radius: 10.0,
        mat: light_get(15.0),
        is_ground: false,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(0.0, 0.0, 800.0),
        radius: 10.0,
        mat: light_get(15.0),
        is_ground: false,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(1200.0, 0.0, 0.0),
        radius: 10.0,
        mat: light_get(15.0),
        is_ground: false,
    }));
    world_list.add(Box::new(Sphere {
        center: Vec3(1200.0, 0.0, 800.0),
        radius: 10.0,
        mat: light_get(15.0),
        is_ground: false,
    }));
}

fn lights_generator(_id: u32, world_list: &mut HittableList, light_list: &mut LightableList) {
    let light_top = XZRect {
        x0: 200.0,
        x1: 400.0,
        z0: 400.0,
        z1: 600.0,
        k: 600.0,
        mat: light_get(50.0),
    };
    world_list.add(Box::new(FlipFace {
        obj: light_top.clone(),
    }));
    light_list.add(Box::new(light_top));
}

fn forest_light_generator(id: u32, world_list: &mut HittableList) {
    // treelight
    let _light_orange = DiffuseLight::new_by_color(Vec3(1.0, 69.0 / 255.0, 0.0) * 3.0);
    let _light_hotpink = DiffuseLight::new_by_color(Vec3(1.0, 105.0 / 255.0, 180.0 / 255.0) * 1.0);
    let _light_green = DiffuseLight::new_by_color(Vec3(0.0, 139.0 / 255.0, 0.0) * 3.0);
    let _light_blue = DiffuseLight::new_by_color(Vec3(30.0 / 255.0, 144.0 / 255.0, 1.0) * 2.0);
    let mut rng = StdRng::seed_from_u64(19260817);
    for i in 0..4 {
        let _treelight_option = LoadOption {
            path: "./raytracer/sources/Treelight/",
            file_name: "treelight",
            zoom_rate: 2000.0,
            zoom_orig: Vec3(0.0, 0.0, 0.0),
            offset: Vec3(
                rng.gen_range(-600.0..200.0) + if i == 2 { -150.0 } else { 0.0 },
                100.0,
                rng.gen_range(800.0..2000.0),
            ),
            set_flag: true,
            r_x: 0.0,
            r_y: rng.gen_range(0.0..360.0),
            r_z: 0.0,
        };
        world_list.add(treelight_loader(
            id,
            _treelight_option,
            _light_orange.clone(),
            // match i {
            //     1 => _light_hotpink.clone(),
            //     2 => _light_green.clone(),
            //     _ => _light_blue.clone(),
            // },
        ));
    }
}

fn moon_generator(id: u32, world_list: &mut HittableList) {
    let _moon_option = LoadOption {
        path: "./raytracer/sources/Moon/",
        file_name: "moon",
        zoom_rate: 70.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(-400.0, 400.0, 1800.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 90.0,
        r_z: 0.0,
    };

    world_list.add(moon_loader(id, _moon_option));
}

fn cuboid_generator(world_list: &mut HittableList, light_list: &mut LightableList) {
    let _alumium = Metal {
        albedo: Vec3(0.8, 0.85, 0.88),
        fuzz: 0.0,
    };
    let _mirror = Metal {
        albedo: Vec3(1.0, 1.0, 1.0),
        fuzz: 0.0,
    };
    let _brass_metal = Metal {
        albedo: Vec3(191.0 / 255.0, 173.0 / 255.0, 111.0 / 255.0),
        fuzz: 0.3,
    };
    let _brass_lam = Lambertian::<SolidColor>::new_by_solid_color(Vec3(1.0, 127.0 / 255.0, 0.0));
    let cube_off = Vec3(2088.0, 548.0, 400.0) * 0.2;
    let p0 = Vec3(150.0, 0.0, -230.0);

    let cuboid = MyBox::new(p0, p0 + cube_off, _alumium);
    world_list.add(Box::new(cuboid));

    let board = XYRect {
        x0: p0.0,
        x1: (p0 + cube_off).0,
        y0: p0.1,
        y1: (p0 + cube_off).1,
        k: p0.2 - 0.1,
        mat: _brass_lam,
    };
    let board = BumpSurface::new_from_obj_and_normal_map(
        board,
        "./raytracer/sources/Images/SJTU_norm.png",
        1,
    );

    world_list.add(Box::new(board));

    let light_front = XYRect {
        x0: 200.0,
        x1: 700.0,
        y0: 500.0,
        y1: 800.0,
        k: -1000.0,
        mat: light_get(9.0),
    };
    world_list.add(Box::new(light_front.clone()));
    light_list.add(Box::new(light_front));

    let piece = 7;
    let len = cube_off / (2 * piece - 1) as f64;
    let mut sx = p0.0;
    for _i in 0..piece {
        let ground_lamp = XZRect {
            x0: sx,
            x1: sx + len.0,
            z0: p0.2 - 40.0,
            z1: p0.2 - 30.0,
            k: 0.0,
            mat: light_get(25.0),
        };
        world_list.add(Box::new(ground_lamp));
        sx += len.0 * 2.0;
    }
}

fn blackboard_generator(id: u32, world_list: &mut HittableList) {
    let _blackboard_option = LoadOption {
        path: "./raytracer/sources/Blackboard/",
        file_name: "blackboard",
        zoom_rate: 1250.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(300.0, 240.0, 830.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 180.0,
        r_z: 0.0,
    };
    let blackboard = my_loader(id, _blackboard_option);
    world_list.add(blackboard);
}

fn city_platform_generator(id: u32, world_list: &mut HittableList, light_list: &mut LightableList) {
    let _platform_option = LoadOption {
        path: "./raytracer/sources/Cityp_Platform/",
        file_name: "city_platform",
        zoom_rate: 7000.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(1200.0, 250.0, 900.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 270.0,
        r_z: 0.0,
    };
    let platform = my_loader(id, _platform_option);
    world_list.add(platform);

    let _ufo_option = LoadOption {
        path: "./raytracer/sources/UFO/",
        file_name: "ufo",
        zoom_rate: 20.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(990.0, 340.0, 760.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 0.0,
        r_z: 0.0,
    };
    let ufo = ufo_loader(id, _ufo_option);
    world_list.add(ufo);

    let _alien_option = LoadOption {
        path: "./raytracer/sources/Alien/",
        file_name: "alien",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(1260.0, 300.0, 760.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 150.0,
        r_z: 0.0,
    };
    let alien = my_loader(id, _alien_option);
    world_list.add(alien);
    let light_top = XZRect {
        x0: 1000.0,
        x1: 1400.0,
        z0: 400.0,
        z1: 800.0,
        k: 700.0,
        mat: light_get(7.0),
    };

    world_list.add(Box::new(FlipFace {
        obj: light_top.clone(),
    }));
    light_list.add(Box::new(light_top));
}

fn beach_generator(id: u32, world_list: &mut HittableList, light_list: &mut LightableList) {
    let _beach_option = LoadOption {
        path: "./raytracer/sources/Beach/",
        file_name: "beach",
        zoom_rate: 25.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(1050.0, 46.5, 55.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 0.0,
        r_z: 0.0,
    };
    let beach = my_loader(id, _beach_option);
    world_list.add(beach);

    let _banana_option = LoadOption {
        path: "./raytracer/sources/Bananalight/",
        file_name: "bananalight",
        zoom_rate: 3000.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(800.0, 80.0, -50.0),
        set_flag: true,
        r_x: 0.0,
        r_y: -60.0,
        r_z: 0.0,
    };
    let banana0 = my_loader(id, _banana_option);
    world_list.add(banana0);

    let _babara_option = LoadOption {
        path: "./raytracer/sources/Babara/",
        file_name: "babara",
        zoom_rate: 24.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(870.0, 290.0, -250.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 140.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _babara_option));

    let _patrick_option = LoadOption {
        path: "./raytracer/sources/Patrick/",
        file_name: "patrick",
        zoom_rate: 100.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(935.0, 90.0, -20.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 150.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _patrick_option));

    let light_front = XYRect {
        x0: 700.0,
        x1: 1000.0,
        y0: 0.0,
        y1: 700.0,
        k: -1000.0,
        mat: light_get(5.0),
    };
    world_list.add(Box::new(light_front.clone()));
    light_list.add(Box::new(light_front));
}

fn pyramid_generator(id: u32, world_list: &mut HittableList) {
    let _pyramid_option = LoadOption {
        path: "./raytracer/sources/Pyramid_Monsters/",
        file_name: "pyramid_monsters",
        zoom_rate: 870.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(390.0, 220.0, 160.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 180.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _pyramid_option));

    let _beepee_option = LoadOption {
        path: "./raytracer/sources/Beepee/",
        file_name: "beepee",
        zoom_rate: 1000.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(20.0, 90.0, 360.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 200.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _beepee_option));
}

fn glass_and_frog_generator(id: u32, world_list: &mut HittableList) {
    let _glass_option = LoadOption {
        path: "./raytracer/sources/Glass/",
        file_name: "glass",
        zoom_rate: 200.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(250.0, 175.5, -245.0),
        set_flag: true,
        r_x: -15.0,
        r_y: 210.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _glass_option));

    let _frog_option = LoadOption {
        path: "./raytracer/sources/Frog/",
        file_name: "frog",
        zoom_rate: 250.0,
        zoom_orig: Vec3(0.0, 0.0, 0.0),
        offset: Vec3(655.0, 14.0, -320.0),
        set_flag: true,
        r_x: 0.0,
        r_y: 150.0,
        r_z: 0.0,
    };
    world_list.add(my_loader(id, _frog_option));
}

pub fn final_scene(id: u32) -> SceneOption {
    let mut world_list = HittableList::default();
    let mut light_list = LightableList::default();

    let background = Color::new(0.015, 0.015, 0.015);
    //let background = Color::new(0.73, 0.73, 0.73);

    ground_generator(&mut world_list);

    lights_generator(id, &mut world_list, &mut light_list);

    cuboid_generator(&mut world_list, &mut light_list);

    blackboard_generator(id, &mut world_list);

    city_platform_generator(id, &mut world_list, &mut light_list);

    beach_generator(id, &mut world_list, &mut light_list);

    moon_generator(id, &mut world_list);

    pyramid_generator(id, &mut world_list);

    forest_light_generator(id, &mut world_list);

    glass_and_frog_generator(id, &mut world_list);

    let _green = Lambertian::<SolidColor>::new_by_solid_color(Vec3(0.0, 1.0, 0.0));
    let _alumium = Metal {
        albedo: Vec3(0.8, 0.85, 0.88),
        fuzz: 0.0,
    };
    let _some_metal = Metal {
        albedo: Vec3(0.8, 0.85, 0.88),
        fuzz: 0.6,
    };

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
