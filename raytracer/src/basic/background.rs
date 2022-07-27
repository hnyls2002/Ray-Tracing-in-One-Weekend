use std::sync::Arc;

use crate::{
    texture::{image_texture::ImageTexture, Texture},
    IMAGE_HEIGHT, IMAGE_WIDTH,
};

use super::vec3::Vec3;

#[derive(Clone)]
pub struct Background {
    pub img: Arc<ImageTexture>,
}

impl Background {
    pub fn value(&self, i: u32, j: u32) -> Vec3 {
        self.img.value(
            i as f64 / IMAGE_WIDTH as f64,
            j as f64 / IMAGE_HEIGHT as f64,
            &Vec3::default(),
        ) * 0.3
    }
}
