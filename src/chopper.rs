extern crate image;
use image::{GenericImageView, SubImage, imageops};

struct Chopper {
}

impl Chopper {
    fn chop<'a, 'b, I: GenericImageView>(&'a self, image: &'b mut I) -> Vec<SubImage<&'b mut I>> {
        let cropped = imageops::crop(image, 0, 0, 8, 8);
        return vec![cropped];
    }
}
