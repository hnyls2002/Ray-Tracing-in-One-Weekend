use console::style;

use crate::{IMAGE_HEIGHT, IMAGE_WIDTH, MAX_DEPTH, SAMPLES_PER_PIXEL, THREAD_NUM};

pub fn show_image_information(path: &str) {
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

pub fn show_thread_information() {
    println!(
        "🚀 {} {} {}",
        style("Rendering with").green(),
        style(THREAD_NUM.to_string()).yellow(),
        style("Threads...").green(),
    );
}
