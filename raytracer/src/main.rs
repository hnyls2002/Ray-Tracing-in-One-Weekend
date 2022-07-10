use console::style;
use hittablelist::hittable::{HitRecord, Hittable};
use image::{ImageBuffer, Rgb, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, process::exit, sync::Arc};

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
        rtweekend::{random_double_unit, vec3::Point3},
        Camera,
    },
    hittablelist::HittableList,
    material::{Dielectric, Lambertian, Metal},
    sphere::Sphere,
};

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

fn write_color(pixel: &mut Rgb<u8>, pixel_colors: &Color, samples_per_pixel: i32) {
    let r = pixel_colors.0 / (samples_per_pixel as f64);
    let g = pixel_colors.1 / (samples_per_pixel as f64);
    let b = pixel_colors.2 / (samples_per_pixel as f64);

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

fn main() {
    let path = "output/image15.jpg";

    // Image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;

    // World
    let mut world = HittableList { objects: vec![] };

    let material_ground = Arc::new(Lambertian {
        albedo: Color::new(0.8, 0.8, 0.0),
    });
    let material_center = Arc::new(Lambertian {
        albedo: Color::new(0.1, 0.2, 0.5),
    });
    let material_left = Arc::new(Dielectric { ir: 1.5 });
    let material_right = Arc::new(Metal {
        albedo: Color::new(0.8, 0.6, 0.2),
        fuzz: 0.0,
    });

    world.add(Box::new(Sphere {
        center: Point3::new(0.0, -100.5, -1.0),
        radius: 100.0,
        mat_ptr: Some(material_ground),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(0.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Some(material_center),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(-1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Some(material_left),
    }));
    world.add(Box::new(Sphere {
        center: Point3::new(1.0, 0.0, -1.0),
        radius: 0.5,
        mat_ptr: Some(material_right),
    }));

    // Camera
    let cam = Camera::default();

    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new(image_height as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    /*    println!("P3");
        println!("{} {}",image_width,image_height);
        println!("255");
    */

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);

            let mut pixel_colors = Color::new(0.0, 0.0, 0.0);

            for _s in 0..samples_per_pixel {
                // a bunch of rays hitting the object
                let u = (i as f64 + random_double_unit()) / (image_width - 1) as f64;
                let v = (j as f64 + random_double_unit()) / (image_height - 1) as f64;
                let r = cam.get_ray(u, v);
                pixel_colors += ray_color(&r, &world, max_depth);
            }

            write_color(pixel, &pixel_colors, samples_per_pixel);
        }
        progress.inc(1);
    }
    progress.finish();

    println!("Ouput image as \"{}\"", style(path).yellow());
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        // Err(_) => panic!("Outputting image fails."),
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
