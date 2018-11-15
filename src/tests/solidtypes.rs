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
    assert!(node.is_ok())
}

#[test]
fn read_visual_objects() {
    let mut handle = handle("a1-foreman-QCIF.mp4");
    let node = handle.searchtree_stype::<::iso_p12::Stsd>("moov.trak1.mdia.minf.stbl.stsd");
    let sample_entries = node.unwrap().sample_entries;
    for sample_entry in sample_entries {
        if let Some(mp4v) = sample_entry.downcast_ref::<::sample_entries::MP4VisualSampleEntry>() {
            let esds_box = &mp4v.esds_box.as_ref().unwrap();
            let esdescr = &esds_box.od;
            let esdescr_descriptors = &esdescr.as_ref().unwrap().descriptors;
            for descr in esdescr_descriptors {
                if let Some(::base_descriptors::DescrBaseTags::DecoderConfigDescrTag) = descr.tag()
                {
                    println!(
                        "{:?}",
                        descr.downcast_ref::<::base_descriptors::DecoderConfigDescriptor>()
                    );
                }
            }
        }
    }
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
