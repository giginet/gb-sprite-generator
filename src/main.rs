extern crate image;
use image::{RgbaImage};
pub mod chopper;
pub mod sprite;
use sprite::{Sprite, Pixel};

use std::env;

struct SourceGenerator { }

fn load_sprite(image: &RgbaImage) -> Sprite {
    let mut pixels = Vec::new();
    let width = image.width();
    let height = image.height();
    for x in 0..width {
        let horizontal_pixels = (0..height).map( |y| {
            let raw_pixel = image.get_pixel(x, y);
            return Pixel::from_raw(raw_pixel.data);
        })
        .collect::<Vec<Pixel>>();
        pixels.push(horizontal_pixels); 
    }
    return Sprite { pixels: pixels };
}

impl SourceGenerator {
    fn generate(&self, squashed: Vec<u8>) -> String {
        let joined = squashed
            .into_iter()
            .map(|value| value.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("unsigned char sprites[] = {{{}}};", joined);
    }
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

fn main() {
    let mut args = env::args();
    args.next();

    let img_path = match args.next() {
        None => {
            eprintln!("Error");
            return;
        },
        Some(s) => s
    };
    println!("Converting {}...", img_path);
    
    let chopper = chopper::Chopper { };

    // TODO
    let loaded_image = image::open(&img_path).unwrap().to_rgba();
    let original_sprite = load_sprite(&loaded_image);
    let chopped = chopper.chop(&original_sprite);
    for sprite in chopped {
        let length = sprite.width() * sprite.height() as u8;
        let mut converted = Vec::new();
        for x in 0..sprite.width() {
            for y in 0..sprite.height() {
                converted.push(&sprite.pixels[x as usize][y as usize]);
            }
        }
        let mut generated: Vec<u8> = Vec::new();
        // TODO padding
        for i in 0..length / 8 {
            let start = (i * 8) as usize;
            let end = ((i + 1) * 8) as usize;
            let chunk = converted[start..end].to_vec();
            let squashed = squash(&chunk).to_vec();
            generated = [generated, squashed].concat();
        }
        let generator = SourceGenerator { };
        println!("sprite_size = {}x{}", sprite.width(), sprite.height());

        let header = generator.generate(generated);
        println!("{}", header);
    }
}
