use std::sync::Arc;

use crate::basic::vec3::{Color, Point3};

use super::{image_texture::ImageTexture, Texture};

#[derive(Clone)]
pub struct ObjTexture {
    pub ptr: Arc<ImageTexture>,
}

impl Texture for ObjTexture {
    fn value(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.ptr.value(u, v, p)
    }
}
