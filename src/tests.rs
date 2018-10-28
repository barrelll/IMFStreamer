#![cfg(test)]

#[test]
fn file_stream() {
    use std::{fs::File, path::Path};
    use MediaStreamTree;
    let cargoman = env!("CARGO_MANIFEST_DIR");
    let path = Path::new(cargoman)
        .join("..")
        .join("..")
        .join("download.tsi.telecom-paristech.fr")
        .join("gpac")
        .join("MPEG")
        .join("ISOBMFF-Conformance")
        .join("isobmff")
        .join("01_simple.mp4");
    let mut handle = File::open(path).expect("Error opening file");
    let node = handle.searchtree_stype::<super::iso_p12::Ftyp>("ftyp");
    println!("Node {:?}", node);
}
