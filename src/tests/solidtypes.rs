#![cfg(test)]
use std::{fs::File, path::Path, path::PathBuf};
use MediaStreamTree;

#[test]
fn ftyp() {
    let mut handle = handle("a1-foreman-QCIF.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Ftyp>("ftyp");
    assert!(node.is_ok())
}

#[test]
fn iods() {
    let mut handle = handle("a1-foreman-QCIF.mp4");
    let node = handle.searchtree_stype::<::iso_p14::Iods>("moov.iods");
    assert!(node.is_ok())
}

#[test]
fn esds() {
    let mut handle = handle("fragment-random-access-1+AF8-rev1.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Stsd>("moov.trak.mdia.minf.stbl.stsd");
    assert!(node.is_ok());
}

#[test]
fn stss() {
    let mut handle = handle("fragment-random-access-1+AF8-rev1.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Stss>("moov.trak.mdia.minf.stbl.stss");
    assert!(node.is_ok());
}

#[test]
fn stsc() {
    let mut handle = handle("fragment-random-access-1+AF8-rev1.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Stsc>("moov.trak.mdia.minf.stbl.stsc");
    assert!(node.is_ok());
}

#[test]
fn stco() {
    let mut handle = handle("fragment-random-access-1+AF8-rev1.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Stco>("moov.trak.mdia.minf.stbl.stco");
    assert!(node.is_ok());
}

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
