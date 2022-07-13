mod perlin;

use std::sync::Arc;

use crate::rtweekend::vec3::{Color, Point3, Vec3};

use self::perlin::Perlin;

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color;
}

pub struct SolidColor {
    color_value: Color,
}

impl SolidColor {
    pub fn new(c: Color) -> SolidColor {
        SolidColor { color_value: c }
    }
    #[allow(dead_code)]
    pub fn new_by_rgb(r: f64, g: f64, b: f64) -> SolidColor {
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

#[derive(Default)]
pub struct NoiseTexture {
    noise: Perlin,
    scale: f64,
}

impl NoiseTexture {
    pub fn new_by_sc(sc: f64) -> NoiseTexture {
        NoiseTexture {
            noise: Default::default(),
            scale: sc,
        }
    }
}
impl Texture for NoiseTexture {
    fn value(&self, _u: f64, _v: f64, p: &Point3) -> Color {
        //Vec3(1.0, 1.0, 1.0) * (1.0 + self.noise.noise(&(*p * self.scale))) * 0.5
        Vec3(1.0, 1.0, 1.0) * self.noise.turb(&(*p * self.scale), 7)
    }
}
