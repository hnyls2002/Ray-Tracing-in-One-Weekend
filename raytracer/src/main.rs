use std::{
    fs::File,
    process::exit,
    sync::{Arc, Mutex},
    thread::{self, JoinHandle},
};

use crate::status_bar::{show_image_information, show_thread_information};
use basic::{
    clamp, random_double_unit,
    ray::Ray,
    vec3::{Color, Vec3},
    INFINITY,
};
use camera::Camera;
use console::style;
use hittable::{hittable_list::HittableList, Hittable};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use pdf::{
    hittable_pdf::HittablePDF, lightable_list::Lightable, lightable_list::LightableList,
    mixture_pdf::MixturePDF, PDF,
};
use scenes::test_scene::test_scene;

mod basic;
mod bvh;
mod camera;
mod hittable;
mod material;
mod obj_loader;
mod pdf;
mod scenes;
mod status_bar;
mod texture;

// Image
const ASPECT_RATIO: f64 = 1.0;
const IMAGE_WIDTH: u32 = 1200;
const IMAGE_HEIGHT: u32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as u32;
const SAMPLES_PER_PIXEL: u32 = 2000;
const MAX_DEPTH: i32 = 100;

// Threads
const THREAD_NUM: u32 = 20;
const LINES_PER_ISSUE: u32 = 1;

fn ray_color(
    r: &Ray,
    background: &Color,
    world: &dyn Hittable,
    lights: &dyn Lightable,
    depth: i32,
) -> Color {
    let mut rec = None;

    // exceed the ray bounce limit, no more light is gathered
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    // ray hits nothing, return the background color
    if !world.hit(r, 0.001, INFINITY, &mut rec) {
        return *background;
    }

    let mut srec = None;

    let rec_data = if let Some(data) = rec {
        data
    } else {
        panic!("No hit record");
    };

    let emitted = rec_data
        .mat_ptr
        .emitted(&r, &rec_data, rec_data.u, rec_data.v, &rec_data.p);

    if !rec_data.mat_ptr.scatter(r, &rec_data, &mut srec) {
        return emitted;
    }

    let srec_data = if let Some(data) = srec {
        data
    } else {
        panic!("No scatter record");
    };

    if srec_data.is_specular {
        return srec_data.attenuation
            * ray_color(
                &srec_data.specular_ray,
                background,
                world,
                lights,
                depth - 1,
            );
    }

    let light_pdf = Box::new(HittablePDF {
        o: rec_data.p,
        ptr: lights,
    });
    let mixed_pdf = MixturePDF::new(
        light_pdf,
        if let Some(data) = srec_data.pdf_func {
            data
        } else {
            panic!("No pdf function");
        },
    );

    let scattered = Ray {
        orig: rec_data.p,
        dir: mixed_pdf.generate(),
        tm: r.tm,
    };

    let pdf_val = mixed_pdf.value(&scattered.direction());

    emitted
        + srec_data.attenuation
            * rec_data.mat_ptr.scattering_pdf(r, &rec_data, &scattered)
            * ray_color(&scattered, background, world, lights, depth - 1)
            / pdf_val
}

fn write_color(pixel: &mut Rgb<u8>, pixel_colors: &Color) {
    let mut r = pixel_colors.0 / (SAMPLES_PER_PIXEL as f64);
    let mut g = pixel_colors.1 / (SAMPLES_PER_PIXEL as f64);
    let mut b = pixel_colors.2 / (SAMPLES_PER_PIXEL as f64);

    // replace NaN with zero
    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // Gamma-correct for gamma=2.0
    r = r.sqrt();
    g = g.sqrt();
    b = b.sqrt();

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
    lights: LightableList,
    background: Color,
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
                        pixel_colors += ray_color(&r, &background, &world, &lights, MAX_DEPTH);
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

#[allow(unused_variables)]
fn world_generator(
    background: &mut Color,
    lookfrom: &mut Vec3,
    lookat: &mut Vec3,
    vfov: &mut f64,
    aperture: &mut f64,
) -> (HittableList, LightableList) {
    // aspect_ratio = 1.0
    // image_width = 600
    // samples_per_pixel = 200
    *background = Color::new(0.0, 0.0, 0.0);
    *lookfrom = Vec3(278.0, 278.0, -800.0);
    *lookat = Vec3(278.0, 278.0, 0.0);
    *vfov = 40.0;
    test_scene()
}

fn main() {
    // Output Path
    let path = "output/test.jpg";

    // Camera
    let mut background = Color::new(0.0, 0.0, 0.0);
    let mut lookfrom: Vec3 = Vec3::default();
    let mut lookat: Vec3 = Vec3::default();
    let mut vfov: f64 = 40.0;
    let mut aperture = 0.0;

    // Camera
    let vup: Vec3 = Vec3(0.0, 1.0, 0.0);
    let dist_to_focus: f64 = 10.0;

    // Show the Image Information
    show_image_information(path);

    // Threads
    let mut thread_list = Vec::<_>::new();
    let line_pool = Arc::new(Mutex::new(0_u32));

    // Threads: progress bar
    let multiprogress = Arc::new(MultiProgress::new());
    multiprogress.set_move_cursor(true);

    // Show the Threads Information
    show_thread_information();

    /*
    let (world, lights) = world_generator(
        &mut background,
        &mut lookfrom,
        &mut lookat,
        &mut vfov,
        &mut aperture,
    );
    exit(0);
    */

    // Multi-Thread
    for _id in 0..THREAD_NUM {
        let (world, lights) = world_generator(
            &mut background,
            &mut lookfrom,
            &mut lookat,
            &mut vfov,
            &mut aperture,
        );
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

        thread_list.push(create_thread(
            line_pool.clone(),
            world,
            lights,
            background,
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
