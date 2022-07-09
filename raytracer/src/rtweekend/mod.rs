pub mod ray;
pub mod vec3;

// Constants
pub use std::f64::consts::PI;
pub use std::f64::INFINITY;

// Utility Functions

#[allow(dead_code)]
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}
