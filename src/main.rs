extern crate image;
use image::{RgbaImage, Rgba, ImageBuffer};
pub mod chopper;
pub mod pixel_container;
use pixel_container::RgbaPixelContainer;

use std::env;
use std::fmt;

enum Pixel {
    Black, Gray, LightGray, White,
}

impl Pixel {
    fn to_binary(&self) -> u8 {
        return match self {
            Pixel::White => 0b00,
            Pixel::LightGray => 0b01,
            Pixel::Gray => 0b10,
            Pixel::Black => 0b11,
        }
    }
}

impl fmt::Display for Pixel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Pixel::Black => write!(f, "Black"),
            Pixel::Gray => write!(f, "Gray"),
            Pixel::LightGray => write!(f, "LightGray"),
            Pixel::White => write!(f, "White"),
        }
    }
}

struct SourceGenerator { }

fn load_pixels(image: RgbaImage) -> RgbaPixelContainer {
    let mut pixels = Vec::new();
    let width = image.width();
    let height = image.height();
    let original_pixels = image.into_raw();
    for x in 0..width {
        let mut horizontal_pixels = Vec::new();
        for y in 0..height {
            let index = (x + y * width) as usize;
            let pixel = original_pixels[index];
            pixel
            horizontal_pixels.push(pixel);
            pixels.push(horizontal_pixels); 
        }
    }
    return RgbaPixelContainer { pixels: pixels }
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

fn convert_to_pixel(data: [u8; 4]) -> Pixel {
    let [r, g, b, _] = data;
    let c = (r as u32) << 0 | (g as u32) << 8 | (b as u32) << 16;
    return match c {
        0x000000 => Pixel::Black,
        0x555555 => Pixel::Gray,
        0xAAAAAA => Pixel::LightGray,
        0xFFFFFF => Pixel::White,
        _ => panic!("Unexpected pixel {}", c)
    };
}

fn squash(chunk: &[Pixel]) -> [u8; 2] {
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
    let mut loaded_image = image::open(&img_path).unwrap().to_rgba();
    let width = loaded_image.width() as u8;
    let height = loaded_image.height() as u8;
    let container = load_pixels(loaded_image);
    let chopped = chopper.chop(width, height, &container);
    for sub_pixels in chopped {
        let img: RgbaImage = RgbaImage::from_vec(8, 8, sub_pixels).unwrap();
        let converted = img.pixels()
            .map(|p| convert_to_pixel(p.data))
            .collect::<Vec<Pixel>>();
        let length = converted.len();
        let mut generated: Vec<u8> = Vec::new();
        // TODO padding
        for i in 0..length/8 {
            let start = i * 8;
            let end = (i + 1) * 8;
            let chunk: &[Pixel] = &converted[start..end];
            let squashed = squash(chunk).to_vec();
            generated = [generated, squashed].concat();
        }
        let generator = SourceGenerator { };
        let header = generator.generate(generated);
        println!("{}", header);
    }
}
