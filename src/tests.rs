#![cfg(test)]

#[test]
fn file_stream() {
    use byteorder::{BigEndian, ReadBytesExt};
    use std::{
        fs::File,
        io::{Seek, SeekFrom, Read, Cursor},
        path::Path,
        str::from_utf8,
    };
    let cargoman = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(cargoman)
        .join("..")
        .join("..")
        .join("download.tsi.telecom-paristech.fr")
        .join("gpac")
        .join("MPEG")
        .join("ISOBMFF-Conformance")
        .join("isobmff")
        .join("14_large.mp4");
    let mut handle = File::open(path).expect("Error opening file");
    let mut buf: [u8; 4] = [0; 4];
    handle.read_exact(&mut buf);
    let cursor_s = 0;
    let cursor_e = Cursor::new(buf).read_u32::<BigEndian>().unwrap();
    let cursor = handle.seek(SeekFrom::Start(size as u64)).expect("Seeking error?");
    println!("{:?} {}", size, cursor);
}
