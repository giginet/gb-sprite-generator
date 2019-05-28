extern crate image;

use std::env;

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
    println!("{}", img_path);

    let img = image::open(&img_path).unwrap();
}
