use crate::basic::vec3::{Color, Point3};

use super::Texture;

#[derive(Clone, Copy)]
pub struct SolidColor {
    pub color_value: Color,
}

impl SolidColor {
    pub fn new_from_color(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
    #[allow(dead_code)]
    pub fn new_from_rgb(r: f64, g: f64, b: f64) -> SolidColor {
        SolidColor {
            color_value: Color::new(r, g, b),
        }
    }
}

impl Texture for SolidColor {
    fn value(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        self.color_value
    }
}
