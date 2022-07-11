use console::style;
use hittablelist::{
    hittable::{HitRecord, Hittable},
    HittableList,
};
use image::{ImageBuffer, Rgb, RgbImage};

use std::{
    fs::File,
    process::exit,
    sync::{Arc, Mutex},
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
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 500;
const MAX_DEPTH: i32 = 50;

// Camera
const LOOKFROM: Point3 = Vec3(13.0, 2.0, 3.0);
const LOOKAT: Point3 = Vec3(0.0, 0.0, 0.0);
const VUP: Vec3 = Vec3(0.0, 1.0, 0.0);
const DIST_TO_FOCUS: f64 = 10.0;
const APERTURE: f64 = 0.1;

// Threads
const THREAD_NUM: u32 = 20;

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
    line_pool: Arc<Mutex<u32>>,
    world: HittableList,
    cam: Camera,
) -> JoinHandle<Vec<(u32, Vec<Color>)>> {
    let mut ret = Vec::<_>::new();
    thread::spawn(move || {
        loop {
            let mut num = line_pool.lock().unwrap();
            if *num == IMAGE_HEIGHT {
                break;
            }
            let py = *num;
            *num += 1_u32;
            println!("now line at {}", *num);
            std::mem::drop(num);

            let mut line_color = Vec::<Color>::new();

            for px in 0..IMAGE_WIDTH {
                let mut pixel_colors = Color::new(0.0, 0.0, 0.0);
                for _s in 0..SAMPLES_PER_PIXEL {
                    // a bunch of rays hitting the object
                    let u = (px as f64 + random_double_unit()) / (IMAGE_WIDTH - 1) as f64;
                    let v = (py as f64 + random_double_unit()) / (IMAGE_HEIGHT - 1) as f64;
                    let r = cam.get_ray(u, v);
                    pixel_colors += ray_color(&r, &world, MAX_DEPTH);
                }
                line_color.push(pixel_colors);
            }
            ret.push((py, line_color));
        }
        ret
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

    let line_pool = Arc::new(Mutex::new(0_u32));

    for _id in 0..THREAD_NUM {
        thread_list.push(create_thread(line_pool.clone(), world.clone(), cam));
    }

    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    for _id in 0..THREAD_NUM {
        match thread_list.remove(0).join() {
            Ok(res) => {
                for line in res {
                    let py = line.0;
                    for px in 0..IMAGE_WIDTH {
                        let pixel = img.get_pixel_mut(px, IMAGE_HEIGHT - py - 1);
                        write_color(pixel, &line.1[px as usize]);
                    }
                }
            }
            Err(_) => println!("Thread Failed!!!"),
        }
    }

    output_image(path, &img, quality);

    exit(0);
}
