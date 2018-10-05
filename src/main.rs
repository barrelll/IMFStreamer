extern crate byteorder;
extern crate glob;
extern crate minifb;

mod atom_tree;
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
        .join("MOV_0044.mp4");
    let file_buffer = read_file_into_u8(&path).unwrap();
    let mpeg = Mpeg::new(&file_buffer);
    mpeg.major_brand();
    mpeg.minor_brands();
    /*Renderer::new("Test render mp4", 640, 360).run();*/
}
