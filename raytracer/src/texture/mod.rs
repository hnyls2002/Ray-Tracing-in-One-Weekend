use crate::basic::vec3::{Color, Point3};

pub mod checker_texture;
pub mod image_texture;
pub mod noise_texture;
pub mod solid_color_texture;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}
