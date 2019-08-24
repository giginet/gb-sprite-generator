extern crate image;
use image::{GenericImageView, SubImage, imageops, ImageBuffer};

pub struct Chopper { }

impl Chopper {
    pub fn chop<'a, 'b, I: GenericImageView>(&'a self, image: &'b mut I) -> Vec<SubImage<&'b mut I>> {
        // TODO image size must be 16x16
        let width = image.width();
        let height = image.height();
        let vertical_count = height / 8;
        let horizontal_count = width / 8;
        let mut cropped = Vec::new();
        for x in 0..horizontal_count {
            for y in 0..vertical_count {
                let cloned = ImageBuffer::new(width, height);
                let piece = imageops::crop(image, x * 8, y * 8, 8, 8);
                cropped.push(piece);
            }
        }
        return cropped;
    }
}
