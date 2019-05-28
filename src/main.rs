extern crate image;

use std::env;
use std::fmt;

enum Pixel {
    Black, Gray, LightGray, White,
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

fn convert_to_pixel(data: [u8; 3]) -> Pixel {
    let r = data[0];
    let g = data[1];
    let b = data[2];
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

    for p in img.pixels() {
        let pixel = convert_to_pixel(p.data);
        // println!("{}", pixel);
    }
}
