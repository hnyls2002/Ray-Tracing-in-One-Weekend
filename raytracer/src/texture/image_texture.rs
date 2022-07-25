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

impl ImageTexture {
    fn bilinear_interpolation(&self, u: f64, v: f64) -> Color {
        let width = self.data.dimensions().0;
        let height = self.data.dimensions().1;
        let color_scale = 1.0 / 255.0;
        if u.floor() == width as f64 - 1.0 || v.floor() == height as f64 - 1.0 {
            let mut i = u as u32;
            let mut j = v as u32;
            i = i.min(width - 1);
            j = j.min(height - 1);
            let col = self.data.get_pixel(i, j).0;
            return Color::new(col[0] as f64, col[1] as f64, col[2] as f64) * color_scale;
        }
        let p = [u.floor(), u.floor() + 1.0];
        let q = [v.floor(), v.floor() + 1.0];
        let mut ret_color = Color::new(0.0, 0.0, 0.0);
        for i in p.iter() {
            for j in q.iter() {
                let col = self.data.get_pixel(*i as u32, *j as u32).0;
                ret_color += Color::new(col[0] as f64, col[1] as f64, col[2] as f64)
                    * color_scale
                    * ((u - i) * (v - j)).abs();
            }
        }
        ret_color
    }
}

impl Texture for ImageTexture {
    #[allow(clippy::many_single_char_names)]
    fn value(&self, mut u: f64, mut v: f64, _p: &Point3) -> Color {
        // no texture data, return solid cyan as a debugging aid.
        if self.data.dimensions().0 == 0 || !self.data.dimensions().1 == 0 {
            return Color::new(0.0, 1.0, 1.0);
        }

        let width = self.data.dimensions().0;
        let height = self.data.dimensions().1;
        u = clamp(u, 0.0, 1.0) * width as f64;
        v = (1.0 - clamp(v, 0.0, 1.0)) * height as f64;
        self.bilinear_interpolation(u, v)
    }
}
