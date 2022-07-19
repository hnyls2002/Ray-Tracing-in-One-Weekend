use crate::basic::vec3::{Color, Point3};

use super::{solid_color_texture::SolidColor, Texture};

#[derive(Clone, Copy)]
pub struct CheckerTexture<TT1, TT2>
where
    TT1: Texture,
    TT2: Texture,
{
    pub odd: TT1,
    pub even: TT2,
}

impl<TT1, TT2> CheckerTexture<TT1, TT2>
where
    TT1: Texture,
    TT2: Texture,
{
    #[allow(dead_code)]
    pub fn new(odd: TT1, even: TT2) -> CheckerTexture<TT1, TT2> {
        CheckerTexture { odd, even }
    }
    pub fn new_by_color(c1: Color, c2: Color) -> CheckerTexture<SolidColor, SolidColor> {
        CheckerTexture {
            odd: SolidColor::new_from_color(c1),
            even: SolidColor::new_from_color(c2),
        }
    }
}

impl<TT1, TT2> Texture for CheckerTexture<TT1, TT2>
where
    TT1: Texture,
    TT2: Texture,
{
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        let sines = (10.0 * p.0).sin() * (10.0 * p.1).sin() * (10.0 * p.2).sin();
        if sines < 0.0 {
            self.odd.value(u, v, p)
        } else {
            self.even.value(u, v, p)
        }
    }
}
