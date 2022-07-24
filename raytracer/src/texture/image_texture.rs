use image::{DynamicImage, GenericImageView};

use crate::basic::{
    clamp,
    vec3::{Color, Point3},
};

use super::Texture;

pub struct ImageTexture {
    data: DynamicImage,
}

impl ImageTexture {
    pub fn load_image_file(filename: &str) -> ImageTexture {
        ImageTexture {
            data: image::open(filename).unwrap(),
        }
    }
}

impl Texture for ImageTexture {
    #[allow(clippy::many_single_char_names)]
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        // no texture data, return solid cyan as a debugging aid.
        if self.data.dimensions().0 == 0 || !self.data.dimensions().1 == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        u = clamp(u, 0.0, 1.0);
        v = 1.0 - clamp(v, 0.0, 1.0);
        let width = self.data.dimensions().0;
        let height = self.data.dimensions().1;
        let mut i = (u * width as f64) as u32;
        let mut j = (v * height as f64) as u32;
        i = i.min(width - 1);
        j = j.min(height - 1);
        let color_scale = 1.0 / 255.0;
        let col = self.data.get_pixel(i, j).0;
        Color::new(col[0] as f64, col[1] as f64, col[2] as f64) * color_scale
    }
}
