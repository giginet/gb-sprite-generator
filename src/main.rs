pub mod sprite;
pub mod converter;
pub mod source_generator;
pub mod utils;
pub mod chopper;
use source_generator::SourceGenerator;
use converter::Converter;

use std::fs;
use std::path;
use std::env;

fn main() {
    let mut args = env::args();
    args.next();

    let dir_path = match args.next() {
        None => {
            eprintln!("Usage: cargo run {{directory_path}}");
            return;
        },
        Some(s) => s
    };


    let base_dir = path::Path::new(&dir_path);
    let entries = fs::read_dir(&dir_path).unwrap();
    let images = entries
        .map({ |entry| entry.ok().unwrap().file_name().into_string().ok().unwrap() } )
        .filter({ |path| path.ends_with(".png") });

    for image_name in images {
        eprintln!("Converting {}...", image_name);

        let converter = Converter { };
        let img_path = base_dir.join(image_name);
        let path_string = img_path.to_str().unwrap();
        let all_headers = converter.load(&path_string.to_string());

        let variable_name = img_path.file_stem().unwrap();
        let generator = SourceGenerator { };
        let header = generator.generate(all_headers, &variable_name.to_str().unwrap());
        println!("{}", header);
    }
}
