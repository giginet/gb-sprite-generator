use image::RgbaImage;
use std::vec;
use crate::sprite::{Sprite, Pixel};
use crate::chopper::Chopper;

fn load_sprite(image: &RgbaImage) -> Sprite {
    let width = image.width();
    let height = image.height();

    let pixels = (0..width).map( |x| {
        return (0..height).map( |y| {
            let raw_pixel = image.get_pixel(x, y);
            return Pixel::from_raw(raw_pixel.data);
        })
        .collect::<Vec<Pixel>>();
    })
    .collect::<Vec<Vec<Pixel>>>();
    return Sprite { pixels: pixels };
}

fn squash(chunk: &Vec<&Pixel>) -> [u8; 2] {
    fn clamp(i: u8) -> u8 {
        if i > 0 {
            return 1;
        }
        return 0;
    }
    let first_bytes = chunk.iter().map(|p| clamp(p.to_binary() & 0b01) );
    let second_bytes = chunk.iter().map(|p| clamp(p.to_binary() & 0b10) );
    let first_squashed = first_bytes.fold(0, |a, b| a << 1 | b);
    let second_squashed = second_bytes.fold(0, |a, b| a << 1 | b);
    return [first_squashed, second_squashed];
}

pub struct Converter { }

impl Converter {
    pub fn load(&self, img_path: &String) -> vec::Vec<u8> {
        let chopper = Chopper { };

        let loaded_image = image::open(img_path).unwrap().to_rgba();
        let original_sprite = load_sprite(&loaded_image);
        let chopped = chopper.chop(&original_sprite);
        let headers: Vec<Vec<u8>>  = chopped.into_iter().map( |sprite| {
            let length = (sprite.width() * sprite.height()) as u32;
            let mut converted = Vec::new();
            for y in 0..sprite.height() {
                for x in 0..sprite.width() {
                    converted.push(&sprite.pixels[x as usize][y as usize]);
                }
            }

            return (0..length / 8).into_iter().enumerate().flat_map( |t| {
                let start = (t.0 * 8) as usize;
                let end = ((t.0 + 1) * 8) as usize;
                let chunk = converted[start..end].to_vec();
                let squashed = squash(&chunk).to_vec();
                return squashed;
            })
            .collect();
        })
        .collect();
        let all_headers = headers
            .into_iter()
            .flat_map( |e| { e })
            .collect();
        return all_headers;
    }
}
