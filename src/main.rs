pub mod sprite;
pub mod converter;
pub mod source_generator;
pub mod utils;
pub mod chopper;
use source_generator::SourceGenerator;
use converter::Converter;
use std::path;

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
    println!("Converting {}...", img_path);

    let converter = Converter { };

    let all_headers = converter.load(&img_path);
    
    let path = path::Path::new(&img_path);
    let variable_name = path.file_stem().unwrap();
    let generator = SourceGenerator { };
    let header = generator.generate(all_headers, &variable_name.to_str().unwrap());
    println!("{}", header);
}
