pub mod ray;
pub mod vec3;

// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

use rand::Rng;

// Utility Functions
#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

#[allow(dead_code)]
pub fn random_double_unit() -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen::<f64>()
}

#[allow(dead_code)]
pub fn random_double(min: f64, max: f64) -> f64 {
    min + (max - min) * random_double_unit()
}

#[allow(dead_code)]
pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        min
    } else if x > max {
        max
    } else {
        x
    }
}
