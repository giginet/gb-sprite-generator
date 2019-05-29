extern crate image;

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

struct SourceGenerator {
}

impl SourceGenerator {
    fn generate(pixels: Vec<Pixel>) -> String {
        let joined = pixels
            .into_iter()
            .map(|pixel| pixel.to_binary().to_string())
            .collect::<Vec<String>>()
            .join(", ");
        return format!("unsigned char sprites[] = {{{}}};", joined);
    }
}

fn convert_to_pixel(data: [u8; 3]) -> Pixel {
    let [r, g, b] = data;
    let c = (r as u32) << 0 | (g as u32) << 8 | (b as u32) << 16;
    return match c {
        0x000000 => Pixel::Black,
        0x555555 => Pixel::Gray,
        0xAAAAAA => Pixel::LightGray,
        0xFFFFFF => Pixel::White,
        _ => panic!("Unexpected pixel {}", c)
    };
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

    let img = image::open(&img_path).unwrap();
    let img = img.to_rgb();

    let converted = img.pixels()
        .map(|p| convert_to_pixel(p.data))
        .collect::<Vec<Pixel>>();
    let generator = SourceGenerator { };
    let header = SourceGenerator::generate(converted);
    println!("{}", header);
}
