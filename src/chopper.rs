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
                let mut new_pixels = Vec::new();
                for x in (h * 8)..(h + 1) * 8 {
                    let mut new_vertical_pixels = Vec::new();
                    for y in (v * 8)..(v + 1) * 8 {
                        let pixel = pixels[x as usize][y as usize].clone();
                        new_vertical_pixels.push(pixel);
                    }
                    new_pixels.push(new_vertical_pixels);
                    println!("x = {}", x);
                }
                let new_sprite = Sprite { pixels: new_pixels.to_vec() };
                sprites.push(new_sprite);
            }
        }
        return sprites;
    }
}
