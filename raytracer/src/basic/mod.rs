pub mod onb;
pub mod ray;
pub mod vec3;

// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::Rng;

// Utility Functions
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double_unit() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double_unit()
}

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
