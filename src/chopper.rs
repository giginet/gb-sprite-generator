extern crate image;
use image::{GenericImageView, SubImage, imageops, RgbaImage, DynamicImage, Rgba};
use crate::sprite::{Sprite, Pixel};

pub struct Chopper { }

impl Chopper {
    pub fn chop<'me, 'generated>(
        &'me self, 
        sprite: &'generated Sprite) -> Vec<Sprite> { 
        let pixels = &sprite.pixels;
        // TODO image size must be 16x16
        let vertical_count = sprite.height() / 8;
        let horizontal_count = sprite.width() / 8;
        let mut sprites = Vec::new();
        for v in 0..horizontal_count {
            for h in 0..vertical_count {
                let x = h * 8;
                let y = v * 8;
                let new_pixel = Pixel::Black;
                let new_sprite = Sprite { pixels: vec![vec![new_pixel]] };
                sprites.push(new_sprite);
            }
        }
        return sprites;
    }
}
