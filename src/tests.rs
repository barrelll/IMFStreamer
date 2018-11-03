#![cfg(test)]
use std::{fs::File, path::Path, path::PathBuf};
use MediaStreamTree;

fn path(filename: &str) -> PathBuf {
    /*D:\download.tsi.telecom-paristech.fr\gpac\MPEG\ISOBMFF-Conformance\isobmff*/
    let path = Path::new("d:\\")
        .join("download.tsi.telecom-paristech.fr")
        .join("gpac")
        .join("MPEG")
        .join("ISOBMFF-Conformance")
        .join("isobmff")
        .join(filename);
    path.to_path_buf()
}

fn handle(filename: &str) -> File {
    File::open(path(filename)).expect("Error opening file")
}

#[test]
fn file_stream() {
    let mut handle = handle("a1-foreman-QCIF.mp4");
    let node = handle.searchtree_stype::<super::iso_p12::Ftyp>("ftyp");
    println!("Node {:?}", node);
}

#[test]
fn iods() {
    let mut handle = handle("01_simple.mp4");
    let node = handle.searchtree_stype::<super::iso_p14::Iods>("moov.iods");
    println!("Node {:?}", node);
}
