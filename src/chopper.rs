extern crate image;
use image::{GenericImageView, SubImage, imageops, RgbaImage, DynamicImage, Rgba};
use crate::pixel_container::RgbaPixelContainer;

pub struct Chopper { }

impl Chopper {
    pub fn chop<'me, 'generated>(
        &'me self, 
        width: u8, 
        height: u8, 
        container: &'generated RgbaPixelContainer) -> Vec<Vec<u8>> { 
        let pixels = &container.pixels;
        // TODO image size must be 16x16
        let vertical_count = height / 8;
        let horizontal_count = width / 8;
        let mut chopped = Vec::new();
        for v in 0..horizontal_count {
            for h in 0..vertical_count {
                let x = h * 8;
                let y = v * 8;
        //         let piece: SubImage<&'b mut RgbaImage> = imageops::crop(&mut image, x * 8, y * 8, 8, 8);
        //         cropped.push(piece);
                let new_raw = vec![1, 1, 1, 1];
                chopped.push(new_raw);
            }
        }
        return chopped;
    }
}
