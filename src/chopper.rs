use crate::sprite::{Sprite, Pixel};
use crate::utils::next_multiple;
//
pub struct Chopper { }

impl Chopper {
    fn pad_sprite(sprite: &Sprite) -> Sprite {
        let wanted_horizontal_len = next_multiple(8, sprite.width()) as usize;
        let wanted_vertical_len = next_multiple(8, sprite.height()) as usize; 
        let mut new_pixels = sprite.pixels.clone();
        let default = (0..wanted_vertical_len).map({ |_| Pixel::White }).collect();
        new_pixels.resize(wanted_horizontal_len, default);
        for x in 0..sprite.width() {
            let vertical = &mut new_pixels[x as usize];
            vertical.resize(wanted_vertical_len, Pixel::White);
        }

        return Sprite { pixels: new_pixels }
    }
    pub fn chop<'me, 'generated>(
        &'me self, 
        sprite: &'generated Sprite) -> Vec<Sprite> { 
        let sprite = Chopper::pad_sprite(&sprite);
        let pixels = sprite.pixels.clone();
        let vertical_count = (sprite.height() as f64 / 8.0).ceil() as u32;
        let horizontal_count = (sprite.width() as f64 / 8.0).ceil() as u32;

        let mut sprites = Vec::new();
        for v in 0..vertical_count {
            for h in 0..horizontal_count {
                let mut new_pixels = Vec::new();
                for x in (h * 8)..((h + 1) * 8) {
                    let mut new_vertical_pixels = Vec::new();
                    for y in (v * 8)..((v + 1) * 8) {
                        let pixel = pixels[x as usize][y as usize].clone();
                        new_vertical_pixels.push(pixel);
                    }
                    new_pixels.push(new_vertical_pixels);
                }
                let new_sprite = Sprite { pixels: new_pixels.to_vec() };
                sprites.push(new_sprite);
            }
        }
        return sprites;
    }
}
