use std::fmt;

pub struct Sprite {
    pub pixels: Vec<Vec<Pixel>>
}

impl Sprite {
    pub fn width(&self) -> u32 {
        return self.pixels.len() as u32;
    }

    pub fn height(&self) -> u32 {
        return self.pixels[0].len() as u32;
    }
}

#[derive(Clone)]
pub enum Pixel {
    Black, Gray, LightGray, White,
}

impl Pixel {
    pub fn from_raw(raw: [u8; 4]) -> Pixel {
        let [r, g, b, _] = raw;
        let c = (r as u32) << 0 | (g as u32) << 8 | (b as u32) << 16;
        return match c {
            0x000000 => Pixel::Black,
            0x555555 => Pixel::Gray,
            0xAAAAAA => Pixel::LightGray,
            0xFFFFFF => Pixel::White,
            _ => panic!("Unexpected pixel {}", c)
        }
    }

    pub fn to_binary(&self) -> u8 {
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
