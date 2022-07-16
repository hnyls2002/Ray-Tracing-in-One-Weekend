use image::GenericImageView;

use crate::basic::{
    clamp,
    vec3::{Color, Point3},
};

use super::Texture;

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
