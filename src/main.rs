extern crate byteorder;
extern crate glob;
extern crate minifb;

mod atoms;
mod mpeg;
mod renderer;

use std::fs::File;
use std::io::{BufReader, Read, Result};
use std::path::Path;

fn read_file_into_u8(f_path: &Path) -> Result<Vec<u8>> {
    let file = File::open(f_path)?;
    let mut buf_reader = BufReader::new(file);
    let mut contents = Vec::new();
    buf_reader.read_to_end(&mut contents)?;
    Ok(contents)
}

fn main() {
    use mpeg::*;
    //use renderer::Renderer;

    let path = Path::new("..")
        .join("assets")
        .join("MP4s")
        .join("file_example_MP4_1920_18MG.mp4");
    let file_buffer = read_file_into_u8(&path).unwrap();
    let mpeg = Mpeg::new(&file_buffer);
    //    println!("{:?}", mpeg);
    println!("{:?}", mpeg.find_iods());
    println!("{:?}", mpeg.major_brand());
    println!("{:?}", mpeg.minor_version());
    println!("{:?}", mpeg.minor_brands());
    //    Renderer::new("Test render mp4", 640, 360).run();
}
