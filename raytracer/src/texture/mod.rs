mod perlin;

use std::sync::Arc;

use image::GenericImageView;

use crate::rtweekend::{
    clamp,
    vec3::{Color, Point3, Vec3},
};

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
        // smoothed higer frequency
        //Vec3(1.0, 1.0, 1.0) * (1.0 + self.noise.noise(&(*p * self.scale))) * 0.5

        // turbulence
        //Vec3(1.0, 1.0, 1.0) * self.noise.turb(&(*p * self.scale), 7)

        // marbled texture
        let tmp = self.scale * p.2 + 10.0 * self.noise.turb(p, 7);
        Vec3(1.0, 1.0, 1.0) * (1.0 + tmp.sin()) * 0.5
    }
}

const BYTES_PER_PIXEL: u32 = 3;

#[derive(Default)]
pub struct ImageTexture {
    data: Vec<u8>,
    width: u32,
    height: u32,
    bytes_per_scanline: u32,
}

impl ImageTexture {
    pub fn load_image_file(filename: &str) -> ImageTexture {
        let components_per_pixel = BYTES_PER_PIXEL;
        let im = image::open(filename).unwrap();
        ImageTexture {
            data: im.to_bytes(),
            width: im.dimensions().0,
            height: im.dimensions().1,
            bytes_per_scanline: im.dimensions().0 * components_per_pixel,
        }
    }
}

impl Texture for ImageTexture {
    #[allow(clippy::many_single_char_names)]
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        // no texture data, return solid cyan as a debugging aid.
        if self.data.is_empty() {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);
        let mut i = (u * self.width as f64) as i32;
        let mut j = (v * self.height as f64) as i32;
        if i >= self.width as i32 {
            i = (self.width - 1) as i32;
        }
        if j >= self.height as i32 {
            j = (self.height - 1) as i32;
        }

        let color_scale = 1.0 / 255.0;
        let start = (j as u32 * self.bytes_per_scanline + i as u32 * BYTES_PER_PIXEL) as usize;
        Color::new(
            self.data[start] as f64,
            self.data[start + 1] as f64,
            self.data[start + 2] as f64,
        ) * color_scale
    }
}
