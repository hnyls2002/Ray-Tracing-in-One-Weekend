use std::sync::Arc;

use crate::basic::vec3::{Color, Point3};

use super::{solid_color_texture::SolidColor, Texture};

pub struct CheckerTexture {
    pub odd: Arc<dyn Texture>,
    pub even: Arc<dyn Texture>,
}

impl CheckerTexture {
    #[allow(dead_code)]
    pub fn new(even: Arc<dyn Texture>, odd: Arc<dyn Texture>) -> CheckerTexture {
        CheckerTexture { odd, even }
    }
    pub fn new_by_color(c1: Color, c2: Color) -> CheckerTexture {
        CheckerTexture {
            odd: Arc::new(SolidColor::new(c1)),
            even: Arc::new(SolidColor::new(c2)),
        }
    }
}

impl Texture for CheckerTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
