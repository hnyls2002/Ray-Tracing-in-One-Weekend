use console::style;
use hittablelist::{
    hittable::{HitRecord, Hittable},
    HittableList,
};
use image::{ImageBuffer, Rgb, RgbImage};

use std::{
    fs::File,
    process::exit,
    thread::{self, JoinHandle},
};

mod camera;
mod hittablelist;
mod material;
mod sphere;

use camera::rtweekend::{
    clamp, INFINITY,
    {ray::Ray, vec3::Color},
};

use crate::{
    camera::{
        rtweekend::{
            random_double_unit,
            vec3::{Point3, Vec3},
        },
        Camera,
    },
    hittablelist::random_scene,
};

// Image
const ASPECT_RATIO: f64 = 3.0 / 2.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;

// Camera
const LOOKFROM: Point3 = Vec3(13.0, 2.0, 3.0);
const LOOKAT: Point3 = Vec3(0.0, 0.0, 0.0);
const VUP: Vec3 = Vec3(0.0, 1.0, 0.0);
const DIST_TO_FOCUS: f64 = 10.0;
const APERTURE: f64 = 0.1;

// Threads
const THREAD_NUM: u32 = 20;
const LINES_PER_SECTION: u32 = IMAGE_HEIGHT / THREAD_NUM;

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    let mut rec: HitRecord = Default::default();

    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(r, 0.001, INFINITY, &mut rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();
        if let Some(mat_ptr) = rec.clone().mat_ptr {
            if mat_ptr.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return ray_color(&scattered, world, depth - 1) * attenuation;
            }
        }
        return Color::new(0.0, 0.0, 0.0);
    }
    let unit_direction = r.direction().unit_vec();
    let t = (unit_direction.1 + 1.0) * 0.5;
    Color::new(1.0, 1.0, 1.0) * (1.0 - t) + Color::new(0.5, 0.7, 1.0) * t
}

fn write_color(pixel: &mut Rgb<u8>, pixel_colors: &Color) {
    let r = pixel_colors.0 / (SAMPLES_PER_PIXEL as f64);
    let g = pixel_colors.1 / (SAMPLES_PER_PIXEL as f64);
    let b = pixel_colors.2 / (SAMPLES_PER_PIXEL as f64);

    // Gamma-correct for gamma=2.0
    let r = r.sqrt();
    let g = g.sqrt();
    let b = b.sqrt();

    let r = (clamp(r, 0.0, 0.999) * (256_f64)).floor() as u8;
    let g = (clamp(g, 0.0, 0.999) * (256_f64)).floor() as u8;
    let b = (clamp(b, 0.0, 0.999) * (256_f64)).floor() as u8;

    //println!("{} {} {}", r, g, b);

    *pixel = image::Rgb([r, g, b]);
}

fn output_image(path: &str, img: &RgbImage, quality: u8) {
    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img.clone());
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }
}

fn create_thread(
    thread_id: u32,
    world: HittableList,
    cam: Camera,
) -> JoinHandle<(Vec<Color>, u32, u32)> {
    let line_beg = thread_id * LINES_PER_SECTION;
    let mut line_end = line_beg + LINES_PER_SECTION;
    if line_end > IMAGE_HEIGHT || (thread_id == THREAD_NUM - 1 && line_end < IMAGE_HEIGHT) {
        line_end = IMAGE_HEIGHT;
    }

    let mut ret = Vec::<Color>::new();
    thread::spawn(move || {
        for y in line_beg..line_end {
            for x in 0..IMAGE_WIDTH {
                let mut pixel_colors = Color::new(0.0, 0.0, 0.0);

                for _s in 0..SAMPLES_PER_PIXEL {
                    // a bunch of rays hitting the object
                    let u = (x as f64 + random_double_unit()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (y as f64 + random_double_unit()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_colors += ray_color(&r, &world, MAX_DEPTH);
                }

                ret.push(pixel_colors);
            }
        }
        (ret, line_beg, line_end)
    })
}

fn main() {
    // Output Path
    let path = "output/image21.jpg";

    // World
    let world = random_scene();

    // Camera
    let cam = Camera::new(
        &LOOKFROM,
        &LOOKAT,
        &VUP,
        20.0,
        ASPECT_RATIO,
        APERTURE,
        DIST_TO_FOCUS,
    );

    let mut thread_list = Vec::<_>::new();

    //let pixel_pool = Arc::new((0 as u32, 0 as u32));

    for id in 0..THREAD_NUM {
        thread_list.push(create_thread(id, world.clone(), cam));
    }

    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for _id in 0..THREAD_NUM {
        match thread_list.remove(0).join() {
            Ok((mut res, line_beg, line_end)) => {
                for y in line_beg..line_end {
                    for x in 0..IMAGE_WIDTH {
                        let pixel = img.get_pixel_mut(x, IMAGE_HEIGHT - y - 1);
                        write_color(pixel, &res.remove(0));
                    }
                }
            }
            Err(_) => println!("Thread Failed!!!"),
        }
    }

    output_image(path, &img, quality);

    exit(0);
}
