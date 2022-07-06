use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use std::{fs::File, process::exit};

mod vec3;

use vec3::Color;

/*fn write_color(pixel_color: &Color) {
    println!(
        "{} {} {}",
        (pixel_color.0 * 255.999) as i32,
        (pixel_color.1 * 255.999) as i32,
        (pixel_color.2 * 255.999) as i32
    );
}*/

fn main() {
    let path = "output/image1.jpg";

    let width = 256;
    let height = 256;
    let quality = 60;
    let mut img: RgbImage = ImageBuffer::new(width, height);

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((height * width) as u64)
    };
    progress.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] [{pos}/{len}] ({eta})")
        .progress_chars("#>-"));

    for j in (0..height).rev() {
        for i in 0..width {
            let pixel = img.get_pixel_mut(i, j);

            let r: f64 = (i as f64) / ((width - 1) as f64);
            let g: f64 = (j as f64) / ((height - 1) as f64);
            let b: f64 = 0.25;
            let pixel_color = Color::new(r, g, b);
            *pixel = image::Rgb(pixel_color.to_array());
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
