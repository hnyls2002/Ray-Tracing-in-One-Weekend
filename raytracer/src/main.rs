use std::{
    fs::File,
    process::exit,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use camera::Camera;
use console::style;
use hittablelist::hittable::{HitRecord, Hittable};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use rtweekend::{
    clamp, random_double_unit,
    ray::Ray,
    vec3::{Color, Vec3},
    INFINITY,
};

use crate::{
    bvh::BvhNode,
    hittablelist::{random_scene, two_perlin_spheres, two_spheres},
};

mod bvh;
mod camera;
mod hittablelist;
mod material;
mod moving_sphere;
mod rtweekend;
mod sphere;
mod texture;

// Image
const ASPECT_RATIO: f64 = 16.0 / 9.0;
const IMAGE_WIDTH: u32 = 400;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 100;
const MAX_DEPTH: i32 = 50;

// Threads
const THREAD_NUM: u32 = 20;
const LINES_PER_ISSUE: u32 = 10;

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
    world: BvhNode,
    cam: Camera,
    bars: Arc<MultiProgress>,
) -> JoinHandle<Vec<(u32, Vec<Color>)>> {
    let mut ret = Vec::<_>::new();
    thread::spawn(move || {
        // Set Progress Bar for this thread
        let now_bar = bars.add(ProgressBar::new((IMAGE_HEIGHT / THREAD_NUM) as u64));
        now_bar.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

        // Catch one avaliable line
        loop {
            let mut num = line_pool.lock().unwrap();
            if *num >= IMAGE_HEIGHT {
                break;
            }
            let py0 = *num;
            let py1 = if py0 + LINES_PER_ISSUE <= IMAGE_HEIGHT {
                py0 + LINES_PER_ISSUE
            } else {
                IMAGE_HEIGHT
            };
            *num += LINES_PER_ISSUE;
            std::mem::drop(num);

            for py in py0..py1 {
                now_bar.inc(1);

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
        }
        now_bar.finish_with_message("Finished.");
        ret
    })
}

fn main() {
    // Output Path
    let path = "output/image2-12.jpg";

    // Camera
    #[allow(unused_assignments)]
    let mut lookfrom: Vec3 = Vec3::default();
    #[allow(unused_assignments)]
    let mut lookat: Vec3 = Vec3::default();
    #[allow(unused_assignments)]
    let mut vfov: f64 = 40.0;
    let mut aperture = 0.0;

    // World
    let world;
    let opt = 0;

    if opt == 1 {
        world = BvhNode::new_list(random_scene(), 0.0, 1.0);
        lookfrom = Vec3(13.0, 2.0, 3.0);
        lookat = Vec3(0.0, 0.0, 0.0);
        vfov = 20.0;
        aperture = 0.1;
    } else if opt == 2 {
        world = BvhNode::new_list(two_spheres(), 0.0, 0.0);
        lookfrom = Vec3(13.0, 2.0, 3.0);
        lookat = Vec3(0.0, 0.0, 0.0);
        vfov = 20.0;
    } else {
        world = BvhNode::new_list(two_perlin_spheres(), 0.0, 0.0);
        lookfrom = Vec3(13.0, 2.0, 3.0);
        lookat = Vec3(0.0, 0.0, 0.0);
        vfov = 20.0;
    }

    // Camera
    let vup: Vec3 = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        ASPECT_RATIO,
        aperture,
        dist_to_focus,
        0.0,
        1.0,
    );

    // Show the Image Information
    {
        print!("{}[2J", 27 as char); // clear screen
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char); // set cursor at 1,1
        println!(
            "         Image name:                {}",
            style(path.to_string()).yellow()
        );
        println!(
            "         Image size:                {}",
            style(IMAGE_WIDTH.to_string() + &"x".to_string() + &IMAGE_HEIGHT.to_string()).yellow()
        );
        println!(
            "         Sample number per pixel:   {}",
            style(SAMPLES_PER_PIXEL.to_string()).yellow()
        );
        println!(
            "         Reflection max depth:      {}",
            style(MAX_DEPTH.to_string()).yellow()
        );
    }

    // Threads
    let mut thread_list = Vec::<_>::new();
    let line_pool = Arc::new(Mutex::new(0_u32));

    // Threads: progress bar
    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    // Show the Threads Information
    println!(
        "🚀 {} {} {}",
        style("Rendering with").green(),
        style(THREAD_NUM.to_string()).yellow(),
        style("Threads...").green(),
    );

    for _id in 0..THREAD_NUM {
        thread_list.push(create_thread(
            line_pool.clone(),
            world.clone(),
            cam,
            multiprogress.clone(),
        ));
    }

    multiprogress.join().unwrap();

    // Generating Image
    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(IMAGE_WIDTH, IMAGE_HEIGHT);

    // Generating Image: Progress Bar
    let generating_progress_bar = ProgressBar::new(IMAGE_HEIGHT as u64);
    println!("🚛 {}", style("Filling up Pixels...").green(),);

    for _id in 0..THREAD_NUM {
        match thread_list.remove(0).join() {
            Ok(res) => {
                for line in res {
                    let py = line.0;
                    for px in 0..IMAGE_WIDTH {
                        let pixel = img.get_pixel_mut(px, IMAGE_HEIGHT - py - 1);
                        write_color(pixel, &line.1[px as usize]);
                    }
                    generating_progress_bar.inc(1);
                }
            }
            Err(_) => println!("Thread Failed!!!"),
        }
    }
    generating_progress_bar.finish();

    output_image(path, &img, quality);

    exit(0);
}
